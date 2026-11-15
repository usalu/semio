import { entryOf, loadSource, scaleMeshes, viewMasks, distanceField, restState, type View } from "./volumes-lib.ts";
const [spec, sc, view, n = "6"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const M = scaleMeshes(asset, Number(sc), src.volumes[sc!]);
const vm = viewMasks(view as View, restState(asset, Number(sc)), M.body);
const run = (from: any, to: any, label: string) => {
  const f = distanceField(to);
  const out: { d: number; x: number; y: number }[] = [];
  for (let i = 0; i < from.cells.length; i++) if (from.cells[i] && f[i]! > 0) out.push({ d: f[i]!, x: vm.box.minX + ((i % from.width) + 0.5) * vm.cell, y: vm.box.minY + (Math.floor(i / from.width) + 0.5) * vm.cell });
  out.sort((a, b) => b.d - a.d);
  console.log(label, out.slice(0, Number(n)).map((o) => `${(o.d * 1000).toFixed(0)}mm@(${o.x.toFixed(3)},${o.y.toFixed(3)})`).join("  "));
};
run(vm.truth, vm.volume, "drawn-not-3D");
run(vm.volume, vm.truth, "3D-not-drawn");
