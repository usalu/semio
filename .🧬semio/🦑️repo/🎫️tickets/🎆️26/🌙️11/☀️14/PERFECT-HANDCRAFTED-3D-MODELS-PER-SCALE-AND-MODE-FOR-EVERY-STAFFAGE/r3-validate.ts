/** ✅️ Validates one whole asset as the product does: the catalog asset with the author's `volumes` source merged over it against the JSON schema (ajv, `AssetDocument`) and through the catalog validator `validateAsset` (references, rig, poses, clips, representations, thermal, structure, volumes analytic rules, exports) with the whole catalog and the environment presets as context.
 *
 * Usage: `bun r3-validate.ts <category>/<slug> [--volumes] [--catalog-only] [--json]` or `bun r3-validate.ts --file <asset document json> [--volumes] [--json]` (a candidate file as it is, for root fixes before they are written to the catalog)
 * Prints every issue (`SCHEMA path: message`, `ASSET code path: message`), then one summary line; exit 1 on any issue, 2 on usage errors. `--volumes` adds the measured volume rules (`checkVolumes`, slow; the numbers of `volumes-check.ts --product`). `--catalog-only` validates the catalog file as it is, without the source.
 * @see ./r3-tools.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts */
import { readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import Ajv from "ajv";

import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { loadEnvironments } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { STAFFAGE, catalogEntries, loadAsset, type Json } from "./volumes-lib.ts";
import { cli, load } from "./r3-lib.ts";

const USAGE = "r3-validate.ts <category>/<slug> | --file <asset document json> [--volumes] [--catalog-only] [--json]";

if (import.meta.main) {
  const { spec, has, flag } = cli(["--file"]);
  const file = flag("--file");
  const loaded = file === undefined ? load(spec, USAGE) : { entry: { category: "file", folder: "file", slug: basename(dirname(resolve(file))), path: resolve(file) }, asset: loadAsset(resolve(file)), catalog: loadAsset(resolve(file)), source: null };
  const asset: Json = has("--catalog-only") ? loaded.catalog : loaded.asset;
  const lines: string[] = [];
  const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const conforms = ajv.compile({ $ref: `${schema.$id}#/$defs/AssetDocument` });
  const document = { ...JSON.parse(readFileSync(loaded.entry.path, "utf8")), asset };
  const schemaErrors = conforms(document) ? [] : (conforms.errors ?? []);
  const seen = new Set<string>();
  for (const e of schemaErrors) {
    if (["oneOf", "anyOf", "if"].includes(e.keyword)) continue;
    const line = `SCHEMA ${e.instancePath || "/"}: ${e.message}${e.params === undefined ? "" : ` ${JSON.stringify(e.params)}`}`;
    if (!seen.has(line)) lines.push(line);
    seen.add(line);
  }
  if (schemaErrors.length > 0 && lines.length === 0) lines.push(`SCHEMA /: ${schemaErrors.length} errors, first ${JSON.stringify(schemaErrors[0])}`);
  const nSchema = lines.length;
  const assets = new Map<string, Json>();
  for (const e of catalogEntries()) {
    try {
      const other = loadAsset(e.path);
      assets.set(other.id, other.id === asset.id ? asset : other);
    } catch {
      void 0;
    }
  }
  const environments = new Set((((await loadEnvironments()) as Json)?.presets ?? []).map((p: Json) => p.id as string));
  let issues: readonly Json[] = [];
  try {
    issues = validateAsset(asset as never, { assets: assets as never, environments: environments as never });
  } catch (error) {
    lines.push(`ASSET validator-threw $: ${(error as Error).message} (validateAsset threw: the asset is so broken that nothing after this point was checked)`);
  }
  for (const i of issues) lines.push(`ASSET ${i.code} ${i.path}: ${i.message.en}`);
  try {
    for (const i of validateRanges(asset as never, DEFAULT_PRINT_PROFILE) as readonly Json[]) lines.push(`PRINT ${i.code} ${i.path}: ${i.message.en}`);
    for (const [scale, rep] of Object.entries((asset.representations ?? {}) as Record<string, Json>)) {
      if (rep.status !== "authored" || rep.geometry === undefined) continue;
      const s = measureSuitability(asset as never, rep as never, Number(scale), DEFAULT_PRINT_PROFILE) as Json;
      if (s.suitable !== true) lines.push(`PRINT unsuitable $.representations.${scale}: the product refuses to draw 1:${scale} (min feature ${Number(s.minFeatureMm).toFixed(3)} mm, min gap ${s.minGapMm} mm, min stroke ${s.minStrokeMm} mm against the default print profile)`);
    }
  } catch (error) {
    lines.push(`PRINT validator-threw $: ${(error as Error).message}`);
  }
  const nAsset = lines.length - nSchema;
  if (has("--volumes")) {
    try {
      for (const f of checkVolumes(asset as never) as readonly Json[]) lines.push(`VOLUMES ${f.rule}${f.scale === undefined ? "" : ` 1:${f.scale}`} ${f.code} ${f.path}: ${f.message.en}`);
    } catch (error) {
      lines.push(`VOLUMES validator-threw $: ${(error as Error).message}`);
    }
  }
  const nVolumes = lines.length - nSchema - nAsset;
  if (has("--json")) process.stdout.write(`${JSON.stringify({ asset: `${loaded.entry.category}/${loaded.entry.slug}`, schema: nSchema, validateAsset: nAsset, volumes: has("--volumes") ? nVolumes : null, issues: lines })}\n`);
  else {
    for (const l of lines) process.stdout.write(`${l}\n`);
    process.stdout.write(`${lines.length === 0 ? "VALID" : "INVALID"} ${loaded.entry.category}/${loaded.entry.slug} (${file !== undefined ? "file as it is" : has("--catalog-only") ? "catalog file as it is" : "catalog asset + volumes source"}): schema ${nSchema}, validateAsset ${nAsset}${has("--volumes") ? `, checkVolumes ${nVolumes}` : ", checkVolumes not run (--volumes)"}\n`);
  }
  process.exitCode = lines.length === 0 ? 0 : 1;
}
