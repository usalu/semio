import { VIEWS, distanceField, entryOf, loadSource, merged, meshMask, scaleMeshes, viewMasks, poseState, type Json } from "./volumes-lib.ts";
const [spec, sc, listName] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const scale = Number(sc);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const list = (listName ?? "structure") as "structure" | "thermal";
const meshes = sm[list];
const solids = sm.solids[list];
for (const view of VIEWS) {
  const vm = viewMasks(view, poseState(asset, scale), sm.body);
  const field = distanceField(vm.volume);
  const res: string[] = [];
  meshes.forEach((mesh, i) => {
    const mk = meshMask(view, [mesh], vm.box, vm.cell);
    let worst = 0;
    for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k]!) worst = Math.max(worst, field[k]!);
    if (worst > 0.004) res.push(`${solids[i]!.id}:${worst.toFixed(3)}`);
  });
  console.log(view, res.join(" "));
}
