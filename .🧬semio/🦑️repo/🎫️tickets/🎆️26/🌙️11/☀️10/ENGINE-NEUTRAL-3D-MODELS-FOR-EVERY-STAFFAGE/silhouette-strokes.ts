/** 🔬️ Per stroke shape: width the check gave it and its ink outside the allowed zone: `bun silhouette-strokes.ts <slug> <view>`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, memberOf, strokeQuad, measureView, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: 50 } } as never) as any;
const m = measureView(asset, state, worldSolids(asset, state.bones, 50), a[1] as never, 50, 800);
const cell = m.cell / 4;
const cutM = new Mask(m.box, cell), fullM = new Mask(m.box, cell), drawn = new Mask(m.box, cell);
for (const g of m.reference) cutM.fill(g);
for (const g of m.full) fullM.fill(g);
for (const s of state.shapes[a[1]!] as any[]) if (s.closed && s.shape.filled !== false) drawn.fill(s.rings);
for (const s of state.shapes[a[1]!] as any[]) {
  if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
  if (s.closed && s.shape.filled !== false) continue;
  const pen = ((s.shape.penMm ?? asset.representations["50"].pens[s.shape.role] ?? 0) * 50) / 1000;
  let out = 0, tot = 0, wsum = 0, n = 0;
  for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) { const mm = memberOf(r[i], r[i + 1], [cutM, fullM], drawn, pen, 0.15); const w = mm.width; wsum += w; n++; const q = new Mask(m.box, cell); q.fill([strokeQuad(r[i], r[i + 1], w)]); for (let k = 0; k < q.cells.length; k++) if (q.cells[k]) { tot++; if (!fullM.cells[k]) out++; } } }
  console.log(s.shape.id.padEnd(22), s.closed ? "closed" : "open", "pen", pen.toFixed(4), "avgW", (wsum / n).toFixed(4), "out/tot", (out * cell * cell).toFixed(4), (tot * cell * cell).toFixed(4));
}
if (a[2]) for (const s of state.shapes[a[1]!] as any[]) if (s.shape.id === a[2]) {
  const pen = ((s.shape.penMm ?? asset.representations["50"].pens[s.shape.role] ?? 0) * 50) / 1000;
  for (const ring of s.rings) for (let i = 0; i + 1 < ring.length; i++) { const mm = memberOf(ring[i], ring[i + 1], [cutM, fullM], drawn, pen, 0.15); console.log(i, ring[i].x.toFixed(3), ring[i].y.toFixed(3), "w", mm.width.toFixed(4), "shift", mm.shift.toFixed(4), "inCut", cutM.has(ring[i].x, ring[i].y), "inFull", fullM.has(ring[i].x, ring[i].y)); }
}
