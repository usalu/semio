/** 🧭 Audit one exact old preview; execution requires separate explicit human authorization. */
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import process from "node:process";

const ticket=dirname(fileURLToPath(import.meta.url));
let root=ticket;
while(!existsSync(join(root,"🧰️framework"))){const parent=dirname(root);if(parent===root)throw Error("Missing repository root");root=parent;}
const expected=JSON.parse(readFileSync(join(ticket,"preview-stop-candidate.json"),"utf8"));
const reference=expected.process;
if(reference.pid!==53880||reference.parent!==46580||!reference.command.includes("render/preview.py")||!reference.command.includes("Beat2_References Excel_references en"))throw Error("Unexpected preview candidate");
const module=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts")).href);
const ports=module.doctorPortsV1(root,{signal:AbortSignal.timeout(60000)});
const observed=await ports.processes();
const current=observed.find((row:any)=>row.pid===reference.pid);
const result:any={at:new Date().toISOString(),candidate:reference,current,signalled:[],mode:process.argv.includes("--execute")?"execute":"audit"};
if(current){
 const born=Date.now()-current.ageSeconds*1000;
 const expectedBorn=Date.parse(expected.snapshot_at)-reference.ageSeconds*1000;
 if(current.parent!==reference.parent||current.command!==reference.command||Math.abs(born-expectedBorn)>30000||Math.abs(expectedBorn-Date.parse("2026-10-30T01:21:01Z"))>30000)throw Error("Preview identity changed; no process signalled");
 if(result.mode==="execute"){
  process.kill(current.pid,"SIGTERM");
  result.signalled.push(current.pid);
  const remaining=await ports.processes();
  result.still_present=remaining.some((row:any)=>row.pid===current.pid);
 }
}
writeFileSync(join(ticket,"🗑️generated/preview-stop-audit.json"),JSON.stringify(result,null,2));
console.log("[DEBUG] Exact preview "+result.mode+": "+JSON.stringify({present:!!current,signalled:result.signalled,still_present:result.still_present}));
