import { entryOf, loadSource, scaleMeshes, viewMasks, restState, Mask, type View } from "./volumes-lib.ts";
const [spec, sc, view, cols = "100"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const M = scaleMeshes(asset, Number(sc), src.volumes[sc!]);
const vm = viewMasks(view as View, restState(asset, Number(sc)), M.body);
const step = Math.max(1, Math.round(vm.truth.width / Number(cols)));
let areaT = 0, areaV = 0, both = 0;
for (let i = 0; i < vm.truth.cells.length; i++) { if (vm.truth.cells[i]) areaT++; if (vm.volume.cells[i]) areaV++; if (vm.truth.cells[i] && vm.volume.cells[i]) both++; }
const cellM2 = vm.cell * vm.cell;
console.log(`truth ${(areaT * cellM2).toFixed(4)} m2  vol ${(areaV * cellM2).toFixed(4)}  both ${(both * cellM2).toFixed(4)}  redArea ${((areaT - both) * cellM2).toFixed(4)} blueArea ${((areaV - both) * cellM2).toFixed(4)}`);
for (let y = vm.truth.height - 1; y >= 0; y -= step) {
  let row = "";
  for (let x = 0; x < vm.truth.width; x += step) {
    let t = 0, v = 0;
    for (let dy = 0; dy < step; dy++) for (let dx = 0; dx < step; dx++) { const i = Math.max(0, y - dy) * vm.truth.width + Math.min(vm.truth.width - 1, x + dx); t += vm.truth.cells[i]!; v += vm.volume.cells[i]!; }
    const tt = t > step * step / 2, vv = v > step * step / 2;
    row += tt && vv ? "#" : tt ? "r" : vv ? "b" : (t || v) ? (t && v ? "+" : t ? "." : ",") : " ";
  }
  console.log(row);
}
