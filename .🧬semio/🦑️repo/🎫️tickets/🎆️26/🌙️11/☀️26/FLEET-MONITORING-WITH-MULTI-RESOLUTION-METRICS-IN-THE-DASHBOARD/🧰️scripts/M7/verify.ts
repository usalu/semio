/** 🔎️ Read-only live check of M7: per origin the ids fed in the last 10 minutes (catalogue ids of the ring, platform-supported ones counted), the collector's own cost against the whole readiness process, AppleDouble stubs, and the fleet rings. Prints JSON. */
import {existsSync, readdirSync, statSync} from "node:fs";
import {join} from "node:path";
import {METRICS_CATALOGUE_V1, metricSupportedOnV1, readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";

const STORE = "/Users/Shared/semio-store/v1", WINDOW = Number(process.argv[2] ?? 600_000), now = Date.now();
const PLATFORM: Record<string, string> = {"g1-m05": "win32", "g1-m04": "linux"};
const mean = (points: readonly (readonly number[])[]) => { let c = 0, s = 0; for (const p of points) { c += p[3]!; s += p[6]!; } return c ? s / c : null; };
const out: Record<string, unknown> = {nowIso: new Date(now).toISOString(), origins: {}, fleet: {}};
for (const origin of readdirSync(join(STORE, "origins")).sort()) {
  const dir = join(STORE, "origins", origin, "metrics");
  if (!existsSync(dir)) { (out.origins as any)[origin] = {metrics: "absent"}; continue; }
  const platform = PLATFORM[origin] ?? "darwin", row: Record<string, unknown> = {};
  row.stubs = readdirSync(dir).filter(name => name.startsWith("._")).length;
  row.t0Age = Number(((now - statSync(join(dir, "t0.ring")).mtimeMs) / 1000).toFixed(0));
  try {
    const s = readSeriesV1({directory: dir, fromMs: now - WINDOW, toMs: now});
    const ids = Object.keys(s.metrics), fed = ids.filter(id => s.metrics[id]!.length > 0);
    const supported = METRICS_CATALOGUE_V1.filter(m => m.scope === "origin" && metricSupportedOnV1(m.id, platform)).map(m => m.id);
    row.ringIds = ids.length; row.fed = fed.length;
    row.missingSupportedOrigin = supported.filter(id => ids.includes(id) && s.metrics[id]!.length === 0);
    const m = (id: string) => { const v = s.metrics[id]; return v ? mean(v as any) : null; };
    row.collectorCpuPct = m("metrics.self.cpu"); row.collectorRetainedKB = (m("metrics.self.memory") ?? 0) / 1024;
    row.readyProcessCpuPct = m("process.ready.cpu"); row.readyProcessRssMB = (m("process.ready.rss") ?? 0) / 1048576;
    row.tickUs = m("metrics.self.tick.duration"); row.localHitPoints = s.metrics["cache.local.hit"]?.length ?? null; row.remoteAcHitPoints = s.metrics["cache.remote.ac.hit"]?.length ?? null;
  } catch (error) { row.error = String(error).slice(0, 200); }
  (out.origins as any)[origin] = row;
}
for (const source of existsSync(join(STORE, "fleet", "metrics")) ? readdirSync(join(STORE, "fleet", "metrics")) : []) {
  const dir = join(STORE, "fleet", "metrics", source), s = readSeriesV1({directory: dir, fromMs: now - WINDOW, toMs: now});
  (out.fleet as any)[source] = Object.fromEntries(Object.entries(s.metrics).map(([id, p]) => [id, {points: p.length, last: p.at(-1)?.[7] ?? null}]));
}
process.stdout.write(JSON.stringify(out, null, 1) + "\n");
