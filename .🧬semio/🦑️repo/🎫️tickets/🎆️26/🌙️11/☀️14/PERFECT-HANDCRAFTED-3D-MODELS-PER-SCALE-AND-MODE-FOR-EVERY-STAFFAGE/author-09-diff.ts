import { projectWorld, entryOf, loadSource, merged, scaleMeshes, poseState, compareView, viewMasks, distanceField, pointInRing, VIEWS, type Json } from "./volumes-lib.ts";

const [spec, scaleArg] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const scale = Number(scaleArg);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const st = poseState(m, scale);
for (const v of VIEWS) {
  const r = compareView(v, st[v], sm.body);
  process.stdout.write(`${v} iou=${r.iou.toFixed(3)} recall=${r.recall.toFixed(3)} precision=${r.precision.toFixed(3)}\n  solids(outside): ${r.worstSolids.map((s) => `${s.id}:${(s.outsideM2 * 1e4).toFixed(0)}cm2/${s.precision}`).join(" ")}\n  shapes(missing): ${r.worstShapes.map((s) => `${s.id}:${(s.missingM2 * 1e4).toFixed(0)}cm2/${s.coverage}`).join(" ")}\n`);
  const vm = viewMasks(v, st, sm.body);
  for (const [name, from, to] of [["drawn-far-from-3D", vm.truth, vm.volume], ["3D-far-from-drawn", vm.volume, vm.truth]] as const) {
    const field = distanceField(to);
    let worst = 0;
    let at = -1;
    for (let i = 0; i < from.cells.length; i++) if (from.cells[i]! && field[i]! > worst) {
      worst = field[i]!;
      at = i;
    }
    if (at < 0) continue;
    const x = vm.box.minX + ((at % from.width) + 0.5) * vm.cell;
    const y = vm.box.minY + (Math.floor(at / from.width) + 0.5) * vm.cell;
    const names = (st[v] as Json[]).filter((s) => s.closed && s.rings.some((ring: Json) => pointInRing({ x, y }, ring))).map((s) => s.shape.id);
    const covering = sm.body.filter((mesh) => {
      for (let t = 0; t < mesh.indices.length; t += 3) {
        const q = [0, 1, 2].map((k) => projectWorld(v, [mesh.world[mesh.indices[t + k]! * 3]!, mesh.world[mesh.indices[t + k]! * 3 + 1]!, mesh.world[mesh.indices[t + k]! * 3 + 2]!]));
        if (pointInRing({ x, y }, q)) return true;
      }
      return false;
    }).map((mesh) => mesh.id);
    process.stdout.write(`  ${name}: ${(worst * 1000).toFixed(0)}mm at (${x.toFixed(3)}, ${y.toFixed(3)}) shapes[${names.join(",")}] solids[${covering.join(",")}]\n`);
  }
}
