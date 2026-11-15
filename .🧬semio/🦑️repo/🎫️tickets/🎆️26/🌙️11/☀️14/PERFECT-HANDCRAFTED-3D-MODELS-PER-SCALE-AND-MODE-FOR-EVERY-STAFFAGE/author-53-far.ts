import { entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks, distanceField, Mask, type Json, type View } from "./volumes-lib.ts";
const [spec, sc, thr] = [process.argv[2]!, Number(process.argv[3]), Number(process.argv[4] ?? 0.017)];
const { entry, asset } = entryOf(spec);
const src = loadSource(entry)!;
const m = merged(asset, src);
const state = poseState(m, sc);
const sm = scaleMeshes(m, sc, m.volumes[String(sc)]);
const bodyIds: string[] = sm.solids.body.map((s: Json) => s.id);
const woody = (id: string): boolean => !/^fol|fill|cross/.test(id);
for (const view of ["elevation", "front", "plan"] as View[]) {
  const vm = viewMasks(view, state, sm.body);
  const w = vm.truth.width, h = vm.truth.height;
  const edge = (mk: Mask): Mask => { const e = new Mask(mk.box, mk.cell); for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { const k = j * w + i; if (mk.cells[k] && (i === 0 || j === 0 || i === w - 1 || j === h - 1 || !mk.cells[k - 1] || !mk.cells[k + 1] || !mk.cells[k - w] || !mk.cells[k + w])) e.cells[k] = 1; } return e; };
  const et = edge(vm.truth), ev = edge(vm.volume);
  const hardMeshes = sm.body.filter((_: Json, i: number) => woody(bodyIds[i]!));
  const hm = viewMasks(view, state, hardMeshes).volume;
  const near = (k: number, r: number): boolean => { const i = k % w, j = Math.floor(k / w); for (let dj = -r; dj <= r; dj++) for (let di = -r; di <= r; di++) { const ii = i + di, jj = j + dj; if (ii >= 0 && jj >= 0 && ii < w && jj < h && hm.cells[jj * w + ii]) return true; } return false; };
  const bins = new Map<string, number>();
  let total = 0, far = 0;
  for (const [name, from, to] of [["drawn->3D", et, ev], ["3D->drawn", ev, et]] as const) {
    const f = distanceField(to);
    for (let k = 0; k < from.cells.length; k++) if (from.cells[k]) {
      total++;
      if (f[k]! > thr) {
        far++;
        const x = vm.box.minX + ((k % w) + 0.5) * vm.cell, y = vm.box.minY + (Math.floor(k / w) + 0.5) * vm.cell;
        const key = `${name} ${(Math.round(x / 0.5) * 0.5).toFixed(1)},${(Math.round(y / 0.5) * 0.5).toFixed(1)}${near(k, 12) ? " woody" : ""}`;
        bins.set(key, (bins.get(key) ?? 0) + 1);
      }
    }
  }
  console.log(view, "cell", (vm.cell * 1000).toFixed(2), "mm boundary cells", total, "farther than", thr, ":", far, ((100 * far) / total).toFixed(1), "%");
  console.log([...bins.entries()].sort((a, b) => b[1] - a[1]).slice(0, 14).map(([k, v]) => `  ${k}: ${v}`).join("\n"));
}
