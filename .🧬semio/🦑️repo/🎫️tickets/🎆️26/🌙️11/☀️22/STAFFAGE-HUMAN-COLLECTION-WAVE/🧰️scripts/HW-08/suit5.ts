/** 🔬️ Visible feature width of one shape per pose and view with its occluders (bun suit5.ts slug scale shapeId). */
import { loadDoc } from "./lib.ts";
const SR = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { prepare, sourceOf } = await import(`${SR}/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluateDrawing } = await import(`${SR}/🧮️evaluation/🟦️.ts`);
const { featureWidthMm, visibleFeatureWidthMm } = await import(`${SR}/🖨️representation/📐️measure/🟦️.ts`);
const [slug, sc, id] = process.argv.slice(2);
const a = loadDoc(slug!).asset; const k = 1000 / Number(sc);
for (const pose of a.poses.filter((p: any) => !p.key)) {
  const state = evaluateDrawing(a, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: Number(sc) } });
  const source = sourceOf(state);
  for (const vp of ["elevation", "front", "plan"]) {
    const prepared = prepare(state, vp, source, k);
    const p = prepared.find((x: any) => x.shape.id === id); if (!p) continue;
    const covers = prepared.filter((q: any) => q.occluder !== null && q.order > p.order && q.box.minX <= p.box.maxX && p.box.minX <= q.box.maxX && q.box.minY <= p.box.maxY && p.box.minY <= q.box.maxY);
    const raw = featureWidthMm(p.rings, 0.2, 5);
    const vis = covers.length ? visibleFeatureWidthMm(p.rings, covers.map((q: any) => ({ rings: q.occluder.rings, penMm: q.weightMm })), 0.2, 5) : null;
    console.log(pose.id, vp, "raw", raw?.toFixed?.(3), "visible", vis ? vis.widthMm.toFixed(3) : "-", "weight", p.weightMm, "covers", covers.map((q: any) => q.shape.id).join(","));
  }
}
