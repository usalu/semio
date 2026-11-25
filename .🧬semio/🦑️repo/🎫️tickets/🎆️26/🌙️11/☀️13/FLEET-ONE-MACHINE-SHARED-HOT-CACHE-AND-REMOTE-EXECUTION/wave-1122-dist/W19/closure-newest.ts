import {statSync} from "node:fs";
const sidecar=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts");
const closure=sidecar.remoteExecutionSidecarWatchV1("agent").closure();
const rows=[...closure.files.keys()].map(p=>({p,m:statSync(p).mtimeMs,mode:(statSync(p).mode&0o777).toString(8)})).sort((a,b)=>b.m-a.m);
for(const r of rows.slice(0,8))console.log("[DEBUG]",new Date(r.m).toISOString(),r.mode,r.p.slice(60));
const modes=new Map<string,number>();for(const r of rows)modes.set(r.mode,(modes.get(r.mode)??0)+1);console.log("[DEBUG] modes",[...modes]);
