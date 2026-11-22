import { loadDoc, evalOffsets, frameOf, rd } from "./lib.ts";
const a = (await loadDoc("jogger")).asset;
const p = (s: any, ids: string[]) => { for (const id of ids) { const f = frameOf(s, id); console.log(id.padEnd(12), "o", f.o.map((x) => rd(x, 3)).join(","), "u", f.u.map((x) => rd(x, 2)).join(","), "v", f.v.map((x) => rd(x, 2)).join(",")); } for (const k of ["contact-l", "heel-l", "contact-r"]) console.log(k, rd(s.anchors[k].at.x, 3), rd(s.anchors[k].at.y, 3), rd(s.anchors[k].z, 3)); };
console.log("== stand"); p(evalOffsets(a, {}, "stretch-stand"), ["pelvis", "thigh-l", "shank-l", "foot-l", "toe-l", "upper-arm-l"]);
console.log("== thigh-r -40, foot-l +30, pelvis -20, upper-arm-l +80"); p(evalOffsets(a, { "thigh-r": { angleDeg: -40 }, "foot-l": { angleDeg: 30 }, pelvis: { angleDeg: -20 }, "upper-arm-l": { angleDeg: 80 } }, "stretch-stand"), ["pelvis", "thigh-r", "foot-l", "upper-arm-l"]);
console.log(JSON.stringify(a.rig.bones.filter((b: any) => b.mass).map((b: any) => [b.id, b.mass.kg])));
