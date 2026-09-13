/** 🕹️ wgpu WORLD3D INTERACTION — hover, selection and camera on the 3-D preview surface, hop by hop.
 *
 * `📓️wgpu-input-hit-runtime-2026-09-13.md` §11 proved the pointer reaches the World3d surface and the
 * intent is admitted, then stopped: `lanes.selection` and `camera` stayed byte-identical. This probe
 * reads the two traces that separate the remaining hops, both added by the `wgpu-world3d-interaction`
 * lane:
 *
 *   `[DEBUG] world3d interaction surface=… enter|leave g=N step=… <interaction_census()>`
 *       — one pair per INTENT: its phase/button/down/point, the active transition (`Pick[Hover hit=…]`,
 *         `Plan[0/1 Some(Camera)]`, `MarqueePick`, …), the gestures, the pick inputs (`draws`,
 *         `objects`, `bounds`, `pick`) and the surface's own `hover=`/`selected=`.
 *   `[DEBUG] frame input action controller=… action=… args=…`
 *       — every action the frame actually took OUT of the bounded input queue, i.e. what the world
 *         authority published (`setCamera`, `interactionHover`, `interactionSelect`).
 *   `[DEBUG] frame deferred action failed: …`
 *       — the guest's refusal, which used to be swallowed into `shell.error`.
 *
 * Every point is DERIVED from the shell's own `[DEBUG] wgpu-shell dock plan` body, never guessed.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-world3d/run-1 bun 🐍️wgpu-world3d-interaction-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const mode = process.env.SEMIO_PROBE_MODE ?? "";
const role = process.env.SEMIO_PROBE_ROLE ?? "";
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${mode ? `&mode=${mode}` : ""}${role ? `&role=${role}` : ""}`;
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 90);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 15);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-world3d/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)} :: ${String(e?.stack ?? "").slice(0, 3000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const nudge = (n) => page.mouse.move(3 + (n % 5), 3 + (n % 5)).catch(() => {});
const pause = (ms) => page.waitForTimeout(ms);

const cameraTrace = () => /camera="(\{.*?\})"/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? null;
const lanes = () => {
  const raw = /lanes=\[(.*?)\]/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? "";
  return Object.fromEntries(raw.split(",").filter(Boolean).map((entry) => entry.split(":")).map(([k, v]) => [k, Number(v)]));
};
const settledCommands = () => has("wgpu-shell command").map((line) => /wgpu-shell command (\S+) settled(.*)$/.exec(line)).filter(Boolean).map((m) => `${m[1]}${m[2]}`);
/** 🎬️ Every action the frame took out of the bounded input queue, newest last. */
const publishedActions = () => has("frame input action").map((line) => /controller=(\S+) action=(\S+) args=(\S+)/.exec(line)).filter(Boolean).map((m) => `${m[1]}/${m[2]}/args=${m[3]}`);
const dispatchFailures = () => has("frame deferred action failed").map((line) => line.split("frame deferred action failed: ").at(-1).slice(0, 240));
/** 🕹️ The world authority's own census lines, trimmed to what a reader needs. */
const interactionTrace = () => has("world3d interaction surface=").map((line) => line.split("[DEBUG] ").at(-1));
/** 👆️ The surface's own hover/selection, as the authority reports it. */
const authorityState = () => {
  const line = interactionTrace().at(-1) ?? "";
  return { hover: /hover=(Some\(".*?"\)|None)/.exec(line)?.[1] ?? null, selected: Number(/selected=(\d+)/.exec(line)?.[1] ?? -1), draws: Number(/draws=(\d+)/.exec(line)?.[1] ?? -1), objects: Number(/objects=(\d+)/.exec(line)?.[1] ?? -1), camera: /camera=(\S+)/.exec(line)?.[1] ?? null };
};

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
  lines.push(`${at()} PROBE ${name} ${JSON.stringify(value).slice(0, 4000)}`);
};

/** 📸️ One gesture's full before/after, so a hop that moves nothing is told apart from one that moves
 * the shell but not the guest. */
