/** 🧭️ wgpu USER-JOURNEY probe — the twin of `🐍️journey-probe.mjs` (React, 6018) on the wgpu renderer.
 *
 * Proves, from a user's seat, that on `http://127.0.0.1:6118/?plugin=generation3d`:
 *   • every bundled example of the open dialect loads with a 3d preview that carries a mesh, in edit
 *     mode / editor role AND in viewer role;
 *   • generate mode's `Add Generation` row yields a generation whose preview carries a mesh;
 *   • hover, a selection click and a wheel orbit reach the World3d surface.
 *
 * NOTHING is clicked at a guessed pixel. Two published sources, and only those two:
 *   • CHROME (navbar picker, example rows, mode/role groups, the World3d cancel control) — the shell's
 *     own `[DEBUG] os_host pointer hit x=… y=… targets=N hit=Some((Kind, Some("control.id")))` trace,
 *     which prints `InputState::hit_at` for every pointer move. A band is SWEPT and the control's rect
 *     is the extent of the points that answered its id. A control that never answers is absent, and the
 *     step is recorded `blocked`, never guessed at.
 *   • RETAINED BODIES (`Add Generation`, the preview surface) — `dumpStructure(windowId)`'s published
 *     `mounted_layout` rects offset by the window's own `[DEBUG] wgpu-shell dock plan` body rect, the
 *     derivation `🐍️wgpu-hit-probe.mjs:62-100` proved.
 *
 * Every click carries the pointer-delivery witness of `🐍️wgpu-hit-probe.mjs:103-113`, so "the shell
 * did not dispatch" is never confused with "the browser delivered no event".
 *
 * The wgpu host tick is INPUT-DRIVEN (`📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §3.2: 75 drains
 * in 80 s with the shell settled), so every wait pumps a 1 px nudge around a parking point rather than
 * sleeping — a bare `waitForTimeout` measures a renderer that was never asked to run.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-journey/run-1 bun 🐍️wgpu-journey-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-journey/run");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 150);
const convergeSeconds = Number(process.env.SEMIO_PROBE_CONVERGE ?? 45);
const viewport = { width: 1440, height: 900 };
/** 🚏️ `boot` stops after the boot gate and its first convergence — the cheap discriminator for
 * "which mode/role can the shell reach at all", without paying for a 20-minute journey. */
const only = process.env.SEMIO_PROBE_ONLY ?? "";
mkdirSync(outDir, { recursive: true });

const lines = [];
const results = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const lastOf = (needle) => has(needle).at(-1) ?? null;

//#region 🩺️Sources
/** 🧹️ A peer's dev-server error overlay can cover the whole page even on a healthy serve. */
const dropOverlay = () =>
  page
    .evaluate(() => {
      const overlay = Array.from(document.body.children).find((node) => node.tagName === "DIV" && node.id !== "root");
      if (!overlay) return null;
      const text = (overlay.textContent ?? "").slice(0, 200);
      overlay.remove();
      return text;
    })
    .catch(() => null);

/** 🔬️ The only introspection surface wgpu has; both calls answer `""` rather than throwing. */
const dump = (windowId) =>
  page
    .evaluate(async (id) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const parse = async (call) => {
        try {
          const raw = await call();
          return raw ? JSON.parse(raw) : null;
        } catch (error) {
          return { error: String(error) };
        }
      };
      return { structure: await parse(() => beacon.dumpStructure(id)), stats: await parse(() => beacon.dumpFrameStats(id)) };
    }, windowId)
    .catch((error) => ({ error: String(error).slice(0, 300) }));

/** 📰️ The only two DOM elements the wgpu host writes outside the canvas. */
const domSignals = () =>
  page
    .evaluate(() => ({
      status: document.querySelector('[role="status"]')?.textContent?.trim() ?? null,
      alert: document.querySelector('[role="alert"]')?.textContent?.trim()?.slice(0, 400) ?? null,
      canvas: (() => {
        const c = document.querySelector("canvas");
        return c ? { w: c.clientWidth, h: c.clientHeight } : null;
      })(),
    }))
    .catch(() => ({ status: null, alert: null, canvas: null }));

