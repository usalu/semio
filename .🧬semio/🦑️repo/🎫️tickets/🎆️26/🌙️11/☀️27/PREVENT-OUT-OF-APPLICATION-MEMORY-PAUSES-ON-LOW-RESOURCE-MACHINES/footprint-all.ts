import {systemDarwinFootprintPortV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🍏️native-footprint/🟦️.ts";
import {execFileSync} from "node:child_process";
const port=systemDarwinFootprintPortV1()!;
const rows=execFileSync("ps",["-A","-o","pid=,rss=,comm="],{encoding:"utf8",maxBuffer:64e6}).split("\n").flatMap(l=>{const m=/^\s*(\d+)\s+(\d+)\s+(.*)$/.exec(l);return m?[{pid:+m[1]!,rss:+m[2]!*1024,name:m[3]!.split("/").pop()!}]:[]});
const t0=performance.now();let F=0,R=0,unread=0;const by=new Map<string,number>();
for(const r of rows){const f=port.footprint(r.pid);if(f===null){unread++;R+=r.rss;continue;}F+=f;R+=r.rss;by.set(r.name,(by.get(r.name)??0)+f);}
const ms=performance.now()-t0;const GB=1073741824;
console.log(`processes=${rows.length} unread=${unread} footprintSum=${(F/GB).toFixed(2)}G rssSum=${(R/GB).toFixed(2)}G readMs=${ms.toFixed(1)}`);
for(const [n,v] of [...by].sort((a,b)=>b[1]-a[1]).slice(0,12))console.log((v/1048576).toFixed(0).padStart(7)+" MB",n);
