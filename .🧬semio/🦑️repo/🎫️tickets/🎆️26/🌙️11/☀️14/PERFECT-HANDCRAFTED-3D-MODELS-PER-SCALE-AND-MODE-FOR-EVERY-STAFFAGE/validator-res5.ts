import { readFileSync } from "node:fs";
const fx = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧊️volumes-rules/🔣️.json", "utf8"));
const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
for (const op of fx.outlines.patch) { const segs = op.path.split("/").slice(1); let n = a; for (const s of segs.slice(0, -1)) n = Array.isArray(n) ? (s.startsWith("id=") ? n.find((i: any) => i.id === s.slice(3)) : n[Number(s)]) : n[s]; n[segs.at(-1)!] = op.value; }
const { evaluate } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts`);
for (const dir of ["🧊️volumes", "🧊️volumes-next"]) {
  const { measureView, worldSolids } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${dir}/🌓️silhouette/🟦️.ts`);
  const state = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 100 } });
  const solids = worldSolids(a, state.bones, 100, "body");
  const out = [Infinity, 64, 96, 128, 200].map((t) => { const m = measureView(a, state, solids, "front", 100, 400, 0.015, t); return `${t}:${(m.p95M * 1000).toFixed(1)}`; });
  console.log(dir, out.join(" "));
}
