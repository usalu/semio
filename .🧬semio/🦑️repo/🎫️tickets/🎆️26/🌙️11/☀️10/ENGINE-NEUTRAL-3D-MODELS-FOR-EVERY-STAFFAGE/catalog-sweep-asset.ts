/** 📐️ Asset-level evidence for the missing `sweep`: for all hints `w` and all hints `v`, the front and plan extents of the rest pose against `dimensions` and the silhouette IoU against the authored `volume` (independent 3D reference).
 * Usage: bun catalog-sweep-asset.ts <slug>... [--scale 50] */
import { VIEWS, analyse, loadAsset, resolveAsset, restState, type Json, type P2 } from "./volume-lib.ts";

const args = process.argv.slice(2);
const at = args.indexOf("--scale");
const denominator = at >= 0 ? Number(args[at + 1]) : 50;
const slugs = args.filter((a, i) => !a.startsWith("--") && args[i - 1] !== "--scale");
function withSweep(asset: Json, sweep: "v" | "w"): Json {
  const clone = structuredClone(asset);
  const walk = (x: Json): void => {
    if (Array.isArray(x)) x.forEach(walk);
    else if (x !== null && typeof x === "object") {
      if (x.kind === "extruded" && x.sweep === undefined) x.sweep = sweep;
      Object.values(x).forEach(walk);
    }
  };
  walk(clone.representations);
  return clone;
}
const span = (list: readonly P2[], axis: "x" | "y"): number => (list.length === 0 ? 0 : Math.max(...list.map((p) => p[axis])) - Math.min(...list.map((p) => p[axis])));
for (const spec of slugs) {
  const asset = loadAsset(resolveAsset(spec).path);
  const d = asset.dimensions;
  const line: string[] = [`${asset.id} dimensions width ${d.widthM} height ${d.heightM} depth ${d.depthM}`];
  for (const sweep of ["w", "v"] as const) {
    const patched = withSweep(asset, sweep);
    const state = restState(patched, denominator);
    const bodies = (v: "elevation" | "front" | "plan") => state[v].filter((s) => s.closed && ["body", "detail", "glazing"].includes(s.shape.role)).flatMap((s) => s.rings.flat());
    const f = bodies("front");
    const p = bodies("plan");
    const analysis = analyse(patched, asset.volume, denominator, state);
    line.push(`  ${sweep}: front lateral ${span(f, "x").toFixed(3)} x height ${span(f, "y").toFixed(3)}; plan forward ${span(p, "x").toFixed(3)} x lateral ${span(p, "y").toFixed(3)}; IoU elevation ${analysis.views.elevation.iou.toFixed(2)} front ${analysis.views.front.iou.toFixed(2)} plan ${analysis.views.plan.iou.toFixed(2)}`);
  }
  console.log(line.join("\n"));
}
