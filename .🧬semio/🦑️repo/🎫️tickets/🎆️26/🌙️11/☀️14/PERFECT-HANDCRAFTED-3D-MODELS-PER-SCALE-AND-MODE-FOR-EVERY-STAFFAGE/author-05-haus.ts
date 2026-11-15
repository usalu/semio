import { distanceField, entryOf, featureM, loadSource, merged, poseState, scaleMeshes, viewMasks, VIEWS } from "./volumes-lib.ts";
const [spec, scaleS, only] = process.argv.slice(2);
const scale = Number(scaleS);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const state = poseState(m, scale);
const feature = featureM(scale, m);
for (const view of VIEWS) {
  if (only && only !== view) continue;
  const vm = viewMasks(view, state, sm.body);
  const w = Math.round((vm.box.maxX - vm.box.minX) / vm.cell);
  const dT = distanceField(vm.truth), dV = distanceField(vm.volume);
  const rep = (name: string, from: Uint8Array | boolean[], field: Float32Array) => {
    const arr: { d: number; x: number; y: number }[] = [];
    for (let i = 0; i < from.length; i++) if (from[i] && field[i]! - vm.cell > feature * 1.0) arr.push({ d: field[i]! - vm.cell, x: vm.box.minX + (i % vm.truth.width + 0.5) * vm.cell, y: vm.box.minY + (Math.floor(i / vm.truth.width) + 0.5) * vm.cell });
    arr.sort((a, b) => b.d - a.d);
    const picks: typeof arr = [];
    for (const a of arr) if (picks.every((p) => Math.hypot(p.x - a.x, p.y - a.y) > 0.12) && picks.length < 6) picks.push(a);
    console.log(view, name, picks.map((p) => `${(p.d * 1000).toFixed(0)}mm@(${p.x.toFixed(2)},${p.y.toFixed(2)})`).join(" "));
  };
  rep("3D-only(blue)", vm.volume.cells as never, dT);
  rep("drawn-only(red)", vm.truth.cells as never, dV);
}
