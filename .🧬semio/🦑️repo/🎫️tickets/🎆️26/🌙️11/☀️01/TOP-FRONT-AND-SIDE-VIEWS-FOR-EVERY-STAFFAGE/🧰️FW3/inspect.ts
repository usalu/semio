import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { prepare, sourceOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { featureWidthMm, visibleFeatureWidthMm } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [id, scale, shapeId] = process.argv.slice(2);
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const asset = catalog.assets.find((a) => a.id === id)!;
for (const pose of asset.poses.filter((p) => !p.key))
  for (const vp of ["elevation", "front", "plan"] as const) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: Number(scale) } });
    const prepared = prepare(state, vp, sourceOf(state), 1000 / Number(scale));
    const p = prepared.find((q) => q.shape.id === shapeId);
    if (!p || !p.closed || !p.shape.filled) continue;
    const covers = prepared.filter((q) => q.occluder && q.order > p.order && q.box.minX <= p.box.maxX && p.box.minX <= q.box.maxX && q.box.minY <= p.box.maxY && p.box.minY <= q.box.maxY).map((q) => ({ rings: q.occluder!.rings, penMm: q.weightMm }));
    const whole = featureWidthMm(p.rings, 0.2);
    const vis = covers.length ? visibleFeatureWidthMm(p.rings, covers, 0.2) : null;
    console.log(pose.id, vp, "pen", p.weightMm, "box", (p.box.maxX - p.box.minX).toFixed(3), (p.box.maxY - p.box.minY).toFixed(3), "whole", whole.toFixed(3), "covers", covers.length, covers.map((c) => c.penMm).join("/"), "vis", vis && JSON.stringify(vis));
  }
