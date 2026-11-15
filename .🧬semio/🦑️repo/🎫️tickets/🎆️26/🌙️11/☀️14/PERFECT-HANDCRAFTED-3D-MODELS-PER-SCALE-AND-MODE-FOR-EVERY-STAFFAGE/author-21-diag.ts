import { entryOf, loadSource, scaleMeshes, featureM, VIEWS } from "./volumes-lib.ts";
import { addMeshes, emptySet, tileCompare } from "./volumes-tiles.ts";
const [spec, scaleS] = process.argv.slice(2);
const scale = Number(scaleS);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const sm = scaleMeshes(asset, scale, src.volumes[String(scale)]);
const f = featureM(scale, asset);
const bodyAll = (view: any) => { const s = emptySet(); addMeshes(s, view, sm.body); return s; };
for (const view of ["elevation", "front", "plan"] as const) {
  const B = bodyAll(view);
  sm.structure.forEach((m, i) => {
    const s = emptySet(); addMeshes(s, view, [m]);
    const r = tileCompare(s, B, { cell: f / 2, capM: 20 * f });
    if (r.maxAB > f) console.log(view, "structure", sm.solids.structure[i]!.id, "outside", (r.maxAB * 1000).toFixed(0), "mm");
  });
}
const ext = (list: any[]) => { const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9]; for (const m of list) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]); hi[k] = Math.max(hi[k]!, m.world[i + k]); } return { lo, hi }; };
const e = ext(sm.body);
sm.thermal.forEach((m, i) => { const x = ext([m]); const out = Math.max(...[0, 1, 2].map((k) => Math.max(e.lo[k]! - x.lo[k]!, x.hi[k]! - e.hi[k]!))); if (out > f) console.log("thermal", sm.solids.thermal[i]!.id, "outside box", (out * 1000).toFixed(0), "mm"); });
