import { readFileSync } from "node:fs";
import { entryOf, extentOf, loadSource, memberBones, memberShapes, merged, meshArea, scaleMeshes } from "./volumes-lib.ts";

let outerArea: any = null;
try {
  const mod: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
  outerArea = mod.outerArea ?? null;
} catch {}
const [spec, file] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = file === undefined ? loadSource(entry) : JSON.parse(readFileSync(file, "utf8"));
const m = merged(asset, source);
const out: Record<string, unknown> = {};
for (const s of [50, 100, 200, 500]) {
  const v = source.volumes[String(s)];
  if (v.status !== "authored") continue;
  const sm = scaleMeshes(m, s, v);
  const rows = (list: "body" | "thermal" | "structure") =>
    sm.solids[list].map((sol: any, i: number) => {
      const mm = sm[list][i];
      const e = mm === undefined ? null : extentOf([mm]);
      return { id: sol.id, region: sol.region, layer: sol.layer, area: mm === undefined ? 0 : meshArea(mm), min: e?.min, max: e?.max };
    });
  const regionArea: Record<string, number> = {};
  for (const r of asset.thermal?.regions ?? []) {
    const layers: string[] = (r.layers ?? []).map((l: any) => l.id);
    const items = sm.solids.thermal.map((sol: any, i: number) => ({ sol, mesh: sm.thermal[i] })).filter((x: any) => x.sol.region === r.id && x.mesh !== undefined);
    if (items.length === 0) continue;
    const idx = (x: any) => (x.sol.layer === undefined ? -1 : layers.indexOf(x.sol.layer));
    const top = Math.max(...items.map(idx));
    regionArea[r.id] = outerArea === null ? items.filter((x: any) => idx(x) === top).reduce((n: number, x: any) => n + meshArea(x.mesh), 0) : outerArea(items.filter((x: any) => idx(x) === top).map((x: any) => ({ id: x.mesh.id, positions: x.mesh.world, indices: x.mesh.indices })));
  }
  out[String(s)] = { body: rows("body"), thermal: rows("thermal"), structure: rows("structure"), bbox: extentOf(sm.body), members: memberBones(asset, s), memberShapes: memberShapes(asset, s), regionArea };
}
process.stdout.write(JSON.stringify(out));
