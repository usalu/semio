import { entryOf, loadSource, merged, scaleMeshes, viewMasks, poseState, distanceField, VIEWS } from "./volumes-lib.ts";
const spec = process.argv[2]!;
const scale = Number(process.argv[3] ?? 50);
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const sm = scaleMeshes(a, scale, source.volumes[String(scale)]);
for (const view of VIEWS) {
  const vm = viewMasks(view, poseState(a, scale), sm.body);
  const dv = distanceField(vm.volume);
  const dt = distanceField(vm.truth);
  const w = vm.truth.width;
  const rep = (label: string, d: Float32Array, mask: any) => {
    const items: { d: number; x: number; y: number }[] = [];
    for (let i = 0; i < d.length; i++) if (mask.cells[i] && d[i]! > 0) items.push({ d: d[i]!, x: vm.box.minX + ((i % w) + 0.5) * vm.cell, y: vm.box.minY + (Math.floor(i / w) + 0.5) * vm.cell });
    items.sort((p, q) => q.d - p.d);
    const picked: typeof items = [];
    for (const it of items) { if (picked.length >= 5) break; if (picked.every((p) => Math.hypot(p.x - it.x, p.y - it.y) > 0.15)) picked.push(it); }
    console.log(view, label, picked.map((p) => `d=${p.d.toFixed(3)} @(${p.x.toFixed(2)},${p.y.toFixed(2)})`).join("  "));
  };
  rep("truth-not-in-3D", dv, vm.truth);
  rep("3D-not-in-truth", dt, vm.volume);
}
