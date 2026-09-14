/** ⏳️ PROGRESS VISIBILITY — the ONE probe that measures, on BOTH renderers, what a user can see and
 * stop while a long evaluation is in flight.
 *
 * The goal requires progress and cancellation for every expensive operation, VISIBLE to the user.
 * Two earlier lanes measured the wgpu half and root-caused the silence to the PRODUCER
 * (`📓️wgpu-a11y-status-i18n-runtime-2026-09-14.md` §3, `📓️wgpu-edit-convergence-perf-2026-09-14.md`
 * §8): 54 preview publications during a 23 s `Sphere Cut With Torus` evaluation, every one of them
 * `phase:"idle" facesTotal:0 cancellable:false`. The React half was never measured the same way at
 * all — `status-parity` asserts only that a status object EXISTS. This probe measures the same
 * timeline on both, from the same gesture, so "the renderer loses it" and "the producer never
 * publishes it" can be told apart.
 *
 * Both targets answer the SAME sampler shape, sampled every `SEMIO_PROBE_SAMPLE_MS` (250 ms):
 *   { t, phase, ratio, unitsDone, unitsTotal, facesDone, facesTotal, inFlight, computing,
 *     cancellable, cancelAction, pill, cancelControl }
 *
 *   • react (6018) — read straight off the DOM contract: `[data-status-json]` on the preview host,
 *     `[data-slot="world-compute-status"]` for the pill and `[data-slot="world-compute-cancel"]`
 *     for the cancel control.
 *   • wgpu (6118) — the canvas is an `OffscreenCanvas` owned by the frame Worker and headless
 *     Chromium captures it blank, so a screenshot is NOT evidence on this target. The producer
 *     status is read off the shell's own `[DEBUG] world3d surface=… status=Some("…")` publication
 *     trace and the pill off `[DEBUG] wgpu world3d status pill …`; the cancel control's presence is
 *     the shell's own `shell.world3d.cancel::<surface>` hit target, located by sweeping the
 *     surface's overlay row with the shell's `os_host pointer hit` trace — never a guessed pixel.
 *
 * Axes:
 *   SEMIO_PROBE_TARGET=react|wgpu   (default: inferred from the URL's port)
 *   SEMIO_PROBE_URL                 (default: the target's own dev server)
 *   SEMIO_PROBE_EXAMPLE             (default: sphere-cut-with-torus — the slowest one)
 *   SEMIO_PROBE_CANCEL=1            click the cancel control the first sample it is offered
 *   SEMIO_PROBE_BOOT_EXAMPLE=1      boot straight into the example instead of picking it live,
 *                                   which is how "does the chrome paint before the first
 *                                   convergence" is measured
 *   SEMIO_PROBE_OUT                 output directory under 🗑️generated/
 *
 * Usage: cd <ticket> && SEMIO_PROBE_TARGET=react SEMIO_PROBE_OUT=wgpu-progress/react-base bun 🐍️progress-visibility-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const target = process.env.SEMIO_PROBE_TARGET ?? (String(process.env.SEMIO_PROBE_URL ?? "").includes("6118") ? "wgpu" : "react");
const port = target === "wgpu" ? 6118 : 6018;
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "sphere-cut-with-torus";
const bootExample = process.env.SEMIO_PROBE_BOOT_EXAMPLE === "1";
const wantCancel = process.env.SEMIO_PROBE_CANCEL === "1";
const baseUrl = `${process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:${port}/?plugin=generation3d`}${bootExample ? `&example=${example}` : ""}`;
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? `wgpu-progress/${target}`);
const sampleMs = Number(process.env.SEMIO_PROBE_SAMPLE_MS ?? 250);
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 200);
const watchSeconds = Number(process.env.SEMIO_PROBE_WATCH ?? 180);
const viewport = { width: 1440, height: 900 };
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport });
await page.addInitScript(() => {
  try {
    localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {}
});
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const stamp = (line) => Number(line.split(" ")[0]);

//#region 🧪️Sampler
/** 📈️ The status contract, normalized to ONE row whatever produced it. */
const row = (status, extra) => ({
  t: at(),
  phase: status?.phase ?? null,
  phaseLabelEn: status?.phaseLabel?.en ?? null,
  ratio: status?.progress?.ratio ?? null,
  unitsDone: status?.progress?.unitsDone ?? null,
  unitsTotal: status?.progress?.unitsTotal ?? null,
  evalUnitsDone: status?.progress?.evalUnitsDone ?? null,
  evalUnitsTotal: status?.progress?.evalUnitsTotal ?? null,
  facesDone: status?.progress?.facesDone ?? null,
  facesTotal: status?.progress?.facesTotal ?? null,
  inFlight: status?.progress?.inFlight ?? null,
  computing: status?.computing ?? null,
  cancellable: status?.cancellable ?? null,
  cancelAction: status?.cancelAction ?? null,
  meshesLen: status?.debug?.meshesLen ?? null,
  ...extra,
});

