/** 🔍️ Where does the Hausdorff distance come from? Usage: bun author-14-diag.ts <category>/<slug> <scale> [view] [--cross] */
import { VIEWS, distanceField, entryOf, featureM, loadSource, merged, poseState, scaleMeshes, viewMasks } from "./volumes-lib.ts";

const [spec, scaleArg, viewArg] = process.argv.slice(2);
const scale = Number(scaleArg);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const st = poseState(m, scale);
const feat = featureM(scale, asset);
for (const view of VIEWS) {
  if (viewArg !== undefined && !viewArg.startsWith("--") && viewArg !== view) continue;
  const vm = viewMasks(view, st, sm.body);
  const dT = distanceField(vm.volume);
  const dV = distanceField(vm.truth);
  const report = (name: string, src: any, dist: Float32Array) => {
    const buckets = new Map<string, { n: number; max: number; x: number; y: number }>();
    for (let j = 0; j < src.height; j++) for (let i = 0; i < src.width; i++) {
      const k = j * src.width + i;
      if (!src.cells[k]) continue;
      const d = dist[k]! - vm.cell;
      if (d <= feat * 0.5) continue;
      const x = vm.box.minX + (i + 0.5) * vm.cell;
      const y = vm.box.minY + (j + 0.5) * vm.cell;
      const key = `${Math.round(x / 0.1)},${Math.round(y / 0.1)}`;
      const b = buckets.get(key) ?? { n: 0, max: 0, x, y };
      b.n++;
      if (d > b.max) { b.max = d; b.x = x; b.y = y; }
      buckets.set(key, b);
    }
    const top = [...buckets.values()].sort((a, b) => b.max - a.max).slice(0, 6);
    process.stdout.write(`${view} ${name} (limit ${(2 * feat).toFixed(3)}): ${top.map((b) => `(${b.x.toFixed(2)},${b.y.toFixed(2)}) d=${b.max.toFixed(3)} n=${b.n}`).join("  ")}\n`);
  };
  report("DRAWN-not-in-3D", vm.truth, dT);
  report("3D-not-drawn  ", vm.volume, dV);
}