const snapshot = () => ({ camera: cameraTrace(), lanes: lanes(), authority: authorityState(), actions: publishedActions().length, commands: settledCommands().length, failures: dispatchFailures().length, intents: interactionTrace().length });
const delta = (before) => ({
  guestCameraChanged: before.camera !== null && cameraTrace() !== null && before.camera !== cameraTrace(),
  localCameraChanged: before.authority.camera !== null && authorityState().camera !== null && before.authority.camera !== authorityState().camera,
  guestCameraAfter: cameraTrace(),
  selectionLaneBefore: before.lanes.selection,
  selectionLaneAfter: lanes().selection,
  authorityBefore: before.authority,
  authorityAfter: authorityState(),
  newActions: publishedActions().slice(before.actions),
  newCommands: settledCommands().slice(before.commands),
  newFailures: dispatchFailures().slice(before.failures),
  newIntents: interactionTrace().slice(before.intents).slice(-24),
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
record("boot", { bootShellLeaveMs: booted, dockPlan: dockPlan() });
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
const corner = [body.x + 12, body.y + body.h - 12];
record("preview", { previewId, body, centre, corner, lanes: lanes(), camera: cameraTrace() });
await page.screenshot({ path: join(outDir, "shot-booted.png"), type: "png" }).catch(() => {});

const settleTicks = async (count) => {
  for (let tick = 0; tick < count; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
};

// ── 1 — hover the mesh ────────────────────────────────────────────────────────────────────────
{
  const before = snapshot();
  await page.mouse.move(centre[0], centre[1]);
  await settleTicks(5);
  record("h1_hover_centre", delta(before));
}

// ── 2 — hover off the mesh (the surface's own empty corner) ────────────────────────────────────
{
  const before = snapshot();
  await page.mouse.move(corner[0], corner[1]);
  await settleTicks(4);
  record("h2_hover_empty", delta(before));
}

// ── 3 — click the mesh ────────────────────────────────────────────────────────────────────────
{
  const before = snapshot();
  await page.mouse.move(centre[0], centre[1]);
  await pause(400);
  await page.mouse.down();
  await pause(120);
  await page.mouse.up();
  await settleTicks(6);
  record("h3_click_select", delta(before));
}

// ── 4 — shift-click a second point on the mesh ────────────────────────────────────────────────
{
  const before = snapshot();
  await page.keyboard.down("Shift");
  await page.mouse.move(centre[0] + 18, centre[1] + 18);
  await pause(300);
  await page.mouse.down();
  await pause(120);
  await page.mouse.up();
  await page.keyboard.up("Shift");
  await settleTicks(5);
  record("h4_shift_add", delta(before));
}

// ── 5 — empty click clears ────────────────────────────────────────────────────────────────────
{
  const before = snapshot();
  await page.mouse.move(corner[0], corner[1]);
  await pause(300);
  await page.mouse.down();
  await pause(120);
  await page.mouse.up();
  await settleTicks(5);
  record("h5_empty_clear", delta(before));
}

// ── 6 — marquee (left drag enclosing the mesh), BEFORE the camera gestures so the band is measured
//        against the boot camera the fixture's own path assumes ──────────────────────────────────
{
  const before = snapshot();
  // 🔲️ A WINDOW marquee (left → right) only takes what it fully encloses, so the band spans nearly
  // the whole preview body — the same shape as the fixture's `marquee-release-replaces` path over its
  // own 800x600 viewport.
  // 🔲️ RIGHT → LEFT is the CROSSING band (`WorldMarqueePickCursor::new`'s `crossing`), which takes
  // whatever it touches rather than only what it fully encloses — the wider of the two rules, so a
  // pick that reports nothing here is a real hole and not a strict-enclosure answer.
  const from = [body.x + body.w - 8, body.y + 8];
  const to = [body.x + 8, body.y + body.h - 8];
  await page.mouse.move(from[0], from[1]);
  await page.mouse.down();
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
    await pause(120);
  }
  await page.mouse.up();
  await settleTicks(6);
  record("h6_marquee", delta(before));
}

// ── 7 — wheel zoom ────────────────────────────────────────────────────────────────────────────
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
  record("h7_wheel_zoom", delta(before));
}

// ── 8 — right-drag with alt → orbit ───────────────────────────────────────────────────────────
{
  const before = snapshot();
  await page.keyboard.down("Alt");
  await page.mouse.move(centre[0] - 80, centre[1]);
  await page.mouse.down({ button: "right" });
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(centre[0] - 80 + step * 20, centre[1] + step * 6);
    await pause(140);
  }
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Alt");
  await settleTicks(5);
  record("h8_right_drag_orbit", delta(before));
}

// ── 9 — right-drag with shift → pan ───────────────────────────────────────────────────────────
{
  const before = snapshot();
  await page.keyboard.down("Shift");
  await page.mouse.move(centre[0], centre[1] - 60);
  await page.mouse.down({ button: "right" });
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(centre[0] + step * 14, centre[1] - 60 + step * 10);
    await pause(140);
  }
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Shift");
  await settleTicks(5);
  record("h9_right_drag_pan", delta(before));
}

await page.screenshot({ path: join(outDir, "shot-final.png"), type: "png" }).catch(() => {});
record("counts", Object.fromEntries(["os_host pointer hit", "world3d interaction surface=", "frame input action", "frame deferred action failed", "world3d surface=", "render begin", "pointer failed", "world3d retained interaction authority faulted", "panicked", "wgpu-worker panicked", "unmapped effect"].map((needle) => [needle, has(needle).length])));

writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
console.log(JSON.stringify(results.steps, null, 2).slice(0, 20000));
await browser.close();
