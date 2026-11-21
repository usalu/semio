import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {mkdir,mkdtemp,rename,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {terminateOwnedChildTree} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import {publishCargoSourceDriftV1,watchCargoSourceDriftV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/👁️drift/🟦️.ts";
import limits from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/👁️drift/🔣️.json" with {type:"json"};
import fixture from "./cargo-source-drift.fixture.json" with {type:"json"};

test("verified admitted Source drift stops and drains an owned child",async()=>{
  const generated=join(dirname(import.meta.path),"🗑️generated");
  await mkdir(generated,{recursive:true});
  const directory=await mkdtemp(join(generated,"source-drift-"));
  const source=join(directory,"input.rs"),replacement=join(directory,"replacement"),unrelated=join(directory,"unrelated");
  const digest=(source:string)=>new Bun.CryptoHasher("sha256").update(source).digest("hex");
  expect(digest(fixture.original)).toBe(fixture.expectedOriginalSha256);
  await writeFile(source,fixture.original);
  const child=spawn(process.execPath,["-e","console.log('ready'); console.error('running'); setInterval(()=>{},1000)"],{detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"]});
  let stdout="",stderr="",closed=false,observed:Readonly<{path:string;expected:string;observed:string}>|undefined;
  const diagnostics:string[]=[];
  const reporter={publish(row:{resource:string}){diagnostics.push(row.resource);}};
  child.stdout.setEncoding("utf8").on("data",chunk=>stdout+=chunk);
  child.stderr.setEncoding("utf8").on("data",chunk=>stderr+=chunk);
  const terminal=new Promise<void>(resolve=>child.once("close",()=>{closed=true;resolve();}));
  const stop=watchCargoSourceDriftV1([{path:source,digest:fixture.expectedOriginalSha256}],Date.now(),mismatch=>{
    observed=mismatch;
    void publishCargoSourceDriftV1(reporter,mismatch);
    terminateOwnedChildTree(child);
  });
  try{
    for(let attempt=0; attempt<40&&!stdout.includes("ready");attempt++)await Bun.sleep(50);
    expect(stdout).toContain("ready");
    await writeFile(source,fixture.original);
    await writeFile(unrelated,fixture.unrelatedEdit);
    await Bun.sleep(450);
    expect(closed).toBe(false);
    expect(observed).toBeUndefined();
    expect(diagnostics).toEqual([]);
    await writeFile(replacement,fixture.sameLengthEdit);
    await rename(replacement,source);
    for(let attempt=0;attempt<50&&!closed;attempt++)await Bun.sleep(50);
    expect(closed).toBe(true);
    expect(stderr).toContain("running");
    expect(observed).toEqual({path:source,expected:fixture.expectedOriginalSha256,observed:digest(fixture.sameLengthEdit)});
    expect(diagnostics).toEqual(["cargo:running-source-digest-mismatch:"+JSON.stringify({version:1,...observed})]);
    await publishCargoSourceDriftV1(reporter,{...observed!,observed:fixture.expectedOriginalSha256});
    await publishCargoSourceDriftV1(reporter,{...observed!,observed:"invalid"});
    await publishCargoSourceDriftV1(reporter,{...observed!,path:"x".repeat(257)});
    expect(diagnostics).toHaveLength(1);
  }finally{await stop();terminateOwnedChildTree(child);await terminal;await rm(directory,{recursive:true,force:true});}
});

test("admitted changes before registration beyond the directory watch budget still get verified",async()=>{
  const generated=join(dirname(import.meta.path),"🗑️generated");
  await mkdir(generated,{recursive:true});
  const directory=await mkdtemp(join(generated,"source-scan-"));
  const inputs=[];
  try{
    for(let index=0;index<=limits.maximumDirectories;index++){
      const parent=join(directory,String(index)),path=join(parent,"input.rs");
      await mkdir(parent);
      await writeFile(path,fixture.original);
      inputs.push({path,digest:fixture.expectedOriginalSha256});
    }
    const last=inputs.at(-1)!;
    await Bun.sleep(limits.metadataClockToleranceMs+50);
    const startedMs=Date.now();
    await writeFile(last.path,fixture.sameLengthEdit);
    let observed:Readonly<{path:string;expected:string;observed:string}>|undefined;
    const stop=watchCargoSourceDriftV1(inputs,startedMs,mismatch=>{observed=mismatch;});
    try{
      for(let attempt=0;attempt<100&&!observed;attempt++)await Bun.sleep(50);
      const digest=new Bun.CryptoHasher("sha256").update(fixture.sameLengthEdit).digest("hex");
      expect(observed).toEqual({path:last.path,expected:fixture.expectedOriginalSha256,observed:digest});
    }finally{await stop();}
  }finally{await rm(directory,{recursive:true,force:true});}
},10000);
