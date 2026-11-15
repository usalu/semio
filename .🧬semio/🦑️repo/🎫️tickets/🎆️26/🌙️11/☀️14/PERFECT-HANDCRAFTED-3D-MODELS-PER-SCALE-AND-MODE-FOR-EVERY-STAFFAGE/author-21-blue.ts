import { entryOf, loadSource, scaleMeshes, viewMasks, restState, meshMask, type View } from "./volumes-lib.ts";
const [spec, sc, view] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const M = scaleMeshes(asset, Number(sc), src.volumes[sc!]);
const vm = viewMasks(view as View, restState(asset, Number(sc)), M.body);
const c2 = vm.cell * vm.cell;
const rows: [string, number, number][] = [];
M.body.forEach((m, i) => {
  const mk = meshMask(view as View, [m], vm.box, vm.cell);
  let out = 0, tot = 0;
  for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k]) { tot++; if (!vm.truth.cells[k]) out++; }
  rows.push([M.solids.body[i]!.id, out * c2, tot * c2]);
});
// union-exclusive outside: only cells covered by no other
rows.sort((a, b) => b[1] - a[1]);
for (const r of rows.slice(0, 14)) console.log(r[0].padEnd(24), "outside", r[1].toFixed(3), "of", r[2].toFixed(3));
