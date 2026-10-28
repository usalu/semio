/** 🩺 Read-only probe: is the installed dashboard current for its recorded sources right now, and what does the check cost? `bun stale.ts <root> [mode]` */
import {resolve,join} from "node:path";
import {statSync,readFileSync} from "node:fs";
const root=resolve(process.argv[2]??"."),mode=(process.argv[3]??"prototype") as "prototype"|"release";
const t0=performance.now();
const m=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"));
const t1=performance.now();
const reason=await m.staleDashboard(root,mode);
const t2=performance.now();
const record=JSON.parse(readFileSync(join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",mode,"installed.json"),"utf8"));
const captured=await m.captureDashboardSources(m.dashboardPackageRoot,root);
const t3=performance.now();
let executable="missing";try{const s=statSync(join(root,record.path));executable=`${s.size===record.bytes?"bytes ok":"bytes differ"} ${(s.mode&0o777)===record.permissions?"permissions ok":"permissions differ"}`;}catch{}
console.log(JSON.stringify({stale:reason??null,recordedDigest:record.sources.digest,capturedDigest:captured.digest,rootsEqual:JSON.stringify(captured.roots)===JSON.stringify(record.sources.roots),manifestsEqual:JSON.stringify(captured.manifests)===JSON.stringify(record.sources.manifests),executable,ms:{import:Math.round(t1-t0),stale:Math.round(t2-t1),capture:Math.round(t3-t2)},rssMb:Math.round(process.memoryUsage().rss/1048576)}));
