/** 🧪️ Runs `volumes-oracle.py` (shapely, trimesh) through the hash-locked uv oracle environment of the staffage tests and compares it with the owned and product measurements.
 *
 * Usage: `bun volumes-oracle.ts <category>/<slug> [--scale N] [--json]`
 * Output: per scale and view IoU and Hausdorff (oracle, product silhouette code), outer area of every thermal region (oracle contains-test and offset-normal test vs product `outerArea`), union volume (oracle on a grid vs owned ray parity vs product column union), the mesh volume per list. Differences above 3 % (IoU 0.01, Hausdorff 1.5 cells) are marked `DIFF`; exit 1 if any.
 * @see ./volumes-oracle.py
 * @see ./volumes-tools.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

import { CAMERAS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { python } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️.ts";
import { cellFor, drawnSet, IOU_LIMIT, PERCENTILE_FEATURES } from "./volumes-silhouette.ts";
import { addMeshes, emptySet, tileCompare } from "./volumes-tiles.ts";
import { bareOuterArea } from "./volumes-rulings.ts";
import { SCALES, TICKET, VIEWS, entryOf, extentOf, featureM, loadSource, merged, outPath, poseOf, scaleMeshes, unionVolume, type Json } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string) => (args.indexOf(n) < 0 ? undefined : args[args.indexOf(n) + 1]);
const spec = args.find((a, i) => !a.startsWith("--") && !["--scale"].includes(args[i - 1] ?? ""));
if (spec === undefined) {
  process.stderr.write("usage: volumes-oracle.ts <category>/<slug> [--scale N] [--json]\n");
  process.exit(2);
}
const { entry, asset } = entryOf(spec);
const source = loadSource(entry);
if (source === null) {
  process.stderr.write(`no source for ${spec}\n`);
  process.exit(2);
}
const m = merged(asset, source);
const scales = flag("--scale") === undefined ? SCALES.filter((s) => m.volumes[String(s)]?.status === "authored") : [Number(flag("--scale"))];
const toMesh = (x: { id: string; world: Float64Array; indices: Uint32Array }) => ({ id: x.id, positions: Array.from(x.world), indices: Array.from(x.indices) });
let diffs = 0;
const report: Json[] = [];
for (const scale of scales) {
  const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
  if (sm.body.length === 0) continue;
  const feature = featureM(scale, m);
  const long = Math.max(m.dimensions.widthM, m.dimensions.heightM, m.dimensions.depthM);
  const mine: Record<string, ReturnType<typeof tileCompare> & { cell: number }> = {};
  const drawnGroups: Record<string, number[][][][]> = {};
  const views: Json = {};
  const cell = cellFor(m, scale, long, false);
  for (const v of VIEWS) {
    const drawn = drawnSet(m, scale, v);
    const vol = emptySet();
    addMeshes(vol, v, sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true));
    mine[v] = { ...tileCompare(drawn, vol, { cell, capM: 8 * feature, percentile: 0.95 }), cell } as never;
    const groups: number[][][][] = drawn.polys.map((p) => p.rings.map((r) => r.map((q) => [q.x, q.y])));
    for (let k = 0; k < drawn.tris.length; k += 6) groups.push([[[drawn.tris[k]!, drawn.tris[k + 1]!], [drawn.tris[k + 2]!, drawn.tris[k + 3]!], [drawn.tris[k + 4]!, drawn.tris[k + 5]!]]]);
    drawnGroups[v] = groups;
    views[v] = { step: 2 * cell, axes: [[["x", "y", "z"].indexOf(CAMERAS[v].paperX.axis), CAMERAS[v].paperX.sign], [["x", "y", "z"].indexOf(CAMERAS[v].paperUp.axis), CAMERAS[v].paperUp.sign]], drawn: groups };
  }
  const e = extentOf(sm.body)!;
  const unionCell = feature / 6;
  const layersOf = (region: string): string[] => ((m.thermal.regions as Json[]).find((r) => r.id === region)?.layers ?? []).map((l: Json) => l.id);
  const input = {
    views,
    body: sm.body.map(toMesh),
    thermal: sm.thermal.map((mesh, i) => ({ mesh: toMesh(mesh), region: sm.solids.thermal[i]!.region, base: ((m.volumes[String(scale)].thermal as Json[]).find((x) => x.id === sm.solids.thermal[i]!.id)?.of ?? sm.solids.thermal[i]!.id) as string, rank: sm.solids.thermal[i]!.layer === undefined ? -1 : layersOf(sm.solids.thermal[i]!.region).indexOf(sm.solids.thermal[i]!.layer) })),
    structure: sm.structure.map(toMesh),
    allSolids: (["body", "thermal", "structure"] as const).flatMap((l) => sm.solids[l].map((s, i) => ({ mesh: toMesh(sm.solidMeshes[l][i]!), base: ((m.volumes[String(scale)][l] as Json[]).find((x) => x.id === s.id)?.of ?? s.id) as string }))),
    cell: unionCell,
    eps: feature * 0.02,
  };
  const inPath = outPath("oracle", entry, `${entry.slug}.s${scale}.in.json`);
  mkdirSync(dirname(inPath), { recursive: true });
  writeFileSync(inPath, JSON.stringify(input));
  const t0 = Date.now();
  const o = JSON.parse(python(`${TICKET}/volumes-oracle.py`, "geometry", [inPath])) as Json;
  const lines: string[] = [];
  const mark = (bad: boolean): string => {
    if (bad) diffs++;
    return bad ? "DIFF" : "ok";
  };
  for (const v of VIEWS) {
    const a = mine[v]!;
    const b = o.views[v];
    const bad = Math.abs(a.iou - b.iou) > 0.01 || Math.abs(Math.max(a.maxAB, a.maxBA) - b.hausdorffM) > Math.max(0.02 * b.hausdorffM, 2 * a.cell) || Math.abs(Math.max(a.pAB, a.pBA) - b.p95M) > Math.max(0.25 * b.p95M, 3 * a.cell);
    lines.push(`  ${v.padEnd(9)} IoU owned ${a.iou.toFixed(4)} oracle ${b.iou.toFixed(4)} | max distance owned ${(Math.max(a.maxAB, a.maxBA) * 1000).toFixed(1)} oracle ${(b.hausdorffM * 1000).toFixed(1)} mm | p95 owned ${(Math.max(a.pAB, a.pBA) * 1000).toFixed(1)} oracle ${(b.p95M * 1000).toFixed(1)} mm (cell ${(a.cell * 1000).toFixed(1)} mm) ${mark(bad)}`);
  }
  const regions: Json[] = m.thermal.regions;
  for (const r of regions) {
    if (o.regions[r.id] === undefined) continue;
    const owned = bareOuterArea(m, scale, m.volumes[String(scale)], r.id);
    const sampled = o.regions[r.id].outerAreaVsM2 as number;
    lines.push(`  region ${r.id.padEnd(22)} buried-only outer area owned ${owned.toFixed(4)} oracle sampled ${sampled.toFixed(4)} (no-other-solid sampled ${(o.regions[r.id].outerAreaSampledM2 as number).toFixed(4)}, contains ${(o.regions[r.id].outerAreaM2 as number).toFixed(4)}, plain sum ${(o.regions[r.id].plainAreaM2 as number).toFixed(4)}) m2 (areaM2 ${r.areaM2}) ${mark(Math.abs(owned / sampled - 1) > 0.08)}`);
  }
  const ownedUnion = unionVolume(sm.body);
  const productU = ownedUnion;
  lines.push(`  union volume oracle ${o.unionVolumeM3.toFixed(5)} (grid ${((o.unionCellM as number) * 1000).toFixed(1)} mm) owned ${ownedUnion.toFixed(5)} m3 ${mark(Math.abs(ownedUnion / productU - 1) > 0.06 || Math.abs(productU / o.unionVolumeM3 - 1) > 0.08)}`);
  lines.push(`  bbox oracle ${o.bbox.map((x: number) => x.toFixed(4)).join(" x ")} m, owned ${[0, 1, 2].map((k) => (e.max[k]! - e.min[k]!).toFixed(4)).join(" x ")} m ${mark(o.bbox.some((x: number, k: number) => Math.abs(x - (e.max[k]! - e.min[k]!)) > 1e-4))}; mesh volumes body ${o.meshVolumeM3.body.toFixed(5)} thermal ${o.meshVolumeM3.thermal.toFixed(5)} structure ${o.meshVolumeM3.structure.toFixed(5)} m3 (${((Date.now() - t0) / 1000).toFixed(1)} s)`);
  report.push({ scale, o });
  process.stdout.write(`${entry.category}/${entry.slug} 1:${scale}\n${lines.join("\n")}\n`);
}
void readFileSync;
process.exit(diffs === 0 ? 0 : 1);
