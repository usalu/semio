import { appendFile, copyFile, readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧮️ Measures where the memory of one PDF editor page goes: the resident megabytes of the browser tree per process type (the warden's own `ps rss` sum over this runner and its children) and, in the page and in every shard worker, the `WebAssembly.Module` compilations and the live modules and `WebAssembly.Memory` instances with their sizes, at boot, after an edit, after each Open File and after the predecessors were retired.
 * Usage: `bun probe-memory.ts [label]` from the workspace root with the host on 6260; `PROBE_OPENS` sets how many times the two-page fixture is opened (default 2), `PROBE_ABORT_MB` closes the browser before the host warden's cap, `PROBE_JS_FLAGS`/`PROBE_CHROME_ARGS` pass diagnostic V8 and Chrome flags, `PROBE_DUMPS=0` skips the memory-infra dumps, `PROBE_TIMELINE=1` logs a sample per second, `PROBE_DUMP_MB`/`PROBE_VMMAP_MB`/`PROBE_SAMPLE_MB` take one memory-infra dump, vmmap summary or `sample` of the renderer once the tree passes that size. Instruments the staged shard worker with a `[DEBUG]` prelude for the run and restores it afterwards. Writes `🗑️generated/memory-<label>.jsonl`.
 * @see ./🧮️pdf-browser-page-memory.md */

const root = process.cwd();
const label = process.argv[2] ?? "probe";
const out = join(import.meta.dir, "🗑️generated", `memory-${label}.jsonl`);
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const shard = join(root, ".cache/b/p/m/browser/prototype/modules/🧵️shard/🟨️shard-worker.js");
const backup = join(import.meta.dir, "🗑️generated", "shard-worker.original.js");
const ABORT_MB = Number(process.env.PROBE_ABORT_MB ?? 3900);
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");

const PRELUDE = `/* [DEBUG] wasm memory probe */
(() => {
  const W = WebAssembly, s = { compiles: 0, largeCompiles: 0, instances: 0, modules: [], memories: [], events: [] };
  const mark = (what) => { s.events.push([Date.now(), what]); console.log("[DEBUG] wasm " + what); };
  Object.defineProperty(self, "__semioWasmProbe", { value: s });
  const track = (module, bytes, how) => { s.compiles++; if (bytes > 1048576) s.largeCompiles++; s.modules.push({ ref: new WeakRef(module), bytes, how }); return module; };
  const compile = W.compile; W.compile = async (b, ...r) => track(await compile.call(W, b, ...r), b.byteLength, "compile");
  const streaming = W.compileStreaming; W.compileStreaming = async (source, ...r) => { const response = await source; mark("compile-start"); const compiled = await streaming.call(W, response, ...r); mark("compile-end"); return track(compiled, Number(response.headers.get("content-length") ?? -1), "streaming " + decodeURIComponent(response.url).split("/").pop()); };
  const instantiate = W.instantiate; W.instantiate = async (a, imports, ...r) => { mark("instantiate-start"); const result = await instantiate.call(W, a, imports, ...r); mark("instantiate-end"); const instance = result instanceof W.Instance ? result : result.instance; if (!(a instanceof W.Module)) track(result.module, a.byteLength, "instantiate-bytes"); s.instances++; for (const [name, value] of Object.entries(instance.exports)) if (value instanceof W.Memory) s.memories.push({ ref: new WeakRef(value), name }); return result; };
  const Memory = W.Memory; W.Memory = function (descriptor) { const memory = new Memory(descriptor); s.memories.push({ ref: new WeakRef(memory), name: "constructed", maximum: descriptor.maximum }); return memory; }; W.Memory.prototype = Memory.prototype;
  if (!self.document) {
    self.addEventListener("message", (event) => { const data = event.data ?? {}; if (data.kind === "dispose" || data.kind === "activate") console.log("[DEBUG] wasm " + data.kind + " " + data.actorId + "#" + data.activationGeneration + (data.coreModules ? " seeds=" + data.coreModules.length : "")); });
    const post = self.postMessage.bind(self);
    self.postMessage = (message, ...rest) => { if (message && (message.kind === "original-source-close" || message.kind === "core-module")) console.log("[DEBUG] wasm " + message.kind + " " + message.actorId + "#" + message.activationGeneration + " " + (message.kind === "core-module" ? decodeURIComponent(message.url).split("/").pop() : "sourceClosed=" + message.sourceClosed)); return post(message, ...rest); };
  }
  const weigh = (value, seen = new Set(), depth = 0) => { if (value === null || typeof value !== "object" || depth > 12 || seen.has(value)) return typeof value === "string" ? value.length : 8; seen.add(value); if (ArrayBuffer.isView(value)) return value.byteLength; if (value instanceof ArrayBuffer) return value.byteLength; let total = 16; for (const key in value) total += weigh(value[key], seen, depth + 1); return total; };
  s.posts = {};
  const record = (direction, message) => { const kind = direction + ":" + (message?.kind ?? "?") + (message?.kind === "frame" ? ":" + (message.frame?.envelope?.payload?.kind ?? "?") : ""); const bytes = weigh(message); const entry = (s.posts[kind] ??= { count: 0, bytes: 0, maxBytes: 0 }); entry.count++; entry.bytes += bytes; entry.maxBytes = Math.max(entry.maxBytes, bytes); };
  if (self.document) { const post = Worker.prototype.postMessage; Worker.prototype.postMessage = function (message, ...rest) { record("main->worker", message); return post.call(this, message, ...rest); }; }
  else { const post = self.postMessage.bind(self); self.postMessage = (message, ...rest) => { record("worker->main", message); return post(message, ...rest); }; }
  let last = "";
  setInterval(() => { const line = s.memories.map(entry => Math.round((entry.ref.deref()?.buffer.byteLength ?? 0) / 1048576)).join(",") + " js=" + (self.performance?.memory ? Math.round(self.performance.memory.usedJSHeapSize / 1048576) : "-"); if (line !== last) { last = line; console.log("[DEBUG] wasm tick " + (self.document ? "main" : "worker") + " mem=" + line); } }, 250);
  s.report = () => { try { globalThis.gc?.(); } catch {} const modules = s.modules.filter(entry => entry.ref.deref() !== undefined); const memories = s.memories.map(entry => ({ name: entry.name, bytes: entry.ref.deref()?.buffer.byteLength })).filter(entry => entry.bytes !== undefined); return { compiles: s.compiles, largeCompiles: s.largeCompiles, instances: s.instances, liveModules: modules.length, liveModuleBytes: modules.reduce((sum, entry) => sum + Math.max(0, entry.bytes), 0), largeModules: modules.filter(entry => entry.bytes > 1e6).map(entry => entry.how), liveMemories: memories.length, liveMemoryBytes: memories.reduce((sum, entry) => sum + entry.bytes, 0), events: s.events.splice(0), posts: Object.fromEntries(Object.entries(s.posts).filter(([, entry]) => entry.bytes > 1048576).map(([kind, entry]) => [kind, { count: entry.count, MB: Math.round(entry.bytes / 1048576), maxMB: Math.round(entry.maxBytes / 1048576 * 10) / 10 }])), memoryMB: memories.map(entry => Math.round(entry.bytes / 1048576)), js: self.performance?.memory ? Math.round(self.performance.memory.usedJSHeapSize / 1048576) : undefined }; };
})();
`;

type Row = { pid: number; ppid: number; rss: number; args: string };
async function tree(): Promise<{ totalMB: number; byType: Record<string, number>; processes: { pid: number; type: string; mb: number }[] }> {
  const text = await new Response(Bun.spawn(["ps", "-A", "-o", "pid=,ppid=,rss=,args="]).stdout).text();
  const rows: Row[] = text.split("\n").map(line => line.trim().match(/^(\d+)\s+(\d+)\s+(\d+)\s+(.*)$/u)).filter(match => match !== null).map(match => ({ pid: Number(match[1]), ppid: Number(match[2]), rss: Number(match[3]), args: match[4]! }));
  const inside = new Set([process.pid]);
  for (let grown = true; grown;) { grown = false; for (const row of rows) if (!inside.has(row.pid) && inside.has(row.ppid)) { inside.add(row.pid); grown = true; } }
  const processes = rows.filter(row => inside.has(row.pid)).map(row => ({ pid: row.pid, type: row.pid === process.pid ? "runner" : row.args.match(/--type=([a-z-]+)/u)?.[1] ?? (row.args.includes("--utility-sub-type") ? "utility" : "browser"), mb: Math.round(row.rss / 1024), sub: row.args.match(/--utility-sub-type=([\w.]+)/u)?.[1] }));
  const byType: Record<string, number> = {};
  for (const entry of processes) byType[entry.type + (entry.sub ? `:${entry.sub}` : "")] = (byType[entry.type + (entry.sub ? `:${entry.sub}` : "")] ?? 0) + entry.mb;
  return { totalMB: processes.reduce((sum, entry) => sum + entry.mb, 0), byType, processes: processes.map(({ pid, type, mb }) => ({ pid, type, mb })) };
}

let peak = 0;
let probedPage: import("playwright").Page | undefined;
const footprint = async (pids: readonly number[]): Promise<number> => {
  const text = await new Response(Bun.spawn(["top", "-l", "1", "-stats", "pid,mem"]).stdout).text();
  const wanted = new Set(pids);
  const scale: Record<string, number> = { B: 1 / 1048576, K: 1 / 1024, M: 1, G: 1024 };
  return Math.round(text.split("\n").map(line => line.trim().match(/^(\d+)\s+([\d.]+)([BKMG])\+?-?$/u)).filter(match => match !== null && wanted.has(Number(match[1]))).reduce((sum, match) => sum + Number(match![2]) * scale[match![3]!]!, 0));
};
let mapped = false;
let dumped = false;
let sampled = false;
const SAMPLE_MB = Number(process.env.PROBE_SAMPLE_MB ?? 0);
const DUMP_MB = Number(process.env.PROBE_DUMP_MB ?? 0);
const VMMAP_MB = Number(process.env.PROBE_VMMAP_MB ?? 0);
const rendererFootprint = async (): Promise<{ pid: number; footprintMB: number; peakFootprintMB: number } | undefined> => {
  const renderer = (await tree()).processes.filter(entry => entry.type === "renderer").sort((left, right) => right.mb - left.mb)[0];
  if (renderer === undefined) return undefined;
  const text = await new Response(Bun.spawn(["vmmap", "-summary", String(renderer.pid)], { stderr: "pipe" }).stdout).text();
  const megabytes = (label: string) => { const match = text.match(new RegExp(`${label}:\\s+([\\d.]+)([KMG])`, "u")); return match ? Math.round(Number(match[1]) * ({ K: 1 / 1024, M: 1, G: 1024 } as Record<string, number>)[match[2]!]!) : -1; };
  return { pid: renderer.pid, footprintMB: megabytes("Physical footprint"), peakFootprintMB: megabytes("Physical footprint \\(peak\\)") };
};
const vmmap = async (name: string) => { const renderer = (await tree()).processes.filter(entry => entry.type === "renderer").sort((left, right) => right.mb - left.mb)[0]; if (renderer === undefined) return; const text = await new Response(Bun.spawn(["vmmap", "-summary", String(renderer.pid)], { stderr: "pipe" }).stdout).text(); await writeFile(join(import.meta.dir, "🗑️generated", `vmmap-${label}-${name}.txt`), text); };
const original = await readFile(shard, "utf8");
if (original.includes("[DEBUG] wasm memory probe")) throw new Error("the staged shard worker is already instrumented");
await copyFile(shard, backup);
await writeFile(shard, PRELUDE + original);
await writeFile(out, "");
const browser = await chromium.launch({ args: [`--js-flags=--expose-gc ${process.env.PROBE_JS_FLAGS ?? ""}`.trim(), ...(process.env.PROBE_CHROME_ARGS ?? "").split(" ").filter(Boolean)], handleSIGHUP: false, handleSIGINT: false, handleSIGTERM: false });
const sampler = setInterval(async () => { const now = await tree(); peak = Math.max(peak, now.totalMB); if (process.env.PROBE_TIMELINE === "1") await appendFile(out, `${JSON.stringify({ stage: "sample", atMs: Date.now(), totalMB: now.totalMB, rendererMB: now.byType.renderer })}\n`); if (VMMAP_MB > 0 && !mapped && now.totalMB > VMMAP_MB) { mapped = true; await vmmap(`peak-${now.totalMB}`); }
  if (SAMPLE_MB > 0 && !sampled && now.totalMB > SAMPLE_MB) { sampled = true; const renderer = now.processes.filter(entry => entry.type === "renderer").sort((left, right) => right.mb - left.mb)[0]; if (renderer) Bun.spawn(["sample", String(renderer.pid), "3", "-file", join(import.meta.dir, "🗑️generated", `sample-${label}.txt`)], { stdout: "ignore", stderr: "ignore" }); }
  if (DUMP_MB > 0 && !dumped && now.totalMB > DUMP_MB) { dumped = true; await appendFile(out, `${JSON.stringify({ stage: "peak-dump", atMs: Date.now(), totalMB: now.totalMB, dump: await allocators() })}\n`); }
  if (now.totalMB > ABORT_MB) { const within = <T,>(work: Promise<T>) => Promise.race([work, new Promise<string>(resolve => setTimeout(() => resolve("busy"), 2000))]).catch(error => String(error).slice(0, 120)); const reports = probedPage ? await Promise.all([within(probedPage.evaluate(() => ({ nodes: document.querySelectorAll("*").length, canvases: [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`) }))), within(probedPage.evaluate(() => (self as unknown as { __semioWasmProbe: { report(): unknown } }).__semioWasmProbe.report())), ...probedPage.workers().map(worker => within(worker.evaluate(() => (self as unknown as { __semioWasmProbe?: { report(): unknown } }).__semioWasmProbe?.report() ?? "uninstrumented")))]) : []; await appendFile(out, `${JSON.stringify({ stage: "abort", tree: now, renderer: await rendererFootprint(), reports })}\n`); await browser.close(); } }, 1000);
const cdp = await browser.newBrowserCDPSession();
const allocators = async (): Promise<Record<string, Record<string, number>>> => {
  const events: { ph: string; pid: number; name: string; args?: { name?: string; dumps?: { allocators?: Record<string, { attrs?: Record<string, { value: string }> }> } } }[] = [];
  const collect = (event: { value: typeof events }) => { events.push(...event.value); };
  cdp.on("Tracing.dataCollected", collect);
  const complete = new Promise(resolve => cdp.once("Tracing.tracingComplete", resolve));
  await cdp.send("Tracing.start", { traceConfig: { includedCategories: ["disabled-by-default-memory-infra", "disabled-by-default-memory-infra.v8.code_stats"], excludedCategories: ["*"], memoryDumpConfig: { triggers: [] } }, transferMode: "ReportEvents" });
  await cdp.send("Tracing.requestMemoryDump", { deterministic: true, levelOfDetail: "detailed" });
  await cdp.send("Tracing.end");
  await complete;
  cdp.off("Tracing.dataCollected", collect);
  if (process.env.PROBE_RAW_DUMP === "1") await writeFile(join(import.meta.dir, "🗑️generated", `dump-${label}-${Date.now()}.json`), JSON.stringify(events));
  const names = new Map(events.filter(event => event.ph === "M" && event.name === "process_name").map(event => [event.pid, event.args?.name ?? "?"]));
  const result: Record<string, Record<string, number>> = {};
  for (const event of events) {
    const dumped = event.ph === "v" ? event.args?.dumps?.allocators : undefined;
    if (!dumped) continue;
    const entry: Record<string, number> = {};
    for (const [name, value] of Object.entries(dumped)) {
      const bytes = parseInt(value.attrs?.size?.value ?? "0", 16);
      if (bytes >= 8 * 1048576 && name.split("/").length <= 4 && !/\/0x|allocated_objects/u.test(name)) entry[name] = Math.round(bytes / 1048576);
    }
    result[`${names.get(event.pid) ?? "?"} ${event.pid}`] = entry;
  }
  return result;
};
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  probedPage = page;
  await page.addInitScript(PRELUDE);
  if (process.env.PROBE_HARDWARE_CONCURRENCY) await page.addInitScript(`Object.defineProperty(Navigator.prototype, "hardwareConcurrency", { get: () => ${Number(process.env.PROBE_HARDWARE_CONCURRENCY)} });`);
  const onConsole = (message: { text(): string }) => { if (message.text().startsWith("[DEBUG] wasm")) void appendFile(out, `${JSON.stringify({ stage: "event", atMs: Date.now(), what: message.text() })}\n`); };
  page.on("console", onConsole);
  const measure = async (stage: string) => {
    await page.waitForTimeout(3000);
    const main = await page.evaluate(() => (self as unknown as { __semioWasmProbe: { report(): unknown } }).__semioWasmProbe.report()).catch(error => String(error));
    const workers = await Promise.all(page.workers().map(worker => Promise.race([worker.evaluate(() => (self as unknown as { __semioWasmProbe?: { report(): unknown } }).__semioWasmProbe?.report() ?? "uninstrumented"), new Promise(resolve => setTimeout(() => resolve("busy"), 20_000))]).then(report => ({ url: decodeURIComponent(worker.url()).split("/").pop(), report })).catch(error => ({ url: worker.url(), report: String(error).slice(0, 120) }))));
    const agent = await page.evaluate(async () => { const measureMemory = (performance as unknown as { measureUserAgentSpecificMemory?: () => Promise<{ bytes: number; breakdown: { bytes: number; types: string[]; attribution: { scope: string; url: string }[] }[] }> }).measureUserAgentSpecificMemory; if (!crossOriginIsolated || !measureMemory) return { crossOriginIsolated }; const result = await measureMemory.call(performance); return { crossOriginIsolated, totalMB: Math.round(result.bytes / 1048576), breakdown: result.breakdown.filter(entry => entry.bytes > 4 * 1048576).map(entry => ({ mb: Math.round(entry.bytes / 1048576), types: entry.types, scope: entry.attribution.map(item => `${item.scope} ${decodeURIComponent(item.url).split("/").pop()}`) })) }; }).catch(error => ({ error: String(error).slice(0, 200) }));
    const dom = await page.evaluate(() => ({ nodes: document.querySelectorAll("*").length, canvases: [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`) })).catch(error => String(error).slice(0, 120));
    const dump = process.env.PROBE_DUMPS === "0" ? undefined : await allocators().catch(error => ({ error: { [String(error).slice(0, 200)]: 0 } }));
    if (VMMAP_MB > 0) await vmmap(stage);
    const now = await tree();
    const row = { stage, atMs: Date.now(), tree: now, renderer: await rendererFootprint(), footprintMB: await footprint(now.processes.map(entry => entry.pid)), peakMB: peak, main, workers, agent, dom, dump };
    console.log(`[DEBUG] ${stage}: tree ${row.tree.totalMB} MB rss, ${row.footprintMB} MB footprint (peak rss ${peak}), renderer footprint ${row.renderer?.footprintMB} MB (peak ${row.renderer?.peakFootprintMB} MB) ${JSON.stringify(row.tree.byType)} workers=${workers.length}`);
    await appendFile(out, `${JSON.stringify(row)}\n`);
  };
  const open = async () => {
    await unfoldActionsRail(page);
    const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30_000 }), clickUncovered(page, '[data-slot="window-action-pane"] [id="action.openArtifactFile"]')]);
    await appendFile(out, `${JSON.stringify({ stage: "open-start", atMs: Date.now() })}\n`);
    await chooser.setFiles(fixture);
    for (const deadline = Date.now() + 120_000; Date.now() < deadline; await page.waitForTimeout(500)) if ((await page.evaluate(() => document.body.innerText)).includes("Page 2 / 2")) { await appendFile(out, `${JSON.stringify({ stage: "open-shown", atMs: Date.now() })}\n`); break; }
  };
  await page.goto("http://127.0.0.1:6260/");
  console.log("beacon", await awaitBeacon(page, Date.now() + 240_000));
  await dismissIntroduction(page);
  await measure("boot");
  const field = page.getByRole("textbox", { name: "Title", exact: true }).first();
  await field.waitFor({ state: "visible", timeout: 30_000 });
  await field.fill("Measured");
  await field.press("Enter");
  await measure("edited");
  for (let index = 1; index <= Number(process.env.PROBE_OPENS ?? 2); index++) {
    await open();
    await measure(`open-${index}`);
  }
  await page.waitForTimeout(20_000);
  await measure("settled");
} finally {
  clearInterval(sampler);
  await browser.close().catch(() => {});
  await writeFile(shard, original);
  console.log(`[DEBUG] peak ${peak} MB, restored ${shard}`);
}
