import { readFileSync } from "node:fs";
import { Mask, distanceField, entryOf, featureM, loadSource, meshArea, meshMask, merged, projectedPoints, scaleMeshes, viewMasks, poseState, VIEWS, type Json } from "./volumes-lib.ts";

const [spec, scaleS, what] = process.argv.slice(2);
const scale = Number(scaleS);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const vol = m.volumes[String(scale)];
const sm = scaleMeshes(m, scale, vol);
const feature = featureM(scale, m);
if (what === "structure" || what === undefined) {
  const state = poseState(m, scale);
  for (const view of VIEWS) {
    const vm = viewMasks(view, state, sm.body);
    const field = distanceField(vm.volume);
    sm.structure.forEach((mesh, i) => {
      const s = sm.solids.structure[i];
      if (s.part === "support") return;
      const mk = meshMask(view, [mesh], vm.box, vm.cell);
      let worst = 0;
      for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k] && field[k]! > worst) worst = field[k]!;
      if (worst - vm.cell > feature) console.log(view, s.id, s.part, "outside by", ((worst - vm.cell) * 1000).toFixed(1), "mm");
    });
  }
}
if (what === "thermal" || what === undefined) {
  const per = new Map<string, number>();
  sm.thermal.forEach((mesh, i) => {
    const s = sm.solids.thermal[i];
    per.set(s.region, (per.get(s.region) ?? 0) + meshArea(mesh));
  });
  for (const r of m.thermal.regions) console.log(r.id, "area", (per.get(r.id) ?? 0).toFixed(4), "want", r.areaM2);
  sm.thermal.forEach((mesh, i) => console.log("  ", sm.solids.thermal[i].id, sm.solids.thermal[i].region, meshArea(mesh).toFixed(4)));
}
