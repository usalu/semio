/** 🧷️ Loads a candidate asset document (`R05_FILE`) in place of the catalog asset for the copies of the round 3 tools that `rework-05-tool.sh` makes, so a new rig can be looked at before the catalog file is written.
 * @see ./r3-lib.ts
 * @see ./rework-05-tool.sh */
import { readFileSync } from "node:fs";
import { entryOf, type CatalogEntry, type Json } from "./volumes-lib.ts";
import { rigAt, queryOf } from "./r3-lib.ts";

export function loadFile(spec: string | undefined, usage: string): { entry: CatalogEntry; asset: Json; catalog: Json; source: Json } {
  if (spec === undefined || process.env.R05_FILE === undefined) {
    process.stderr.write(`usage (R05_FILE set): ${usage}\n`);
    process.exit(2);
  }
  const { entry } = entryOf(spec);
  const asset = JSON.parse(readFileSync(process.env.R05_FILE, "utf8")).asset;
  return { entry, asset, catalog: asset, source: { volumes: asset.volumes } };
}

if (import.meta.main) {
  const [file, ...ids] = process.argv.slice(2);
  const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
  const out: Record<string, unknown> = {};
  const queries = ids.length > 0 ? ids : asset.poses.map((p: Json) => p.id);
  for (const id of queries) {
    const [clip, t] = id.split("@");
    const rig = rigAt(asset, 50, t === undefined ? queryOf(asset, id, undefined, undefined) : queryOf(asset, undefined, clip, t));
    out[id] = Object.fromEntries(rig.bones.map((b) => [b.id, { ...rig.frames[b.id], tip: b.tip }]));
  }
  process.stdout.write(`${JSON.stringify(out)}\n`);
}
