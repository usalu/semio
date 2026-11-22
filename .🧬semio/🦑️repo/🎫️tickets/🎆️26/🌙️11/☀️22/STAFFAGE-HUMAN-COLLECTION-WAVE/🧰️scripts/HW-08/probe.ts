import { loadDoc, evaluate } from "./lib.ts";
const [slug, poseId, ...ids] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
const s = evaluate(a, { kind: "pose", pose: poseId! });
const f = (v: any) => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
console.log("root", s.root);
for (const id of ids.length ? ids : Object.keys(s.bones)) { const b = s.bones[id]!; console.log(id.padEnd(20), "o", f(b.frame.o), "u", f(b.frame.u), "v", f(b.frame.v), "tip", f(b.tip)); }
for (const [id, x] of Object.entries(s.anchors)) console.log("anchor", id, x.at.x.toFixed(3), x.at.y.toFixed(3), x.z.toFixed(3));
