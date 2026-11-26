/** 🔎️ Read-only audit of the published fleet metric rings: file inventory per origin, windowed reads through readSeriesV1, plausibility and overhead checks. Writes nothing; prints JSON to stdout. */
import {existsSync, readdirSync, statSync} from "node:fs";
import {join} from "node:path";
import {METRICS_CATALOGUE_V1, readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";

const ORIGINS = "/Users/Shared/semio-store/v1/origins";
const FLEET = "/Users/Shared/semio-store/v1/fleet/metrics";
const TIERS = ["t0", "t1", "t2", "t3", "t4"];
const WINDOWS = [{name: "5min", ms: 5 * 60_000}, {name: "1h", ms: 3_600_000}];
const FOCUS = ["host.cpu.busy", "host.memory.available", "host.disk.free", "build.units.compiled", "cache.local.hit", "cache.remote.ac.hit", "metrics.self.cpu", "metrics.self.memory"];
const TOTALS = ["host.memory.total", "host.disk.total"];
const OVERHEAD_CPU_PERCENT = 0.5;
const OVERHEAD_RSS_BYTES = 30 * 1024 * 1024;
const now = Date.now();
const iso = (ms: number | null) => (ms === null ? null : new Date(ms).toISOString());

type Point = readonly [number, number, number, number, number, number, number, number, number];

function summarize(points: readonly Point[], fromMs: number, toMs: number) {
  const sorted = [...points].sort((a, b) => a[0] - b[0]);
  let cursor = fromMs, covered = 0, gaps = 0, largestGap = 0, partial = 0, count = 0, sum = 0;
  let min = Infinity, max = -Infinity;
  const tiers: Record<string, number> = {};
  for (const [start, width, tier, c, mn, mx, sm, , pa] of sorted) {
    const s = Math.max(start, fromMs), e = start + width;
    if (s > cursor) { gaps++; largestGap = Math.max(largestGap, s - cursor); }
    covered += Math.max(0, e - Math.max(s, cursor));
    cursor = Math.max(cursor, e);
    partial += pa; count += c; sum += sm;
    min = Math.min(min, mn); max = Math.max(max, mx);
    tiers[`t${tier}`] = (tiers[`t${tier}`] ?? 0) + 1;
  }
  if (toMs > cursor) { gaps++; largestGap = Math.max(largestGap, toMs - cursor); }
  const last = sorted.at(-1);
  return {
    points: sorted.length,
    tiers,
    partial,
    coveredPct: Number((100 * covered / (toMs - fromMs)).toFixed(2)),
    gaps,
    largestGapS: Number((largestGap / 1000).toFixed(1)),
    samples: count,
    min: count ? min : null,
    max: count ? max : null,
    mean: count ? Number((sum / count).toPrecision(6)) : null,
    lastValue: last ? last[7] : null,
    lastEndAgeS: last ? Number(((now - (last[0] + last[1])) / 1000).toFixed(1)) : null,
  };
}

function inventory(dir: string) {
  return TIERS.map(tier => {
    const path = join(dir, `${tier}.ring`);
    if (!existsSync(path)) return {tier, present: false};
    const s = statSync(path);
    return {tier, present: true, bytes: s.size, mtime: iso(s.mtimeMs)};
  });
}

const origins = readdirSync(ORIGINS).filter(name => statSync(join(ORIGINS, name)).isDirectory()).sort();
const result: Record<string, unknown> = {nowIso: iso(now), origins: {}, fleetDir: existsSync(FLEET) ? readdirSync(FLEET) : "absent", emptyEverywhere: []};
const fedBy: Record<string, string[]> = {};

for (const origin of origins) {
  const dir = join(ORIGINS, origin, "metrics");
  if (!existsSync(dir)) {
    (result.origins as Record<string, unknown>)[origin] = {metricsDir: "absent"};
    continue;
  }
  const entry: Record<string, unknown> = {files: inventory(dir), windows: {}};
  for (const w of WINDOWS) {
    try {
      const series = readSeriesV1({directory: dir, fromMs: now - w.ms, toMs: now, ids: [...FOCUS, ...TOTALS]});
      const perMetric: Record<string, unknown> = {};
      for (const id of [...FOCUS, ...TOTALS]) perMetric[id] = summarize(series.metrics[id] as Point[], now - w.ms, now);
      (entry.windows as Record<string, unknown>)[w.name] = perMetric;
    } catch (error) {
      (entry.windows as Record<string, unknown>)[w.name] = {error: String(error).slice(0, 300)};
    }
  }
  try {
    const all = readSeriesV1({directory: dir, fromMs: 0, toMs: now});
    const fed: Record<string, number> = {};
    for (const metric of METRICS_CATALOGUE_V1) {
      const n = (all.metrics[metric.id] ?? []).length;
      if (n > 0) { fed[metric.id] = n; (fedBy[metric.id] ??= []).push(origin); }
    }
    entry.fedMetricsAllHistory = Object.keys(fed).length;
  } catch (error) {
    entry.fullHistoryError = String(error).slice(0, 300);
  }
  (result.origins as Record<string, unknown>)[origin] = entry;
}

result.emptyEverywhere = METRICS_CATALOGUE_V1.map(m => m.id).filter(id => !fedBy[id]);
result.fedBy = fedBy;
result.overheadBudget = {cpuPercentOfCore: OVERHEAD_CPU_PERCENT, rssBytes: OVERHEAD_RSS_BYTES};
process.stdout.write(JSON.stringify(result, null, 1) + "\n");
