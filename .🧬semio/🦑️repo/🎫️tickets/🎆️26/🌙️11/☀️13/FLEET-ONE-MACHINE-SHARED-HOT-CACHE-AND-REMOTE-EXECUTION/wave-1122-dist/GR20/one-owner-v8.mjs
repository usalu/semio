import { readFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os"; import { join } from "node:path"; import { deserialize } from "node:v8";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/", PN = INF + "📦️project-nodes/";
const wave = process.env.WAVE ?? R + "/.cache/f/owner-pool-waves/6a8e3cb0a266b981", names = (process.env.UNITS ?? "").split(",").filter(Boolean);
const MB = 1048576, rss = () => Math.round(process.memoryUsage().rss / MB), peak = () => Math.round(process.resourceUsage().maxRSS / 1024);
const phases = [], profiles = []; let cpu0 = process.cpuUsage(), t0 = performance.now();
const mark = (label, extra = {}) => { const c = process.cpuUsage(cpu0); phases.push({ label, wallMs: Math.round(performance.now() - t0), cpuMs: Math.round((c.user + c.system) / 1000), rssMB: rss(), peakMB: peak(), files: globalThis.__gr16?.calls.readFileSync ?? 0, spawns: globalThis.__gr16?.spawns.length ?? 0, hashes: globalThis.__gr16?.hashes ?? 0, ...extra }); cpu0 = process.cpuUsage(); t0 = performance.now(); };
const head = JSON.parse(readFileSync(wave + "/head.json", "utf8")), units = JSON.parse(readFileSync(process.env.UNITS_FILE ?? wave + "/units.json", "utf8")), bytes = readFileSync(wave + "/session.bin");
mark("start");
const { libraryBootstrap } = await import(INF + "🟨️.mjs"); await libraryBootstrap; mark("bootstrap");
const nodes = await import(PN + "🟨️.mjs"), service = await import(PN + "🏊️project-farm/👷️service/🟨️.mjs"); mark("import-modules");
const base = mkdtempSync(join(realpathSync(tmpdir()), "gr16-"));
const sent = deserialize(bytes), session = { ...sent, workspaceRoot: R, closureStore: undefined, analysisStore: nodes.analysisStoreDirectoryOfV1(sent.closureStore), stagingBase: base, authority: undefined };
await service.open(session, { slot: 0, generation: 1 }); mark("open-session");
if (process.env.STORE) { const si0 = await import(INF + "🧭️script-inputs/🟨️.mjs"); si0.bindClosureStoreV1(R, process.env.STORE, true); }
const env = service.sessionEnvironmentV1(), out = [];
import inspector from "node:inspector";
const session2 = new inspector.Session(); session2.connect(); const post = (m, p) => new Promise((res, rej) => session2.post(m, p, (e, r) => e ? rej(e) : res(r)));
let profiled = 0;
const si1 = await import(INF + "🧭️script-inputs/🟨️.mjs"); const tick = setInterval(() => { try { console.error("[GR19] " + Math.round(process.cpuUsage().user / 1e6) + "s cpu rss " + rss() + " " + JSON.stringify(si1.closureEntryCountsV1())); } catch {} }, 30000); tick.unref();
const cutoff = Number(process.env.CUTOFF_MS ?? 0); if (cutoff) setTimeout(async () => { try { if (process.env.PROFILE) { const { profile } = await post("Profiler.stop"); (await import("node:fs")).writeFileSync(process.env.PROFILE, JSON.stringify(profile)); } console.error("[GR19] cutoff cpu " + Math.round((process.cpuUsage().user + process.cpuUsage().system) / 1e6) + "s peak " + peak() + " calls " + JSON.stringify(globalThis.__gr16?.calls) + " counts " + JSON.stringify(si1.closureEntryCountsV1()) + " lane " + JSON.stringify((await si1.closureFleetLaneV1(R))?.stats?.())); } catch (e) { console.error(e); } process.exit(0); }, cutoff).unref();
for (const name of names) {
  const unit = units.find(u => u.name === name); if (!unit) { console.error("no unit", name); continue; }
  const job = nodes.ownerJobV1(env, head.globals, unit); mark("job:" + name);
  if (process.env.PROFILE && !profiled++) { await post("Profiler.enable"); await post("Profiler.setSamplingInterval", { interval: 2000 }); await post("Profiler.start"); }
  let reply, error; try { reply = await service.run(job); } catch (e) { error = String(e?.stack ?? e).slice(0, 400); }
  const si = await import(INF + "🧭️script-inputs/🟨️.mjs"); const lane = await si.closureFleetLaneV1(R); const counts = { closure: si.closureEntryCountsV1(), lane: lane?.stats?.(), verdicts: si.closureVerdictCountsV1?.(), layers: si.layerVerdictCountsV1?.() };
  profiles.push({ counts, name });
  if (process.env.WALK_STATS) { const nodes = env.scripts?.walkNodes; const names = []; for (const [, entry] of nodes ?? []) if (entry.node) names.push((entry.node.source ? "1" : "0") + entry.node.name); (await import("node:fs")).appendFileSync(process.env.WALK_STATS, JSON.stringify({ name, nodes: nodes?.size ?? 0, names }) + "\n"); }
  mark("run:" + name, { kind: unit.kind, error, names: reply?.names?.length, cpuReply: reply?.cpuMilliseconds, peakReply: reply?.peakMegabytes });
  let recordSummary; try { const obs = await import(R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs"); const portable = await obs.exportProjectRecordV1(join(base, "w0-g1", "inference", "np", job.cacheKey + ".json")); const rows = portable.observations ?? portable.rows ?? []; const byKind = {}; for (const row of rows) byKind[row.kind] = (byKind[row.kind] ?? 0) + 1; recordSummary = { keys: Object.keys(portable), rows: rows.length, byKind, closure: rows.filter(r => r.kind === "closure").slice(0, 6).map(r => JSON.stringify(r).slice(0, 260)) }; } catch (e) { recordSummary = String(e?.message ?? e).slice(0, 200); }
  phases[phases.length - 1].recordSummary = recordSummary; t0 = performance.now(); cpu0 = process.cpuUsage();
}
{ const t = performance.now(), c = process.cpuUsage(), lane = await si1.closureFleetLaneV1(R); const before = lane?.progress?.(); if (process.env.DRAIN) await si1.flushClosureOffersV1(); const used = process.cpuUsage(c); console.error("[GR19] drain " + JSON.stringify({ queuedBefore: before?.queued, offering: before?.offering, wallMs: Math.round(performance.now() - t), cpuMs: Math.round((used.user + used.system) / 1000), progress: lane?.progress?.(), stats: lane?.stats?.() })); }
if (process.env.PROFILE) { const { profile } = await post("Profiler.stop"); (await import("node:fs")).writeFileSync(process.env.PROFILE, JSON.stringify(profile)); }
if (process.env.STALE_REPORT) {
  const si = await import(INF + "🧭️script-inputs/🟨️.mjs"), { createClosureStoreV1 } = await import(INF + "🧭️script-inputs/💾️entries/🟨️.mjs"), { brotliDecompressSync } = await import("node:zlib"), { readdirSync, readFileSync: rf } = await import("node:fs");
  const dir = process.env.STORE, store = createClosureStoreV1({ directory: dir, readOnly: true }), report = [];
  for (const name of readdirSync(dir + "/e")) { const rec = JSON.parse(brotliDecompressSync(rf(dir + "/e/" + name)).toString()); const stored = await store.load(rec.id); if (!stored) continue; const entry = { ...stored, layers: undefined, store, answers: new Map(), checked: null }; const row = si.firstMovedRowOfEntryV1(entry, 60000); report.push({ fn: rec.recipe?.fn, name: rec.recipe?.name ?? rec.recipe?.port, layers: stored.digests.length, moved: row ? row.kind + " " + String(row.path).slice(-150) : null }); }
  console.error("STALE_REPORT " + JSON.stringify(report, null, 1));
}
const C = globalThis.__gr16;
console.log(JSON.stringify({ gr17: globalThis.__gr17, profiles, phases, calls: C?.calls, bytes: C?.bytes, distinctFilesRead: C?.files.size, spawns: C?.spawns.slice(0, 40), topPaths: Object.entries(C?.paths ?? {}).sort((a, b) => b[1] - a[1]).slice(0, 40), hashes: C?.hashes, hashBytes: C?.hashBytes, load: (await import("node:os")).loadavg() }, null, 1));
rmSync(base, { recursive: true, force: true }); process.exit(0);
