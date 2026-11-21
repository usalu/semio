import {expect,test} from "bun:test";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {captureOwnedCommandResultV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {runNativeBuildJobAttemptsV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️job-attempts/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import fixture from "./wasm-live-drift.fixture.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"test:wasm-live-drift",maximumElapsedMilliseconds:300000,capabilities:{}});
test("Flow-shaped compiler source drift reaches exit 75 only for a proven edit",async()=>withBuildTestInvocationV1(async invocation=>{
  const generated=join(dirname(import.meta.path),"🗑️generated");
  await mkdir(generated,{recursive:true});
  const cwd=await mkdtemp(join(generated,"wasm-drift-")),source=join(cwd,"source.rs"),program=join(dirname(import.meta.path),"wasm-live-drift.fixture.ts");
  try{
    for(const scenario of ["change","ordinary","abort"]){
      await writeFile(source,fixture.original);
      const control=createIssuedResourceControlV1(invocation,"fixture:wasm-drift-driver",fixture.budgetMilliseconds);
      const result=await captureOwnedCommandResultV1(process.execPath,[program],cwd,{control,env:{...process.env,SEMIO_WASM_DRIFT_SOURCE:source,SEMIO_WASM_DRIFT_CASE:scenario},output:{maximumBytes:65536,maximumLines:512}});
      expect(result.status,result.stderr).toBe(scenario==="change"?fixture.restartStatus:1);
      expect(result.signal).toBeNull();
      const marker=result.stderr.split("\n").find(line=>line.startsWith("fixture-compiler "))!;
      expect(marker,result.stderr).toBeTruthy();
      const launched=JSON.parse(marker.slice("fixture-compiler ".length));
      expect(launched.budget).toBe(fixture.budgetMilliseconds);
      expect(launched.argv).toEqual([
        ...fixture.expectedWrapperFlags,"--out-dir",join(cwd,"pkg"),"--out-name","flow_core","--no-pack","--",
        "--locked","--manifest-path",join(cwd,"Cargo.toml"),"--target","wasm32-unknown-unknown","--config",join(cwd,"config.toml"),"--no-default-features",
        ...fixture.features.flatMap(feature=>["--features",feature]),...fixture.selectors.flatMap(value=>["--config",value]),
      ]);
      const detail=scenario==="change"?source:scenario==="ordinary"?`fixture-WASM failed (${fixture.ordinaryCompilerStatus})`:"fixture parent cancellation";
      expect(result.stderr).toContain(detail);
      expect(result.stderr).not.toContain("cancelled its original control");
      const prefix="cargo:running-source-digest-mismatch:";
      const diagnostics=result.stderr.split("\n").filter(line=>line.startsWith("[script] {")).flatMap(line=>{
        const resource=JSON.parse(line.slice(9)).activity?.resource;
        return typeof resource==="string"&&resource.startsWith(prefix)?[JSON.parse(resource.slice(prefix.length))]:[];
      });
      const digest=(text:string)=>new Bun.CryptoHasher("sha256").update(text).digest("hex");
      expect(diagnostics).toEqual(scenario==="change"?[{version:1,path:source,expected:digest(fixture.original),observed:digest(fixture.edited)}]:[]);
      let alive:boolean;
      try{alive=process.kill(launched.pid,0);}catch{alive=false;}
      expect(alive).toBe(false);
    }
    await writeFile(source,fixture.original);
    const control=createIssuedResourceControlV1(invocation,"fixture:wasm-drift-retry",fixture.budgetMilliseconds),statuses:number[]=[],digests:string[]=[],notices:string[]=[];
    await runNativeBuildJobAttemptsV1({id:"flow-wasm",logPath:join(cwd,"job-flow-wasm.log"),signal:control.signal,check:()=>control.signal.throwIfAborted(),log:line=>notices.push(line),logLimitBytes:65536,tailLines:30,tailLineChars:512},async(capture,attempt)=>{
      const result=await captureOwnedCommandResultV1(process.execPath,[program],cwd,{control,env:{...process.env,SEMIO_WASM_DRIFT_SOURCE:source,SEMIO_WASM_DRIFT_CASE:attempt===1?"change":"stable"},output:{maximumBytes:65536,maximumLines:512}});
      for(const line of result.stderr.trimEnd().split("\n")){
        capture(line);
        if(line.startsWith("fixture-admitted "))digests.push(line.slice("fixture-admitted ".length));
      }
      expect(result.signal).toBeNull();
      statuses.push(result.status!);
      return result.status!;
    });
    expect(statuses).toEqual([fixture.restartStatus,0]);
    expect(digests).toEqual([fixture.original,fixture.edited].map(source=>new Bun.CryptoHasher("sha256").update(source).digest("hex")));
    expect(notices).toHaveLength(1);
  }finally{await rm(cwd,{recursive:true,force:true});}
}),30000);
