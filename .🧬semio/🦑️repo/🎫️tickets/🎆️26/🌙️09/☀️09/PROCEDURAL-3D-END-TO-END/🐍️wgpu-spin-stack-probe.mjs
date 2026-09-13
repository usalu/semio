/** 🔥️ SPIN-STACK probe — names the function a blocked worker is burning its core in.
 *
 * `🐍️wgpu-shard-liveness-probe.mjs` proves WHICH worker is blocked (it stops answering `evaluate`) and
 * that a core is pinned. This probe answers the next question — WHAT it is executing — by talking raw
 * CDP to that worker's own target: Playwright's `CDPSession` only attaches to page targets, so the
 * browser is launched with `--remote-debugging-port` and each worker's `webSocketDebuggerUrl` from
 * `/json/list` is opened directly.
 *
 * Two independent readings, because either can come back empty on a wasm-heavy stack:
 *   • `Profiler` — a 4 s CPU profile, reported as the self-time ranking. Names the hot frame even when
 *     the stack is entirely inside one wasm call.
 *   • `Debugger.pause` — one interrupt with its `callFrames`. Chrome can break into a spinning wasm
 *     loop, so this gives the exact call chain when it lands.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=… SEMIO_PROBE_OUT=wgpu-examples/spin bun 🐍️wgpu-spin-stack-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-examples/spin");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 25);
const profileSeconds = Number(process.env.SEMIO_PROBE_PROFILE ?? 4);
const port = Number(process.env.SEMIO_PROBE_CDP_PORT ?? 9333);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const note = (text) => {
  lines.push(`${Date.now() - t0} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

/** 🔌️ One raw CDP session over a target's own debugger socket — no Playwright routing in the way. */
class RawCdp {
  #socket;
  #next = 1;
  #pending = new Map();
  events = [];
  static async open(url) {
    const session = new RawCdp();
    session.#socket = new WebSocket(url);
    await new Promise((resolve, reject) => {
      session.#socket.onopen = () => resolve();
      session.#socket.onerror = (error) => reject(new Error(`cdp-open-failed: ${String(error?.message ?? error)}`));
    });
    session.#socket.onmessage = (message) => {
      const payload = JSON.parse(message.data);
      if (payload.id !== undefined) {
        const waiter = session.#pending.get(payload.id);
        session.#pending.delete(payload.id);
        waiter?.(payload);
      } else session.events.push(payload);
    };
    return session;
  }
  send(method, params = {}, timeoutMs = 20000) {
    const id = this.#next++;
    this.#socket.send(JSON.stringify({ id, method, params }));
    return new Promise((resolve) => {
      const timer = setTimeout(() => {
        this.#pending.delete(id);
        resolve({ timedOut: true, method });
      }, timeoutMs);
      this.#pending.set(id, (payload) => {
        clearTimeout(timer);
        resolve(payload);
      });
    });
  }
  waitFor(method, timeoutMs) {
    const deadline = Date.now() + timeoutMs;
    return (async () => {
      while (Date.now() < deadline) {
        const found = this.events.find((event) => event.method === method);
        if (found) return found;
        await new Promise((resolve) => setTimeout(resolve, 100));
      }
      return null;
    })();
  }
  close() { try { this.#socket.close(); } catch {} }
}

/** 📊️ Self-time ranking of a `Profiler.stop` profile — the hot frame, not the whole tree. */
const rankSelfTime = (profile) => {
  if (!profile?.nodes) return [];
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const total = profile.samples?.length || 1;
  const hits = new Map();
  for (const id of profile.samples ?? []) hits.set(id, (hits.get(id) ?? 0) + 1);
  return [...hits.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, 20)
    .map(([id, count]) => {
      const frame = byId.get(id)?.callFrame ?? {};
      return { pct: Math.round((count / total) * 1000) / 10, fn: frame.functionName || "(anonymous)", url: (frame.url || "").slice(-80), line: frame.lineNumber, col: frame.columnNumber };
    });
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", `--remote-debugging-port=${port}`] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1000)}`));

await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`goto ${String(error).slice(0, 300)}`));
for (let second = 0; second < settleSeconds; second += 1) {
  await page.waitForTimeout(1000);
  await page.mouse.move(3 + (second % 2), 3).catch(() => {});
}
note(`settled ${settleSeconds}s; status=${JSON.stringify(await page.evaluate(() => document.querySelector('[role="status"]')?.textContent ?? null).catch(() => null))}`);

const targets = await fetch(`http://127.0.0.1:${port}/json/list`).then((response) => response.json());
const workers = targets.filter((target) => target.type === "worker" || target.type === "service_worker" || target.type === "shared_worker");
note(`cdp targets=${targets.length} workers=${workers.length} urls=${JSON.stringify(workers.map((worker) => worker.url.slice(-46)))}`);

const findings = [];
for (const target of workers) {
  const label = decodeURIComponent(target.url).slice(-46);
  const session = await RawCdp.open(target.webSocketDebuggerUrl).catch((error) => ({ error: String(error) }));
  if (!(session instanceof RawCdp)) {
    findings.push({ label, error: session.error });
    continue;
  }
  const responsive = await session.send("Runtime.evaluate", { expression: "1+1", returnByValue: true }, 3000);
  const blocked = responsive.timedOut === true;
  note(`worker ${label} blocked=${blocked}`);
  const finding = { label, url: target.url, blocked };
  if (blocked) {
    await session.send("Profiler.enable", {}, 3000);
    await session.send("Profiler.setSamplingInterval", { interval: 200 }, 3000);
    await session.send("Profiler.start", {}, 3000);
    await new Promise((resolve) => setTimeout(resolve, profileSeconds * 1000));
    const stopped = await session.send("Profiler.stop", {}, 25000);
    finding.hot = rankSelfTime(stopped?.result?.profile);
    note(`worker ${label} hot=${JSON.stringify(finding.hot.slice(0, 8))}`);
    await session.send("Debugger.enable", {}, 5000);
    void session.send("Debugger.pause", {}, 5000);
    const paused = await session.waitFor("Debugger.paused", 15000);
    finding.callFrames = (paused?.params?.callFrames ?? []).slice(0, 40).map((frame) => ({ fn: frame.functionName || "(anonymous)", url: (frame.url || "").slice(-80), line: frame.location?.lineNumber, col: frame.location?.columnNumber, scriptId: frame.location?.scriptId }));
    note(`worker ${label} callFrames=${JSON.stringify(finding.callFrames.slice(0, 12))}`);
    await session.send("Debugger.resume", {}, 3000);
  }
  findings.push(finding);
  session.close();
}

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "findings.json"), JSON.stringify({ url: baseUrl, findings }, null, 2));
await browser.close();
console.log("DONE", JSON.stringify({ workers: workers.length, blocked: findings.filter((row) => row.blocked).length, out: outDir }, null, 2));
