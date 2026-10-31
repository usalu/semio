/** 🛑 Stop only identity-verified descendants of this exact native Excel replay. */
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import process from 'node:process';
const ticket=dirname(fileURLToPath(import.meta.url));let root=ticket;
while(!existsSync(join(root,'🧰️framework'))){const parent=dirname(root);if(parent===root)throw Error('Repository root missing');root=parent;}
const reference=JSON.parse(readFileSync(join(ticket,'excel-stop-reference.json'),'utf8'));
if(reference.root_pid!==46996||!reference.targets.some((row:any)=>row.pid===23860))throw Error('Unexpected owned Excel reference');
const module=await import(pathToFileURL(join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts')).href);
const ports=module.doctorPortsV1(root,{signal:AbortSignal.timeout(60000)});
const rows=await ports.processes();
const targets=[];
for(const expected of reference.targets){
 const current=rows.find((row:any)=>row.pid===expected.pid);if(!current)continue;
 const born=Date.now()-current.ageSeconds*1000;const oldBorn=Date.parse(reference.snapshot_at)-expected.ageSeconds*1000;
 if(current.parent!==expected.parent||current.command!==expected.command||Math.abs(born-oldBorn)>30000)throw Error('Owned process identity changed: '+expected.pid);
 targets.push(current);
}
const order=(row:any)=>{let count=0,pid=row.parent;const seen=new Set();while(pid&&!seen.has(pid)){seen.add(pid);const parent=reference.targets.find((item:any)=>item.pid===pid);if(!parent)break;count++;pid=parent.parent;}return count;};
targets.sort((a:any,b:any)=>order(b)-order(a));
const execute=process.argv.includes('--execute'),signalled=[];
if(execute)for(const row of targets){try{process.kill(row.pid,'SIGTERM');signalled.push(row.pid);}catch(error:any){if(error.code!=='ESRCH')throw error;}}
const remaining=execute?await ports.processes():rows;
const result={at:new Date().toISOString(),mode:execute?'execute':'audit',owned_root:reference.root_pid,observed:targets,signalled,remaining:remaining.filter((row:any)=>reference.targets.some((item:any)=>item.pid===row.pid))};
writeFileSync(join(ticket,'🗑️generated/owned-excel-stop-'+(execute?'execute':'audit')+'.json'),JSON.stringify(result,null,2));
console.log('[DEBUG] Owned Excel stop '+JSON.stringify({mode:result.mode,observed:targets.map((row:any)=>row.pid),signalled,remaining:result.remaining.map((row:any)=>row.pid)}));
