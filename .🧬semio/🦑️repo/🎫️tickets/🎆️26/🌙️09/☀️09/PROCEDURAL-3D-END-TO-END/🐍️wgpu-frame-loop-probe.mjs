/** 🔁 wgpu FRAME LOOP AFTER SELECTION — does the UI↔Worker frame wire keep turning once selections apply?
 *
 * `📓️wgpu-spawn-job-effect-2026-09-13.md` §7.1: a few APPLIED selections in, the wgpu shell's frame
 * loop stops. The page's main thread stays alive (rAF ticks), the worker logs no fault, and the last
 * line is always `wgpu-shell render leave surface=framework.panel.history`.
 *
 * This probe reads the ONE hop those two facts straddle — the `BrowserFrameTransport` ⇄ frame Worker
 * message wire — by wrapping `Worker` before the boot module loads. Every `{kind:"batch"}` posted up
 * and every `{kind:"frame"|"wake"|"fault"}` posted down is recorded with its `sequence`/`generation`,
 * so a stop is attributable:
 *
 *   • last record is an UP batch with no matching DOWN frame → the WORKER stopped answering;
 *   • last record is a DOWN frame and no UP batch follows a pointer move → the UI side parked
 *     (`flush()` refused: `inFlight` still set, or `frameRequested` never re-armed).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-frame-loop/run-1 bun 🐍️wgpu-frame-loop-probe.mjs
 *   SEMIO_PROBE_GESTURES=20  how many selection gestures to run after boot
 *   SEMIO_PROBE_ROLE=viewer  boot axis
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const mode = process.env.SEMIO_PROBE_MODE ?? "";
const role = process.env.SEMIO_PROBE_ROLE ?? "";
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "";
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${mode ? `&mode=${mode}` : ""}${role ? `&role=${role}` : ""}${example ? `&example=${example}` : ""}`;
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 120);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
const gestures = Number(process.env.SEMIO_PROBE_GESTURES ?? 6);
const perGestureSettle = Number(process.env.SEMIO_PROBE_GESTURE_SETTLE ?? 6);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-frame-loop/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });

/** 🪝️ Wraps `Worker` so every frame-wire message is timestamped on the page before boot runs. */
await page.addInitScript(() => {
  const log = [];
  globalThis.__semioWire = log;
  const Native = globalThis.Worker;
  /** 🎯️ Only the FRAME wire — the shard Workers' `turn`/`heartbeat`/`result` traffic is tens of
   * thousands of messages a minute and would evict the very tail this probe exists to read. */
  const WIRE = new Set(["boot", "batch", "frame", "wake", "fault", "booted", "boot-phase", "boot-progress", "boot-liveness", "close", "closed", "introspect", "introspection", "worker-error"]);
  const shape = (data, dir) => {
    if (!data || typeof data !== "object") return null;
    const { kind, sequence, generation, requestFrame, quarantined, faultCode, faultDetail, code, detail, stage, phase, state } = data;
    if (!WIRE.has(kind)) return null;
    return { t: Math.round(performance.now()), dir, kind, sequence, generation, requestFrame, quarantined, faultCode, faultDetail: faultDetail === undefined ? undefined : String(faultDetail).slice(0, 400), code, detail: detail === undefined ? undefined : String(detail).slice(0, 400), stage, phase, state };
  };
  const push = (entry) => { if (entry && log.length < 200000) log.push(entry); };
  class WrappedWorker extends Native {
    constructor(...args) {
      super(...args);
      this.addEventListener("message", (event) => push(shape(event.data, "down")));
      this.addEventListener("error", (event) => push({ t: Math.round(performance.now()), dir: "down", kind: "worker-error", detail: String(event?.message ?? "") }));
    }
    postMessage(data, transfer) {
      push(shape(data, "up"));
      return super.postMessage(data, transfer);
    }
  }
  globalThis.Worker = WrappedWorker;
});

page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const pause = (ms) => page.waitForTimeout(ms);
const nudge = (n) => page.mouse.move(3 + (n % 5), 3 + (n % 5)).catch(() => {});

const wire = () => page.evaluate(() => globalThis.__semioWire ?? []).catch(() => []);
/** 📊️ The wire's verdict: the last batch posted up, and whether the worker answered it. */
const wireVerdict = async () => {
  const log = await wire();
  const batches = log.filter((entry) => entry.kind === "batch");
  const frames = log.filter((entry) => entry.kind === "frame");
  const lastBatch = batches.at(-1) ?? null;
  const lastFrame = frames.at(-1) ?? null;
  return {
    batches: batches.length,
    frames: frames.length,
    lastBatch,
    lastFrame,
    lastBatchAnswered: lastBatch ? frames.some((frame) => Number(frame.sequence) === Number(lastBatch.sequence)) : null,
    wakes: log.filter((entry) => entry.kind === "wake").length,
    faults: log.filter((entry) => entry.kind === "fault" || entry.kind === "worker-error").map((entry) => `${entry.kind}:${entry.code ?? ""}:${String(entry.detail ?? "").slice(0, 200)}`),
    requestFrameFalse: frames.filter((frame) => frame.requestFrame === false).length,
    quarantines: frames.filter((frame) => frame.quarantined).map((frame) => `seq=${frame.sequence} ${frame.faultCode}: ${frame.faultDetail ?? ""}`),
    tail: log.slice(-14),
  };
};

