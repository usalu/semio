/** 🖱️ wgpu INPUT DELIVERABLES — the six runtime hops of the `wgpu-input-hit-runtime` brief, each
 * with its own witness, on one boot.
 *
 * Every click point is DERIVED, never guessed. Chrome control ids and their page rects come from the
 * shell's own `[DEBUG] os_host pointer hit x=… y=… targets=N hit=Some((Kind, Some("id")))` trace,
 * swept across the navbar; retained-body rects come from `dumpStructure(windowId)` (the published
 * `mounted_layout`, the same rectangles `events::hit_test` reads) offset by the window's own
 * `[DEBUG] wgpu-shell dock plan` body.
 *
 * Witnesses, all read off traces the renderer already emits:
 *   hover      → `dumpStructure`'s `state.hovered`, sampled idle → on → away → back
 *   selection  → the `selection:N` lane of `[DEBUG] world3d surface=…`, plus settled shell commands
 *   orbit      → the `camera="{…}"` field of that same trace, before vs after
 *   dispatch   → `[DEBUG] wgpu-shell command <id> settled` / `deferred action` after the press
 *   chords     → `[DEBUG] wgpu-shell session switch` / `boot mode` / mode-group pressed state
 *
 * Usage: cd <ticket> && SEMIO_PROBE_MODE= SEMIO_PROBE_OUT=wgpu-input/deliverables-edit bun 🐍️wgpu-input-deliverables-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const mode = process.env.SEMIO_PROBE_MODE ?? "";
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${mode ? `&mode=${mode}` : ""}`;
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 60);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-input/deliverables");
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

/** 🎥️ The world3d trace's own camera JSON — the field is emitted ESCAPED inside the line, so the
 * brace run is matched rather than the bare `{"fov"` a naive reader looks for. */
const cameraTrace = () => {
  const line = has("world3d surface=").at(-1) ?? "";
  return /camera="(\{.*?\})"/.exec(line)?.[1] ?? null;
};
/** 🎯️ The per-lane byte counts of the last world3d trace (`lanes=[meshes:2,instances:2,selection:172,…]`). */
const lanes = () => {
  const line = has("world3d surface=").at(-1) ?? "";
  const raw = /lanes=\[(.*?)\]/.exec(line)?.[1] ?? "";
  return Object.fromEntries(raw.split(",").filter(Boolean).map((entry) => entry.split(":")).map(([k, v]) => [k, Number(v)]));
};
const sceneInstances = () => {
  const line = has("world3d surface=").at(-1) ?? "";
  return Number(/ instances=(\d+)/.exec(line)?.[1] ?? /state-instances=(\d+)/.exec(line)?.[1] ?? -1);
};
const settledCommands = () => has("wgpu-shell command").map((line) => /wgpu-shell command (\S+) settled(.*)$/.exec(line)).filter(Boolean).map((m) => `${m[1]}${m[2]}`);

const dump = (windowId) =>
  page
    .evaluate(async (id) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      try {
        const raw = await beacon.dumpStructure(id);
        return raw ? JSON.parse(raw) : null;
      } catch (error) {
        return { error: String(error) };
      }
    }, windowId)
    .catch((error) => ({ error: String(error).slice(0, 300) }));

const hoveredIds = async (windowId) => ((await dump(windowId))?.nodes ?? []).filter((node) => node?.state?.hovered === true).map((node) => node.path);

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

/** 🎯️ The control the shell itself reports under a point, read back from its own hit trace. */
const hitAt = async (x, y) => {
  const before = has("os_host pointer hit").length;
  await page.mouse.move(x, y);
  for (let wait = 0; wait < 20; wait += 1) {
    await pause(150);
    const found = has("os_host pointer hit").slice(before);
    const line = found.at(-1);
    if (line) {
      const match = /hit=Some\(\((\w+), Some\("(.*?)"\)\)\)/.exec(line);
      return { x, y, kind: match?.[1] ?? null, controlId: match?.[2] ?? null, raw: line.slice(0, 240) };
    }
  }
  return { x, y, kind: null, controlId: null, raw: "no hit trace" };
};

