import { loadDoc } from "./lib.ts";
import { stateOf } from "./ik.ts";
const [slug, json, ...ids] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
const s = stateOf(a, JSON.parse(json!), ["heel-l", "ball-l"]);
const f = (v: any) => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
for (const id of ids) { const b = s.bones[id]; console.log(id.padEnd(12), "o", f(b.frame.o), "tip", f(b.tip), "u", f(b.frame.u)); }