/** 🪟️ The shell's last dock plan as `{ id: {x,y,w,h} }` — the window bodies on screen. */
const dockPlan = () => {
  const line = lastOf("wgpu-shell dock plan");
  if (!line) return {};
  const plan = {};
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

/** 🌍️ The renderer's own per-surface World3d census (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1384`):
 * `instances=` is the scene pass's instance total, `meshesHead=` the head of the published mesh JSON,
 * `camera=` the head of the camera JSON, `status=` the guest's own status payload. */
const world3dTraces = () => {
  const byId = {};
  for (const line of has("world3d surface=")) {
    const surface = /world3d surface=(\S+)/.exec(line)?.[1];
    if (!surface) continue;
    byId[surface] = {
      instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0),
      draws: Number(/ draws=(\d+)/.exec(line)?.[1] ?? 0),
      meshesHead: /meshesHead="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
      camera: /camera="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
      status: /status=Some\("((?:[^"\\]|\\.)*)"\)/.exec(line)?.[1] ?? null,
      count: (byId[surface]?.count ?? 0) + 1,
    };
  }
  return byId;
};

/** 🎯️ Every `os_host pointer hit` the shell printed, as `{x, y, targets, kind, id}`. */
const hitLines = (fromIndex = 0) => {
  const parsed = [];
  for (const line of lines.slice(fromIndex)) {
    const match = /os_host pointer hit x=([-\d.]+) y=([-\d.]+) targets=(\d+) hit=(.*)$/.exec(line);
    if (!match) continue;
    const tail = match[4];
    const some = /Some\(\((\w+), Some\("([^"]+)"\)\)\)/.exec(tail);
    const anon = /Some\(\((\w+), None\)\)/.exec(tail);
    parsed.push({
      x: Number(match[1]),
      y: Number(match[2]),
      targets: Number(match[3]),
      kind: some?.[1] ?? anon?.[1] ?? null,
      id: some?.[2] ?? null,
    });
  }
  return parsed;
};
//#endregion 🩺️Sources

//#region ⏱️Pump
/** 🫀️ The wgpu browser tick only fires on input, so a wait that does not nudge measures a renderer
 * that was never asked to run. `park` keeps the pointer where a hover test put it (± 1 px). */
const pump = async (ms, park) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  const point = park ?? [3, 3];
  while (Date.now() < deadline) {
    await page.waitForTimeout(180);
    flip = 1 - flip;
    await page.mouse.move(point[0] + flip, point[1]).catch(() => {});
  }
};
//#endregion ⏱️Pump

//#region 🖱️PointerWitness
/** 🖱️ Counts what the BROWSER delivers to the canvas, so "no dispatch" is distinguishable from
 * "no pointer event ever arrived" — the two have identical console evidence otherwise. */
const installWitness = () =>
  page
    .evaluate(() => {
      const canvas = document.querySelector("canvas");
      const host = canvas ?? document.body;
      globalThis.__semioProbePointer = { downs: 0, moves: 0, wheels: 0, keys: 0, last: null, tag: host.tagName, canvas: canvas ? canvas.getBoundingClientRect().toJSON() : null };
      const w = globalThis.__semioProbePointer;
      host.addEventListener("pointerdown", (e) => { w.downs += 1; w.last = { x: e.clientX, y: e.clientY }; }, true);
      host.addEventListener("pointermove", () => { w.moves += 1; }, true);
      host.addEventListener("wheel", () => { w.wheels += 1; }, true);
      globalThis.addEventListener("keydown", () => { w.keys += 1; }, true);
      return true;
    })
    .catch(() => false);

