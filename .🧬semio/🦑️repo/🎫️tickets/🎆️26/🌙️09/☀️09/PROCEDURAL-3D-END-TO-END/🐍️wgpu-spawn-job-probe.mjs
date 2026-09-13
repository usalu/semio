/** 🧵 wgpu SPAWN-JOB — does the wgpu host RUN the framework reserved tool job an interaction admits?
 *
 * `📓️wgpu-world3d-interaction-2026-09-13.md` §7 proved every gesture publishes the right wire
 * message and then stops: `interactionSelect`/`interactionHover` are reserved TOOL JOBS delivered as
 * an `Effect::SpawnJob{kind:"framework.reserved.tool"}`, and the wgpu bridge dropped it. This probe
 * reads the traces that separate "admitted" from "applied":
 *
 *   `[DEBUG] wire spawn-job …`            — the raw wire shape (temporary, measurement only)
 *   `wireEffectToFriendly: unmapped …`    — the drop this lane closes
 *   `[DEBUG] wgpu-bridge spawn-job …`     — the pump: start → steps → terminal → job-completed turn
 *   `[DEBUG] world3d interaction surface=…`— the authority's own `hover=`/`selected=`
 *   `[DEBUG] frame input action …`        — what the frame published (`interactionSelect`, …)
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-spawn-job/run-1 bun 🐍️wgpu-spawn-job-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const mode = process.env.SEMIO_PROBE_MODE ?? "";
const role = process.env.SEMIO_PROBE_ROLE ?? "";
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${mode ? `&mode=${mode}` : ""}${role ? `&role=${role}` : ""}`;
const bootBudget = Number(process.env.SEMIO_PROBE_BOOT ?? 90);
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 15);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-spawn-job/run");
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

const publishedActions = () => has("frame input action").map((line) => /controller=(\S+) action=(\S+) args=(\S+)/.exec(line)).filter(Boolean).map((m) => `${m[2]}/args=${m[3]}`);
const interactionTrace = () => has("world3d interaction surface=").map((line) => line.split("[DEBUG] ").at(-1));
const authorityState = () => {
  const line = interactionTrace().at(-1) ?? "";
  return { hover: /hover=(Some\(".*?"\)|None)/.exec(line)?.[1] ?? null, selected: Number(/selected=(\d+)/.exec(line)?.[1] ?? -1) };
};
/** 🎯️ The guest's OWN view, straight off the world3d surface trace the shell prints each frame. */
const guestLanes = () => {
  const raw = /lanes=\[(.*?)\]/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? "";
  return Object.fromEntries(raw.split(",").filter(Boolean).map((entry) => entry.split(":")).map(([k, v]) => [k, Number(v)]));
};
const guestHoverTarget = () => /hoverTarget=(\S+)/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? null;
const guestSelected = () => /selectedIds=(\[[^\]]*\])/.exec(has("world3d surface=").at(-1) ?? "")?.[1] ?? null;

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
const skip = (name) => only !== "" && name !== only && name.startsWith("s");
const record = async (name, value) => {
  const alive = await liveness();
  const entry = { ...value, alive };
  results.steps[name] = entry;
  lines.push(`${at()} PROBE ${name} ${JSON.stringify(entry).slice(0, 6000)}`);
};

const snapshot = () => ({
  wire: has("[DEBUG] wire spawn-job").length,
  drops: has('unmapped effect "spawn-job"').length,
  pumps: has("[DEBUG] wgpu-bridge spawn-job").length,
  actions: publishedActions().length,
  intents: interactionTrace().length,
});
/** 💓️ Is the PAGE still alive when the log goes quiet? A rAF that never fires is a wedged main
 * thread; one that fires while nothing logs is a wedged frame/actor chain instead. */
const liveness = async () => {
  try {
    return await page.evaluate(() => new Promise((resolve) => {
      const started = performance.now();
      let ticks = 0;
      const tick = () => {
        ticks += 1;
        if (ticks >= 3 || performance.now() - started > 2000) resolve({ ticks, ms: Math.round(performance.now() - started), canvas: !!document.querySelector("canvas") });
        else requestAnimationFrame(tick);
      };
      requestAnimationFrame(tick);
      setTimeout(() => resolve({ ticks, ms: Math.round(performance.now() - started), timedOut: true, canvas: !!document.querySelector("canvas") }), 2500);
    }));
  } catch (error) {
    return { error: String(error).slice(0, 200) };
  }
};

