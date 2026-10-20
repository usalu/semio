/** 📸️ Captures and restores a generator snapshot of a tracked file and an untracked directory in a fixture checkout, then reports whether a hit rewrites identical bytes (inode and mtime), repairs different bytes, and refuses a missing declared output (usage: bun snapshot-probe.ts <outJson>). */
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const repo=process.cwd(),library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const api=await import(join(library,"🏗️build/🔧️tools/🏗️bazel/📜️script.ts")),{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1}=await import(routing);

const base=mkdtempSync(join(realpathSync(tmpdir()),"xs-")),root=join(base,"checkout");mkdirSync(join(root,"gen/out"),{recursive:true});mkdirSync(join(root,".cache/b/l/m/data"),{recursive:true});
writeFileSync(join(root,"package.json"),'{"name":"fx","workspaces":["*"]}\n');writeFileSync(join(root,"project.json"),"{}\n");
const readme=join(root,"gen/README.md"),palette=join(root,"gen/out/palette.css");
writeFileSync(readme,"# generated\n");writeFileSync(palette,"a{color:red}\n");
const task=(outputs:string[],cache=true,metadata?:unknown)=>({id:"gen:generate",project:"gen",target:"generate",label:"t_"+"a".repeat(16),root:"gen",args:[],dependencies:[],definition:{execution:{kind:"command",command:"true"},outputs,cache,...(metadata?{metadata}:{})}}) as any;
const stat=(path:string)=>{const facts=statSync(path,{bigint:true});return {ino:String(facts.ino),mtimeNs:String(facts.mtimeNs)};};
const report:Record<string,unknown>={};
const capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","gen:generate"],"bazel",{});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:snapshot-probe",maximumElapsedMilliseconds:120000},capabilities,Date.now()),async original=>{
 const control=()=>createIssuedResourceControlV1(original,"bazel:snapshot-probe",0),outputs=["{projectRoot}/README.md","{projectRoot}/out"],snapshot=join(base,"snap");
 await api.captureBazelSnapshotV1({workspace:root,task:task(outputs)},snapshot,original);
 const hashes=()=>({readme:Bun.hash(readFileSync(readme)).toString(),palette:Bun.hash(readFileSync(palette)).toString()});
 const before=hashes(),statBefore={readme:stat(readme),palette:stat(palette)};
 await new Promise(r=>setTimeout(r,50));
 await api.restoreBazelSnapshotV1(root,snapshot,original,control());
 report.identicalRestore={sameBytes:JSON.stringify(before)===JSON.stringify(hashes()),inodeAndMtimeUnchanged:JSON.stringify(statBefore)===JSON.stringify({readme:stat(readme),palette:stat(palette)})};
 writeFileSync(readme,"# hand edited\n");writeFileSync(palette,"a{color:blue}\n");
 await api.restoreBazelSnapshotV1(root,snapshot,original,control());
 report.differentRestore={bytesRestoredToSnapshot:JSON.stringify(before)===JSON.stringify(hashes())};
 rmSync(palette);
 let strict:string|undefined;try{await api.captureBazelSnapshotV1({workspace:root,task:task(["{projectRoot}/README.md","{projectRoot}/out/missing.css"])},join(base,"snap2"),original);}catch(error){strict=String(error).slice(0,200);}
 report.missingOutputCached={refused:strict!==undefined,message:strict};
 let optional:string|undefined;try{await api.captureBazelSnapshotV1({workspace:root,task:task(["{projectRoot}/out/missing.css"],true,{semio:{optionalOutputs:true}})},join(base,"snap3"),original);optional="accepted";}catch(error){optional=String(error).slice(0,200);}
 report.missingOutputOptional=optional;
 let uncached:string|undefined;try{await api.captureBazelSnapshotV1({workspace:root,task:task(["{projectRoot}/out/missing.css"],false)},join(base,"snap4"),original);uncached="accepted";}catch(error){uncached=String(error).slice(0,200);}
 report.missingOutputUncached=uncached;
});
rmSync(base,{recursive:true,force:true});
writeFileSync(process.argv[2]??"snapshot-probe.json",JSON.stringify(report,null,1));console.log(JSON.stringify(report,null,1));