const witness = () => page.evaluate(() => (globalThis.__semioProbePointer ? { ...globalThis.__semioProbePointer } : null)).catch(() => null);

/** 👆️ One click, wrapped in its delivery witness and in the shell's own hit trace at that point. */
const clickWitnessed = async (point, label) => {
  const before = await witness();
  const mark = lines.length;
  await page.mouse.move(point[0], point[1]);
  await page.waitForTimeout(220);
  const hitUnderPointer = hitLines(mark).at(-1) ?? null;
  await page.mouse.click(point[0], point[1]);
  await page.waitForTimeout(400);
  const after = await witness();
  const record = {
    label,
    point,
    hitUnderPointer,
    delivered: (after?.downs ?? 0) - (before?.downs ?? 0),
    downsTotal: after?.downs ?? null,
  };
  note(`click ${label} at ${point.map((n) => Math.round(n)).join(",")} delivered=${record.delivered} hit=${hitUnderPointer ? `${hitUnderPointer.kind}:${hitUnderPointer.id}` : "none"}`);
  return record;
};
//#endregion 🖱️PointerWitness

//#region 🧭️ChromeDerivation
/** 🧹️ Sweeps a band of points and returns, per control id the shell answered, the extent of the
 * points that answered it and that extent's centre — the chrome equivalent of a published rect. */
const sweep = async (points, label) => {
  const mark = lines.length;
  for (const [x, y] of points) {
    await page.mouse.move(x, y);
    await page.waitForTimeout(45);
  }
  const hits = hitLines(mark);
  const controls = {};
  for (const hit of hits) {
    if (!hit.id) continue;
    const entry = (controls[hit.id] ??= { id: hit.id, kind: hit.kind, x0: hit.x, x1: hit.x, y0: hit.y, y1: hit.y, samples: 0 });
    entry.x0 = Math.min(entry.x0, hit.x);
    entry.x1 = Math.max(entry.x1, hit.x);
    entry.y0 = Math.min(entry.y0, hit.y);
    entry.y1 = Math.max(entry.y1, hit.y);
    entry.samples += 1;
  }
  for (const entry of Object.values(controls)) entry.point = [(entry.x0 + entry.x1) / 2, (entry.y0 + entry.y1) / 2];
  const targets = hits.at(-1)?.targets ?? null;
  note(`sweep ${label}: ${points.length} points, targets=${targets}, controls=${JSON.stringify(Object.keys(controls))}`);
  return { controls, targets, points: points.length, hits: hits.length };
};

/** 📏️ The navbar band is everything above the topmost dock body — derived from the shell's own plan. */
const navbarBand = () => {
  const plan = dockPlan();
  const tops = Object.values(plan).map((body) => body.y);
  const bottom = tops.length ? Math.min(...tops) : 40;
  return { bottom, mid: Math.max(6, bottom / 2) };
};

const sweepNavbar = async (label) => {
  const { mid, bottom } = navbarBand();
  const rows = bottom > 28 ? [mid - 6, mid, mid + 6] : [mid];
  const points = [];
  for (const y of rows) for (let x = 6; x < viewport.width - 4; x += 14) points.push([x, y]);
  return await sweep(points, `navbar ${label} (band 0..${Math.round(bottom)})`);
};
//#endregion 🧭️ChromeDerivation

//#region 🌳️RetainedDerivation
/** 🌳️ A retained node's page point: its published `mounted_layout` rect offset by its window's own
 * dock body rect (`🐍️wgpu-hit-probe.mjs:88-100`). */
const retainedTargets = async (windowIds, needle) => {
  const plan = dockPlan();
  const found = [];
  for (const id of windowIds) {
    const body = plan[id];
    if (!body) continue;
    const sample = await dump(id);
    for (const node of sample?.structure?.nodes ?? []) {
      const path = String(node.path ?? "");
      const text = String(node.text ?? "");
      if (!path.includes(needle) && !text.toLowerCase().includes(needle.toLowerCase())) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      found.push({ windowId: id, path, text: node.text ?? null, kind: node.kind, rect: node.rect, point: [body.x + rx + rw / 2, body.y + ry + rh / 2] });
    }
  }
  return found;
};

