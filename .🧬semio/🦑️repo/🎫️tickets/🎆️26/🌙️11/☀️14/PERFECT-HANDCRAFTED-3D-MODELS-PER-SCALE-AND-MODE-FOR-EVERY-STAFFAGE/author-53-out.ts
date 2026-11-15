import { entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks, distanceField, meshMask, type Json, type View } from "./volumes-lib.ts";
const [spec, sc] = [process.argv[2]!, Number(process.argv[3])];
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!);
const state = poseState(m, sc);
const sm = scaleMeshes(m, sc, m.volumes[String(sc)]);
for (const view of ["elevation", "front", "plan"] as View[]) {
  const vm = viewMasks(view, state, sm.body);
  const df = distanceField(vm.volume);
  const rows: [string, number][] = [];
  sm.structure.forEach((mesh: any, i: number) => {
    const mk = meshMask(view, [mesh], vm.box, vm.cell);
    let worst = 0;
    for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k] && df[k]! > worst) worst = df[k]!;
    if (worst > 0.005) rows.push([sm.solids.structure[i].id, worst]);
  });
  console.log(view, rows.sort((a, b) => b[1] - a[1]).slice(0, 6).map(([id, d]) => `${id} ${d.toFixed(3)}`).join("; "));
}
