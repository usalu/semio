import {readFileSync} from "node:fs";
import {join} from "node:path";
import {realHost} from "./real-host.ts";
const {host}=await realHost("");
const t0=performance.now();let dirs=0,files=0;const pending=["✏️s"];
while(pending.length){const d=pending.pop()!;dirs++;for(const e of host.listDirectory(d)){if(e.name==="node_modules"||e.name===".git")continue;const p=d+"/"+e.name;if(e.directory)pending.push(p);else files++;}}
console.log(JSON.stringify({dirs,files,walkMs:Math.round(performance.now()-t0)}));
