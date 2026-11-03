import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const a = JSON.parse(readFileSync(path("curtain-wall-bay"),"utf8")).asset;
console.log(a.thermal.regions.map((r:any)=>`${r.id}:${r.areaM2}`).join(" "));
console.log(a.provenance.references.map((r:any)=>r.id).join(","));
for (const p of a.poses) console.log(p.id, (p.loads??[]).map((l:any)=>l.id).join());
console.log(JSON.stringify(a.poses.find((p:any)=>p.id==="dead-load").loads).slice(0,500));