const delta = (before) => ({
  newWire: has("[DEBUG] wire spawn-job").slice(before.wire).map((l) => l.split("[DEBUG] ").at(-1)),
  newDrops: has('unmapped effect "spawn-job"').length - before.drops,
  newPumps: has("[DEBUG] wgpu-bridge spawn-job").slice(before.pumps).map((l) => l.split("[DEBUG] ").at(-1)),
  newActions: publishedActions().slice(before.actions),
  authority: authorityState(),
  guestLanes: guestLanes(),
  guestHoverTarget: guestHoverTarget(),
  guestSelected: guestSelected(),
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
await record("boot", { bootShellLeaveMs: booted });
for (let tick = 0; tick < settle; tick += 1) {
  await nudge(tick);
  await pause(1000);
}

const plan = dockPlan();
const previewId = Object.keys(plan).find((id) => id.includes("preview")) ?? null;
const body = previewId ? plan[previewId] : null;
if (!body) {
  await record("fatal", { reason: "no preview window in the dock plan", plan });
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
  await browser.close();
  process.exit(1);
}
const centre = [body.x + body.w / 2, body.y + body.h / 2];
const corner = [body.x + 12, body.y + body.h - 12];
await record("preview", { previewId, body, centre, corner, guestLanes: guestLanes(), guestSelected: guestSelected() });
await page.screenshot({ path: join(outDir, "shot-booted.png"), type: "png" }).catch(() => {});

/** ⏳️ A gesture that APPLIES costs the shell a full re-render of every panel (2+ s each after a
 * selection), so a settle budget sized for the old drop-the-job behaviour reads as a wedge. */
const settleScale = Number(process.env.SEMIO_PROBE_SETTLE_SCALE ?? 1);
/** 🎯️ Run ONE gesture instead of the whole ladder. The shell wedges a few applied selections in
 * (see the lane report §6), so a gesture late in the ladder needs its own fresh boot to be measured
 * at all — `SEMIO_PROBE_ONLY=s5_marquee`. */
const only = process.env.SEMIO_PROBE_ONLY ?? "";
const settleTicks = async (count) => {
  for (let tick = 0; tick < Math.round(count * settleScale); tick += 1) {
    await pause(1000);
    await nudge(tick);
  }
};

// ── 1 — hover the mesh → interactionHover ─────────────────────────────────────────────────────
{
  if (skip("s1_hover")) {
    lines.push(`${at()} PROBE s1_hover skipped`);
  } else {
    const before = snapshot();
    await page.mouse.move(centre[0], centre[1]);
    await settleTicks(6);
    await record("s1_hover", delta(before));
  }
}

// ── 2 — click the mesh → interactionSelect replace ────────────────────────────────────────────
{
  if (skip("s2_click")) {
    lines.push(`${at()} PROBE s2_click skipped`);
  } else {
    const before = snapshot();
    await page.mouse.move(centre[0], centre[1]);
    await pause(400);
    await page.mouse.down();
    await pause(120);
    await page.mouse.up();
    await settleTicks(7);
    await record("s2_click", delta(before));
  }
}
await page.screenshot({ path: join(outDir, "shot-selected.png"), type: "png" }).catch(() => {});

// ── 3 — shift-click → additive ────────────────────────────────────────────────────────────────
{
  if (skip("s3_shift_click")) {
    lines.push(`${at()} PROBE s3_shift_click skipped`);
  } else {
    const before = snapshot();
    await page.keyboard.down("Shift");
    await page.mouse.move(centre[0] + 18, centre[1] + 18);
    await pause(300);
    await page.mouse.down();
    await pause(120);
    await page.mouse.up();
    await page.keyboard.up("Shift");
    await settleTicks(6);
    await record("s3_shift_click", delta(before));
  }
}

// ── 4 — empty click clears ────────────────────────────────────────────────────────────────────
{
  if (skip("s4_empty_click")) {
    lines.push(`${at()} PROBE s4_empty_click skipped`);
  } else {
    const before = snapshot();
    await page.mouse.move(corner[0], corner[1]);
    await pause(300);
    await page.mouse.down();
    await pause(120);
    await page.mouse.up();
    await settleTicks(6);
    await record("s4_empty_click", delta(before));
  }
}

// ── 5 — marquee (crossing band, right → left) ─────────────────────────────────────────────────
{
  if (skip("s5_marquee")) {
    lines.push(`${at()} PROBE s5_marquee skipped`);
  } else {
    const before = snapshot();
    const from = [body.x + body.w - 8, body.y + 8];
    const to = [body.x + 8, body.y + body.h - 8];
    await page.mouse.move(from[0], from[1]);
    await page.mouse.down();
    for (let step = 1; step <= 8; step += 1) {
      await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
      await pause(120);
    }
    await page.mouse.up();
    await settleTicks(7);
    await record("s5_marquee", delta(before));
  }
}

// ── 6 — orbit still reaches setCamera (the typed-operation lane must not regress) ──────────────
{
  if (skip("s6_orbit")) {
    lines.push(`${at()} PROBE s6_orbit skipped`);
  } else {
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
    await record("s6_orbit", delta(before));
  }
}

await page.screenshot({ path: join(outDir, "shot-final.png"), type: "png" }).catch(() => {});
await record("counts", Object.fromEntries([
  "[DEBUG] wire spawn-job",
  'unmapped effect "spawn-job"',
  "[DEBUG] wgpu-bridge spawn-job",
  "frame input action",
  "frame deferred action failed",
  "world3d interaction surface=",
  "panicked",
  "wgpu-worker panicked",
  "unmapped effect",
].map((needle) => [needle, has(needle).length])));

writeFileSync(join(outDir, "console.txt"), lines.join("\n"), "utf8");
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2), "utf8");
console.log(JSON.stringify(results.steps.counts, null, 2));
await browser.close();
