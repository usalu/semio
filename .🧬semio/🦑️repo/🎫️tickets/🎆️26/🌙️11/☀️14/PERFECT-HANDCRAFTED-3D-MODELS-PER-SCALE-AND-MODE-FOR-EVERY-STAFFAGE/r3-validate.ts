/** ✅️ Validates one whole asset as the product does: the catalog asset with the author's `volumes` source merged over it against the JSON schema (ajv, `AssetDocument`) and through the catalog validator `validateAsset` (references, rig, poses, clips, representations, thermal, structure, volumes analytic rules, exports) with its referenced asset alternatives and the environment presets as context.
 *
 * Usage: `bun r3-validate.ts <category>/<slug> [--volumes [--scale N] [--rules a,b]] [--no-print] [--catalog-only] [--json]` or `bun r3-validate.ts --file <asset document json> [--volumes [--scale N] [--rules a,b]] [--no-print] [--json]` (a candidate file as it is, for root fixes before they are written to the catalog)
 * Prints every issue (`SCHEMA path: message`, `ASSET code path: message`), then one summary line; exit 1 on any issue, 2 on usage errors. `--volumes` adds the measured volume rules (`checkVolumes`, slow; the numbers of `volumes-check.ts --product`). `--catalog-only` validates the catalog file as it is, without the source.
 * Fast inner loop: `--scale N` limits `checkVolumes` to the volumes of 1:N (the cross-scale rules then compare 1:N with the next finer scale only) and `--rules a,b` to the named rules (`VOLUME_RULES`, with or without the `volumes-` prefix); both imply `--volumes` and mark the summary `partial`. `--no-print` skips the `PRINT` lines (the print measurement is most of the time of a run without `--volumes`). The run without these three flags stays the final proof.
 * @see ./r3-tools.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts */
import { readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import Ajv from "ajv";

import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { loadEnvironments } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { VOLUME_RULES, authoredVolumes, checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { STAFFAGE, catalogEntries, resolveAsset, loadAsset, type CatalogEntry, type Json } from "./volumes-lib.ts";
import { cli, load } from "./r3-lib.ts";

const USAGE = "r3-validate.ts <category>/<slug> | --file <asset document json> [--volumes [--scale N] [--rules a,b]] [--no-print] [--catalog-only] [--json]";

/** 🔗 Resolve exactly the cross-asset alternatives read by validateAsset; scale and symbol alternatives are local. */
export function referencedAssets(asset: Json, entries = catalogEntries, read = loadAsset): Map<string, Json> {
  const assets = new Map<string, Json>([[asset.id, asset]]);
  const requested = new Set<string>();
  for (const rep of Object.values(asset.representations ?? {}) as Json[]) {
    for (const alt of rep.alternatives ?? []) if (alt.kind === "asset" && alt.asset !== asset.id) requested.add(alt.asset);
  }
  if (requested.size === 0) return assets;
  for (const entry of entries()) {
    if (!requested.has(entry.slug)) continue;
    try {
      const target = read(entry.path);
      if (target.id === entry.slug) assets.set(target.id, target);
    } catch {
      continue;
    }
  }
  return assets;
}

if (import.meta.main) {
  const { spec, has, flag } = cli(["--file", "--scale", "--rules"]);
  const file = flag("--file");
  const only = { scale: flag("--scale"), rules: flag("--rules")?.split(",").filter((name) => name !== "").map((name) => (name.startsWith("volumes-") ? name : `volumes-${name}`)) };
  const unknown = (only.rules ?? []).filter((name) => !(VOLUME_RULES as readonly string[]).includes(name));
  if (unknown.length > 0 || only.rules?.length === 0 || (has("--scale") && !(Number(only.scale) > 0)) || (has("--rules") && only.rules === undefined)) {
    process.stderr.write(`${unknown.length > 0 ? `unknown rule ${unknown.join(", ")}` : has("--rules") && !(only.rules?.length! > 0) ? "--rules needs a list" : `--scale ${only.scale} is no scale`}; rules: ${VOLUME_RULES.join(", ")}\nusage: ${USAGE}\n`);
    process.exit(2);
  }
  const partial = only.scale !== undefined || only.rules !== undefined;
  const withVolumes = has("--volumes") || partial;
  const noPrint = has("--semantics") || has("--no-print");
  if (file === undefined && spec === undefined) {
    process.stderr.write(`usage: ${USAGE}\n`);
    process.exit(2);
  }
  const entry: CatalogEntry = file === undefined ? resolveAsset(spec!) :
    { category: "file", folder: "file", slug: basename(dirname(resolve(file))), path: resolve(file) };
  const document = JSON.parse(readFileSync(entry.path, "utf8"));
  const asset: Json = file !== undefined || has("--catalog-only") ? document.asset : load(spec, USAGE).asset;
  const lines: string[] = [];
  const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, inlineRefs: !has("--compact"), allErrors: !has("--compact") });
  ajv.addSchema(schema);
  const conforms = ajv.compile({ $ref: `${schema.$id}#/$defs/AssetDocument` });
  const schemaErrors = conforms({ ...document, asset }) ? [] : (conforms.errors ?? []);
  const seen = new Set<string>();
  for (const e of schemaErrors) {
    if (["oneOf", "anyOf", "if"].includes(e.keyword)) continue;
    const line = `SCHEMA ${e.instancePath || "/"}: ${e.message}${e.params === undefined ? "" : ` ${JSON.stringify(e.params)}`}`;
    if (!seen.has(line)) lines.push(line);
    seen.add(line);
  }
  if (schemaErrors.length > 0 && lines.length === 0) lines.push(`SCHEMA /: ${schemaErrors.length} errors, first ${JSON.stringify(schemaErrors[0])}`);
  const nSchema = lines.length;
  if (has("--compact")) { Bun.gc(true); console.error("[DEBUG] schema", nSchema); }
  const assets = referencedAssets(asset);
  const environments = new Set((((await loadEnvironments()) as Json)?.presets ?? []).map((p: Json) => p.id as string));
  let issues: readonly Json[] = [];
  try {
    issues = validateAsset(asset as never, { assets: assets as never, environments: environments as never });
  } catch (error) {
    lines.push(`ASSET validator-threw $: ${(error as Error).message} (validateAsset threw: the asset is so broken that nothing after this point was checked)`);
  }
  if (has('--compact')) { Bun.gc(true); console.error('[DEBUG] semantic rules',issues.length); }
  for (const i of issues) lines.push(`ASSET ${i.code} ${i.path}: ${i.message.en}`);
  try {
    if (!noPrint) for (const i of validateRanges(asset as never, DEFAULT_PRINT_PROFILE) as readonly Json[]) lines.push(`PRINT ${i.code} ${i.path}: ${i.message.en}`);
    for (const [scale, rep] of Object.entries((asset.representations ?? {}) as Record<string, Json>)) {
      if (noPrint || rep.status !== "authored" || rep.geometry === undefined) continue;
      const s = measureSuitability(asset as never, rep as never, Number(scale), DEFAULT_PRINT_PROFILE) as Json;
      if (s.suitable !== true) lines.push(`PRINT unsuitable $.representations.${scale}: the product refuses to draw 1:${scale} (min feature ${Number(s.minFeatureMm).toFixed(3)} mm, min gap ${s.minGapMm} mm, min stroke ${s.minStrokeMm} mm against the default print profile)`);
    }
  } catch (error) {
    lines.push(`PRINT validator-threw $: ${(error as Error).message}`);
  }
  const nAsset = lines.length - nSchema;
  if (withVolumes) {
    try {
      if (only.scale !== undefined && !authoredVolumes(asset as never).some((volume) => volume.denominator === Number(only.scale))) lines.push(`VOLUMES no-such-scale $.volumes: no authored volumes at 1:${only.scale} (authored: ${authoredVolumes(asset as never).map((volume) => `1:${volume.denominator}`).join(", ") || "none"})`);
      for (const f of checkVolumes(asset as never, { ...(only.scale === undefined ? {} : { scale: Number(only.scale) }), ...(only.rules === undefined ? {} : { rules: only.rules }) } as never) as readonly Json[]) lines.push(`VOLUMES ${f.rule}${f.scale === undefined ? "" : ` 1:${f.scale}`} ${f.code} ${f.path}: ${f.message.en}`);
    } catch (error) {
      lines.push(`VOLUMES validator-threw $: ${(error as Error).message}`);
    }
  }
  const nVolumes = lines.length - nSchema - nAsset;
  if (has("--json")) process.stdout.write(`${JSON.stringify({ asset: `${entry.category}/${entry.slug}`, schema: nSchema, validateAsset: nAsset, volumes: withVolumes ? nVolumes : null, partial: partial || noPrint, issues: lines })}\n`);
  else {
    for (const l of lines) process.stdout.write(`${l}\n`);
    process.stdout.write(`${lines.length === 0 ? "VALID" : "INVALID"} ${entry.category}/${entry.slug} (${file !== undefined ? "file as it is" : has("--catalog-only") ? "catalog file as it is" : "catalog asset + volumes source"}): schema ${nSchema}, validateAsset ${nAsset}${withVolumes ? `, checkVolumes ${nVolumes}` : ", checkVolumes not run (--volumes)"}${partial ? ` — PARTIAL: checkVolumes limited to ${[only.scale === undefined ? "" : `1:${only.scale}`, only.rules === undefined ? "" : only.rules.join(", ")].filter((part) => part !== "").join(", ")}; the run without --scale/--rules is the final proof` : ""}${noPrint ? " — PRINT checks skipped (--no-print)" : ""}\n`);
  }
  process.exitCode = lines.length === 0 ? 0 : 1;
}
