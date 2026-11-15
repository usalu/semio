import { entryOf, loadSource, scaleMeshes, viewMasks, restState, distanceField, type View } from "./volumes-lib.ts";
const [spec, sc, view, cols = "150", dmm = "17"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const M = scaleMeshes(asset, Number(sc), src.volumes[sc!]);
const vm = viewMasks(view as View, restState(asset, Number(sc)), M.body);
const w = vm.truth.width, h = vm.truth.height;
const bnd = (m: any) => { const o = new (m.constructor as any)(m.box, m.cell); for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) { const k = j * w + i; if (m.cells[k] && (!m.cells[k - 1] || !m.cells[k + 1] || !m.cells[k - w] || !m.cells[k + w])) o.cells[k] = 1; } return o; };
const df = distanceField(bnd(vm.volume));
const dt = distanceField(bnd(vm.truth));
const lim = (Number(dmm) / 1000 / vm.cell) ** 2;
const step = Math.max(1, Math.round(w / Number(cols)));
let far = 0, tot = 0;
const bd = (m: Uint8Array, i: number, j: number) => m[j * w + i] === 1 && (i === 0 || !m[j * w + i - 1] || !m[j * w + i + 1] || j === 0 || !m[(j - 1) * w + i] || !m[(j + 1) * w + i]);
const farTruth = new Uint8Array(w * h), farVol = new Uint8Array(w * h);
for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) {
  if (bd(vm.truth.cells, i, j)) { tot++; if (df[j * w + i]! > lim) { far++; farTruth[j * w + i] = 1; } }
  if (bd(vm.volume.cells, i, j)) { tot++; if (dt[j * w + i]! > lim) { far++; farVol[j * w + i] = 1; } }
}
console.log(`cell ${(vm.cell * 1000).toFixed(1)} mm boundary cells ${tot} far(>${dmm}mm) ${far} = ${(100 * far / tot).toFixed(1)} %`);
for (let y = h - 1; y >= 0; y -= step) {
  let row = "";
  for (let x = 0; x < w; x += step) {
    let t = 0, v = 0, ft = 0, fv = 0;
    for (let dy = 0; dy < step; dy++) for (let dx = 0; dx < step; dx++) { const i = Math.max(0, y - dy) * w + Math.min(w - 1, x + dx); t += vm.truth.cells[i]!; v += vm.volume.cells[i]!; ft += farTruth[i]!; fv += farVol[i]!; }
    row += ft ? "R" : fv ? "B" : t && v ? "." : t ? "r" : v ? "b" : " ";
  }
  console.log(row);
}
