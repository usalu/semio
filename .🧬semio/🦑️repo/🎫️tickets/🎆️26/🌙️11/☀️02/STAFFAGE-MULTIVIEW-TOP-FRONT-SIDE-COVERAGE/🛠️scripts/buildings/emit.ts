import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset;
console.log(JSON.stringify(a.rig.bones.map((b:any)=>({id:b.id,x:b.x,y:b.y,a:b.angleDeg,l:b.lengthM,j:b.joint.type}))));
console.log(JSON.stringify(a.thermal.emitters ?? a.thermal.vents ?? Object.keys(a.thermal)).slice(0,1800));