/** ⚛️ React: the whole contract is in the DOM. */
const sampleReact = async () => {
  const snapshot = await page
    .evaluate(() => {
      const parse = (text) => {
        try {
          return JSON.parse(text ?? "");
        } catch {
          return null;
        }
      };
      const hosts = [...document.querySelectorAll("[data-status-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), status: parse(el.getAttribute("data-status-json")) }));
      const host = hosts.find((entry) => (entry.surfaceId ?? "").endsWith("preview")) ?? hosts[0] ?? null;
      const pane = document.querySelector('[data-slot="world-compute-status"]');
      const cancel = document.querySelector('[data-slot="world-compute-cancel"]');
      return {
        status: host?.status ?? null,
        surfaceId: host?.surfaceId ?? null,
        pill: pane ? { phase: pane.getAttribute("data-compute-phase"), ratio: pane.getAttribute("data-compute-ratio"), text: pane.innerText.replace(/\s+/g, " ").trim().slice(0, 120) } : null,
        cancelControl: cancel ? { action: cancel.getAttribute("data-cancel-action"), text: (cancel.textContent ?? "").trim().slice(0, 60) } : null,
      };
    })
    .catch(() => null);
  return row(snapshot?.status, { surfaceId: snapshot?.surfaceId ?? null, pill: snapshot?.pill ?? null, cancelControl: snapshot?.cancelControl ?? null });
};

/** 🧊️ wgpu: the shell's own publication + pill traces. Nothing is read from a pixel. */
let wgpuCursor = 0;
let lastWgpuStatus = null;
let lastWgpuPill = null;
const drainWgpu = () => {
  for (const line of lines.slice(wgpuCursor)) {
    if (line.includes("world3d surface=") && line.includes("-preview")) {
      const raw = /status=Some\("((?:[^"\\]|\\.)*)"\)/.exec(line)?.[1];
      if (raw !== undefined) {
        try {
          lastWgpuStatus = JSON.parse(JSON.parse(`"${raw}"`));
        } catch {
          lastWgpuStatus = null;
        }
      }
    }
    if (line.includes("wgpu world3d status pill")) {
      lastWgpuPill = {
        phase: /phase=(\S+)/.exec(line)?.[1] ?? null,
        label: /label="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
        ratio: /ratio=(Some\([\d.]+\)|None)/.exec(line)?.[1] ?? null,
        computing: /computing=(true|false)/.exec(line)?.[1] ?? null,
        at: stamp(line),
      };
    }
  }
  wgpuCursor = lines.length;
};
const sampleWgpu = async () => {
  drainWgpu();
  const fresh = lastWgpuPill && at() - lastWgpuPill.at < 4000 ? lastWgpuPill : null;
  return row(lastWgpuStatus, { surfaceId: "procedural.play.preview", pill: fresh, cancelControl: lastWgpuStatus?.cancellable === true ? { action: lastWgpuStatus?.cancelAction ?? null, source: "status" } : null });
};

const sample = target === "wgpu" ? sampleWgpu : sampleReact;
//#endregion 🧪️Sampler

//#region 🖱️Driving
/** 🖱️ The wgpu browser tick is INPUT-DRIVEN, so every wait pumps a 1 px nudge. */
const pump = async (ms) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  while (Date.now() < deadline) {
    await page.waitForTimeout(Math.min(150, Math.max(20, deadline - Date.now())));
    flip = 1 - flip;
    if (target === "wgpu") await page.mouse.move(3 + flip, 3).catch(() => {});
  }
};

