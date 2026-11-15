import { readFileSync } from "node:fs";
import { LISTS, entryOf, extentOf, featureM, loadSource, merged, poseState, scaleMeshes, type V3 } from "./volumes-lib.ts";
import { exposedArea } from "./volumes-rulings.ts";

const [spec, scaleArg, file, regionFilter] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = file === undefined || file === "-" ? loadSource(entry) : JSON.parse(readFileSync(file, "utf8"));
const m = merged(asset, source);
const scale = Number(scaleArg);
const vol = source.volumes[String(scale)];
const sm = scaleMeshes(m, scale, vol);
const frames = poseState(m, scale).frames;
const items: Record<string, any[]> = { body: [], thermal: [], structure: [] };
for (const l of LISTS) {
  sm.solids[l].forEach((solid: any, i: number) => {
    const mesh = sm.solidMeshes[l][i];
    if (mesh === undefined) return;
    const raw = (vol[l] as any[]).find((s) => s.id === solid.id);
    const f = frames[solid.bone] ?? { u: [1, 0, 0] as V3, v: [0, 1, 0] as V3, w: [0, 0, 1] as V3 };
    const e = extentOf([mesh])!;
    items[l]!.push({ id: solid.id, base: raw?.of ?? solid.id, mesh, box: e, solid, frame: f });
  });
}
const all = [...items.body!, ...items.thermal!, ...items.structure!];
const feature = featureM(scale, m);
const regions: Record<string, any> = Object.fromEntries((m.thermal?.regions ?? []).map((r: any) => [r.id, r]));
const rows: any[] = [];
for (const it of items.thermal!) {
  const r = it.solid.region;
  const R = regions[r];
  const layers: string[] = (R?.layers ?? []).map((x: any) => x.id);
  const rank = it.solid.layer === undefined ? -1 : layers.indexOf(it.solid.layer);
  const top = Math.max(-1, ...items.thermal!.filter((x) => x.solid.region === r).map((x) => (x.solid.layer === undefined ? -1 : layers.indexOf(x.solid.layer))));
  const { area, total } = exposedArea([it], all, [], feature);
  rows.push({ id: it.id, bone: it.solid.bone, region: r, layer: it.solid.layer, outer: rank === top, exposed: area, mesh: total });
}
const byRegion: Record<string, number> = {};
for (const r of rows) if (r.outer) byRegion[r.region] = (byRegion[r.region] ?? 0) + r.exposed;
console.log("REGION", JSON.stringify(Object.fromEntries(Object.entries(byRegion).map(([k, v]) => [k, [Number(v.toFixed(4)), regions[k]?.areaM2]]))));
for (const r of rows) if (r.outer && (regionFilter === undefined || r.region === regionFilter) && r.exposed > 0.003) console.log(`${r.region.padEnd(16)} ${r.id.padEnd(34)} ${r.bone.padEnd(12)} exp ${r.exposed.toFixed(4)} mesh ${r.mesh.toFixed(4)}`);
