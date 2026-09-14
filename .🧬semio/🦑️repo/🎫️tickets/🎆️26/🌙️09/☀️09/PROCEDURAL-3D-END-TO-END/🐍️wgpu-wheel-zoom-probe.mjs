/** 🖱️ wgpu WHEEL ZOOM — the residual of `world3d-editor / h7_wheel_zoom`, lane `wgpu-wheel-zoom-a11y-live`.
 *
 * The battery's own wheel hop scrolls over the preview centre and nudges the pointer 1 px into the
 * top-left corner between notches (the wgpu tick is input-driven, so the nudge is what makes the
 * frame run). Playwright fires `mouse.wheel` at the CURRENT pointer, so that gesture is a wheel
 * stream that genuinely travels: `Scroll {x:1208,y:461}`, `{x:3,y:3}`, `{x:4,y:4}`, …
 *
 * This probe measures three wheel shapes against the same preview, in EDIT mode, and reads the
 * renderer's own `[DEBUG] wheel apply …` line (the point and delta each frame applies, the hit the
 * scene-surface gate resolves there, and whether another application is still owed) beside the
 * world authority's `wheel=` and the published `setCamera`:
 *
 *   w1 — the battery shape: five notches at the centre, one corner nudge between each.
 *   w2 — a stationary burst: five notches at the centre, nothing else.
 *   w3 — a single notch at the centre.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-wheel-a11y/wheel-1 bun 🐍️wgpu-wheel-zoom-probe.mjs
 * @see 🐍️wgpu-world3d-interaction-probe.mjs (h7_wheel_zoom), 🧫️fixtures/🖱️wheel-application-point/🔣️.json
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column";
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 150);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-wheel-a11y/wheel");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const nudge = (n) => page.mouse.move(3 + (n % 5), 3 + (n % 5)).catch(() => {});
const pause = (ms) => page.waitForTimeout(ms);

const cameraTrace = () => /camera="(\{.*?\})"/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? null;
const authorityCamera = () => /camera=(\S+)/.exec(has("world3d interaction surface=").at(-1) ?? "")?.[1] ?? null;
const publishedActions = () => has("frame input action").map((line) => /action=(\S+)/.exec(line)?.[1] ?? "?");
/** 🖱️ Every wheel the FRAME applied, with the point it applied it at. */
const wheelApplies = () => has("[DEBUG] wheel apply").map((line) => line.split("[DEBUG] ").at(-1));
/** 🖱️ Every wheel the BROWSER delivered, with the point the DOM carried. */
const scrollEvents = () => has("dispatch_normalized_event Scroll").map((line) => /Scroll \{ x: ([-\d.]+), y: ([-\d.]+), delta_x: [-\d.]+, delta_y: ([-\d.]+) \}/.exec(line)).filter(Boolean).map((m) => ({ x: Number(m[1]), y: Number(m[2]), delta: Number(m[3]) }));
/** 🕹️ The wheel the world authority saw on its own intents. */
const intentWheels = () => has("world3d interaction surface=").map((line) => Number(/wheel=([-\d.]+)/.exec(line)?.[1] ?? 0));

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

const results = { url, steps: {} };
const record = (name, value) => {
  results.steps[name] = value;
  lines.push(`${at()} PROBE ${name} ${JSON.stringify(value).slice(0, 3000)}`);
  console.log(`[DEBUG] ${name} ${JSON.stringify(value).slice(0, 700)}`);
};

const snapshot = () => ({ camera: cameraTrace(), authorityCamera: authorityCamera(), actions: publishedActions().length, applies: wheelApplies().length, scrolls: scrollEvents().length, wheels: intentWheels().length });
const delta = (before) => ({
  cameraBefore: before.camera,
  cameraAfter: cameraTrace(),
  guestCameraChanged: before.camera !== null && cameraTrace() !== null && before.camera !== cameraTrace(),
  authorityCameraBefore: before.authorityCamera,
  authorityCameraAfter: authorityCamera(),
  newActions: [...new Set(publishedActions().slice(before.actions))],
  setCameraCount: publishedActions().slice(before.actions).filter((action) => action === "setCamera").length,
  newScrolls: scrollEvents().slice(before.scrolls),
  newApplies: wheelApplies().slice(before.applies),
  nonZeroIntentWheels: intentWheels().slice(before.wheels).filter((wheel) => wheel !== 0),
});

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

let booted = null;
for (let tick = 0; tick < bootBudget * 2; tick += 1) {
  await nudge(tick);
  await pause(500);
  if (has("boot_shell leave").length) {
    booted = at();
    break;
  }
}
record("boot", { bootShellLeaveMs: booted });
for (let tick = 0; tick < settle; tick += 1) {
  await nudge(tick);
  await pause(1000);
}

const plan = dockPlan();
const previewId = Object.keys(plan).find((id) => id.includes("preview")) ?? null;
const body = previewId ? plan[previewId] : null;
if (!body) {
  record("fatal", { reason: "no preview window in the dock plan", plan });
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
  await browser.close();
  process.exit(1);
}
const centre = [body.x + body.w / 2, body.y + body.h / 2];
record("preview", { previewId, body, centre, camera: cameraTrace() });

const settleTicks = async (count) => {
  for (let tick = 0; tick < count; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
};

// ── w1 — the battery's own shape: scroll at the centre, nudge to the corner between notches ────
{
  const before = snapshot();
  await page.mouse.move(centre[0], centre[1]);
  await pause(500);
  for (let tick = 0; tick < 5; tick += 1) {
    await page.mouse.wheel(0, 200);
    await pause(700);
    await nudge(tick);
  }
  await settleTicks(4);
  record("w1_battery_shape", delta(before));
}

// ── w2 — a stationary burst: five notches, pointer never leaves the centre ─────────────────────
{
  const before = snapshot();
  await page.mouse.move(centre[0], centre[1]);
  await pause(500);
  for (let tick = 0; tick < 5; tick += 1) {
    await page.mouse.wheel(0, -200);
    await pause(200);
  }
  await settleTicks(5);
  record("w2_stationary_burst", delta(before));
}

// ── w3 — a single notch at the centre ─────────────────────────────────────────────────────────
{
  const before = snapshot();
  await page.mouse.move(centre[0], centre[1]);
  await pause(500);
  await page.mouse.wheel(0, 200);
  await settleTicks(5);
  record("w3_single_notch", delta(before));
}

record("counts", {
  scrolls: scrollEvents().length,
  applies: wheelApplies().length,
  setCamera: publishedActions().filter((action) => action === "setCamera").length,
  nonZeroIntentWheels: intentWheels().filter((wheel) => wheel !== 0).length,
  frameBuilds: has("frame build admitted").length,
  lastGate: (has("os_host frame gate").at(-1) ?? "").slice(0, 200),
  faults: has("frame fault recorded").slice(0, 5),
  pageErrors: has(" pageerror ").length,
});

writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
await browser.close();
process.exit(0);
