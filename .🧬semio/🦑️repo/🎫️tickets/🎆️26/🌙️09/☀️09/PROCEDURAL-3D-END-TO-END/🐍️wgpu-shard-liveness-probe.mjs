/** 🫀️ wgpu SHARD-LIVENESS probe — answers the ONE question the console cannot: when the host's first
 * `submitTurn(surface-visible)` never returns, is the shard worker COMPUTE-BOUND inside a guest wasm
 * call, or is it IDLE because the turn message never reached it?
 *
 * The shard workers are page-level dedicated workers (`MainThreadShardWorker` posts `shard-spawn` up to
 * the UI isolate, which is what constructs the real `Worker` — `🐚️plugin-bridge/🟦️.ts` §🧵️MainThreadShardWorkers),
 * so Playwright's `page.workers()` sees every one of them. A `worker.evaluate()` that does not answer
 * inside `SEMIO_PROBE_PING_MS` proves that worker's event loop is BLOCKED (a synchronous wasm call);
 * one that answers proves it is idle and the message is the thing that is missing.
 *
 * Browser-process CPU is sampled beside it (`ps`) so "blocked" is separable from "blocked and burning
 * a core" — a deadlocked atomic wait parks at 0 %, a spinning guest pins one core.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-examples/shard-liveness bun 🐍️wgpu-shard-liveness-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { execFileSync } from "node:child_process";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-examples/shard-liveness");
const watchSeconds = Number(process.env.SEMIO_PROBE_WATCH ?? 90);
const pingMs = Number(process.env.SEMIO_PROBE_PING_MS ?? 4000);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 1500)}`));
page.on("worker", (worker) => note(`worker created ${worker.url().slice(-70)}`));

/** 🫀️ A worker whose event loop is free answers a trivial evaluate; a blocked one never does. */
const ping = async (worker) => {
  const started = Date.now();
  const verdict = await Promise.race([
    worker.evaluate(() => ({ now: Date.now(), scope: typeof self, raf: typeof self.requestAnimationFrame })).then((value) => ({ alive: true, value })),
    new Promise((resolve) => setTimeout(() => resolve({ alive: false }), pingMs)),
  ]).catch((error) => ({ alive: false, error: String(error).slice(0, 200) }));
  return { url: worker.url().slice(-70), ms: Date.now() - started, ...verdict };
};

/** 📈️ Total CPU of every headless-browser process, and the busiest single one. */
const cpu = () => {
  try {
    const rows = execFileSync("ps", ["-A", "-o", "pid=,%cpu=,comm="], { encoding: "utf8" })
      .split("\n")
      .map((row) => row.trim())
      .filter((row) => /Chromium|Google Chrome for Testing|headless_shell/i.test(row))
      .map((row) => {
        const [pid, pct] = row.split(/\s+/);
        return { pid: Number(pid), pct: Number(pct) };
      });
    return { total: Math.round(rows.reduce((sum, row) => sum + row.pct, 0)), busiest: Math.round(Math.max(0, ...rows.map((row) => row.pct))), processes: rows.length };
  } catch (error) {
    return { error: String(error).slice(0, 120) };
  }
};

await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 400)}`));

const samples = [];
for (let second = 0; second < watchSeconds; second += 5) {
  await page.waitForTimeout(5000);
  await page.mouse.move(3 + (second % 2), 3).catch(() => {});
  const workers = page.workers();
  const pings = [];
  for (const worker of workers) pings.push(await ping(worker));
  const status = await page.evaluate(() => document.querySelector('[role="status"]')?.textContent?.trim() ?? null).catch(() => null);
  const sample = { t: at(), status, cpu: cpu(), workers: pings };
  samples.push(sample);
  note(`t=${Math.round(sample.t / 1000)}s status=${JSON.stringify(status)} cpu=${JSON.stringify(sample.cpu)} workers=${JSON.stringify(pings)}`);
}

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify({ url: baseUrl, samples }, null, 2));
await browser.close();
console.log("DONE", JSON.stringify({ samples: samples.length, out: outDir }, null, 2));
