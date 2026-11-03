import { readdirSync, readFileSync } from "node:fs";
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const d of readdirSync(root)) {
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset;
  const rb = a.rig.bones.find((b: any) => b.id === a.rig.root);
  const regs = new Set<string>(); for (const s of a.representations["200"].geometry.elevation) regs.add(`${s.region ?? (s.regions ? s.regions.join("+") : "-")}`);
  console.log(`## ${d} root ${rb.id} ang ${rb.angleDeg} x${rb.x} y${rb.y} ${rb.joint.type}; regions ${[...regs]}`);
  for (const s of a.symbols) console.log(`  sym ${s.id} pens ${JSON.stringify(s.pens)} shapes el ${JSON.stringify(s.geometry.elevation.map((x:any)=>[x.id,x.bone,x.role,x.filled,x.kind]))} pl ${JSON.stringify(s.geometry.plan.map((x:any)=>[x.id,x.bone,x.role,x.filled,x.kind,x.region]))} note ${JSON.stringify(s.dimensionalNote.en).slice(0,100)}`);
}