/** 🎬️ Every live scene surface: its window, its published rect and its page point. */
const sceneSurfaces = async (windowIds) => {
  const plan = dockPlan();
  const surfaces = [];
  for (const id of windowIds) {
    const body = plan[id];
    const sample = await dump(id);
    for (const node of sample?.structure?.nodes ?? []) {
      if (String(node.kind ?? "") !== "componentScene") continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      surfaces.push({
        windowId: id,
        path: String(node.path ?? ""),
        rect: node.rect,
        point: body ? [body.x + rx + rw / 2, body.y + ry + rh / 2] : null,
        stats: sample?.stats ?? null,
      });
    }
  }
  return surfaces;
};
//#endregion 🌳️RetainedDerivation

//#region ⚖️Convergence
const liveWindowIds = async () => (await dump(undefined))?.structure?.windowIds ?? [];

/** ⚖️ A step converged when a live scene surface carries instances AND the renderer's own World3d
 * census published a non-empty mesh list AND no `[role=alert]` is up — stable across two samples. */
const measure = async () => {
  const ids = await liveWindowIds();
  const stats = {};
  for (const id of ids) stats[id] = (await dump(id))?.stats ?? null;
  const dom = await domSignals();
  const traces = world3dTraces();
  const sceneInstances = Math.max(0, ...Object.values(stats).map((s) => s?.sceneInstances ?? 0));
  const quadCount = Math.max(0, ...Object.values(stats).map((s) => s?.quadCount ?? 0));
  const meshy = Object.entries(traces).filter(([, t]) => t.instances > 0 && t.meshesHead && t.meshesHead !== "[]");
  return {
    windowIds: ids,
    stats,
    dom,
    traces,
    sceneInstances,
    quadCount,
    meshSurfaces: meshy.map(([id, t]) => ({ surface: id, instances: t.instances, meshesHead: (t.meshesHead ?? "").slice(0, 120) })),
    converged: sceneInstances > 0 && meshy.length > 0 && !dom.alert,
  };
};

const waitConverged = async (label, seconds, extra = {}) => {
  const start = Date.now();
  let sample = await measure();
  let stable = sample.converged ? 1 : 0;
  for (let second = 0; second < seconds && stable < 2; second += 1) {
    await pump(1000);
    sample = await measure();
    stable = sample.converged ? stable + 1 : 0;
  }
  const row = {
    label,
    t: at(),
    seconds: Math.round((Date.now() - start) / 10) / 100,
    verdict: sample.converged ? "pass" : "fail",
    ...extra,
    windowIds: sample.windowIds,
    sceneInstances: sample.sceneInstances,
    quadCount: sample.quadCount,
    meshSurfaces: sample.meshSurfaces,
    status: sample.dom.status,
    alert: sample.dom.alert,
    renderBegin: has("wgpu-shell render begin").length,
  };
  results.push(row);
  note(`${label}: ${row.verdict} in ${row.seconds}s sceneInstances=${row.sceneInstances} meshSurfaces=${JSON.stringify(row.meshSurfaces)} alert=${JSON.stringify(row.alert)}`);
  await dropOverlay();
  await page.screenshot({ path: join(outDir, `${String(results.length).padStart(2, "0")}-${label.replace(/[^a-z0-9]+/gi, "-").slice(0, 60)}.png`) }).catch(() => {});
  return row;
};
//#endregion ⚖️Convergence

