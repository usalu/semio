import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , id, scale, view, boneFilter] = process.argv;
const a = catalog().find((x) => x.id === id)!;
const r: any = (a.representations as any)[scale];
for (const s of r.geometry[view]) if (!boneFilter || boneFilter.split(",").includes(s.bone)) console.log(JSON.stringify(s));
