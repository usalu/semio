import { distanceField, entryOf, featureM, loadSource, merged, meshMask, poseState, scaleMeshes, viewMasks, VIEWS } from "./volumes-lib.ts";
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
  const dT = distanceField(vm.truth);
  const out: string[] = [];
  sm.body.forEach((mesh, i) => {
    const mk = meshMask(view, [mesh], vm.box, vm.cell);
    let worst = 0, at = "";
    for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k] && dT[k]! > worst) { worst = dT[k]!; at = `(${(vm.box.minX + (k % mk.width + 0.5) * vm.cell).toFixed(2)},${(vm.box.minY + (Math.floor(k / mk.width) + 0.5) * vm.cell).toFixed(2)})`; }
    if (worst - vm.cell > feature * 0.8) out.push(`${sm.solids.body[i].id}:${((worst - vm.cell) * 1000).toFixed(0)}mm${at}`);
  });
  console.log(view, "solids beyond drawing:", out.join("  "));
}
