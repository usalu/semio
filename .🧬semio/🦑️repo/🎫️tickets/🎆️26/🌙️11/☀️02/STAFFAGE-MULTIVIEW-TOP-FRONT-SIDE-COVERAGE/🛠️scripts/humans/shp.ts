import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/${process.argv[2]}/🔣️.json`, "utf8")).asset;
const r = a.representations[process.argv[3]]; const re = new RegExp(process.argv[4]);
console.log(JSON.stringify(a.rig.bones.filter((b:any)=>re.test(b.id)).map((b:any)=>({id:b.id,p:b.parent,x:b.x,y:b.y,z:b.z,a:b.angleDeg,l:b.lengthM,j:b.joint.type}))));
for (const v of ["elevation","front","plan"]) for (const s of r.geometry[v]) if (re.test(s.bone)||re.test(s.id)) console.log(v.padEnd(9), JSON.stringify(s).slice(0,300));
