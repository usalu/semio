/** 🔺️ Census of the triangle budgets under candidate tessellation rules: per catalog asset and scene the triangles of every solid at the candidate's quality (the kernel `tessellateSolid`), the largest scene at 1:50 against the oracle budget (400 000) and the sum over all scenes (what a GLB holds), and whether a mode grows towards a coarser scale.
 *
 * Usage: `$BUN r4-tess-census.ts [category/slug ...]`; writes `🗑️generated/r4-tessellation/census.json` and prints one line per asset.
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts — `auditGeometry`, the budget this census predicts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { scenesOf, tessellateSolid, type Quality } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";

type Rule = (largest: number, scale: number) => Quality;
const old: Rule = (largest, scale) => {
  const finest = Math.min(0.02, Math.max(0.0005, largest / 800));
  return { chordM: Math.min(Math.max(scale * 0.0001, finest), Math.max(finest, largest / 60)), minSegments: scale <= 20 ? 12 : 8, maxSegments: scale <= 20 ? 64 : scale <= 100 ? 48 : 32 };
};
const feature = (scale: number): number => scale * 0.00015;
const floored = (caps: Record<number, number>, chord: (largest: number, scale: number) => number, min = 12, segments = 24): Rule => (largest, scale) => ({ chordM: chord(largest, scale), minSegments: min, maxSegments: caps[scale] ?? 32, round: { radiusM: feature(scale), segments } });
const FALLING = { 50: 64, 100: 48, 200: 40, 500: 32 };
const RULES: Record<string, Rule> = {
  old,
  "e-size3200": floored(FALLING, (largest) => largest / 3200),
  "f-size1600": floored(FALLING, (largest) => largest / 1600),
  "g-brief": floored(FALLING, (largest, scale) => Math.min(largest / 3200, 0.02 * feature(scale))),
  "h-size3200-high": floored({ 50: 64, 100: 64, 200: 48, 500: 48 }, (largest) => largest / 3200),
};
const only = process.argv.slice(2).map((id) => id.split("/").pop()!);
const out = join(import.meta.dirname, "🗑️generated", "r4-tessellation");
mkdirSync(out, { recursive: true });
const rows: Record<string, unknown>[] = [];
const names = [...Object.keys(RULES), "product"];
for (const entry of await assetPathsFromFiles()) {
  if (only.length > 0 && !only.includes(entry.folder)) continue;
  const { asset } = await readStreamedAsset(entry);
  const largest = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
  const document = await modelOfAsset(asset, { poses: [asset.presentation.defaultPose], clips: [] });
  const row: Record<string, unknown> = { id: entry.folder, largest };
  const started = Date.now();
  for (const name of names) {
    const scenes: Record<string, number> = {};
    for (const scene of scenesOf(document)) {
      const scale = Number(/^s(\d+)-/u.exec(scene.id)![1]);
      const quality = name === "product" ? qualityOf(asset, scale) : RULES[name]!(largest, scale);
      const members = new Set(scene.nodes);
      let triangles = 0;
      for (const solid of document.solids) if (members.has(solid.node)) triangles += tessellateSolid(solid, quality).indices.length / 3;
      scenes[scene.id] = triangles;
    }
    const total = Object.values(scenes).reduce((a, b) => a + b, 0);
    const finest = Math.max(...Object.entries(scenes).filter(([id]) => id.startsWith("s50-")).map(([, n]) => n), 0);
    const grows = ["body", "thermal", "structure"].some((mode) => {
      const series = Object.entries(scenes).filter(([id]) => id.endsWith(`-${mode}`)).sort((a, b) => Number(/\d+/u.exec(a[0])![0]) - Number(/\d+/u.exec(b[0])![0])).map(([, n]) => n);
      return series.some((n, i) => i > 0 && n > series[i - 1]!);
    });
    row[name] = { total, finest, grows, scenes };
  }
  rows.push(row);
  console.log(`${entry.folder.padEnd(48)} ${names.map((n) => `${n}=${(row[n] as { total: number }).total}/${(row[n] as { finest: number }).finest}${(row[n] as { grows: boolean }).grows ? "!" : ""}`).join(" ")} (${Date.now() - started} ms)`);
}
writeFileSync(join(out, only.length > 0 ? "census-some.json" : "census.json"), JSON.stringify(rows, null, 1) + "\n");
