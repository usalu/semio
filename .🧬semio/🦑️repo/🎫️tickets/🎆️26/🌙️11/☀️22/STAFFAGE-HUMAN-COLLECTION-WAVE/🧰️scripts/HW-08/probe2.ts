import { loadDoc, evaluate } from "./lib.ts";
const [slug, base, json, ...ids] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
const p = structuredClone(a.poses.find((x: any) => x.id === base));
p.id = "probe"; p.key = true; p.bones = { ...(base === "rest" ? {} : p.bones), ...JSON.parse(json!) };
if (base === "rest") for (const k of Object.keys(p.bones)) if (!JSON.parse(json!)[k]) delete p.bones[k];
a.poses.push(p);
const s = evaluate(a, { kind: "pose", pose: "probe" });
const f = (v: any) => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
for (const id of ids) { const b = s.bones[id]!; console.log(id.padEnd(16), "o", f(b.frame.o), "u", f(b.frame.u), "v", f(b.frame.v), "w", f(b.frame.w), "tip", f(b.tip)); }