const liveness = () => page.evaluate(() => new Promise((resolve) => {
  const started = performance.now();
  let ticks = 0;
  const tick = () => {
    ticks += 1;
    if (ticks >= 3 || performance.now() - started > 2000) resolve({ ticks, ms: Math.round(performance.now() - started) });
    else requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
  setTimeout(() => resolve({ ticks, ms: Math.round(performance.now() - started), timedOut: true }), 2500);
})).catch((error) => ({ error: String(error).slice(0, 200) }));

const dockPlan = () => {
  const line = has("wgpu-shell dock plan").at(-1);
  if (!line) return {};
  const plan = {};
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

const publishedActions = () => has("frame input action").length;
const renderSweeps = () => has("render leave surface=framework.panel.history").length;
const frameAdmits = () => has("frame build admitted").length;
const guestSelectionLane = () => {
  const raw = /lanes=\[(.*?)\]/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? "";
  return Object.fromEntries(raw.split(",").filter(Boolean).map((entry) => entry.split(":")).map(([k, v]) => [k, Number(v)]));
};
const appliedSelections = () => has('"selected":true').length;

const results = { url, gestures, steps: [] };
const record = async (name, extra = {}) => {
  const entry = { name, at: at(), actions: publishedActions(), sweeps: renderSweeps(), admits: frameAdmits(), applied: appliedSelections(), lanes: guestSelectionLane(), wire: await wireVerdict(), alive: await liveness(), ...extra };
  results.steps.push(entry);
  lines.push(`${at()} PROBE ${name} ${JSON.stringify({ ...entry, wire: { ...entry.wire, tail: undefined } })}`);
  return entry;
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

let booted = null;
for (let tick = 0; tick < bootBudget * 2; tick += 1) {
  await nudge(tick);
  await pause(500);
  if (has("boot_shell leave").length) { booted = at(); break; }
}
await record("boot", { bootShellLeaveMs: booted });
for (let tick = 0; tick < settle; tick += 1) { await nudge(tick); await pause(1000); }

const plan = dockPlan();
const previewId = Object.keys(plan).find((id) => id.includes("preview")) ?? null;
const body = previewId ? plan[previewId] : null;
if (!body) {
  await record("fatal", { reason: "no preview window in the dock plan", plan });
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
  writeFileSync(join(outDir, "wire.json"), JSON.stringify(await wire(), null, 1), "utf8");
  await browser.close();
  process.exit(1);
}
const centre = [body.x + body.w / 2, body.y + body.h / 2];
const corner = [body.x + 12, body.y + body.h - 12];
await record("preview", { previewId, body });

const settleTicks = async (count) => { for (let tick = 0; tick < count; tick += 1) { await pause(1000); await nudge(tick); } };

/** 🎬️ The gesture ladder, cycled: hover → click → shift-click → empty click → marquee. */
const gesture = async (index) => {
  const kind = ["hover", "click", "shift_click", "empty_click", "marquee"][index % 5];
  if (kind === "hover") {
    await page.mouse.move(centre[0] + (index % 7) - 3, centre[1] + (index % 5) - 2);
  } else if (kind === "click") {
    await page.mouse.move(centre[0], centre[1]);
    await pause(300);
    await page.mouse.down(); await pause(120); await page.mouse.up();
  } else if (kind === "shift_click") {
    await page.keyboard.down("Shift");
    await page.mouse.move(centre[0] + 18, centre[1] + 18);
    await pause(300);
    await page.mouse.down(); await pause(120); await page.mouse.up();
    await page.keyboard.up("Shift");
  } else if (kind === "empty_click") {
    await page.mouse.move(corner[0], corner[1]);
    await pause(300);
    await page.mouse.down(); await pause(120); await page.mouse.up();
  } else {
    const from = [body.x + body.w - 8, body.y + 8];
    const to = [body.x + 8, body.y + body.h - 8];
    await page.mouse.move(from[0], from[1]);
    await page.mouse.down();
    for (let step = 1; step <= 8; step += 1) {
      await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
      await pause(120);
    }
    await page.mouse.up();
  }
  await settleTicks(perGestureSettle);
  return kind;
};

for (let index = 0; index < gestures; index += 1) {
  const kind = await gesture(index);
  await record(`g${index + 1}_${kind}`);
}

await page.screenshot({ path: join(outDir, "shot-final.png"), type: "png" }).catch(() => {});
const final = await record("final");
writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
writeFileSync(join(outDir, "wire.json"), JSON.stringify(await wire(), null, 1), "utf8");
console.log(JSON.stringify(results.steps.map((step) => ({ name: step.name, actions: step.actions, sweeps: step.sweeps, admits: step.admits, batches: step.wire.batches, frames: step.wire.frames, answered: step.wire.lastBatchAnswered, faults: step.wire.faults.length })), null, 1));
console.log(JSON.stringify(final.wire.tail, null, 1));
await browser.close();
