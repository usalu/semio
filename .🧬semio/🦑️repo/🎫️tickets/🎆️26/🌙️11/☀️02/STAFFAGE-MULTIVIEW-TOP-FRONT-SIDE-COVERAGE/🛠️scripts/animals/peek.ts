import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , id, scale, bone] = process.argv;
const a: any = catalog().find((x) => x.id === id)!;
console.log(JSON.stringify(a.rig.bones.filter((b: any) => !bone || b.id === bone || b.parent === bone)));
const g = a.representations[scale].geometry;
for (const v of ["elevation", "front", "plan"]) for (const s of g[v]) if (!bone || s.bone === bone) console.log(v, JSON.stringify(s));
