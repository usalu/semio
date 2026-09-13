/** 🔌️ wgpu UI→Worker WIRE probe.
 *
 * Answers the one question `📓️wgpu-tree-row-hit-test-2026-09-12.md` §5.3 could not: when a pointer
 * lands on `#semio-wgpu-canvas` and nothing dispatches, WHICH hop dropped it. Four witnesses, all on
 * the UI isolate and none of them inside the shell:
 *
 *   1. `Worker.prototype.postMessage` — every `{kind}` the transport actually transfers, counted.
 *   2. the Worker's `message` replies — every `{kind}` the frame worker answers with.
 *   3. `requestAnimationFrame` — whether the UI isolate's frame clock is running at all
 *      (`BrowserFrameTransport.requestFrame` schedules its one flush on it).
 *   4. the canvas's own listener count for each input type, and the DOM events delivered to it.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-input/wire-1 bun 🐍️wgpu-wire-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&mode=generate";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
const afterSeconds = Number(process.env.SEMIO_PROBE_AFTER ?? 12);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-input/wire");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.addInitScript(() => {
  const wire = { sent: {}, received: {}, sentLog: [], receivedLog: [], rafScheduled: 0, rafFired: 0, workers: 0, listeners: {}, domEvents: {} };
  globalThis.__semioWire = wire;
  const bump = (bag, key) => void (bag[key] = (bag[key] ?? 0) + 1);
  const originalPost = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (message, transfer) {
    const kind = message && typeof message === "object" ? String(message.kind ?? "?") : typeof message;
    bump(wire.sent, kind);
    if (kind !== "batch" || message?.replaceable?.length || message?.lossless?.length) {
      wire.sentLog.push({ t: Math.round(performance.now()), kind, seq: message?.sequence, gen: message?.generation, replaceable: message?.replaceable?.map((e) => e.kind), lossless: message?.lossless?.map((e) => e.kind ?? e.event?.kind) });
      if (wire.sentLog.length > 120) wire.sentLog.shift();
    }
    return originalPost.call(this, message, transfer);
  };
  const OriginalWorker = Worker;
  globalThis.Worker = new Proxy(OriginalWorker, {
    construct(target, args) {
      const instance = Reflect.construct(target, args);
      wire.workers += 1;
      instance.addEventListener("message", (event) => {
        const kind = event.data && typeof event.data === "object" ? String(event.data.kind ?? "?") : typeof event.data;
        bump(wire.received, kind);
        wire.lastFrame = kind === "frame" ? { t: Math.round(performance.now()), seq: event.data?.sequence, gen: event.data?.generation, requestFrame: event.data?.requestFrame, quarantined: event.data?.quarantined, faultCode: event.data?.faultCode, faultDetail: event.data?.faultDetail, verdict: event.data?.workerStepVerdict, ms: event.data?.workerExecutingMs } : wire.lastFrame;
        if (kind !== "frame" && kind !== "heartbeat") {
          wire.receivedLog.push({ t: Math.round(performance.now()), kind, seq: event.data?.sequence, gen: event.data?.generation, code: event.data?.code, detail: String(event.data?.detail ?? "").slice(0, 200) });
          if (wire.receivedLog.length > 120) wire.receivedLog.shift();
        }
      });
      return instance;
    },
  });
  const originalRaf = globalThis.requestAnimationFrame.bind(globalThis);
  globalThis.requestAnimationFrame = (callback) => {
    wire.rafScheduled += 1;
    return originalRaf((timestamp) => {
      wire.rafFired += 1;
      return callback(timestamp);
    });
  };
  const originalAdd = EventTarget.prototype.addEventListener;
  EventTarget.prototype.addEventListener = function (type, listener, options) {
    if (this instanceof HTMLCanvasElement) bump(wire.listeners, type);
    return originalAdd.call(this, type, listener, options);
  };
  addEventListener("pointerdown", (event) => { bump(wire.domEvents, "window:pointerdown"); wire.lastPoint = { x: event.clientX, y: event.clientY, target: event.target?.tagName ?? "?" }; }, true);
  addEventListener("wheel", () => bump(wire.domEvents, "window:wheel"), true);
  addEventListener("keydown", (event) => { bump(wire.domEvents, "window:keydown"); wire.lastKey = { key: event.key, target: event.target?.tagName ?? "?", active: document.activeElement?.tagName ?? "?" }; }, true);
});

const wire = () => page.evaluate(() => JSON.parse(JSON.stringify(globalThis.__semioWire ?? {}))).catch((error) => ({ error: String(error) }));
const dump = (windowId) =>
  page
    .evaluate(async (id) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const parse = async (call) => { try { const raw = await call(); return raw ? JSON.parse(raw) : null; } catch (error) { return { error: String(error) }; } };
      return { structure: await parse(() => beacon.dumpStructure(id)), stats: await parse(() => beacon.dumpFrameStats(id)) };
    }, windowId)
    .catch((error) => ({ error: String(error).slice(0, 300) }));

const has = (needle) => lines.filter((line) => line.includes(needle));
const dockPlan = () => {
  const line = has("wgpu-shell dock plan").at(-1);
  const plan = {};
  if (!line) return plan;
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
for (let second = 0; second < settleSeconds; second += 1) await page.waitForTimeout(1000);

const settled = await wire();
const structure = await dump(undefined);
const plan = dockPlan();
const windowIds = structure?.structure?.windowIds ?? [];
const rows = [];
for (const id of windowIds) {
  const sample = await dump(id);
  const body = plan[id];
  for (const node of sample?.structure?.nodes ?? []) if (body && String(node.path ?? "").includes(process.env.SEMIO_PROBE_ROW ?? "add-generation")) rows.push({ id, path: node.path, page: [body.x + node.rect[0] + node.rect[2] / 2, body.y + node.rect[1] + node.rect[3] / 2] });
}

const afterSettleWire = await wire();
const target = rows.at(-1)?.page ?? [160, 138];
await page.mouse.move(target[0], target[1]);
await page.waitForTimeout(500);
const afterHover = await wire();
await page.mouse.click(target[0], target[1]);
await page.waitForTimeout(500);
const afterClick = await wire();
await page.mouse.wheel(0, 120);
await page.waitForTimeout(500);
await page.keyboard.press("f");
await page.waitForTimeout(500);
const afterAll = await wire();
for (let second = 0; second < afterSeconds; second += 1) await page.waitForTimeout(1000);
const final = await wire();
await page.screenshot({ path: join(outDir, "shot.png"), type: "png" }).catch(() => {});

const verdict = { url, windowIds, plan, rows, target, settled, afterSettleWire, afterHover, afterClick, afterAll, final, renderBegin: has("render begin").length, consoleTail: lines.slice(-25) };
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "wire.json"), JSON.stringify(verdict, null, 2));
console.log("DONE", JSON.stringify({ windowIds, target, rows, stages: [["settled", settled], ["afterHover", afterHover], ["afterClick", afterClick], ["afterAll", afterAll], ["final", final]].map(([name, w]) => ({ name, batches: w.sent?.batch, frames: w.received?.frame, lastFrame: w.lastFrame })), sentLog: final.sentLog, receivedLog: final.receivedLog, domEvents: final.domEvents, lastPoint: final.lastPoint, lastKey: final.lastKey, renderBegin: verdict.renderBegin }, null, 2));
await browser.close();
