import { writeFileSync } from "node:fs";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { prepare, sourceOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { featureWidthMm } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const out: string[] = [];
for (const [id, scale, shapeId, view] of [["standing-adult", 500, "sil-hand", "elevation"], ["standing-phone-user", 500, "phone", "elevation"]] as const) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const prepared = prepare(state, view, sourceOf(state), 1000 / scale);
  const p = prepared.find((q) => q.shape.id === shapeId);
  if (!p) { out.push(`${id} ${shapeId}: not found; ids ${prepared.map((q) => q.shape.id).slice(0, 40).join(",")}`); continue; }
  const xs = p.rings.flat().map((v) => v.x); const ys = p.rings.flat().map((v) => v.y);
  out.push(`${id} 1:${scale} ${shapeId}: box ${(Math.max(...xs) - Math.min(...xs)).toFixed(4)} x ${(Math.max(...ys) - Math.min(...ys)).toFixed(4)} mm, pen ${p.weightMm}, filled ${p.shape.filled} kind ${p.shape.kind}`);
  for (const atLeast of [Infinity, 0.2 - p.weightMm + 1e-9, 0.05]) out.push(`   featureWidth(minFeature 0.2, atLeast ${atLeast}) = ${featureWidthMm(p.rings, 0.2, atLeast)}`);
  for (const f of [2, 4, 8]) { const big = p.rings.map((r) => r.map((v) => ({ x: v.x * f, y: v.y * f }))); out.push(`   scaled x${f}: exact ${featureWidthMm(big, 0.2)}`); }
}
writeFileSync(process.env.FW_LOG!, out.join("\n") + "\n");
