/** 🔎️ Prints bone frames and anchors of a catalog human in one pose: bun probe.ts <slug> <pose> [bone ...] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [slug, pose, ...only] = process.argv.slice(2);
const asset = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`).json()).asset;
const s = evaluate(asset, { kind: "pose", pose: pose! });
const f = (v: { x: number; y: number; z: number }) => `(${v.x.toFixed(3)} ${v.y.toFixed(3)} ${v.z.toFixed(3)})`;
for (const [id, b] of Object.entries(s.bones)) if (!only.length || only.includes(id)) console.log(id.padEnd(20), "o", f(b.frame.o), "u", f(b.frame.u), "v", f(b.frame.v), "w", f(b.frame.w), "tip", f(b.tip));
for (const [id, a] of Object.entries(s.anchors)) console.log("anchor", id.padEnd(16), a.at.x.toFixed(3), a.at.y.toFixed(3), a.z.toFixed(3));
console.log("root", JSON.stringify(s.root));
