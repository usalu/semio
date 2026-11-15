import { readFileSync } from "node:fs";
import { entryOf, loadSource, merged, scaleMeshes, poseState, viewMasks, distanceField, projectWorld, featureM, meshesAt, type View } from "./volumes-lib.ts";
const spec = process.argv[2]!; const scale = Number(process.argv[3] ?? 50); const thr = Number(process.argv[4] ?? 0);
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const sm = scaleMeshes(a, scale, source.volumes[String(scale)]);
const st = poseState(a, scale);
const feat = featureM(scale, a);
const limit = thr > 0 ? thr : 2 * feat;
for (const view of ["elevation", "front", "plan"] as View[]) {
  const vm = viewMasks(view, st as never, sm.body);
  const dT = distanceField(vm.truth), dV = distanceField(vm.volume);
  const W = vm.truth.width;
  const clusters: { kind: string; d: number; x: number; y: number }[] = [];
  for (let i = 0; i < vm.truth.cells.length; i++) {
    const x = vm.box.minX + ((i % W) + 0.5) * vm.cell, y = vm.box.minY + (Math.floor(i / W) + 0.5) * vm.cell;
    if (vm.volume.cells[i] && dT[i]! - vm.cell > limit * 0.7) clusters.push({ kind: "3D-only", d: dT[i]!, x, y });
    if (vm.truth.cells[i] && dV[i]! - vm.cell > limit * 0.7) clusters.push({ kind: "drawn-only", d: dV[i]!, x, y });
  }
  clusters.sort((p, q) => q.d - p.d);
  const picked: typeof clusters = [];
  for (const c of clusters) { if (picked.length >= 8) break; if (picked.every((p) => Math.hypot(p.x - c.x, p.y - c.y) > 0.05 || p.kind !== c.kind)) picked.push(c); }
  console.log(view, "clusters>" + (limit * 0.7).toFixed(3));
  for (const c of picked) {
    const owners = sm.body.filter((m) => { let hit = false; for (let k = 0; k < m.world.length; k += 3) { const p = projectWorld(view, [m.world[k]!, m.world[k + 1]!, m.world[k + 2]!]); if (Math.hypot(p.x - c.x, p.y - c.y) < 0.02) { hit = true; break; } } return hit; }).map((m) => m.id);
    console.log("  ", c.kind.padEnd(10), "d=" + c.d.toFixed(3), "at", c.x.toFixed(3), c.y.toFixed(3), c.kind === "3D-only" ? owners.slice(0, 4).join(",") : "");
  }
}
