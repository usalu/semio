/** 🧭 Read process facts through the existing portable repo doctor port without signalling any process. */
import {existsSync,writeFileSync,mkdirSync} from "node:fs";
import {dirname,join,basename} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
const ticket=dirname(fileURLToPath(import.meta.url));
let root=ticket;
while(!existsSync(join(root,"🧰️framework"))){const parent=dirname(root);if(parent===root)throw Error("Repository root missing");root=parent;}
const module=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts")).href);
const ports=module.doctorPortsV1(root,{signal:AbortSignal.timeout(20000)});
const rows=await ports.processes();
const pids=new Set(rows.map((row:any)=>row.pid));
const marker=basename(ticket);
const matching=rows.filter((row:any)=>row.command.includes(marker)).map((row:any)=>({...row,parentMissing:row.parent>0&&!pids.has(row.parent)})).sort((a:any,b:any)=>b.rssMegabytes-a.rssMegabytes);
const image=(command:string)=>{const head=/^"([^"]+)"|^(\S+)/u.exec(command);return basename(head?.[1]??head?.[2]??"unknown");};
const top=[...rows].sort((a:any,b:any)=>b.rssMegabytes-a.rssMegabytes).slice(0,20).map((row:any)=>({pid:row.pid,parent:row.parent,rssMegabytes:row.rssMegabytes,image:image(row.command),ticketMatch:row.command.includes(marker)}));
const output=join(ticket,"🗑️generated");mkdirSync(output,{recursive:true});
writeFileSync(join(output,"ticket-process-audit.json"),JSON.stringify({at:new Date().toISOString(),ticketProcesses:matching,top},null,2));
const lines=["# Ticket-Matching Process Audit","","Read-only host inspection through the repository's existing cross-platform doctor interface. No process was signalled. A ticket-path match does not establish ownership by this Codex agent: other agents use the same ticket. Only matching commands are retained; unrelated process arguments are omitted.","","| PID | Parent | RSS MB | Parent Missing | Command |","|---|---|---:|---|---|",...matching.map((row:any)=>"| "+row.pid+" | "+row.parent+" | "+row.rssMegabytes+" | "+row.parentMissing+" | "+row.command.replaceAll("|","\\|")+" |"),"","## Largest Resident Processes","","| PID | Parent | RSS MB | Image | This Ticket |","|---|---|---:|---|---|",...top.map((row:any)=>"| "+row.pid+" | "+row.parent+" | "+row.rssMegabytes+" | "+row.image+" | "+row.ticketMatch+" |")];
writeFileSync(join(ticket,"ticket-process-audit.md"),lines.join("\n")+"\n");
console.log("[DEBUG] ticket-matching process memory "+JSON.stringify(matching.map((row:any)=>({pid:row.pid,parent:row.parent,mb:row.rssMegabytes,parentMissing:row.parentMissing}))));
