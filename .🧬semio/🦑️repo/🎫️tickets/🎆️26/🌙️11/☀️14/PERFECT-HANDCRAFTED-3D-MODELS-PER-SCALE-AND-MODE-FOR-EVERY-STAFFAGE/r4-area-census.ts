/** 🌊️ Census of the product thermal area rule over every catalog asset (source merged over it), for a before/after comparison of the C31 flood.
 *
 * Usage: `bun r4-area-census.ts <label> [--shard k/n] [--snapshot save|load] [--only category/slug,...]`
 * Writes `🗑️generated/r4-area/census/<label>/<category>.<slug>.json` = `{ seconds, measurements: [{ scale, name, value, limit, ok }] }` per asset.
 * `--snapshot save` stores the merged asset under `🗑️generated/r4-area/snapshot/`, `--snapshot load` measures that stored asset, so that two runs judge the very same input while the catalog changes live.
 * @see ./r4-area-flood.md */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED, catalogEntries, loadAsset, loadSource, merged } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const label = args[0]!;
const flag = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
const [shard, shards] = (flag("--shard") ?? "0/1").split("/").map(Number) as [number, number];
const snapshot = flag("--snapshot");
const only = flag("--only")?.split(",");
const out = join(GENERATED, "r4-area", "census", label);
const store = join(GENERATED, "r4-area", "snapshot");
mkdirSync(out, { recursive: true });
mkdirSync(store, { recursive: true });
const mod = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const entries = catalogEntries().filter((e) => only === undefined || only.includes(`${e.category}/${e.slug}`));
for (const [k, entry] of entries.entries()) {
  if (k % shards !== shard) continue;
  const name = `${entry.category}.${entry.slug}.json`;
  const kept = join(store, name);
  let asset: any;
  if (snapshot === "load") {
    if (!existsSync(kept)) continue;
    asset = JSON.parse(readFileSync(kept, "utf8"));
  } else {
    const base = loadAsset(entry.path);
    const source = loadSource(entry) ?? (base.volumes === undefined ? null : { volumes: base.volumes });
    if (source === null) continue;
    asset = merged(base, source);
    if (snapshot === "save") writeFileSync(kept, JSON.stringify(asset));
  }
  const t0 = Date.now();
  let measurements: unknown[] = [];
  let error: string | undefined;
  try {
    measurements = mod.measureVolumes(asset as never, { rules: ["volumes-thermal-coverage"] as never }).map((m: any) => ({ scale: m.scale, name: m.name, value: m.value, limit: m.limit, ok: m.ok }));
  } catch (e) {
    error = (e as Error).message.split("\n")[0];
  }
  const seconds = (Date.now() - t0) / 1000;
  writeFileSync(join(out, name), JSON.stringify({ seconds, error, measurements }));
  process.stderr.write(`[census ${label}] ${entry.category}/${entry.slug} ${seconds.toFixed(1)} s${error === undefined ? "" : ` ERROR ${error}`}\n`);
}