//#region 🚀️Boot
const bootGate = async (url) => {
  const start = Date.now();
  await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
  await installWitness();
  for (let second = 0; second < bootSeconds; second += 1) {
    await pump(1000);
    const ids = await liveWindowIds();
    if (ids.length > 0 && Object.keys(dockPlan()).length > 0) {
      const dom = await domSignals();
      note(`boot gate: ${Math.round((Date.now() - start) / 1000)}s windowIds=${JSON.stringify(ids)} status=${JSON.stringify(dom.status)} alert=${JSON.stringify(dom.alert)}`);
      await installWitness();
      return { booted: true, seconds: Math.round((Date.now() - start) / 1000), windowIds: ids };
    }
  }
  const dom = await domSignals();
  note(`boot gate: TIMEOUT after ${bootSeconds}s status=${JSON.stringify(dom.status)} alert=${JSON.stringify(dom.alert)}`);
  return { booted: false, seconds: bootSeconds, windowIds: [] };
};
//#endregion 🚀️Boot

//#region 🎬️Journey
const journey = { url: baseUrl, startedAt: new Date().toISOString(), phases: {} };

const boot = await bootGate(baseUrl);
journey.boot = boot;
if (!boot.booted) {
  results.push({ label: "boot", verdict: "fail", t: at(), reason: "the beacon never reported a live window" });
} else {
  await waitConverged("boot:edit-editor", convergeSeconds);
}

/** 📚️ The example picker: the navbar trigger, then the dropdown's own rows. The trigger's derived
 * point is cached per phase — the navbar does not move between two picks of the same layout, and a
 * full band sweep per example costs 14 s of a probe that is already minutes long. */
const openPicker = async (label, cached) => {
  const navbar = cached ? { controls: { "playground.navbar.fixture": cached }, targets: null, cached: true } : await sweepNavbar(label);
  const trigger = navbar.controls["playground.navbar.fixture"] ?? null;
  if (!trigger) return { navbar, trigger: null, rows: {}, clicked: null };
  const clicked = await clickWitnessed(trigger.point, `picker trigger (${label})`);
  await pump(900);
  const { bottom } = navbarBand();
  const points = [];
  for (const dx of [0, 40, -40]) for (let y = bottom + 4; y < Math.min(viewport.height - 8, bottom + 520); y += 9) points.push([Math.max(6, Math.min(viewport.width - 6, trigger.point[0] + dx)), y]);
  const overlay = await sweep(points, `picker rows (${label})`);
  const rows = Object.fromEntries(Object.entries(overlay.controls).filter(([id]) => id.startsWith("shell.example.")));
  return { navbar, trigger, clicked, rows, overlaySweep: overlay };
};

const pickExample = async (row, label) => {
  const clicked = await clickWitnessed(row.point, `example row ${row.id} (${label})`);
  await pump(1200);
  return clicked;
};

const runExamples = async (phase) => {
  const picker = await openPicker(phase);
  journey.phases[phase] = { picker: { trigger: picker.trigger, rows: Object.keys(picker.rows), navbarControls: Object.keys(picker.navbar.controls), targets: picker.navbar.targets } };
  if (!picker.trigger) {
    results.push({ label: `${phase}:picker`, verdict: "blocked", t: at(), reason: "no `playground.navbar.fixture` hit target answered anywhere in the navbar band", navbarControls: Object.keys(picker.navbar.controls), targets: picker.navbar.targets });
    return;
  }
  const rowIds = Object.keys(picker.rows);
  if (rowIds.length === 0) {
    results.push({ label: `${phase}:picker-rows`, verdict: "blocked", t: at(), reason: "the picker trigger answered but no `shell.example.*` row did", trigger: picker.trigger, delivered: picker.clicked?.delivered ?? null });
    return;
  }
  note(`${phase}: ${rowIds.length} example rows — ${JSON.stringify(rowIds)}`);
  let rows = picker.rows;
  for (const id of rowIds) {
    const target = rows[id];
    const clicked = target ? await pickExample(target, phase) : null;
    await waitConverged(`${phase}:${id.replace("shell.example.", "")}`, convergeSeconds, { exampleId: id.replace("shell.example.", ""), clicked, mode: phase });
    const reopened = await openPicker(`${phase} reopen`, picker.trigger);
    rows = Object.keys(reopened.rows).length ? reopened.rows : rows;
  }
  await page.keyboard.press("Escape").catch(() => {});
  await pump(600);
};

