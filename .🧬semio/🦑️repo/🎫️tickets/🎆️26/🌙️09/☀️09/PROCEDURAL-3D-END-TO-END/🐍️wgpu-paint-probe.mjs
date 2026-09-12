/** 🖌️ wgpu BLANK-PAINT probe — counts the frame batches the UI isolate actually transfers.
 *
 * `🐍️wgpu-probe.mjs` reads the Worker's introspection beacon; it cannot see whether the UI isolate is
 * driving frames at all. This one patches `Worker.prototype.postMessage` and the worker's `message`
 * handler in the page BEFORE boot, so every `batch`/`frame` crossing is counted with its
 * `requestFrame` answer. A renderer that ingests one document page per frame step needs thousands of
 * frames; a loop that stops after the first `booted` frame produces exactly one.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-paint/run-1 bun 🐍️wgpu-paint-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-paint/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.addInitScript(() => {
  const tally = { sent: {}, received: {}, requestFrameTrue: 0, requestFrameFalse: 0, firstBatchAt: 0, lastBatchAt: 0, sequences: [], cursors: {} };
  globalThis.__semioFrameTally = tally;
  const originalPost = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function patched(message, transfer) {
    const kind = message && typeof message === "object" ? String(message.kind ?? "?") : typeof message;
    tally.sent[kind] = (tally.sent[kind] ?? 0) + 1;
    if (kind === "batch") {
      if (!tally.firstBatchAt) tally.firstBatchAt = Math.round(performance.now());
      tally.lastBatchAt = Math.round(performance.now());
      if (tally.sequences.length < 12) tally.sequences.push(message.sequence);
    }
    const observe = (event) => {
      const data = event.data;
      if (!data || typeof data !== "object") return;
      const replyKind = String(data.kind ?? "?");
      tally.received[replyKind] = (tally.received[replyKind] ?? 0) + 1;
      if (replyKind === "frame") {
        if (data.requestFrame) tally.requestFrameTrue += 1;
        else tally.requestFrameFalse += 1;
        const cursor = String(data.cursor ?? "?");
        tally.cursors[cursor] = (tally.cursors[cursor] ?? 0) + 1;
      }
    };
    if (!this.__semioObserved) {
      this.__semioObserved = true;
      this.addEventListener("message", observe);
    }
    return originalPost.call(this, message, transfer);
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

const samples = [];
for (let second = 0; second < seconds; second += 1) {
  await page.waitForTimeout(1000);
  const sample = await page.evaluate(async () => {
    const beacon = globalThis.semioWgpuIntrospection;
    let nodes = 0;
    let stats = null;
    if (typeof beacon?.dumpStructure === "function") {
      try { const raw = await beacon.dumpStructure(); const dump = raw ? JSON.parse(raw) : null; nodes = Array.isArray(dump?.nodes) ? dump.nodes.length : 0; } catch { nodes = -1; }
      try { const raw = await beacon.dumpFrameStats(); stats = raw ? JSON.parse(raw) : null; } catch { stats = null; }
    }
    return { tally: JSON.parse(JSON.stringify(globalThis.__semioFrameTally ?? {})), nodes, stats, alert: document.querySelector('[role="alert"]')?.textContent?.slice(0, 1200) ?? null };
  }).catch((error) => ({ error: String(error).slice(0, 300) }));
  sample.t = at();
  samples.push(sample);
  if (sample.alert) { lines.push(`${at()} PROBE fault banner`); break; }
  if (sample.nodes > 0) { lines.push(`${at()} PROBE nodes ${sample.nodes}`); break; }
}

const last = samples.at(-1);
await page.screenshot({ path: join(outDir, "final.png"), type: "png" }).catch(() => {});
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples, null, 2));
console.log("DONE", JSON.stringify({ seconds: Math.round(at() / 1000), nodes: last?.nodes, stats: last?.stats, tally: last?.tally, alert: last?.alert?.slice(0, 200) ?? null }, null, 2));
await browser.close();
