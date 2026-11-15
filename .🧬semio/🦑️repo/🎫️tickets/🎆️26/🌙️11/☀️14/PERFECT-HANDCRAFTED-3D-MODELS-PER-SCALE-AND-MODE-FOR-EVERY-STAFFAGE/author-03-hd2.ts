import { entryOf, loadSource, merged, scaleMeshes, viewMasks, poseState, distanceField, VIEWS } from "./volumes-lib.ts";
const [spec, sc, thr] = [process.argv[2]!, Number(process.argv[3] ?? 50), Number(process.argv[4] ?? 0.03)];
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const sm: any = scaleMeshes(a, sc, source.volumes[String(sc)]);
for (const view of VIEWS) {
  const vm = viewMasks(view, poseState(a, sc), sm.body);
  const dv = distanceField(vm.volume);
  const dt = distanceField(vm.truth);
  const w = vm.truth.width;
  for (const [label, d, mask] of [["truth-missing", dv, vm.truth], ["3D-extra", dt, vm.volume]] as const) {
    const bins = new Map<string, number>();
    for (let i = 0; i < d.length; i++) if ((mask as any).cells[i] && d[i]! > thr) {
      const x = vm.box.minX + ((i % w) + 0.5) * vm.cell, y = vm.box.minY + (Math.floor(i / w) + 0.5) * vm.cell;
      const k = `(${(Math.round(x / 0.2) * 0.2).toFixed(1)},${(Math.round(y / 0.2) * 0.2).toFixed(1)})`;
      bins.set(k, (bins.get(k) ?? 0) + 1);
    }
    console.log(view, label, [...bins.entries()].sort((p, q) => q[1] - p[1]).slice(0, 8).map(([k, n]) => `${k}:${n}`).join(" "));
  }
}
