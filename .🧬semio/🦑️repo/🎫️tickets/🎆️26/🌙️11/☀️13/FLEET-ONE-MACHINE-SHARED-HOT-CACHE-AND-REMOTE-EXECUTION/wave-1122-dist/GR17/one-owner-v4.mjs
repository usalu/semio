import { readFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os"; import { join } from "node:path"; import { deserialize } from "node:v8";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/", PN = INF + "📦️project-nodes/";
const wave = process.env.WAVE ?? R + "/.cache/f/owner-pool-waves/6a8e3cb0a266b981", names = (process.env.UNITS ?? "").split(",").filter(Boolean);
const MB = 1048576, rss = () => Math.round(process.memoryUsage().rss / MB), peak = () => Math.round(process.resourceUsage().maxRSS / 1024);
const phases = [], profiles = []; let cpu0 = process.cpuUsage(), t0 = performance.now();
const mark = (label, extra = {}) => { const c = process.cpuUsage(cpu0); phases.push({ label, wallMs: Math.round(performance.now() - t0), cpuMs: Math.round((c.user + c.system) / 1000), rssMB: rss(), peakMB: peak(), files: globalThis.__gr16?.calls.readFileSync ?? 0, spawns: globalThis.__gr16?.spawns.length ?? 0, hashes: globalThis.__gr16?.hashes ?? 0, ...extra }); cpu0 = process.cpuUsage(); t0 = performance.now(); };
const head = JSON.parse(readFileSync(wave + "/head.json", "utf8")), units = JSON.parse(readFileSync(wave + "/units.json", "utf8")), bytes = readFileSync(wave + "/session.bin");
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
for (const name of names) {
  const unit = units.find(u => u.name === name); if (!unit) { console.error("no unit", name); continue; }
  const job = nodes.ownerJobV1(env, head.globals, unit); mark("job:" + name);
  if (process.env.PROFILE) { await post("Profiler.enable"); await post("Profiler.setSamplingInterval", { interval: 5000 }); await post("Profiler.start"); }
  let reply, error; try { reply = await service.run(job); } catch (e) { error = String(e?.stack ?? e).slice(0, 400); }
  const si = await import(INF + "🧭️script-inputs/🟨️.mjs"); const lane = await si.closureFleetLaneV1(R); const counts = { closure: si.closureEntryCountsV1(), lane: lane?.stats?.(), verdicts: si.closureVerdictCountsV1?.(), layers: si.layerVerdictCountsV1?.() };
  profiles.push({ counts, name });
  mark("run:" + name, { kind: unit.kind, error, names: reply?.names?.length, cpuReply: reply?.cpuMilliseconds, peakReply: reply?.peakMegabytes });
  let recordSummary; try { const obs = await import(R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs"); const portable = await obs.exportProjectRecordV1(join(base, "w0-g1", "inference", "np", job.cacheKey + ".json")); const rows = portable.observations ?? portable.rows ?? []; const byKind = {}; for (const row of rows) byKind[row.kind] = (byKind[row.kind] ?? 0) + 1; recordSummary = { keys: Object.keys(portable), rows: rows.length, byKind, closure: rows.filter(r => r.kind === "closure").slice(0, 6).map(r => JSON.stringify(r).slice(0, 260)) }; } catch (e) { recordSummary = String(e?.message ?? e).slice(0, 200); }
  phases[phases.length - 1].recordSummary = recordSummary; t0 = performance.now(); cpu0 = process.cpuUsage();
}
const C = globalThis.__gr16;
console.log(JSON.stringify({ profiles, phases, calls: C?.calls, bytes: C?.bytes, distinctFilesRead: C?.files.size, spawns: C?.spawns.slice(0, 40), topPaths: Object.entries(C?.paths ?? {}).sort((a, b) => b[1] - a[1]).slice(0, 40), hashes: C?.hashes, hashBytes: C?.hashBytes, load: (await import("node:os")).loadavg() }, null, 1));
rmSync(base, { recursive: true, force: true }); process.exit(0);
