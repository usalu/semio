/** 🌳 Validate one live owned capture root, then use the first-party portable tree terminator. */
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join,resolve,relative,sep,isAbsolute} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const ticket=dirname(fileURLToPath(import.meta.url));let root=ticket;
while(!existsSync(join(root,'🧰️framework'))){const parent=dirname(root);if(parent===root)throw Error('Repository root missing');root=parent;}
const input=process.argv.indexOf('--request');if(input<0)throw Error('Owned stop request missing');
const requestPath=resolve(process.argv[input+1]!);const generated=join(ticket,'🗑️generated');
const inside=(path:string)=>{const value=relative(generated,resolve(path));return !isAbsolute(value)&&value!== '..'&&!value.startsWith('..'+sep)&&!value.startsWith(sep);};
if(!inside(requestPath))throw Error('Stop request outside generated output');
const request=JSON.parse(readFileSync(requestPath,'utf8'));if(!inside(request.record)||!Number.isSafeInteger(request.root_pid)||!Number.isSafeInteger(request.parent_pid)||!Number.isFinite(request.started_at)||!/^[A-Za-z0-9][A-Za-z0-9_.-]{0,63}$/.test(request.label))throw Error('Malformed owned stop request');
const doctor=await import(pathToFileURL(join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts')).href);
const termination=await import(pathToFileURL(join(root,'🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts')).href);
const ports=doctor.doctorPortsV1(root,{signal:AbortSignal.timeout(20000)});const rows=await ports.processes();const current=rows.find((row:any)=>row.pid===request.root_pid);
const ids=new Set<number>([request.root_pid]);for(let changed=true;changed;){changed=false;for(const row of rows)if(ids.has(row.parent)&&!ids.has(row.pid)){ids.add(row.pid);changed=true;}}
const initial=rows.filter((row:any)=>ids.has(row.pid));
const report:any={at:new Date().toISOString(),request,initial_descendants:initial,signalled:[],remaining:[],success:false};
try{
 if(current){
  const marker=ticket.split(/[\\/]/).at(-1)!;
  const escaped=request.label.replace(/[.*+?^${}()|[\]\\]/g,'\\$&');
  const label=new RegExp('(?:^|\\s)--label\\s+'+escaped+'(?=\\s|$)');
  const born=Date.now()/1000-current.ageSeconds;
  if(current.parent!==request.parent_pid||!current.command.includes(marker)||!current.command.includes('verify.py')||!label.test(current.command)||Math.abs(born-request.started_at)>30)throw Error('Capture root identity mismatch; no process signalled');
  termination.terminateOwnedProcessTree(request.root_pid);report.signalled.push(request.root_pid);
 }else if(initial.length)throw Error('Capture root absent while unverified descendants remain');
 const deadline=Date.now()+10000;
 do{
  const latest=await ports.processes();report.remaining=latest.filter((row:any)=>ids.has(row.pid));
  if(!report.remaining.length)break;
  await new Promise(resolve=>setTimeout(resolve,100));
 }while(Date.now()<deadline);
 report.success=report.remaining.length===0;
 if(!report.success)throw Error('Initial owned descendants remain after first-party termination');
}finally{writeFileSync(request.record,JSON.stringify(report,null,2));console.log('[DEBUG] Capture tree '+JSON.stringify({success:report.success,root:request.root_pid,initial:initial.map((row:any)=>row.pid),remaining:report.remaining.map((row:any)=>row.pid),signalled:report.signalled}));}
