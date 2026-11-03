import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const a = JSON.parse(readFileSync(path("curtain-wall-bay"),"utf8")).asset;
console.log(JSON.stringify(a.dimensions), JSON.stringify(a.rig.bones.map((b:any)=>({id:b.id,x:b.x,y:b.y,a:b.angleDeg,l:b.lengthM,j:b.joint,mass:b.mass?.kg,sec:b.section?.title?.en}))));
console.log("anchors",JSON.stringify(a.anchors));
console.log("structure",JSON.stringify(a.structure).slice(0,2500));
const p=a.poses.find((x:any)=>x.id==="seismic-in-plane"); console.log("POSE",JSON.stringify(p).slice(0,2500));
console.log("clip",JSON.stringify(a.clips.find((c:any)=>c.id==="seismic-pulse")).slice(0,1500));
console.log("provenance",JSON.stringify(a.provenance).slice(0,800));
console.log("presentation",JSON.stringify(a.presentation));