const results = { url, steps: {} };
const record = (name, value) => {
  results.steps[name] = value;
  lines.push(`${at()} PROBE ${name} ${JSON.stringify(value).slice(0, 2000)}`);
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

// 🫀️ Boot. The host ticks on input, so the pointer is nudged while we wait.
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
await page.screenshot({ path: join(outDir, "shot-booted.png"), type: "png" }).catch(() => {});

/** ⌨️ One chord, held explicitly. The shell decides a chord on the KeyDown that carries both
 * modifiers, and gates the two reserved axes on `idle` — no focused control, no overlay, no drag —
 * so the same chords are measured BEFORE any click and again after, and the two are reported apart. */
const pressChord = async (chord) => {
  const before = { commands: settledCommands().length, switches: has("shell session switch").length, renders: has("render begin").length, keys: has("dispatch_normalized_event KeyDown").length };
  await page.mouse.move(700, 400);
  await page.keyboard.down("Meta");
  await page.keyboard.down("Alt");
  await page.keyboard.press(chord.split("+").at(-1));
  await page.keyboard.up("Alt");
  await page.keyboard.up("Meta");
  for (let tick = 0; tick < 4; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
  return {
    chord,
    keyDownDelta: has("dispatch_normalized_event KeyDown").length - before.keys,
    newCommands: settledCommands().slice(before.commands),
    sessionSwitchDelta: has("shell session switch").length - before.switches,
    renderDelta: has("render begin").length - before.renders,
  };
};

// ── Deliverable 6a — the chords on a freshly booted shell, before any click can take focus ─────
const chordsIdle = [];
for (const chord of ["Meta+Alt+ArrowRight", "Meta+Alt+ArrowLeft"]) chordsIdle.push(await pressChord(chord));
record("d6a_chords_idle", { chordWitness: chordsIdle, dockPlanAfter: dockPlan() });

const plan = dockPlan();
const previewId = Object.keys(plan).find((id) => id.includes("preview")) ?? "generation3d-generate-preview";
const previewBody = plan[previewId];
const previewPoint = previewBody ? [previewBody.x + previewBody.w / 2, previewBody.y + previewBody.h / 2] : null;
record("preview", { previewId, previewBody, previewPoint, sceneInstances: sceneInstances(), lanes: lanes() });

// ── Deliverable 1 — hover over the World3d preview ────────────────────────────────────────────
if (previewPoint) {
  const before = { camera: cameraTrace(), lanes: lanes(), commands: settledCommands().length };
  const hit = await hitAt(previewPoint[0], previewPoint[1]);
  await pause(2500);
  const onSurface = { hit, lanes: lanes(), commands: settledCommands().slice(before.commands) };
  await page.mouse.move(previewPoint[0], previewPoint[1] + 120);
  await pause(2000);
  await page.mouse.move(previewPoint[0] + 60, previewPoint[1] - 60);
  await pause(2500);
  record("d1_hover_world3d", { before, onSurface, afterMoving: { lanes: lanes(), commands: settledCommands().slice(before.commands) } });
}

// ── Deliverable 2 — click the preview → selection ─────────────────────────────────────────────
if (previewPoint) {
  const before = { lanes: lanes(), commands: settledCommands().length, renders: has("render begin").length };
  await page.mouse.click(previewPoint[0], previewPoint[1]);
  for (let tick = 0; tick < 6; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
  record("d2_click_selection", {
    lanesBefore: before.lanes,
    lanesAfter: lanes(),
    selectionChanged: JSON.stringify(before.lanes.selection) !== JSON.stringify(lanes().selection),
    newCommands: settledCommands().slice(before.commands),
    renderDelta: has("render begin").length - before.renders,
  });
}

// ── Deliverable 3 — wheel over the preview → orbit (setCamera) ────────────────────────────────
if (previewPoint) {
  const cameraBefore = cameraTrace();
  const commandsBefore = settledCommands().length;
  await page.mouse.move(previewPoint[0], previewPoint[1]);
  await pause(600);
  for (let tick = 0; tick < 5; tick += 1) {
    await page.mouse.wheel(0, 200);
    await pause(700);
    await nudge(tick);
  }
  await pause(2500);
  const cameraAfterWheel = cameraTrace();
  // 🖱️ React orbits on a RIGHT-button drag (`WorldOrbitGated`), so the wheel is not the only gesture
  // the deliverable can be satisfied by — both are measured, and reported separately.
  await page.mouse.move(previewPoint[0] - 80, previewPoint[1]);
  await page.mouse.down({ button: "right" });
  for (let step = 1; step <= 8; step += 1) {
    await page.mouse.move(previewPoint[0] - 80 + step * 20, previewPoint[1] + step * 6);
    await pause(120);
  }
  await page.mouse.up({ button: "right" });
  for (let tick = 0; tick < 5; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
  record("d3_wheel_orbit", {
    cameraBefore,
    cameraAfterWheel,
    wheelChanged: cameraBefore !== null && cameraAfterWheel !== null && cameraBefore !== cameraAfterWheel,
    cameraAfterRightDrag: cameraTrace(),
    rightDragChanged: cameraAfterWheel !== null && cameraTrace() !== null && cameraAfterWheel !== cameraTrace(),
    newCommands: settledCommands().slice(commandsBefore),
  });
}
await page.screenshot({ path: join(outDir, "shot-after-orbit.png"), type: "png" }).catch(() => {});

// ── Deliverable 4 — a chrome control dispatches its action ────────────────────────────────────
// 🧭️ The chrome band is DISCOVERED, not assumed: a sweep across the rows above and below the dock
// body, reporting every control the shell's own hit trace names.
const navbarSweep = [];
const bandRows = (process.env.SEMIO_PROBE_CHROME_ROWS ?? "10,20,28,36,44,52,62,880,894").split(",").map(Number);
for (const y of bandRows) {
  for (let x = 24; x <= 1416; x += 56) {
    const hit = await hitAt(x, y);
    if (hit.controlId) navbarSweep.push({ x, y, kind: hit.kind, controlId: hit.controlId });
  }
}
const chromeTarget = navbarSweep.find((entry) => entry.controlId?.startsWith("playground.navbar.modes.")) ?? navbarSweep.find((entry) => entry.controlId?.startsWith("playground.navbar.")) ?? navbarSweep[0] ?? null;
if (chromeTarget) {
  const before = { commands: settledCommands().length, renders: has("render begin").length, deferred: has("deferred action").length };
  await page.mouse.click(chromeTarget.x, chromeTarget.y);
  for (let tick = 0; tick < 6; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
  record("d4_chrome_control", {
    navbarSweep,
    clicked: chromeTarget,
    newCommands: settledCommands().slice(before.commands),
    renderDelta: has("render begin").length - before.renders,
    newDeferred: has("deferred action").slice(before.deferred).map((line) => line.slice(0, 200)),
    sessionSwitch: has("shell session switch").slice(-3).map((line) => line.slice(0, 200)),
  });
} else {
  record("d4_chrome_control", { navbarSweep, clicked: null, why: "no chrome control resolved on the navbar row" });
}
await page.screenshot({ path: join(outDir, "shot-after-chrome.png"), type: "png" }).catch(() => {});

// ── Deliverable 5 — a retained tree row dispatches its action ─────────────────────────────────
const rowNeedle = process.env.SEMIO_PROBE_ROW ?? "add-generation";
const rowTargets = [];
for (const [id, body] of Object.entries(plan)) {
  const structure = await dump(id);
  for (const node of structure?.nodes ?? []) {
    if (!String(node.path ?? "").includes(rowNeedle)) continue;
    const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
    rowTargets.push({ windowId: id, path: node.path, rect: node.rect, page: [body.x + rx + rw / 2, body.y + ry + rh / 2] });
  }
}
const row = rowTargets.at(-1);
if (row) {
  const hoverWitness = { row: row.path, idle: await hoveredIds(row.windowId) };
  await page.mouse.move(row.page[0], row.page[1]);
  await pause(2500);
  hoverWitness.onRow = await hoveredIds(row.windowId);
  await page.mouse.move(row.page[0], row.page[1] + 300);
  await pause(2500);
  hoverWitness.away = await hoveredIds(row.windowId);
  await page.mouse.move(row.page[0], row.page[1]);
  await pause(2500);
  hoverWitness.back = await hoveredIds(row.windowId);

  const before = { commands: settledCommands().length, renders: has("render begin").length, failures: has("pointer failed").length };
  const hit = await hitAt(row.page[0], row.page[1]);
  await page.mouse.click(row.page[0], row.page[1]);
  for (let tick = 0; tick < 8; tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
  record("d5_retained_row", {
    row,
    hit,
    hoverWitness,
    hoverClearsOnLeave: hoverWitness.away.length < hoverWitness.onRow.length,
    newCommands: settledCommands().slice(before.commands),
    renderDelta: has("render begin").length - before.renders,
    newPointerFailures: has("pointer failed").slice(before.failures).map((line) => line.slice(0, 300)),
    rowMentions: has(rowNeedle).length,
  });
} else {
  record("d5_retained_row", { row: null, why: `no retained row matching ${rowNeedle}` });
}

// ── Deliverable 6 — keyboard chords ───────────────────────────────────────────────────────────
const chordWitness = [];
for (const chord of ["Meta+Alt+ArrowRight", "Meta+Alt+ArrowLeft", "Meta+Alt+e", "Meta+Alt+v"]) {
  chordWitness.push(await pressChord(chord));
}
record("d6_chords", { chordWitness, keyDispatches: has("dispatch_normalized_event Key").length });
await page.screenshot({ path: join(outDir, "shot-final.png"), type: "png" }).catch(() => {});

results.counts = Object.fromEntries(
  ["os_host handle_event", "os_host pointer hit", "dispatch_normalized_event", "dock plan", "world3d surface", "render begin", "pointer failed", "surface fault", "panicked"].map((needle) => [needle, has(needle).length]),
);
results.pointerFailures = has("pointer failed").slice(-10).map((line) => line.slice(0, 400));
results.allSettledCommands = settledCommands().slice(-40);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
console.log(JSON.stringify(results, null, 2).slice(0, 14000));
await browser.close();