const hitLines = (fromIndex = 0) => {
  const parsed = [];
  for (const line of lines.slice(fromIndex)) {
    const match = /os_host pointer hit x=([-\d.]+) y=([-\d.]+) targets=(\d+)(?:\s+(?!hit=)\w+=\S+)*\s+hit=(.*)$/.exec(line);
    if (!match) continue;
    const some = /Some\(\((\w+), Some\("([^"]+)"\)\)\)/.exec(match[4]);
    parsed.push({ x: Number(match[1]), y: Number(match[2]), kind: some?.[1] ?? null, id: some?.[2] ?? null });
  }
  return parsed;
};

/** 🎯️ Sweeps a band and answers the centre of every control the shell's own hit trace named there. */
const sweep = async (label, y0, y1, x0, x1, step) => {
  const mark = lines.length;
  for (let y = y0; y < y1; y += step) {
    for (let x = x0; x < x1; x += 18) {
      await page.mouse.move(x, y);
      await page.waitForTimeout(26);
    }
  }
  const controls = {};
  for (const hit of hitLines(mark)) {
    if (!hit.id) continue;
    const entry = (controls[hit.id] ??= { id: hit.id, x0: hit.x, x1: hit.x, y0: hit.y, y1: hit.y });
    entry.x0 = Math.min(entry.x0, hit.x);
    entry.x1 = Math.max(entry.x1, hit.x);
    entry.y0 = Math.min(entry.y0, hit.y);
    entry.y1 = Math.max(entry.y1, hit.y);
  }
  for (const entry of Object.values(controls)) entry.point = [(entry.x0 + entry.x1) / 2, (entry.y0 + entry.y1) / 2];
  note(`sweep ${label}: ${JSON.stringify(Object.keys(controls))}`);
  return controls;
};

const pickReact = async () => {
  const label = example
    .split("-")
    .map((word) => word[0].toUpperCase() + word.slice(1))
    .join(" ");
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 8000 });
  await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: label }).first().click({ timeout: 8000 });
  note(`picked ${label} via the combobox`);
  return { id: `shell.example.${example}`, via: "combobox" };
};

const pickWgpu = async () => {
  const navbar = await sweep("navbar", 8, 48, 6, viewport.width - 4, 8);
  const trigger = navbar["playground.navbar.fixture"];
  if (!trigger) {
    note("BLOCKED: no playground.navbar.fixture control in the navbar band");
    return null;
  }
  await page.mouse.click(trigger.point[0], trigger.point[1]);
  await pump(2500);
  const rows = await sweep("dropdown", 60, Math.round(viewport.height * 0.6), Math.round(viewport.width * 0.5 - 190), Math.round(viewport.width * 0.5 + 190), 10);
  const chosen = rows[`shell.example.${example}`];
  if (!chosen) {
    note(`BLOCKED: no shell.example.${example} row among ${JSON.stringify(Object.keys(rows))}`);
    return null;
  }
  await page.mouse.click(chosen.point[0], chosen.point[1]);
  note(`picked ${chosen.id} at ${JSON.stringify(chosen.point)}`);
  return { id: chosen.id, via: "hit-trace", point: chosen.point };
};

/** 🛑️ Clicks the surface's OWN cancel control. On wgpu it is located by sweeping the preview
 * surface's overlay row for `shell.world3d.cancel::…`; a sweep that finds none is the finding. */
const clickCancel = async () => {
  if (target === "react") {
    const control = page.locator('[data-slot="world-compute-cancel"]').first();
    if ((await control.count()) === 0) return { clicked: false, reason: "no world-compute-cancel in the DOM" };
    await control.click({ timeout: 4000 });
    return { clicked: true, via: "data-slot" };
  }
  const bounds = /world3d surface=\S*-preview\S* pane=\S+ bounds=(\d+)x(\d+)\+(-?\d+),(-?\d+)/.exec(has("world3d surface=").filter((line) => line.includes("-preview")).at(-1) ?? "");
  const band = bounds ? { x: Number(bounds[3]), y: Number(bounds[4]), w: Number(bounds[1]), h: Number(bounds[2]) } : { x: viewport.width / 2, y: 60, w: viewport.width / 2, h: 300 };
  const controls = await sweep("overlay", band.y + 2, band.y + 60, band.x + 2, band.x + Math.min(band.w, 460), 6);
  const cancel = Object.values(controls).find((entry) => entry.id.startsWith("shell.world3d.cancel::"));
  if (!cancel) return { clicked: false, reason: `no shell.world3d.cancel:: in the overlay row; offered ${JSON.stringify(Object.keys(controls))}` };
  await page.mouse.click(cancel.point[0], cancel.point[1]);
  return { clicked: true, via: "hit-trace", id: cancel.id, point: cancel.point };
};
//#endregion 🖱️Driving

//#region 🏃️Run
note(`target=${target} url=${baseUrl} bootExample=${bootExample} cancel=${wantCancel}`);
await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

/** 🎨️ When the CHROME first exists, against when the first example first converges — the boot-axis
 * question: does anything paint before the whole convergence has run? */
const chromeLive = async () => {
  if (target === "react") return (await page.locator("[data-window-instance-id]").count()) > 0;
  return has("world3d surface=").length > 0 || has("wgpu-shell dock plan").length > 0;
};

/** 🎬️ The surface is not merely EXISTING but painting: the picker cannot be driven before the shell
 * answers pointer hits, which needs several published frames (the recon probe's own gate). */
const surfacePainting = async () => {
  if (target === "react") return (await page.locator("[data-status-json]").count()) > 0;
  return has("world3d surface=").length > 3;
};

