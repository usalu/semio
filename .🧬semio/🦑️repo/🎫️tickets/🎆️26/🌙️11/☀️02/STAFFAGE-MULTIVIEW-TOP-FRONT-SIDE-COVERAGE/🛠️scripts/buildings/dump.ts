import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset; const s=process.argv[3];
console.log(JSON.stringify(a.dimensions), "bones:", a.rig.bones.map((b:any)=>`${b.id}(${b.x},${b.y},${b.angleDeg},${b.lengthM},${b.joint.type})`).join(" "));
for (const v of ["elevation","front","plan"]) { console.log("--",v); for (const sh of a.representations[s].geometry[v]) console.log(JSON.stringify(sh).slice(0,Number(process.env.W??260))); }
