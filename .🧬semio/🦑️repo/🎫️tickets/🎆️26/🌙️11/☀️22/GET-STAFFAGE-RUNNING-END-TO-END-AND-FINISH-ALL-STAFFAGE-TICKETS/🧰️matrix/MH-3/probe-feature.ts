/** 🔬️ Which filled shapes of a scale print below the smallest feature, per pose and view, and what covers them: `bun probe-feature.ts <id|file.json> <scale> [shape id]` prints the width the print suitability measures (the visible part) for every shape below 0.25 mm, or for the named shape everywhere, with the nearer shapes that cover its box. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const load = (path: string): Promise<any> => import(join(S, "🔨️modules", path));
const { prepare, sourceOf } = await load("🎨️depiction/🧩️shapes/🟦️.ts");
const { evaluateDrawing } = await load("🧮️evaluation/🟦️.ts");
const { featureWidthMm, visibleFeatureWidthMm } = await load("🖨️representation/📐️measure/🟦️.ts");
const [arg, key, only] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
for (const pose of asset.poses.filter((candidate: any) => !candidate.key)) {
  const state = evaluateDrawing(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: Number(key) } });
  const source = sourceOf(state);
  for (const view of ["elevation", "front", "plan"]) {
    const prepared = prepare(state, view, source, 1000 / Number(key)) as any[];
    for (const p of prepared.filter((q) => q.closed && q.shape.filled && q.shape.role !== "ground" && (only === undefined || q.shape.id === only))) {
      const covers = prepared.filter((q) => q.occluder !== null && q.order > p.order && q.box.minX <= p.box.maxX && p.box.minX <= q.box.maxX && q.box.minY <= p.box.maxY && p.box.minY <= q.box.maxY);
      const visible = covers.length === 0 ? { widthMm: featureWidthMm(p.rings, 0.2, 10), boxMinMm: Math.min(p.box.maxX - p.box.minX, p.box.maxY - p.box.minY) } : visibleFeatureWidthMm(p.rings, covers.map((q) => ({ rings: q.occluder.rings, penMm: q.weightMm })), 0.2, 10);
      const width = visible === null ? null : Math.max(visible.widthMm, Math.min(visible.boxMinMm, 0.2)) + p.weightMm;
      if (only !== undefined || (width !== null && width < 0.25)) console.log(`${asset.id} 1:${key} ${pose.id} ${view} ${p.shape.id}: ${width === null ? "hidden" : `${width.toFixed(3)} mm`}; covered by ${covers.map((q) => q.shape.id).join(" ") || "nothing"}`);
    }
  }
}
