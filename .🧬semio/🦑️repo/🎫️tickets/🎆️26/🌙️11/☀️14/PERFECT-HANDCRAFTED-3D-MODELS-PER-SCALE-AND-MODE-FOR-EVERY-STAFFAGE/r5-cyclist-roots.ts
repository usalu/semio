/** 🛠️ Round 5 root fixes of humans/cyclist in the catalog asset, stated as final values (idempotent): the 1:50 front view draws the carrier stays 10.5 mm (real 10 mm tube, drawn 8 mm before) and the platform tube 12 mm instead of 9 mm; the 1:100 front view drops the `carrier` detail block (the rear end of the rack behind the rider, of which the front view shows only 14.5 mm slivers between the rack bars, under 0.2 mm at 1:75); the 1:500 front and plan views draw each tyre as a 72 mm band (the scale's smallest feature, the real tyre being 40 mm) so the band keeps 0.2 mm with its pen at the end of the range (1:1000). Writes the candidate `🗑️generated/r5-cyclist/🔣️.json`; with `--write` re-reads the catalog file and writes it (two-space JSON as the catalog keeps it; `volumes-insert.py` afterwards).
 * Usage: `bun r5-cyclist-roots.ts [--write]`
 * @see ./r5-cyclist-ranges.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type Json = Record<string, any>;
const CATALOG = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/cyclist/🔣️.json");
const CARRIER_FRONT_50: number[][] = [[-0.021, 0.7175], [-0.014, 0.718], [-0.012, 0.722], [-0.0225, 0.758], [0.0225, 0.758], [0.012, 0.722], [0.014, 0.718], [0.021, 0.7175], [0.0235, 0.7215], [0.0335, 0.7585], [0.034, 0.7585], [0.036, 0.7625], [0.036, 0.7695], [0.0665, 0.7935], [0.072, 0.7945], [0.072, 0.8055], [-0.072, 0.8055], [-0.072, 0.7945], [-0.0665, 0.7935], [-0.036, 0.7695], [-0.036, 0.7625], [-0.034, 0.7585], [-0.0335, 0.7585], [-0.0235, 0.7215]];
const TYRE_BAND_500 = 0.036;

const doc: Json = JSON.parse(readFileSync(CATALOG, "utf8"));
const g = doc.asset.representations;
const shape = (scale: string, view: string, id: string): Json => {
  const s = (g[scale].geometry[view] as Json[]).find((x) => x.id === id);
  if (s === undefined) throw new Error(`no shape ${id} in 1:${scale} ${view}`);
  return s;
};
shape("50", "front", "carrier").points = CARRIER_FRONT_50;
g["100"].geometry.front = (g["100"].geometry.front as Json[]).filter((x) => x.id !== "carrier");
for (const view of ["front", "plan"]) for (const id of ["front-tyre", "rear-tyre"]) Object.assign(shape("500", view, id).solid, { z0: -TYRE_BAND_500, z1: TYRE_BAND_500 });
const text = `${JSON.stringify(doc, null, 2)}\n`;
const out = join(import.meta.dir, "🗑️generated", "r5-cyclist");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "🔣️.json"), text);
if (process.argv.includes("--write")) {
  const now: Json = JSON.parse(readFileSync(CATALOG, "utf8"));
  now.asset.representations = doc.asset.representations;
  writeFileSync(CATALOG, `${JSON.stringify(now, null, 2)}\n`);
  console.log(`written ${CATALOG}`);
} else console.log(join(out, "🔣️.json"));
