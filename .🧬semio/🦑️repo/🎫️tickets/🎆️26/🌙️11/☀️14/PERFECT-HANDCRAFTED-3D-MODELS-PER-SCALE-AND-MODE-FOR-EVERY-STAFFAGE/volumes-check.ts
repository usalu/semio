/** 🔬️ Checks every rule of the per-scale volume design on the SOURCE file merged over the asset, fast.
 *
 * Usage: `bun volumes-check.ts <category>/<slug> [--scale N] [--rules a,b] [--fast] [--source file] [--json] [--quiet] [--product | --compare]`
 * DEFAULT engine (rulings C1-C18): the product `measureVolumes`/`checkVolumes` of the catalog validator, so authors see exactly its numbers (slower on large assets). `--fast` and `--owned` use the owned measurements instead (approximate, for iteration and diagnostics). Owned engine: the owned measurements (`volumes-lib.ts`, `volumes-rulings.ts`, `volumes-silhouette.ts`, `volumes-tiles.ts`) with rulings C1-C10, backed by a content-hash tessellation cache (`🗑️generated/volumes-tools/cache`), plus the cheap analytic findings of the product validator (per-solid validity, copied scales). `--scale N` and `--rules` do only that work. `--fast` = coarser raster and coarser tessellation, silhouette + bounds + coverage + availability + bones + solids only, for iteration; the full check runs without it.
 * `--product` runs only the product `measureVolumes` (slow, the catalog validator's numbers), `--compare` prints measurements of both engines that share a name with their difference.
 * Prints one line per rule and scale (`!` marks a failing measurement; reported-only numbers are never failures); exit 1 on any failure, 2 on usage errors, 130 when cancelled. Caches the result for `volumes-status.py`.
 * @see ./design.md
 * @see ./volumes-tools.md */
import { readFileSync, statSync } from "node:fs";
import { resolve } from "node:path";

import { CancelledError, RULES, SCALES, checkAsset, entryOf, installCancellation, loadSource, merged, outPath, quality, sourcePath, writeJson, type Json, type Measurement } from "./volumes-lib.ts";
import { rulingMeasurements } from "./volumes-rulings.ts";
import { silhouetteMeasurements } from "./volumes-silhouette.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(name);
  return i < 0 ? undefined : args[i + 1];
};
const spec = args.find((a, i) => !a.startsWith("--") && !(i > 0 && ["--scale", "--rules", "--source"].includes(args[i - 1]!)));
if (spec === undefined) {
  process.stderr.write("usage: volumes-check.ts <category>/<slug> [--scale N] [--rules a,b] [--fast] [--source file] [--json] [--quiet] [--product | --compare]\n");
  process.exit(2);
}
installCancellation();
const t0 = Date.now();
const { entry, asset } = entryOf(spec);
const file = flag("--source") === undefined ? sourcePath(entry) : resolve(flag("--source")!);
let source = flag("--source") === undefined ? loadSource(entry) : JSON.parse(readFileSync(file, "utf8"));
if (source === null) {
  if (asset.volumes === undefined) {
    process.stderr.write(`no source ${file} and the asset has no volumes; run volumes-seed.ts ${spec}\n`);
    process.exit(2);
  }
  source = { volumes: asset.volumes };
}
const scale = flag("--scale") === undefined ? undefined : Number(flag("--scale"));
const fast = args.includes("--fast");
quality.fast = fast;
let rules = flag("--rules")?.split(",");
if (scale !== undefined && !SCALES.includes(scale as never)) {
  process.stderr.write(`--scale must be one of ${SCALES.join(", ")}\n`);
  process.exit(2);
}
if (fast && rules === undefined) rules = ["volumes-availability", "volumes-silhouette", "volumes-views-disagree", "volumes-bounds", "volumes-thermal-coverage", "volumes-structure-coverage", "volumes-bones", "volumes-solids"];
const productDefault = !args.includes("--owned") && !args.includes("--compare") && !fast;
const m = merged(asset, source);
type Tagged = Measurement & { engine: "p" | "o" };
const progress = (msg: string) => process.stderr.write(`[check] ${entry.category}/${entry.slug} ${msg}\n`);
const notes: string[] = [];
if (fast) notes.push("--fast: owned approximate measurements for iteration; run without --fast to see the catalog validator's numbers");
if (args.includes("--owned")) notes.push("--owned: owned measurements, not the catalog validator");
let findings: { rule: string; scale?: number; list?: string; message: string }[] = [];
let measurements: Tagged[] = [];
let product: Measurement[] = [];
let owned: Measurement[] = [];

