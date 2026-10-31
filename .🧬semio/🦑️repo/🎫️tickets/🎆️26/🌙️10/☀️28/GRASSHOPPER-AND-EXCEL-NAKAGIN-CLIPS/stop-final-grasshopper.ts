/** 🛡️ Cancel only the explicitly delegated Grasshopper tree under measured resource pressure. */
import {existsSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const ticket=dirname(fileURLToPath(import.meta.url));let root=ticket;
while(!existsSync(join(root,'🧰️framework')))root=dirname(root);
const doctor=await import(pathToFileURL(join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts')).href);
const termination=await import(pathToFileURL(join(root,'🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts')).href);
const ports=doctor.doctorPortsV1(root,{signal:AbortSignal.timeout(20000)});
const before=await ports.processes(),at=Date.now(),expected=before.find((row:any)=>row.pid===40200);
if(!expected||expected.ageSeconds<7000||expected.ageSeconds>20000||!expected.command.includes('verify.py')||!expected.command.includes('replay-native-pairs.py')||!expected.command.includes('--label final-native-grasshopper --series grasshopper'))throw Error('Delegated Grasshopper root identity mismatch');
const ids=new Set<number>([40200]);for(let changed=true;changed;){changed=false;for(const row of before)if(ids.has(row.parent)&&!ids.has(row.pid)){ids.add(row.pid);changed=true;}}
if(ids.has(12384))throw Error('Capture root entered delegated Grasshopper tree');
const latest=await ports.processes(),current=latest.find((row:any)=>row.pid===40200);
if(!current||current.parent!==expected.parent||current.command!==expected.command||Math.abs((Date.now()-current.ageSeconds*1000)-(at-expected.ageSeconds*1000))>3000)throw Error('Delegated identity changed before cancellation');
termination.terminateOwnedProcessTree(40200);
let remaining:any[]=[];const deadline=Date.now()+15000;
do{remaining=(await ports.processes()).filter((row:any)=>ids.has(row.pid));if(!remaining.length)break;await new Promise(resolve=>setTimeout(resolve,100));}while(Date.now()<deadline);
const result={at:new Date().toISOString(),authorization:'Explicit root delegation for resource protection; capture excluded',root_pid:40200,expected,initial:before.filter((row:any)=>ids.has(row.pid)),remaining,success:remaining.length===0};
writeFileSync(join(ticket,'🗑️generated/final-grasshopper-resource-stop.json'),JSON.stringify(result,null,2));
console.log('[DEBUG] Delegated Grasshopper resource stop '+JSON.stringify({success:result.success,initial:[...ids],remaining:remaining.map((row:any)=>row.pid)}));
if(!result.success)throw Error('Delegated Grasshopper descendants remain');
