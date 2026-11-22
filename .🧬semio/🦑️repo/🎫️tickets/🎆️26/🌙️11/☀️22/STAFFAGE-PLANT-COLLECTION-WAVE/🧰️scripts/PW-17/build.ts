/** 🏗️ PW-17 build: `bun build.ts <asset-id> [--dry]` builds the asset of a species file, measures its 1:50 body in the default pose for `dimensions`, writes `S/🗂️catalog/🌳️vegetation/<id>/🔣️.json` and the manifest row `T/📥️manifest-rows/<id>.json` (with `--dry` only `T/🗑️generated/PW-17/<id>/🔣️.json`).
 * @see ./lib.ts */
import { join } from "node:path";
import { GENERATED, STAFFAGE, WAVE, boxOf, manifestRow, r3, worldMeshes, writeJson, type Json } from "./lib.ts";

const id = process.argv[2];
if (id === undefined) {
  process.stderr.write("usage: bun build.ts <potted-monstera|fiddle-leaf-fig|snake-plant> [--dry]\n");
  process.exit(2);
}
const species = await import(`./${id}.ts`);
const { doc, distinction, notes } = species.build() as { doc: Json; distinction: Json; notes: string[] };
for (const n of notes) process.stdout.write(`${n}\n`);
const asset = doc.asset;
if (species.finish) await species.finish(asset);
const box = boxOf(worldMeshes(asset, 50, asset.volumes["50"].body));
asset.dimensions = { heightM: r3(box.max[1]), widthM: r3(box.max[0] - box.min[0]), depthM: r3(box.max[2] - box.min[2]), massKg: asset.dimensions.massKg };
process.stdout.write(`[DEBUG] 1:50 body box x ${box.min[0].toFixed(3)}..${box.max[0].toFixed(3)} y ${box.min[1].toFixed(3)}..${box.max[1].toFixed(3)} z ${box.min[2].toFixed(3)}..${box.max[2].toFixed(3)} -> ${JSON.stringify(asset.dimensions)}\n`);
for (const s of ["50", "100", "200", "500"]) {
  const v = asset.volumes[s];
  const rep = asset.representations[s];
  const b = boxOf(worldMeshes(asset, Number(s) as never, v.body));
  process.stdout.write(`[DEBUG] 1:${s} body ${v.body.length} thermal ${v.thermal.length} structure ${v.structure.length} shapes ${rep.status === "authored" ? rep.geometry.elevation.length + "/" + rep.geometry.front.length + "/" + rep.geometry.plan.length : rep.status} box ${(b.max[0] - b.min[0]).toFixed(3)} x ${b.max[1].toFixed(3)} x ${(b.max[2] - b.min[2]).toFixed(3)}\n`);
}
const dry = process.argv.includes("--dry");
const path = dry ? join(GENERATED, id, "🔣️.json") : join(STAFFAGE, "🗂️catalog/🌳️vegetation", id, "🔣️.json");
writeJson(path, doc);
if (!dry) writeJson(join(WAVE, "📥️manifest-rows", `${id}.json`), manifestRow(asset, distinction));
process.stdout.write(`[DEBUG] written ${path}\n`);
