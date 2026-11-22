/** 🔬️ Prints the visible feature width of every filled shape of one representation of the candidate, per named pose and view, at a requested scale: `bun author-print-probe.ts <scale> <requested> [max mm]`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { PLANAR_VIEWPOINTS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { prepare, sourceOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureWidthMm, visibleFeatureWidthMm } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const scale = Number(process.argv[2]);
const requested = Number(process.argv[3]);
const limit = Number(process.argv[4] ?? 0.2);
const k = 1000 / requested;
for (const pose of asset.poses.filter((p: { key?: boolean }) => !p.key)) {
  const state = evaluateDrawing(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: scale } });
  const source = sourceOf(state);
  for (const view of PLANAR_VIEWPOINTS) {
    const prepared = prepare(state, view, source, k);
    for (const p of prepared.filter((q) => q.closed && q.shape.filled && q.shape.role !== "ground")) {
      const over = prepared.filter((q) => q.occluder !== null && q.order > p.order && q.box.minX <= p.box.maxX && p.box.minX <= q.box.maxX && q.box.minY <= p.box.maxY && p.box.minY <= q.box.maxY);
      const covers = over.map((q) => ({ rings: q.occluder!.rings, penMm: q.weightMm }));
      const atLeast = 0.2 - p.weightMm;
      let raw: number, boxMin: number;
      if (covers.length === 0) { raw = featureWidthMm(p.rings, 0.2, atLeast); boxMin = Math.min(p.box.maxX - p.box.minX, p.box.maxY - p.box.minY); }
      else { const v = visibleFeatureWidthMm(p.rings, covers, 0.2, atLeast); if (v === null) continue; raw = v.widthMm; boxMin = v.boxMinMm; }
      const width = Math.max(raw, Math.min(boxMin, 0.2)) + p.weightMm;
      if (width < limit) console.log(`[DEBUG] ${pose.id} ${view} ${p.shape.id} width ${width.toFixed(3)} raw ${raw.toFixed(3)} box ${boxMin.toFixed(3)} covers ${over.map((q) => q.shape.id + JSON.stringify(q.box)).join(' ')} self ${JSON.stringify(p.box)}`);
    }
  }
}