if (boot.booted && only !== "boot") await runExamples("edit");

/** 🔀️ Mode: the `mod+alt+→` chord the chrome-parity lane reserved, else the navbar mode group,
 * else a boot-time `?mode=generate` reload recorded as `fallback`. */
const enterGenerate = async () => {
  const before = JSON.stringify((await liveWindowIds()).sort());
  await page.keyboard.press("Meta+Alt+ArrowRight").catch(() => {});
  await pump(4000);
  let after = JSON.stringify((await liveWindowIds()).sort());
  if (after !== before) return { via: "chord mod+alt+ArrowRight", verdict: "pass", before, after };
  const navbar = await sweepNavbar("generate-mode lookup");
  const button = Object.entries(navbar.controls).find(([id]) => id.startsWith("playground.navbar.modes.") && id.includes("generate"));
  if (button) {
    const clicked = await clickWitnessed(button[1].point, "mode button generate");
    await pump(4000);
    after = JSON.stringify((await liveWindowIds()).sort());
    if (after !== before) return { via: `navbar ${button[0]}`, verdict: "pass", before, after, clicked };
    return { via: `navbar ${button[0]}`, verdict: "fail", before, after, clicked, navbarControls: Object.keys(navbar.controls) };
  }
  const boot2 = await bootGate(`${baseUrl}&mode=generate`);
  return { via: "?mode=generate reload", verdict: "fallback", before, after: JSON.stringify(boot2.windowIds.sort()), booted: boot2.booted };
};

if (boot.booted && only !== "boot") {
  const generate = await enterGenerate();
  journey.phases.generate = { entry: generate };
  results.push({ label: "generate:enter", verdict: generate.verdict, t: at(), ...generate });
  await waitConverged("generate:layout", 20, { entry: generate.via });

  const ids = await liveWindowIds();
  const addRows = await retainedTargets(ids, "add-generation");
  const addByText = addRows.length ? addRows : await retainedTargets(ids, "Add Generation");
  journey.phases.generate.addRows = addByText;
  if (!addByText.length) {
    results.push({ label: "generate:add-generation", verdict: "blocked", t: at(), reason: "no retained node whose path or text names `Add Generation` in any live window", windowIds: ids });
  } else {
    const target = addByText.at(-1);
    const clicked = await clickWitnessed(target.point, "Add Generation row");
    const row = await waitConverged("generate:add-generation", convergeSeconds, { clicked, path: target.path, rect: target.rect });
    row.addGenerationLines = lines.filter((line) => line.toLowerCase().includes("addgeneration")).slice(-8);
    row.verdict = row.verdict === "pass" && clicked.delivered > 0 && clicked.hitUnderPointer?.id?.includes("add-generation") ? "pass" : row.verdict === "pass" ? "pass" : clicked.delivered === 0 ? "fail (no pointer delivered)" : "fail";
  }
}

/** 🧑‍🤝‍🧑 Role: the `mod+alt+v` chord, else the navbar roles group, else `?role=viewer` as `fallback`. */
const enterViewer = async () => {
  const mark = lines.length;
  await page.keyboard.press("Meta+Alt+v").catch(() => {});
  await pump(5000);
  const ladder = lines.slice(mark).filter((line) => line.includes("shell session switch"));
  if (ladder.length) return { via: "chord mod+alt+v", verdict: "pass", ladder: ladder.slice(0, 8) };
  const navbar = await sweepNavbar("viewer-role lookup");
  const button = Object.entries(navbar.controls).find(([id]) => id.startsWith("playground.navbar.roles.") && id.includes("viewer"));
  if (button) {
    const clicked = await clickWitnessed(button[1].point, "role button viewer");
    await pump(6000);
    const ladder2 = lines.slice(mark).filter((line) => line.includes("shell session switch"));
    return { via: `navbar ${button[0]}`, verdict: ladder2.length ? "pass" : "fail", ladder: ladder2.slice(0, 8), clicked };
  }
  const boot3 = await bootGate(`${baseUrl}&role=viewer`);
  return { via: "?role=viewer reload", verdict: "fallback", booted: boot3.booted, windowIds: boot3.windowIds };
};

