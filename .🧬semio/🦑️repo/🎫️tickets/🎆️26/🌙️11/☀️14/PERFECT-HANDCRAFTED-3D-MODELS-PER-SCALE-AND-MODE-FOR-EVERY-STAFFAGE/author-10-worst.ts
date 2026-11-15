import { entryOf, loadSource, merged, scaleMeshes, poseState, compareView, type View } from "./volumes-lib.ts";
const spec = process.argv[2]!; const scale = Number(process.argv[3] ?? 50);
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const sm = scaleMeshes(a, scale, source.volumes[String(scale)]);
const st: any = poseState(a, scale);
for (const view of ["elevation", "front", "plan"] as View[]) {
  const m = compareView(view, st[view], sm.body);
  console.log(view, "iou", m.iou.toFixed(3), "recall", m.recall.toFixed(3), "precision", m.precision.toFixed(3));
  console.log("  3D outside:", m.worstSolids.slice(0, 6).map((s) => `${s.id}:${(s.outsideM2 * 1e4).toFixed(0)}cm2`).join("  "));
  console.log("  drawn missing:", m.worstShapes.slice(0, 6).map((s) => `${s.id}:${(s.missingM2 * 1e4).toFixed(0)}cm2`).join("  "));
}
