import { readFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os"; import { join } from "node:path"; import { deserialize } from "node:v8";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/", PN = INF + "📦️project-nodes/";
const wave = process.env.WAVE ?? R + "/.cache/f/owner-pool-waves/6a8e3cb0a266b981", names = (process.env.UNITS ?? "").split(",").filter(Boolean);
const MB = 1048576, rss = () => Math.round(process.memoryUsage().rss / MB), peak = () => Math.round(process.resourceUsage().maxRSS / 1024);
const phases = []; let cpu0 = process.cpuUsage(), t0 = performance.now();
const mark = (label, extra = {}) => { const c = process.cpuUsage(cpu0); phases.push({ label, wallMs: Math.round(performance.now() - t0), cpuMs: Math.round((c.user + c.system) / 1000), rssMB: rss(), peakMB: peak(), files: globalThis.__gr16?.calls.readFileSync ?? 0, spawns: globalThis.__gr16?.spawns.length ?? 0, hashes: globalThis.__gr16?.hashes ?? 0, ...extra }); cpu0 = process.cpuUsage(); t0 = performance.now(); };
const head = JSON.parse(readFileSync(wave + "/head.json", "utf8")), units = JSON.parse(readFileSync(wave + "/units.json", "utf8")), bytes = readFileSync(wave + "/session.bin");
mark("start");
const { libraryBootstrap } = await import(INF + "🟨️.mjs"); await libraryBootstrap; mark("bootstrap");
const nodes = await import(PN + "🟨️.mjs"), service = await import(PN + "🏊️project-farm/👷️service/🟨️.mjs"); mark("import-modules");
const base = mkdtempSync(join(realpathSync(tmpdir()), "gr16-"));
const sent = deserialize(bytes), session = { ...sent, workspaceRoot: R, closureStore: undefined, analysisStore: nodes.analysisStoreDirectoryOfV1(sent.closureStore), stagingBase: base, authority: undefined };
await service.open(session, { slot: 0, generation: 1 }); mark("open-session");
const env = service.sessionEnvironmentV1(), out = [];
for (const name of names) {
  const unit = units.find(u => u.name === name); if (!unit) { console.error("no unit", name); continue; }
  const job = nodes.ownerJobV1(env, head.globals, unit); mark("job:" + name);
  let reply, error; try { reply = await service.run(job); } catch (e) { error = String(e?.stack ?? e).slice(0, 400); }
  mark("run:" + name, { kind: unit.kind, error, names: reply?.names?.length, cpuReply: reply?.cpuMilliseconds, peakReply: reply?.peakMegabytes });
}
const C = globalThis.__gr16;
console.log(JSON.stringify({ phases, calls: C?.calls, bytes: C?.bytes, distinctFilesRead: C?.files.size, spawns: C?.spawns.slice(0, 40), topPaths: Object.entries(C?.paths ?? {}).sort((a, b) => b[1] - a[1]).slice(0, 40), hashes: C?.hashes, hashBytes: C?.hashBytes, load: (await import("node:os")).loadavg() }, null, 1));
rmSync(base, { recursive: true, force: true }); process.exit(0);
