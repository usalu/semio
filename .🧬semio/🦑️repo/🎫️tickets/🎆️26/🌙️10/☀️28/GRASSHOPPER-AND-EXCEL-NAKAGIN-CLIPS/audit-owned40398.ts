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
const owned=new Set([2380,49760,3540]);for(let i=0;i<8;i++)for(const row of rows)if(owned.has(row.parent))owned.add(row.pid);const matching=rows.filter(row=>owned.has(row.pid));writeFileSync(join(ticket,"🗑️generated/owned40398-processes.json"),JSON.stringify({at:new Date().toISOString(),rows:matching},null,2));console.log(JSON.stringify(matching));