if (boot.booted && only !== "boot") {
  const viewer = await enterViewer();
  journey.phases.viewer = { entry: viewer };
  results.push({ label: "viewer:enter", verdict: viewer.verdict, t: at(), ...viewer });
  await waitConverged("viewer:layout", convergeSeconds, { entry: viewer.via });
  await runExamples("view");
}

/** 🌍️ The World3d surface itself: hover, a selection click and a wheel orbit. */
if (boot.booted && only !== "boot") {
  const ids = await liveWindowIds();
  const surfaces = await sceneSurfaces(ids);
  const preview = surfaces.find((s) => s.windowId.includes("preview") && s.point) ?? surfaces.find((s) => s.point) ?? null;
  journey.phases.world3d = { surfaces, preview };
  if (!preview) {
    results.push({ label: "world3d", verdict: "blocked", t: at(), reason: "no componentScene node with a dock body in any live window", windowIds: ids, surfaces });
  } else {
    const cameraOf = () => world3dTraces()[Object.keys(world3dTraces()).find((id) => preview.path.includes(id) || preview.windowId.includes(id)) ?? Object.keys(world3dTraces())[0]]?.camera ?? null;

    const hoveredPaths = async () => ((await dump(preview.windowId))?.structure?.nodes ?? []).filter((n) => n?.state?.hovered === true).map((n) => n.path);
    const away = [preview.point[0], Math.max(6, preview.point[1] - 320)];
    const hover = { idle: await hoveredPaths(), traceBefore: (world3dTraces()[preview.path] ?? null) };
    const hoverMark = lines.length;
    await page.mouse.move(preview.point[0], preview.point[1]);
    await pump(3000, preview.point);
    hover.onSurface = await hoveredPaths();
    hover.hoverLines = lines.slice(hoverMark).filter((line) => line.toLowerCase().includes("hover")).slice(0, 8);
    hover.world3dLines = lines.slice(hoverMark).filter((line) => line.includes("world3d surface=")).length;
    hover.pointerHits = hitLines(hoverMark).slice(-3);
    await page.mouse.move(away[0], away[1]);
    await pump(2500, away);
    hover.away = await hoveredPaths();
    const hoverReached = hover.pointerHits.some((hit) => hit.id && (hit.kind === "World3d" || hit.id.includes(preview.path.split("/").at(-1) ?? "%%")));
    results.push({ label: "world3d:hover", verdict: hover.world3dLines > 0 && hover.pointerHits.length > 0 ? (hoverReached || hover.onSurface.length > hover.idle.length ? "pass" : "partial") : "fail", t: at(), ...hover, surface: preview.path, point: preview.point });
    note(`hover: pointerHits=${JSON.stringify(hover.pointerHits)} world3dLines=${hover.world3dLines} onSurface=${JSON.stringify(hover.onSurface)}`);

    const selectMark = lines.length;
    const cameraBeforeClick = cameraOf();
    const clicked = await clickWitnessed(preview.point, "world3d selection click");
    await pump(4000, preview.point);
    const selectionLines = lines.slice(selectMark).filter((line) => /interactionSelect|selection|selected/i.test(line)).slice(0, 10);
    results.push({ label: "world3d:selection-click", verdict: clicked.delivered > 0 && selectionLines.length > 0 ? "pass" : clicked.delivered > 0 ? "fail (delivered, no selection verb)" : "fail (no pointer delivered)", t: at(), clicked, selectionLines, cameraBeforeClick });

    const wheelMark = lines.length;
    const cameraBefore = cameraOf();
    const renderBefore = has("wgpu-shell render begin").length;
    await page.mouse.move(preview.point[0], preview.point[1]);
    await page.waitForTimeout(400);
    for (let tick = 0; tick < 5; tick += 1) {
      await page.mouse.wheel(0, 150);
      await page.waitForTimeout(500);
      await page.mouse.move(preview.point[0] + (tick % 2), preview.point[1]);
    }
    await pump(4000, preview.point);
    const cameraAfter = cameraOf();
    const wheelWitness = await witness();
    results.push({
      label: "world3d:wheel-orbit",
      verdict: cameraBefore && cameraAfter && cameraBefore !== cameraAfter ? "pass" : (wheelWitness?.wheels ?? 0) === 0 ? "fail (no wheel delivered)" : "fail (camera unchanged)",
      t: at(),
      cameraBefore,
      cameraAfter,
      wheelsDelivered: wheelWitness?.wheels ?? null,
      renderBeginDelta: has("wgpu-shell render begin").length - renderBefore,
      setCameraLines: lines.slice(wheelMark).filter((line) => line.includes("setCamera")).slice(0, 6),
    });

    const cancelSweep = await sweep(
      (() => {
        const points = [];
        const plan = dockPlan()[preview.windowId];
        if (!plan) return [[preview.point[0], preview.point[1]]];
        for (let y = plan.y + 6; y < plan.y + Math.min(plan.h, 160); y += 12) for (let x = plan.x + 6; x < plan.x + plan.w - 6; x += 26) points.push([x, y]);
        return points.slice(0, 320);
      })(),
      "world3d cancel affordance",
    );
    const cancel = Object.entries(cancelSweep.controls).find(([id]) => id.startsWith("shell.world3d.cancel"));
    if (cancel) {
      const clickedCancel = await clickWitnessed(cancel[1].point, "world3d cancel control");
      await pump(2500);
      results.push({ label: "world3d:cancel", verdict: lines.filter((line) => line.includes("shell world3d cancel")).length > 0 ? "pass" : "fail", t: at(), control: cancel[0], clicked: clickedCancel, cancelLines: has("shell world3d cancel").slice(-4) });
    } else {
      const statuses = Object.entries(world3dTraces()).map(([id, t]) => [id, (t.status ?? "").slice(0, 160)]);
      results.push({ label: "world3d:cancel", verdict: "not-observed", t: at(), reason: "no `shell.world3d.cancel::*` hit target while sampling; the surface published no cancellable status during the probe", statuses });
    }
  }
}
//#endregion 🎬️Journey

await dropOverlay();
await page.screenshot({ path: join(outDir, "zz-final.png") }).catch(() => {});
journey.finishedAt = new Date().toISOString();
journey.seconds = Math.round(at() / 1000);
journey.results = results;
journey.consoleCensus = {
  renderBegin: has("wgpu-shell render begin").length,
  renderLeave: has("render leave").length,
  dockPlan: has("wgpu-shell dock plan").length,
  world3dSurface: has("world3d surface=").length,
  pointerHits: has("os_host pointer hit").length,
  dispatchNormalized: has("dispatch_normalized_event").length,
  faults: has("faulted").length + has("panicked").length,
  capacity: has("Capacity").length,
};
journey.failureLines = lines.filter((line) => line.includes("pageerror") || line.includes("panicked") || line.includes("surface fault") || line.includes("faulted")).slice(-20);
writeFileSync(join(outDir, "results.json"), JSON.stringify(journey, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE", JSON.stringify({ seconds: journey.seconds, census: journey.consoleCensus, verdicts: results.map((r) => [r.label, r.verdict]) }, null, 2));
await browser.close();