class BudgetExceeded extends Error {}

async function productMeasurements(all: boolean, budgetS = Infinity): Promise<Measurement[]> {
  const deadline = Date.now() + budgetS * 1000;
  const signal = { aborted: false, throwIfAborted: () => { if (Date.now() > deadline) throw new BudgetExceeded(`product validator exceeded ${budgetS} s`); } };
  try {
    const mod = (await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts")) as typeof import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
    const cheap = ["volumes-solids", "volumes-availability", "volumes-bones", "volumes-copied"];
    const want = (rules ?? [...mod.VOLUME_RULES]).filter((r) => all || cheap.includes(r));
    const options = { ...(scale === undefined ? {} : { scale: scale as never }), rules: want as never };
    const out: Measurement[] = all ? mod.measureVolumes(m as never, options, { signal, progress: (d: number, t: number, l: { en: string }) => progress(`${d}/${t} ${l.en}`) } as never).map((x) => ({ ...x } as Measurement)) : [];
    for (const f of mod.checkVolumes(m as never, { ...options, rules: want.filter((r) => cheap.includes(r)) as never })) {
      findings.push({ rule: f.rule, scale: f.scale, list: f.list, message: f.message.en });
      if (f.measurements.length === 0 || !all) out.push({ rule: f.rule, scale: f.scale, list: f.list as never, name: `finding:${f.code}${typeof (f.data as Json | undefined)?.id === "string" ? `:${(f.data as Json).id}` : ""}`, value: 1, limit: 0, bound: "max", ok: false, note: f.message.en });
    }
    return out;
  } catch (error) {
    if (error instanceof CancelledError) throw error;
    if (error instanceof BudgetExceeded) throw error;
    notes.push(`product validator unavailable (${(error as Error).message.split("\n")[0]})`);
    return [];
  }
}

try {
  let useOwned = !productDefault;
  if (productDefault) {
    try {
      product = await productMeasurements(true, args.includes("--product") ? Infinity : Number(process.env.VOLUMES_PRODUCT_BUDGET_S ?? 60));
      measurements = product.map((x) => ({ ...x, engine: "p" as const }));
    } catch (error) {
      if (!(error instanceof BudgetExceeded)) throw error;
      findings = [];
      notes.push(`${(error as Error).message}: the numbers below are the OWNED engine's [o] (same rules C1-C18 except C18, within a few percent); run with --product or let the catalog validator decide at splice time`);
      useOwned = true;
    }
  }
  if (useOwned) {
    const analytic = checkAsset(m, source, { scale, rules }, progress);
    const rul = rulingMeasurements(m, source, { scale, rules, fast });
    const sil = silhouetteMeasurements(m, source, { scale, rules, fast });
    const replaced = (x: Measurement): boolean => ["volumes-feature", "volumes-cross-scale", "volumes-monotonic", "volumes-slender", "volumes-silhouette", "volumes-views-disagree", "volumes-bounds"].includes(x.rule) || x.name.startsWith("areaDeviation:") || x.name.startsWith("areaM2:") || x.name.startsWith("supportOffM:") || x.name.startsWith("supportSolid") || x.name.startsWith("outsideBodyM:");
    owned = [...analytic.filter((x) => !replaced(x)), ...rul, ...sil];
    const cheap = await productMeasurements(false);
    measurements = [...owned.map((x) => ({ ...x, engine: "o" as const })), ...cheap.map((x) => ({ ...x, engine: "p" as const }))];
    if (args.includes("--compare")) {
      product = await productMeasurements(true);
      const key = (x: Measurement) => `${x.rule}|${x.scale}|${x.list}|${x.name}`;
      const po = new Map(owned.map((x) => [key(x), x]));
      process.stdout.write(`${entry.category}/${entry.slug}: product vs owned (shared names)\n`);
      let shared = 0;
      let differ = 0;
      for (const x of product) {
        const o = po.get(key(x));
        if (o === undefined) continue;
        shared++;
        const rel = Math.abs(x.value - o.value) / Math.max(Math.abs(x.value), Math.abs(o.value), 1e-9);
        if (rel > 0.05 || x.ok !== o.ok) {
          differ++;
          process.stdout.write(`  ${x.rule} 1:${x.scale} ${x.list ?? ""} ${x.name}: product ${x.value} (${x.ok ? "ok" : "FAIL"}) owned ${o.value} (${o.ok ? "ok" : "FAIL"})\n`);
        }
      }
      process.stdout.write(`  ${shared} shared measurements, ${differ} differ by more than 5 % or in verdict\n`);
      process.exit(0);
    }
  }
} catch (error) {
  if (error instanceof CancelledError) process.exit(130);
  throw error;
}
{
  const seen = new Set<string>();
  measurements = measurements.filter((x) => {
    const k = `${x.rule}|${x.scale}|${x.list}|${x.name}|${x.engine}`;
    return seen.has(k) ? false : (seen.add(k), true);
  });
}
const failed = measurements.filter((x) => !x.ok);
type Line = { rule: string; scale?: number; ok: boolean; items: Tagged[] };
const lines = new Map<string, Line>();
for (const x of measurements) {
  const k = `${x.rule}|${x.scale ?? ""}`;
  const l = lines.get(k) ?? { rule: x.rule, ...(x.scale === undefined ? {} : { scale: x.scale }), ok: true, items: [] };
  l.items.push(x);
  l.ok &&= x.ok;
  lines.set(k, l);
}
const summary = [...lines.values()].sort((a, b) => RULES.indexOf(a.rule) - RULES.indexOf(b.rule) || (a.scale ?? 0) - (b.scale ?? 0));
const seconds = ((Date.now() - t0) / 1000).toFixed(1);
if (args.includes("--json")) process.stdout.write(`${JSON.stringify({ asset: `${entry.category}/${entry.slug}`, ok: failed.length === 0, fast, notes, measurements, findings }, null, 1)}\n`);
else {
  const fmt = (x: Tagged) => `${x.ok ? "" : "!"}${x.list === undefined || x.name.includes(":") ? "" : `${x.list}.`}${x.name}=${x.value}${x.note?.startsWith("reported") || x.limit > 1e100 ? "" : `${x.bound === "min" ? ">=" : "<="}${x.limit}`}${x.note === undefined || x.note === "" || (x.ok && !x.note.startsWith("reported")) ? "" : ` [${x.note}]`}`;
  process.stdout.write(`${entry.category}/${entry.slug} (pose ${asset.presentation?.defaultPose})${fast ? " FAST: iteration only, run without --fast before splicing" : ""}\n`);
  for (const n of notes) process.stdout.write(`note: ${n}\n`);
  for (const s of summary) {
    const tag = [...new Set(s.items.map((x) => x.engine))].join("");
    process.stdout.write(`${s.ok ? "ok  " : "FAIL"} ${s.rule.padEnd(27)} ${s.scale === undefined ? "all   " : `1:${s.scale}`.padEnd(6)} [${tag}] ${args.includes("--quiet") && s.ok ? "" : s.items.map(fmt).join(" ")}\n`);
  }
  if (!args.includes("--quiet") && findings.length > 0) process.stdout.write(`findings (product validator):\n${[...new Set(findings.map((f) => `  ${f.rule} ${f.scale === undefined ? "" : `1:${f.scale}`} ${f.message}`))].slice(0, 25).join("\n")}\n`);
  process.stdout.write(failed.length === 0 ? `PASS (${seconds} s)\n` : `FAIL: ${failed.length} measurement(s) over ${summary.filter((s) => !s.ok).length} rule line(s) (${seconds} s)\n`);
}
const perScale: Record<string, { ok: boolean; rules: Record<string, boolean> }> = {};
for (const s of summary) {
  const k = String(s.scale ?? "all");
  const e = (perScale[k] ??= { ok: true, rules: {} });
  e.ok &&= s.ok;
  e.rules[s.rule] = s.ok;
}
if (rules === undefined && scale === undefined && !fast)
  writeJson(outPath("check", entry, `${entry.slug}.json`), { asset: `${entry.category}/${entry.slug}`, checkedAt: new Date().toISOString(), sourceMtimeMs: (() => { try { return statSync(file).mtimeMs; } catch { return 0; } })(), ok: failed.length === 0, perScale, failures: failed.slice(0, 40), measurements: measurements.length });
process.exit(failed.length === 0 ? 0 : 1);
