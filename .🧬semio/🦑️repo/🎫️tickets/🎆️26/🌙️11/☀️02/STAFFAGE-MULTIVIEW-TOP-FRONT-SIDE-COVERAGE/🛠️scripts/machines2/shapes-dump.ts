import { load } from "../machines/lib.ts";
const [id, band, re] = [process.env.ID!, process.env.BAND ?? "50", new RegExp(process.env.RE ?? ".")];
const d = load(id).asset;
console.log(JSON.stringify(d.rig.bones.map((b:any)=>({id:b.id,parent:b.parent,x:b.x,y:b.y,z:b.z,len:b.length,ang:b.angleDeg??b.restDeg})) ));
const r = d.representations[band];
for (const v of ["elevation","front","plan"]) for (const s of r.geometry[v]) if (re.test(s.id)) console.log(v, JSON.stringify(s).slice(0,400));
