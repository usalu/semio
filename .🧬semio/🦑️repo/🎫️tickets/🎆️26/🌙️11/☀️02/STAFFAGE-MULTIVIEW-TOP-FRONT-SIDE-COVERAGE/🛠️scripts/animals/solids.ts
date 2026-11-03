import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , id, scale] = process.argv;
const a = catalog().find((x) => x.id === id)!;
const r: any = (a.representations as any)[scale];
for (const v of ["elevation", "front", "plan"]) for (const s of r.geometry[v]) if (s.solid) console.log(v, s.id, s.bone, s.kind, JSON.stringify(s.solid));
console.log(a.rig.bones.filter((b) => ["lumbar", "thorax", "neck", "pelvis"].includes(b.id)).map((b) => `${b.id} x${b.x} y${b.y} ang${b.angleDeg} L${b.lengthM}`).join(" | "));
