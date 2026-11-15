/** 🌱️ Seeds the authoring source `volumes/<category>/<slug>.json` = `{ "volumes": { "50", "100", "200", "500" } }` from the asset.
 *
 * Usage: `bun volumes-seed.ts <category>/<slug> [--force]`
 * 1:50 `body` = the existing `asset.volume.solids` visible at 1:50 without `region`, `regions` and `maxDenominator`; `thermal` = `{ id, of, region }` references from the old bindings; `structure` empty; coarser authored scales are empty authored volumes; scales whose representation is unavailable mirror it. Never overwrites an existing source without `--force`.
 * @see ./design.md
 * @see ./volumes-tools.md */
import { existsSync } from "node:fs";

import { SCALES, entryOf, sourcePath, writeJson, type Json } from "./volumes-lib.ts";

export const strip = (s: Json): Json => {
  const { region: _r, regions: _rs, maxDenominator: _m, ...rest } = s;
  return rest;
};

export const regionsOf = (s: Json): string[] => [...(s.region === undefined ? [] : [s.region]), ...(s.regions ?? [])];

export const thermalRefs = (solids: readonly Json[]): Json[] =>
  solids.flatMap((s) => {
    const rs = regionsOf(s);
    return rs.map((region) => ({ id: rs.length === 1 ? `th-${s.id}` : `th-${region}-${s.id}`, of: s.id, region }));
  });

export function seed(asset: Json): Json {
  const volumes: Record<string, Json> = {};
  for (const scale of SCALES) {
    const rep = asset.representations?.[String(scale)];
    if (rep?.status !== "authored") {
      volumes[String(scale)] = { status: "unavailable", denominator: scale };
      continue;
    }
    if (scale === 50 && asset.volume?.solids !== undefined) {
      const visible = asset.volume.solids.filter((s: Json) => s.maxDenominator === undefined || s.maxDenominator >= 50);
      volumes["50"] = { status: "authored", denominator: 50, body: visible.map(strip), thermal: thermalRefs(visible), structure: [], decisions: [] };
    } else volumes[String(scale)] = { status: "authored", denominator: scale, body: [], thermal: [], structure: [], decisions: [] };
  }
  return { volumes };
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const spec = args.find((a) => !a.startsWith("--"));
  if (spec === undefined) {
    process.stderr.write("usage: volumes-seed.ts <category>/<slug> [--force]\n");
    process.exit(2);
  }
  const { entry, asset } = entryOf(spec);
  const path = sourcePath(entry);
  if (existsSync(path) && !args.includes("--force")) {
    process.stderr.write(`${path} exists; pass --force to overwrite\n`);
    process.exit(1);
  }
  const source = seed(asset);
  writeJson(path, source, true);
  const v = source.volumes;
  process.stdout.write(`seeded ${path}: ${Object.entries(v).map(([k, x]: [string, Json]) => `1:${k} ${x.status}${x.body === undefined ? "" : ` body ${x.body.length} thermal ${x.thermal.length}`}`).join("; ")}\n`);
}
