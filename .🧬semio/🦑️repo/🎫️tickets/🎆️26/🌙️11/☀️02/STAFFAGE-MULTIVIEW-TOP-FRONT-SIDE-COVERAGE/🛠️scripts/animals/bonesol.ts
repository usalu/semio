import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , id, scale, ...bones] = process.argv;
const a = catalog().find((x) => x.id === id)!;
console.log(a.rig.bones.filter((b) => bones.includes(b.id)).map((b) => `${b.id} parent=${b.parent} x${b.x} y${b.y} z${b.z ?? 0} L${b.lengthM}`).join("\n"));
const r: any = (a.representations as any)[scale];
for (const v of ["elevation", "front", "plan"]) for (const s of r.geometry[v]) if (bones.includes(s.bone)) console.log(v, s.id, s.bone, s.kind, s.role, JSON.stringify(s.solid ?? null));
