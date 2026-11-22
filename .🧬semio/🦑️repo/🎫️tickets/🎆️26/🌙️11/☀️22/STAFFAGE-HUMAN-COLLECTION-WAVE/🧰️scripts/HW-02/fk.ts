/** 🦴️ Prints the world frames of bones (origin, tip, u, v, w) of an asset in a pose or clip instant, plus the drawn extents of every view at a scale.
 *  bun fk.ts <file.json> <pose | clip@time> [bone,bone,...] [scale] */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [file, q, only, scaleArg] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
if (q === "rest") asset.poses = [...asset.poses, { id: "rest", title: { en: "Rest", de: "Ruhe" }, bones: {}, ground: "none", contacts: [], thermal: asset.poses[0].thermal, loads: [] }];
const query = q!.includes("@") ? { kind: "clip", clip: q!.split("@")[0], timeS: Number(q!.split("@")[1]) } : { kind: "pose", pose: q };
const scale = Number(scaleArg ?? 50);
const state = process.env.GEOM ? evaluate(asset, query as never, { scale: { denominator: scale } } as never) : evaluate(asset, query as never);
const f = (v: { x: number; y: number; z: number }) => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
if (process.env.JSON) { console.log(JSON.stringify(Object.fromEntries(Object.entries(state.bones).map(([id, b]: [string, any]) => [id, { ...b.frame, tip: b.tip }])))); process.exit(0); }
const want = only && only !== "-" ? new Set(only.split(",")) : undefined;
for (const [id, b] of Object.entries(state.bones) as [string, any][]) {
  if (want && !want.has(id)) continue;
  console.log(`${id.padEnd(18)} o${f(b.frame.o)} tip${f(b.tip)} u${f(b.frame.u)} v${f(b.frame.v)} w${f(b.frame.w)}`);
}
console.log("root", JSON.stringify(state.root), "bounds", JSON.stringify(state.bounds));
