import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const id = process.argv[2]!;
const denominator = Number(process.argv[3] ?? 50);
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const asset = catalog.assets.find((a) => a.id === id)!;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } });
console.log("dimensions", asset.dimensions);
for (const viewpoint of ["elevation", "front", "plan"] as const) {
  const rows = state.shapes[viewpoint].filter((s) => s.shape.role !== "ground").map((s) => {
    const p = s.rings.flat();
    return { id: s.shape.id, role: s.shape.role, x: [Math.min(...p.map((q) => q.x)), Math.max(...p.map((q) => q.x))].map((v) => +v.toFixed(3)), y: [Math.min(...p.map((q) => q.y)), Math.max(...p.map((q) => q.y))].map((v) => +v.toFixed(3)) };
  });
  const xs = rows.flatMap((r) => r.x);
  const ys = rows.flatMap((r) => r.y);
  console.log(viewpoint, "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
  const widest = [...rows].sort((a, b) => b.x[1] - b.x[0] - (a.x[1] - a.x[0])).slice(0, 3);
  console.log(JSON.stringify(widest));
}
