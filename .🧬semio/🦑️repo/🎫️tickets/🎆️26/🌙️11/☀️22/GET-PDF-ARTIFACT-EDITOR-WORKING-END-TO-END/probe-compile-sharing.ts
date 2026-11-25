import { appendFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🔬️ Isolates what one compilation of the staged PDF core module costs in a renderer and whether a second worker pays it again: per mode, one fresh headless browser compiles the served core wasm in worker 1, then gives worker 2 the module either by compiling the same bytes again (`recompile`) or by structured clone of worker 1's `WebAssembly.Module` (`shared`), and samples the resident megabytes of the browser tree (peak and settled) after each step.
 * Usage: `bun probe-compile-sharing.ts` from the workspace root with the host on 6260; `PROBE_CORE` names the staged core module (default the PDF 1.7 editors component's), `PROBE_MODES` the comma-separated modes (default both). Appends to `🗑️generated/compile-sharing.jsonl`.
 * @see ./🧮️pdf-browser-page-memory.md */

const root = process.cwd();
const out = join(import.meta.dir, "🗑️generated", "compile-sharing.jsonl");
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const core = process.env.PROBE_CORE ?? "semio_hub_stdio_pdf_1_7_editors_component.core.wasm";
const wasm = `http://127.0.0.1:6260/🔌️plugin-modules/📘️stdio-pdf/${core}`;

async function treeMB(): Promise<number> {
  const text = await new Response(Bun.spawn(["ps", "-A", "-o", "pid=,ppid=,rss="]).stdout).text();
  const rows = text.split("\n").map(line => line.trim().split(/\s+/u).map(Number)).filter(row => row.length === 3);
  const inside = new Set([process.pid]);
  for (let grown = true; grown;) { grown = false; for (const [pid, ppid] of rows) if (!inside.has(pid!) && inside.has(ppid!)) { inside.add(pid!); grown = true; } }
  return Math.round(rows.filter(([pid]) => inside.has(pid!)).reduce((sum, row) => sum + row[2]!, 0) / 1024);
}

const WORKER = `onmessage = async ({ data }) => {
  if (data.kind === "compile") { const module = await WebAssembly.compileStreaming(fetch(data.url)); self.module = module; postMessage({ kind: "compiled", module: data.share ? module : undefined }); }
  if (data.kind === "adopt") { self.module = data.module; postMessage({ kind: "adopted" }); }
  if (data.kind === "drop") { self.module = undefined; postMessage({ kind: "dropped" }); }
};`;

for (const mode of (process.env.PROBE_MODES ?? "recompile,shared").split(",") as ("recompile" | "shared")[]) {
  const browser = await chromium.launch({ handleSIGHUP: false, handleSIGINT: false, handleSIGTERM: false });
  let peak = 0;
  const sampler = setInterval(async () => { peak = Math.max(peak, await treeMB()); }, 250);
  try {
    const page = await browser.newPage();
    await page.route("http://127.0.0.1:6260/__compile-probe", route => route.fulfill({ contentType: "text/html", body: "<!doctype html><title>probe</title>" }));
    await page.goto("http://127.0.0.1:6260/__compile-probe");
    const step = async (name: string, run: () => Promise<unknown>) => { peak = await treeMB(); await run(); await page.waitForTimeout(4000); const row = { core, mode, step: name, peakMB: peak, settledMB: await treeMB() }; console.log(JSON.stringify(row)); await appendFile(out, `${JSON.stringify(row)}\n`); };
    await step("blank", async () => {});
    await page.evaluate(source => { const url = URL.createObjectURL(new Blob([source], { type: "text/javascript" })); const ask = (worker: Worker, message: unknown) => new Promise<{ module?: WebAssembly.Module }>(resolve => { worker.onmessage = ({ data }) => resolve(data); worker.postMessage(message); }); Object.assign(window, { ask, workers: [new Worker(url), new Worker(url)] }); }, WORKER);
    await step("worker 1 compiles", () => page.evaluate(([url, share]) => (window as unknown as { ask: (worker: Worker, message: unknown) => Promise<{ module?: WebAssembly.Module }>; workers: Worker[] }).ask((window as unknown as { workers: Worker[] }).workers[0]!, { kind: "compile", url, share }).then(reply => { (window as unknown as { module?: WebAssembly.Module }).module = reply.module; }), [wasm, mode === "shared"] as const));
    await step(mode === "shared" ? "worker 2 adopts the clone" : "worker 2 compiles again", () => page.evaluate(([url, shared]) => { const scope = window as unknown as { ask: (worker: Worker, message: unknown) => Promise<unknown>; workers: Worker[]; module?: WebAssembly.Module }; return scope.ask(scope.workers[1]!, shared ? { kind: "adopt", module: scope.module } : { kind: "compile", url, share: false }); }, [wasm, mode === "shared"] as const));
  } finally {
    clearInterval(sampler);
    await browser.close();
  }
}
