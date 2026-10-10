import assert from "node:assert/strict";
import {readFileSync,writeFileSync,mkdirSync,lstatSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";
import {terminateOwnedProcessTree} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";

const started=performance.now(),deadline=Date.now()+60000,ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../..");
const diagnostic=join(root,"🧰️framework/🔨️modules/⚠️diagnostic"),plugin=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const [command,epoch,expectedProducer,expectedManifest]=process.argv.slice(2);
const toolRun=command==="receive-tool-run",label=toolRun?"ToolRun":"Diagnostic",selected=toolRun?join(root,"🧰️framework/🔨️modules/⏯️tool-run"):diagnostic;
if(command==="oracle"){
 const {timeTravelScenarioOracle}=await import(join(plugin,"🧪️tests/🧪️time-travel/🟦️.ts"));
 console.log(`[DEBUG] Actual selected OS TimeTravel oracle cases=${timeTravelScenarioOracle(root)}`);
}else if(command==="oracle-tool-run-policy"){
 const {interactivityToolRunPolicySelfTests}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🧪️interactivity-tool-run-policy/🟦️.ts"));
 const cases=interactivityToolRunPolicySelfTests();assert.equal(cases,29);console.log(`[DEBUG] Actual complete Repository ToolRun policy oracle cases=${cases}`);
}else{
 assert.ok(command==="receive"||toolRun);assert.equal(process.argv.length,6);assert.ok(epoch&&/^\d+$/.test(epoch));assert.ok([expectedProducer,expectedManifest].every(value=>typeof value==="string"&&/^[0-9a-f]{64}$/.test(value)));
 const output=join(ticket,"🗑️generated/"+(toolRun?"rt":"rdg")+epoch,"run");assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
 let observationBytes=0,captureBytes=0;
 const check=()=>assert.ok(Date.now()<deadline&&performance.now()-started<60000,"Original Diagnostic source authority exhausted");
 const read=(path:string)=>{check();assert.ok(path.length<=256&&[...path].length<=256);const before=lstatSync(path);assert.ok(before.isFile()&&!before.isSymbolicLink()&&before.size<=16777216);const bytes=readFileSync(path),after=lstatSync(path);observationBytes+=bytes.length;assert.ok(observationBytes<=268435456);assert.ok(before.ino===after.ino&&before.size===after.size&&before.mtimeMs===after.mtimeMs&&bytes.length===before.size);check();return bytes;};
 const capture=(path:string)=>{const bytes=read(path);return{path,bytes:bytes.length,sha256:createHash("sha256").update(bytes).digest("hex")};};
 const manifestPath=join(import.meta.dir,(toolRun?"tool-run-":"")+"source-"+epoch+"-defining.json"),manifestBytes=read(manifestPath);assert.equal(createHash("sha256").update(manifestBytes).digest("hex"),expectedManifest);
 const definitions=JSON.parse(manifestBytes.toString()) as {path:string;bytes:number;sha256:string}[];assert.ok(Array.isArray(definitions)&&definitions.length>0&&definitions.length<=4096);assert.equal(new Set(definitions.map(row=>row.path)).size,definitions.length);
 const producer=capture(import.meta.path);assert.equal(producer.sha256,expectedProducer);const before=definitions.map(row=>capture(row.path));assert.deepEqual(before,definitions);
 writeFileSync(join(output,"admission.json"),JSON.stringify({producer,expectedManifest,before,deadline,maximumElapsedMilliseconds:60000,maximumChildElapsedMilliseconds:30000,maximumCaptureBytes:4194304}));
 const env={...process.env,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify({version:1,policy:{version:1,owner:toolRun?"semio.tool-run.source":"semio.diagnostic.source",maximumElapsedMilliseconds:60000},deadlineEpochMilliseconds:deadline,capabilities:{}})};
 const stages:{name:string;code:number;timedOut:boolean}[]=[];
 const own=async(name:string,args:string[])=>{
  check();const budget=Math.floor(Math.min(30000,deadline-Date.now(),60000-(performance.now()-started)));assert.ok(budget>0);
  const child=Bun.spawn([process.execPath,...args],{cwd:root,env,stdout:"pipe",stderr:"pipe"});let timedOut=false,captureError:unknown;
  const cancel=()=>{if(child.exitCode===null)terminateOwnedProcessTree(child.pid);};process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
  const timeout=setTimeout(()=>{timedOut=true;cancel();},budget);
  const drain=async(stream:ReadableStream<Uint8Array>,path:string)=>{const chunks:Uint8Array[]=[];const reader=stream.getReader();try{for(;;){const row=await reader.read();if(row.done)break;captureBytes+=row.value.length;if(captureBytes>4194304){captureError=Error("Original Diagnostic capture exhausted");cancel();}else chunks.push(row.value);}writeFileSync(path,Buffer.concat(chunks));}finally{reader.releaseLock();}};
  const drains=[drain(child.stdout,join(output,name+"-out.txt")),drain(child.stderr,join(output,name+"-err.txt"))];
  const code=await child.exited;await Promise.all(drains);clearTimeout(timeout);process.off("SIGINT",cancel);process.off("SIGTERM",cancel);if(captureError)throw captureError;
  stages.push({name,code,timedOut});console.log(`[DEBUG] ${label} source ${name} physically closed=${code} timedOut=${timedOut}`);return code||Number(timedOut);
 };
 const tests=toolRun?[join(selected,"🧪️tests/🧪️conformance/🟦️.ts"),join(selected,"🏛️ownership/🧪️tests/🟦️.ts"),join(selected,"🎞️tick/🛫️encode/🧪️tests/🟦️.ts"),join(plugin,"⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts")]:[join(diagnostic,"🧾️retained/🧪️tests/🟦️.ts"),join(diagnostic,"🚧️text-error/🧪️tests/🟦️.ts"),join(diagnostic,"🧾️retained/🏛️ownership/🧪️tests/🟦️.ts"),join(plugin,"🧪️tests/🧪️time-travel/🟦️.ts")];
 let code=await own("types",[join(root,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...tests]);
 if(!code){if(!toolRun)code|=await own("controlled",[join(diagnostic,"📦️packages/🦀️rust/📜️script.ts"),"test-controlled-oracle"]);code|=await own("laws",[join(selected,"📦️packages/🦀️rust/📜️script.ts"),"test-source"]);code|=await own("product",toolRun?[join(plugin,"📦️packages/🟦️typescript/📜️script.ts"),"test-tick-payload"]:[import.meta.path,"oracle"]);if(toolRun)code|=await own("repository",[import.meta.path,"oracle-tool-run-policy"]);}
 let after:ReturnType<typeof capture>[]=[],custodyError:string|undefined;try{after=definitions.map(row=>capture(row.path));assert.equal(capture(import.meta.path).sha256,expectedProducer);assert.equal(capture(manifestPath).sha256,expectedManifest);}catch(error){custodyError=String(error);}
 const exact=!custodyError&&before.every((row,index)=>row.sha256===after[index]!.sha256&&row.bytes===after[index]!.bytes);
 writeFileSync(join(output,"terminal.json"),JSON.stringify({code,stages,exact,custodyError,before,after,expectedProducer,expectedManifest,observationBytes,captureBytes,deadline,maximumElapsedMilliseconds:60000,maximumChildElapsedMilliseconds:30000,trackedSourceCopiesCreated:0}));
 console.log(`[DEBUG] ${label} complete source gate exact=${exact} status=${code}`);process.exitCode=code||Number(!exact);
}
