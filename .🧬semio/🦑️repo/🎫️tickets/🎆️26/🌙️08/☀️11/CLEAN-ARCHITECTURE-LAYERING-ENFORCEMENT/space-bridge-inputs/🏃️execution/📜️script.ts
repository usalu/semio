import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import Ajv2020 from "ajv/dist/2020";

const ticket=resolve(import.meta.dir,"../.."),root=resolve(ticket,"../../../../../../.."),plugin="✏️s/🔌️plugins/🪐️space",[command,epoch]=process.argv.slice(2);
assert.ok(["prepare","run"].includes(command)&&/^\d+$/u.test(epoch??""));
const output=join(ticket,"🗑️generated/space-child-inventory",command+"-"+epoch);assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const jobs=JSON.parse(readFileSync(join(import.meta.dir,"../execution-jobs.json"),"utf8")),controller=new AbortController(),stop=()=>controller.abort(),results:any[]=[];
process.once("SIGINT",stop);process.once("SIGTERM",stop);
const sourcePaths=()=>{const paths:string[]=[],walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){controller.signal.throwIfAborted();if(["target","dist","node_modules","🗑️generated",".git",".nx",".venv","__pycache__"].includes(entry.name))continue;const path=join(directory,entry.name);assert.ok(!entry.isSymbolicLink(),"Source custody refuses symlink: "+path);if(entry.isDirectory())walk(path);else if(/\.(?:[cm]?[jt]sx?|rs|toml|json|lock|semio|wgsl|glsl|c|cpp|h|hpp|patch)$/u.test(path))paths.push(path);}};for(const directory of [plugin,"✏️s/🔌️plugins/🗄️stdio","🧰️framework"])walk(join(root,directory));for(const path of ["Cargo.toml","Cargo.lock","package.json","bun.lock","nx.json","rust-toolchain.toml",".cargo/config.toml",".config/nextest.toml",".vscode/launch.json"])if(existsSync(join(root,path)))paths.push(join(root,path));paths.push(import.meta.path,join(import.meta.dir,"../execution-jobs.json"));return [...new Set(paths)].sort();};
const capture=(path:string)=>{const bytes=readFileSync(path);return {path,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length};},before=sourcePaths().map(capture);
const {executeCommandV1}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🟦️.ts")),{acquireCargoBuildLeaseV1}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts"));
const foreign=(text:string)=>text.split(/(?=^\[\[package\]\])/mu).filter(block=>/^source\s*=/mu.test(block));
let code=1,error:string|undefined;
try{
 const launches=JSON.parse(readFileSync(join(root,".vscode/launch.json"),"utf8")).configurations;
 for(const job of jobs){const launch=launches.find((row:any)=>row.name===job.name);assert.ok(launch,"Exact launch route missing: "+job.name);assert.equal(launch.command.replaceAll("${workspaceFolder}",root),"bun "+job.request.args.join(" "));for(const[key,value]of Object.entries(job.environment))assert.equal(launch.env[key].replaceAll("${workspaceFolder}",root),value);}
 const wire=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json"),"utf8")),validate=new Ajv2020({strict:true}).compile({$schema:"https://json-schema.org/draft/2020-12/schema",$defs:wire.$defs,$ref:"#/$defs/RuntimeMutationInventory"});
 const decode=(text:string)=>{const value=JSON.parse(text);assert.ok(validate(value),JSON.stringify(validate.errors));return value;};for(const noise of ["noise\n{}","{}\n{}",'{"schema":"unrelated"}'])assert.throws(()=>decode(noise));
 if(command==="prepare"){
  for(const job of jobs.filter((row:any)=>row.mode==="test")){
   const manifest=job.request.manifests[0],owner=dirname(dirname(dirname(manifest))),lock=join(owner,"Cargo.lock"),originalLock=join(ticket,"space-bridge-inputs/original",plugin,"🗿️artifacts",job.artifact==="home"?"🏠️home":"🪐️space","Cargo.lock"),oldForeign=foreign(readFileSync(originalLock,"utf8"));assert.ok(oldForeign.length>0);
   const beforeForeign=foreign(readFileSync(lock,"utf8"));
   const lease=await acquireCargoBuildLeaseV1({directory:job.policy.cargoArtifactPolicy.leaseDirectory,buildDirectory:job.policy.cargoArtifactPolicy.buildDirectory,args:["space-prepare",job.artifact],signal:controller.signal,onWait:()=>console.log("[DEBUG] Space graph "+job.artifact+" waiting for compiler lease")});
   try{
    const request={...job.request,command:"cargo",args:["metadata","--offline","--format-version","1","--manifest-path",manifest]},result=await executeCommandV1(request,job.policy,{environment:{...process.env,...job.environment},cancelled:()=>controller.signal.aborted,onProgress:event=>console.log("[DEBUG] Space graph "+job.artifact+" "+event.phase)});
    const afterForeign=foreign(readFileSync(lock,"utf8"));results.push({name:"graph-"+job.artifact,...result,foreignCount:afterForeign.length});assert.equal(result.status,0);
    if(job.artifact==="core"){
     assert.deepEqual(afterForeign,beforeForeign,"Fresh core resolved foreign blocks advanced");
     const originalPackages:any=(Bun.TOML.parse(readFileSync(originalLock,"utf8")) as any).package,currentPackages:any=(Bun.TOML.parse(readFileSync(lock,"utf8")) as any).package,identity=(pkg:any)=>JSON.stringify([pkg.name,pkg.version,pkg.source,pkg.checksum]),known=new Set(originalPackages.filter((pkg:any)=>pkg.source).map(identity));
     assert.ok(afterForeign.length>0);for(const pkg of currentPackages.filter((pkg:any)=>pkg.source))assert.ok(known.has(identity(pkg)),"Fresh core introduced unknown foreign package: "+pkg.name);
    }else assert.deepEqual(afterForeign,oldForeign,"Original child foreign lock blocks advanced");
    console.log("[DEBUG] Space graph "+job.artifact+" foreignBlocks="+afterForeign.length+" exact=true");
   }finally{lease.release();}
  }
 }else{
  for(const job of jobs){
   const captureDirectory=JSON.parse(job.environment.SEMIO_MUTATION_INVENTORY_POLICY).compilerStorage.captureDirectory,existing=new Set(existsSync(captureDirectory)?readdirSync(captureDirectory):[]);let at=0;
   const result=await executeCommandV1({...job.request,command:process.execPath},job.policy,{environment:{...process.env,...job.environment},cancelled:()=>controller.signal.aborted,onProgress:event=>{if(Date.now()-at>=10000){at=Date.now();console.log("[DEBUG] Space child "+job.artifact+" "+job.mode+" "+event.phase);}}});
   let inventory:any=null;
   if(result.status===0&&job.mode==="inventory"){
    const created=readdirSync(captureDirectory).filter(name=>!existing.has(name)&&name.startsWith("mutation-inventory-"));assert.equal(created.length,1);const receipt=JSON.parse(readFileSync(join(captureDirectory,created[0],"receipt.json"),"utf8"));assert.equal(receipt.reason,"exit");assert.equal(receipt.code,0);for(const key of ["processClosed","leaseReleased","stdoutClosed","stderrClosed"])assert.equal(receipt[key],true);const stdout=readFileSync(receipt.stdout.path,"utf8");assert.equal(Buffer.byteLength(stdout),receipt.stdout.bytes);inventory=decode(stdout);assert.ok(result.stdout.includes(stdout.trim()));assert.equal(inventory.artifact,"s.space."+job.artifact);assert.equal(inventory.standard,"1");assert.equal(inventory.subset,"any");assert.equal(inventory.surface??null,null);assert.equal(inventory.producedBy,"semio-space-mutation-bridge");assert.ok(inventory.mutations.length>0);assert.equal(new Set(inventory.mutations.map((row:any)=>row.id)).size,inventory.mutations.length);
   }
   results.push({name:job.name,...result,inventory});writeFileSync(join(output,"results.json"),JSON.stringify(results));console.log("[DEBUG] Space child "+job.artifact+" "+job.mode+" status="+result.status+" reason="+result.reason);
  }
  assert.equal(results.length,7);assert.ok(results.every(row=>row.status===0));
 }
 code=0;
}catch(failure){error=String(failure);console.error("[DEBUG] Space execution refusal "+error);}
finally{
 const after=sourcePaths().map(capture),initial=new Map(before.map(row=>[row.path,row.sha256])),terminal=new Map(after.map(row=>[row.path,row.sha256])),changed=after.filter(row=>initial.has(row.path)&&initial.get(row.path)!==row.sha256).map(row=>row.path),added=after.filter(row=>!initial.has(row.path)).map(row=>row.path),removed=before.filter(row=>!terminal.has(row.path)).map(row=>row.path),exact=changed.length+added.length+removed.length===0,ownedLocks=jobs.filter((row:any)=>row.mode==="test").map((row:any)=>join(dirname(dirname(dirname(row.request.manifests[0]))),"Cargo.lock")),onlyOwnedLockChanges=added.length===0&&removed.length===0&&changed.every(path=>ownedLocks.includes(path));
 if(!exact&&!(command==="prepare"&&onlyOwnedLockChanges))code=1;
 writeFileSync(join(output,"terminal.json"),JSON.stringify({command,epoch,code,error,before,after,changed,added,removed,exact,onlyOwnedLockChanges,results,wholeRootAccepted:false}));console.log("[DEBUG] Space execution terminal code="+code+" exact="+exact+" jobs="+results.length);process.off("SIGINT",stop);process.off("SIGTERM",stop);process.exitCode=code;
}
