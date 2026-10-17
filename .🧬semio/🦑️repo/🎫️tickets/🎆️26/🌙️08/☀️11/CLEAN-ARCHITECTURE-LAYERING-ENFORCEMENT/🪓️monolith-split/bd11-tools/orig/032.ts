import {test,expect} from "vitest";
import {mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {join as oracleJoin} from "pathe";
import {emitRustArtifacts} from "../../📽️projection/🟦️.ts";
import vectors from "../🧫️fixtures/🎚️component-modes.json";
import {buildModesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1,acquireCompilerCapacityLeaseV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
test("native component resolution selects exact original modes with byte-neutral generated source",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Component mode test output required");
 const root=mkdtempSync(join(output,"cm-")),source=join(root,"owner.rs"),body=emitRustArtifacts([],process.cwd()),modes=buildModesV1(),admit=(value:unknown)=>modes.some(mode=>mode===value),control=scriptCompilerAdmissionControlV1(original,"registry:component-mode",120000);
 const declared=process.env.SEMIO_BUILD_MODE;
 try{for(const row of vectors.modes){process.env.SEMIO_BUILD_MODE=row.mode;expect(emitRustArtifacts([],process.cwd())).toBe(body);}}finally{if(declared===undefined)delete process.env.SEMIO_BUILD_MODE;else process.env.SEMIO_BUILD_MODE=declared;}
 writeFileSync(source,body+`fn main(){let selected=std::env::args().nth(1).unwrap_or_else(||env!("${vectors.originalEnvironment}").to_string());match plugin_component_directory(&selected){Ok(directory)=>println!("{}",std::path::Path::new(${JSON.stringify(vectors.owner)}).join(directory).join(${JSON.stringify(vectors.file)}).display()),Err(_)=>println!("refused")}}\n`);
 const execute=async(command:string,args:string[],environment:NodeJS.ProcessEnv)=>{let text="";await runOwnedCommand(command,args,root,"registry:component-mode",compilerAdmissionBudgetV1(control),{env:environment,signal:control.signal,stdout:"stderr",onLine:line=>{text+=line+"\n";},onProgress:async()=>{await control.publish({resource:"registry:component-mode",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}});return text.trim().replaceAll("\\","/");};
 for(const row of vectors.modes){expect(admit(row.mode)).toBe(true);const binary=join(root,row.mode+(process.platform==="win32"?".exe":"")),dep=join(root,row.mode+".d"),environment={...process.env,[vectors.originalEnvironment]:row.mode},claim=await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:join(process.cwd(),".cache/framework/leases"),control,environment});try{await execute("rustc",[source,"--edition=2021","--crate-name","component_mode","--emit=link,dep-info="+dep,"-o",binary],environment);}finally{claim.release();}expect(readFileSync(dep,"utf8")).toContain(`# env-dep:${vectors.originalEnvironment}=${row.mode}`);expect(await execute(binary,[],environment)).toBe(oracleJoin(vectors.owner,row.directory,vectors.file));for(const mode of vectors.rejected){expect(admit(mode)).toBe(false);expect(await execute(binary,[mode],environment)).toBe("refused");}}
}),120000);
