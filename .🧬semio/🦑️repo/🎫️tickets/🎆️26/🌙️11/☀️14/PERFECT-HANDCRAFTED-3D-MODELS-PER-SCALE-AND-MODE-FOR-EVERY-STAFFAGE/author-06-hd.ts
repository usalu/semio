import { VIEWS, distanceField, entryOf, loadSource, merged, scaleMeshes, viewMasks, poseState } from "./volumes-lib.ts";
const [spec, sc] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const scale = Number(sc);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
for (const view of VIEWS) {
  const vm = viewMasks(view, poseState(asset, scale), sm.body);
  const fv = distanceField(vm.volume);
  const ft = distanceField(vm.truth);
  const w = vm.truth.width;
  const worst = (cells: Uint8Array, f: Float32Array) => {
    let best = 0, bi = -1;
    for (let i = 0; i < cells.length; i++) if (cells[i]! && f[i]! > best) { best = f[i]!; bi = i; }
    if (bi < 0) return "none";
    const x = vm.box.minX + ((bi % w) + 0.5) * vm.cell, y = vm.box.minY + (Math.floor(bi / w) + 0.5) * vm.cell;
    return `${best.toFixed(3)}@(${x.toFixed(3)},${y.toFixed(3)})`;
  };
  console.log(view, "drawn-not-3d", worst(vm.truth.cells, fv), "3d-not-drawn", worst(vm.volume.cells, ft));
}
