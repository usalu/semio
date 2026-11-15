/** Runs checkVolumes over the whole catalog (every asset, every authored scale) and writes catalog-check.md next to this file.
 *  bun T/schema-catalog-check.ts [--workers 4] [--only slug,slug] [--out catalog-check.md]   (re-runnable; shards run as child processes)
 *  Output under 🗑️generated/schema/catalog/. */
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { closureHash } from "./validator4-closure.ts";
import { VOLUME_RULES, checkVolumes, measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

type Worst = { name: string; value: number; limit: number; bound: string; ratio: number };
type Row = { rule: string; scale: number | null; list: string | null; code: string; path: string; message: string; worst: Worst | null };
type Candidate = { region: string; kind: string; model: string; areaM2: number; exposedM2: number };
type Result = { timedOut?: boolean; cached?: boolean; id: string; category: string; spliced: boolean; ms: number; rows: Row[]; candidates?: Candidate[]; error?: string };

const here = dirname(fileURLToPath(import.meta.url));
const generated = join(here, "🗑️generated", "schema", "catalog");
const { values } = parseArgs({ options: { workers: { type: "string", default: "4" }, only: { type: "string" }, out: { type: "string", default: "catalog-check.md" }, shard: { type: "string" }, json: { type: "string" }, "no-cache": { type: "boolean", default: false }, one: { type: "string" }, timeout: { type: "string", default: "900" } } });

const ratioOf = (m: { value: number; limit: number; bound: string }): number => (m.bound === "max" ? (m.limit === 0 ? (m.value > 0 ? Infinity : 0) : m.value / m.limit) : m.value === 0 ? Infinity : m.limit / m.value);
const entries = (await assetPathsFromFiles()).filter((e) => values.only === undefined || values.only.split(",").includes(e.folder));

const MODULES = join(here, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules");
const { hash: moduleHash, files: closureFiles } = closureHash([join(MODULES, "🗂️catalog/🧊️volumes/🟦️.ts"), join(MODULES, "🗂️catalog/📥️files/🟦️.ts"), join(MODULES, "🗂️catalog/🟦️.ts")]);
const cacheDir = join(here, "🗑️generated", "validator", "cache");

async function lookup(entry: (typeof entries)[number]): Promise<{ key: string; file: string; hit?: Result; id: string }> {
  const { asset } = await readStreamedAsset(entry);
  const key = createHash("sha1").update(moduleHash).update(JSON.stringify(asset)).digest("hex");
  const file = join(cacheDir, `${asset.id}.json`);
  if (!values["no-cache"] && existsSync(file)) {
    const hit = JSON.parse(readFileSync(file, "utf8")) as { key: string; result: Result };
    if (hit.key === key) return { key, file, id: asset.id as string, hit: { ...hit.result, cached: true } };
  }
  return { key, file, id: asset.id as string };
}

async function compute(entry: (typeof entries)[number], asset: Awaited<ReturnType<typeof readStreamedAsset>>["asset"]): Promise<Result> {
  const base = { id: asset.id as string, category: asset.category as string };
  if ((asset as { volumes?: unknown }).volumes === undefined) return { ...base, spliced: false, ms: 0, rows: [] };
  const t = Date.now();
  try {
    const findings = checkVolumes(asset);
    const rows = findings.map((f): Row => {
      const worst = f.measurements.map((m) => ({ name: m.name, value: m.value, limit: m.limit, bound: m.bound, ratio: ratioOf(m) })).sort((p, q) => q.ratio - p.ratio)[0] ?? null;
      return { rule: f.rule, scale: f.scale ?? null, list: f.list ?? null, code: f.code, path: f.path, message: f.message.en, worst };
    });
    const sound = !rows.some((r) => r.scale === 50 && ["volumes-bones", "volumes-solids", "volumes-availability"].includes(r.rule));
    const candidates: Candidate[] = [];
    if (sound && rows.some((r) => r.rule === "volumes-thermal-coverage" && r.scale === 50) && (asset as { volumes: Record<string, { status: string }> }).volumes["50"]?.status === "authored")
      for (const m of measureVolumes(asset, { scale: 50, rules: ["volumes-thermal-coverage"] })) {
        const id = m.name.startsWith("areaDeviation:") ? m.name.slice(14) : undefined;
        const region = id === undefined ? undefined : (asset.thermal.regions as { id: string; kind: string; model: string; areaM2: number; areaBasis?: string }[]).find((r) => r.id === id);
        if (region !== undefined && region.areaBasis !== "exchange" && m.value > 2 / 3) candidates.push({ region: region.id, kind: region.kind, model: region.model, areaM2: region.areaM2, exposedM2: region.areaM2 * (1 - m.value) });
      }
    console.error(`${asset.id} ${Date.now() - t} ms`);
    return { ...base, spliced: true, ms: Date.now() - t, rows, candidates };
  } catch (error) {
    return { ...base, spliced: true, ms: Date.now() - t, rows: [], error: (error as Error).message };
  }
}

if (values.one !== undefined) {
  const entry = entries.find((e) => e.folder === values.one)!;
  const { asset } = await readStreamedAsset(entry);
  const found = await lookup(entry);
  const result = { ...(await compute(entry, asset)), cached: false };
  mkdirSync(cacheDir, { recursive: true });
  writeFileSync(`${found.file}.${process.pid}.tmp`, JSON.stringify({ key: found.key, result }));
  renameSync(`${found.file}.${process.pid}.tmp`, found.file);
  writeFileSync(values.json!, JSON.stringify(result));
  process.exit(0);
}

mkdirSync(generated, { recursive: true });
const lock = join(generated, "catalog-check.lock");
try {
  writeFileSync(lock, String(process.pid), { flag: "wx" });
} catch {
  const holder = Number(readFileSync(lock, "utf8"));
  let alive = true;
  try { process.kill(holder, 0); } catch { alive = false; }
  if (alive) { console.error(`another catalog check (pid ${holder}) holds ${lock}; refusing to run`); process.exit(2); }
  writeFileSync(lock, String(process.pid));
}
process.on("exit", () => { try { unlinkSync(lock); } catch { } });
const started = Date.now();
const workers = Math.max(1, Number(values.workers));
const timeoutS = Number(values.timeout);
const runDir = join(generated, `run-${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`);
mkdirSync(runDir, { recursive: true });
const { spawn } = require("node:child_process") as typeof import("node:child_process");
const slots = new Map<string, Result>();
const pending: (typeof entries)[number][] = [];
for (const entry of entries) {
  const found = await lookup(entry);
  if (found.hit !== undefined) { console.error(`${found.id} cached`); slots.set(entry.folder, found.hit); } else pending.push(entry);
}
const slowest = (e: (typeof entries)[number]): number => { try { return (JSON.parse(readFileSync(join(cacheDir, `${e.folder}.json`), "utf8")) as { result: Result }).result.ms; } catch { return 0; } };
pending.sort((p, q) => slowest(q) - slowest(p));
const runOne = (entry: (typeof entries)[number]): Promise<void> => new Promise((resolve) => {
  const file = join(runDir, `${entry.folder}.json`);
  const t = Date.now();
  const child = spawn(process.execPath, [fileURLToPath(import.meta.url), "--one", entry.folder, "--json", file], { stdio: ["ignore", "inherit", "inherit"] });
  const timer = setTimeout(() => child.kill("SIGKILL"), timeoutS * 1000);
  child.on("exit", () => {
    clearTimeout(timer);
    const ms = Date.now() - t;
    if (existsSync(file)) slots.set(entry.folder, JSON.parse(readFileSync(file, "utf8")) as Result);
    else slots.set(entry.folder, { id: entry.folder, category: "", spliced: true, ms, rows: [], cached: false, error: ms >= timeoutS * 1000 - 500 ? `timed out after ${timeoutS} s` : "worker died without a result", timedOut: ms >= timeoutS * 1000 - 500 });
    resolve();
  });
});
const queue = [...pending];
await Promise.all(Array.from({ length: workers }, async () => { for (let e = queue.shift(); e !== undefined; e = queue.shift()) await runOne(e); }));
const missing = entries.filter((e) => !slots.has(e.folder)).map((e) => e.folder);
if (missing.length > 0) { console.error(`MISSING ${missing.join(", ")}`); process.exit(3); }
const results = entries.map((e) => slots.get(e.folder)!).sort((p, q) => p.id.localeCompare(q.id));
if (new Set(results.map((r) => r.id)).size !== results.length) { console.error("DUPLICATE ids in the results"); process.exit(3); }
const wall = Date.now() - started;
if (values.only === undefined) {
  writeFileSync(join(generated, "results.json.tmp"), JSON.stringify(results));
  renameSync(join(generated, "results.json.tmp"), join(generated, "results.json"));
}
writeFileSync(join(runDir, "results.json"), JSON.stringify(results));

const spliced = results.filter((r) => r.spliced);
const failing = spliced.filter((r) => r.rows.length > 0 || r.error);
const clean = spliced.filter((r) => r.rows.length === 0 && !r.error);
const fmt = (w: Worst | null): string => {
  if (w === null) return "";
  const m = w.name.includes("M") && !w.name.includes("Ratio") && !w.name.startsWith("iou") && !w.name.includes("Deviation") && !w.name.includes("Excess");
  const num = (x: number): string => (x >= 1e300 ? "inf" : m ? `${(x * 1000).toFixed(1)} mm` : x.toFixed(3));
  return `${w.name} ${num(w.value)} (${w.bound === "max" ? "<=" : ">="} ${num(w.limit)})`;
};
const byRule = new Map<string, { assets: Set<string>; findings: number }>();
for (const r of failing) for (const row of r.rows) {
  const e = byRule.get(row.rule) ?? { assets: new Set<string>(), findings: 0 };
  e.assets.add(r.id);
  e.findings++;
  byRule.set(row.rule, e);
}
const lines: string[] = [];
lines.push("# Catalog check: `checkVolumes` over every asset and scale", "", `Generated by \`schema-catalog-check.ts\` (re-run: \`bun schema-catalog-check.ts --workers 4\`; \`--only slug,slug\` for a subset). Data: \`🗑️generated/schema/catalog/results.json\`.`, "");
lines.push(`- assets: ${results.length}; with \`volumes\` (spliced): ${spliced.length}; not spliced yet (still the old \`volume\`): ${results.length - spliced.length}`, `- clean: ${clean.length}; failing: ${failing.length}`, `- covers ${results.length} ids, none missing, none duplicated; timed out: ${results.filter((r) => r.timedOut).map((r) => r.id).join(", ") || "none"}`, `- recomputed: ${spliced.filter((r) => r.cached === false).length} of ${spliced.length} spliced (module closure of ${closureFiles} files, hash ${moduleHash.slice(0, 10)})`, `- wall time ${(wall / 1000).toFixed(0)} s with ${workers} worker(s); summed per-asset time ${(spliced.reduce((s, r) => s + r.ms, 0) / 1000).toFixed(0)} s; slowest: ${[...spliced].sort((p, q) => q.ms - p.ms).slice(0, 5).map((r) => `${r.id} ${(r.ms / 1000).toFixed(1)} s`).join(", ")}`, "");
lines.push("## Rules ranked by number of failing assets", "", "| rule | failing assets | findings |", "|---|---|---|");
for (const rule of VOLUME_RULES) {
  const e = byRule.get(rule);
  lines.push(`| ${rule} | ${e?.assets.size ?? 0} | ${e?.findings ?? 0} |`);
}
lines.splice(lines.length - VOLUME_RULES.length, VOLUME_RULES.length, ...[...VOLUME_RULES].sort((p, q) => (byRule.get(q)?.assets.size ?? 0) - (byRule.get(p)?.assets.size ?? 0)).map((rule) => `| ${rule} | ${byRule.get(rule)?.assets.size ?? 0} | ${byRule.get(rule)?.findings ?? 0} |`));
lines.push("", "## Failing assets (worst measurement per asset, scale and rule)", "");
for (const r of failing) {
  lines.push(`### ${r.category}/${r.id} (${(r.ms / 1000).toFixed(1)} s)`, "");
  if (r.error) lines.push(`- ERROR ${r.error}`);
  const groups = new Map<string, Row[]>();
  for (const row of r.rows) groups.set(`${row.scale ?? "-"}|${row.rule}`, [...(groups.get(`${row.scale ?? "-"}|${row.rule}`) ?? []), row]);
  for (const [key, rows] of [...groups].sort()) {
    const [scale, rule] = key.split("|");
    const worst = rows.map((x) => x.worst).filter((w): w is Worst => w !== null).sort((p, q) => q.ratio - p.ratio)[0] ?? null;
    lines.push(`- 1:${scale} ${rule} (${rows.length}): ${worst === null ? rows[0]!.message.slice(0, 140) : fmt(worst)}`);
  }
  lines.push("");
}
const candidateAssets = spliced.filter((r) => (r.candidates?.length ?? 0) > 0);
lines.push("## Candidates for areaBasis exchange (for the coordinator's decision; not marked)", "", "Regions of a sound 1:50 model (no bones/solids/availability finding at 1:50) whose measured exposed area is below one third of `areaM2` and that are not yet `areaBasis: exchange`.", "", "| asset | region | kind / model | areaM2 | exposed 1:50 |", "|---|---|---|---|---|");
for (const r of candidateAssets) for (const c of r.candidates!) lines.push(`| ${r.id} | ${c.region} | ${c.kind} / ${c.model} | ${c.areaM2} | ${c.exposedM2.toFixed(4)} |`);
lines.push("");
lines.push("## Not spliced yet", "", results.filter((r) => !r.spliced).map((r) => r.id).join(", "), "", "## Clean", "", clean.map((r) => r.id).join(", "), "");
writeFileSync(join(here, `${values.out!}.${process.pid}.tmp`), lines.join("\n"));
renameSync(join(here, `${values.out!}.${process.pid}.tmp`), join(here, values.out!));
console.log(`wrote ${values.out}: ${results.length} assets, ${spliced.length} spliced, ${failing.length} failing, ${(wall / 1000).toFixed(0)} s`);
