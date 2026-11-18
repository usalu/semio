// R129: memory of a farm child over a sequence of owners. env: R129_OWNERS=csv R129_WAVE=dir R129_GC=0|1 (worker's own Bun.gc(true) after each job) R129_CODE=root of code
import { readFileSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { deserialize } from "node:v8";
import * as JSC from "bun:jsc";
const root = "/Users/ueli/Documents/semio", code = process.env.R129_CODE ?? root;
const B = code + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build", PN = B + "/🕸️graph/📥️inference/📦️project-nodes";
const MB = 1048576, mem = () => { const m = process.memoryUsage(); return { rss: Math.round(m.rss / MB), heapUsed: Math.round(m.heapUsed / MB), heapTotal: Math.round(m.heapTotal / MB), external: Math.round(m.external / MB), arrayBuffers: Math.round(m.arrayBuffers / MB) }; };
const top = (n = 8) => { const h: any = (JSC as any).heapStats(); return Object.entries(h.objectTypeCounts as Record<string, number>).sort((a, b) => b[1] - a[1]).slice(0, n).map(([k, v]) => k + ":" + v).join(" "); };
import { createRequire } from "node:module";
const req = createRequire(import.meta.url);
const mods = () => { const keys = Object.keys(req.cache ?? {}); const ts = keys.filter(k => /\/typescript\/(lib\/)?typescript\.js$/.test(k)); return { count: keys.length, typescript: ts.length }; };
import { spawnSync } from "node:child_process";
const vm = () => { if (process.env.R129_VMMAP !== "1") return undefined; const r = spawnSync("vmmap", ["-summary", String(process.pid)], { encoding: "utf8" }).stdout ?? ""; const sec = r.split("\n"), at = sec.findIndex(l => l.startsWith("REGION TYPE")), keep = sec.filter((l, i) => /^Physical footprint/.test(l) || (i > at && i < at + 40 && l.trim() !== "" && !/^(Stack|__|dyld|Activity|shared|mapped file)/.test(l.trim()))); return keep.map(l => l.trim().replace(/\s+/g, " ").slice(0, 110)); };
const FP: any = await import(B + "/🧠️memory/🦶️footprint/🟨️.mjs"), BC: any = await import(B + "/🧊️bounded/🟨️.mjs");
const fp = async () => Math.round(((await FP.readProcessFootprintBytesV1(process.pid)) ?? 0) / MB);
const caches = () => { const s: any[] = BC.boundedCacheStatsV1().filter((c: any) => c.bytes > 0); const total = s.reduce((a: number, c: any) => a + c.bytes, 0); return { totalMb: Math.round(total / MB), top: s.sort((a: any, b: any) => b.bytes - a.bytes).slice(0, 4).map((c: any) => c.name.replace("inference:", "") + ":" + Math.round(c.bytes / MB) + "/" + Math.round(c.maxBytes / MB)).join(" ") }; };
const OB: any = await import(B + "/👁️observation/🟨️.mjs");
const sizes = () => { const z = OB.inferenceCacheSizesV1(); return { interned: z.interned, rowStore: z.rowStore, analysisMb: Math.round(z.analysisBytes / MB), settledMb: Math.round(z.settledReadBytes / MB), fileRecMb: Math.round(z.fileRecordImageBytes / MB), blockRows: z.verifiedBlockRows, projectImages: z.projectImages, pubs: z.publications }; };
const colls = () => { try { return OB.observationCollectionsV1().map((c: any) => ({ n: String(c.name).replace("observation:", ""), e: c.entries, mb: Math.round((c.bytes ?? 0) / MB * 10) / 10 })).filter((c: any) => c.mb >= 1).sort((a: any, b: any) => b.mb - a.mb).slice(0, 8).map((c: any) => c.n + ":" + c.e + "/" + c.mb).join(" "); } catch (e: any) { return "err " + e.message; } };
const out = (label: string, extra: any = {}) => console.log(JSON.stringify({ label, ...mem(), ...extra }));
out("start");
const { libraryBootstrap } = await import(B + "/🕸️graph/📥️inference/🟨️.mjs"); await libraryBootstrap; out("bootstrap", { modules: mods() });
const R: any = await import(PN + "/🟨️.mjs"); out("project-nodes", { modules: mods() });
const dir = process.env.R129_WAVE ?? "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129/slim";
const head = JSON.parse(readFileSync(dir + "/head.json", "utf8")), units: any[] = JSON.parse(readFileSync(dir + "/units.json", "utf8")), session = deserialize(readFileSync(dir + "/session.bin"));
const base = mkdtempSync(join(realpathSync(tmpdir()), "semio-r129-"));
const service: any = await import(PN + "/🏊️project-farm/👷️service/🟨️.mjs"); out("service", { modules: mods() });
await service.open({ ...session, workspaceRoot: root, closureStore: process.env.R129_STORE ?? undefined, stagingBase: base, authority: undefined }, { slot: 0, generation: 1, megabytes: Number(process.env.R129_MB ?? 1536) });
out("idle", { modules: mods(), vm: vm(), footprint: await fp(), types: top() });
const envr = service.sessionEnvironmentV1(), names = (process.env.R129_OWNERS ?? "").split(",").filter(Boolean);
for (const name of names) {
  const unit = units.find(u => u.name === name); if (!unit) { console.log("missing", name); continue; }
  const job = R.ownerJobV1(envr, head.globals, unit), t = performance.now(), c = process.cpuUsage();
  let status = "ok"; try { await service.run(job); } catch (e: any) { status = "err:" + String(e?.code ?? e?.message).slice(0, 80); }
  const during = mem(), u = process.cpuUsage(c), fpDuring = await fp(), cachesDuring = caches();
  const retained = sizes();
  if (process.env.R129_RELEASE === "1") { OB.releaseInferenceMemoryV1(); BC.shrinkBoundedCachesV1(0); }
  (globalThis as any).Bun.gc(true); const trimmed = typeof service.trimRetainedV1 === "function" && service.trimRetainedV1() === true; if (trimmed) (globalThis as any).Bun.gc(true); const afterGc = mem().rss;
  (globalThis as any).Bun.shrink(); const afterShrink = mem().rss; (globalThis as any).Bun.gc(true);
  out(name.replace("@semio-tech/", ""), { status, kind: unit.kind, seconds: Math.round((performance.now() - t) / 100) / 10, cpuS: Math.round((u.user + u.system) / 1e5) / 10, maxRss: Math.round(process.resourceUsage().maxRSS / 1024), duringRss: during.rss, footprint: fpDuring, afterGcRss: afterGc, afterShrinkRss: afterShrink, footprintAfterShrink: await fp(), trimmed, caches: cachesDuring, retained, colls: colls(), modules: mods(), vm: vm(), types: top() });
}
out("end", { modules: mods(), types: top(14) });
rmSync(base, { recursive: true, force: true }); process.exit(0);