let chromeAtMs = null;
let paintingAtMs = null;
const timeline = [];
for (let tick = 0; tick < (bootSeconds * 1000) / sampleMs; tick += 1) {
  await pump(sampleMs);
  timeline.push({ ...(await sample()), stage: "boot" });
  if (chromeAtMs === null && (await chromeLive())) {
    chromeAtMs = at();
    note(`chrome live at ${chromeAtMs} ms`);
  }
  if (paintingAtMs === null && (await surfacePainting())) {
    paintingAtMs = at();
    note(`preview surface painting at ${paintingAtMs} ms`);
  }
  if (paintingAtMs !== null && at() - paintingAtMs > (bootExample ? 25_000 : 9_000)) break;
  if (bootExample && timeline.at(-1)?.meshesLen > 400) break;
}
const bootShellLeave = has("boot_shell leave").at(-1) ?? null;
await page.screenshot({ path: join(outDir, "01-boot.png") }).catch(() => {});

let picked = null;
if (!bootExample) {
  picked = target === "react" ? await pickReact().catch((error) => ({ id: null, error: String(error).slice(0, 200) })) : await pickWgpu();
}
const pickedAtMs = at();

let cancelled = null;
let cancelAtMs = null;
for (let tick = 0; tick < (watchSeconds * 1000) / sampleMs; tick += 1) {
  await pump(sampleMs);
  const current = { ...(await sample()), stage: "watch" };
  timeline.push(current);
  if (wantCancel && !cancelled && current.cancellable === true) {
    cancelAtMs = at();
    cancelled = await clickCancel();
    note(`cancel attempt ${JSON.stringify(cancelled)}`);
  }
  if (tick % 40 === 0) note(`t=${Math.round(at() / 1000)}s phase=${current.phase} ratio=${current.ratio} inFlight=${current.inFlight} cancellable=${current.cancellable} pill=${Boolean(current.pill)} meshes=${current.meshesLen}`);
  const settledFor = timeline.slice(-12);
  if (tick > 20 && settledFor.length === 12 && settledFor.every((entry) => entry.phase === "idle" || entry.phase === "cancelled") && current.meshesLen > 400) break;
  if (cancelled?.clicked && current.phase === "cancelled") break;
}
await page.screenshot({ path: join(outDir, "02-settled.png") }).catch(() => {});
//#endregion 🏃️Run

//#region 📊️Verdict
const watched = timeline.filter((entry) => entry.stage === "watch");
const nonIdle = watched.filter((entry) => entry.phase && entry.phase !== "idle");
const computingRows = watched.filter((entry) => entry.computing === true || entry.inFlight > 0);
const cancellableRows = watched.filter((entry) => entry.cancellable === true);
const pillRows = watched.filter((entry) => entry.pill);
const ratios = nonIdle.map((entry) => entry.ratio).filter((value) => typeof value === "number");
const monotone = ratios.every((value, index) => index === 0 || value >= ratios[index - 1] - 1e-9);
const result = {
  target,
  url: baseUrl,
  example,
  bootExample,
  picked,
  pickedAtMs,
  chromeLiveAtMs: chromeAtMs,
  surfacePaintingAtMs: paintingAtMs,
  bootShellLeave,
  samples: timeline.length,
  watchedSamples: watched.length,
  firstNonIdleAtMs: nonIdle[0]?.t ?? null,
  nonIdleSamples: nonIdle.length,
  distinctPhases: [...new Set(watched.map((entry) => entry.phase))],
  computingSamples: computingRows.length,
  cancellableSamples: cancellableRows.length,
  pillSamples: pillRows.length,
  ratiosMonotone: monotone,
  ratioSpan: ratios.length ? [Math.min(...ratios), Math.max(...ratios)] : null,
  maxFacesTotal: Math.max(0, ...watched.map((entry) => entry.facesTotal ?? 0)),
  maxInFlight: Math.max(0, ...watched.map((entry) => entry.inFlight ?? 0)),
  finalMeshesLen: watched.at(-1)?.meshesLen ?? null,
  cancel: { requested: wantCancel, atMs: cancelAtMs, outcome: cancelled, settledCancelled: watched.some((entry) => entry.phase === "cancelled") },
  pageErrors: has("pageerror").length,
};
note(`RESULT nonIdle=${result.nonIdleSamples}/${result.watchedSamples} phases=${JSON.stringify(result.distinctPhases)} cancellable=${result.cancellableSamples} pill=${result.pillSamples} monotone=${result.ratiosMonotone} chrome=${result.chromeLiveAtMs}ms`);
writeFileSync(join(outDir, "result.json"), JSON.stringify(result, null, 2));
writeFileSync(join(outDir, "timeline.json"), JSON.stringify(timeline, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
//#endregion 📊️Verdict
