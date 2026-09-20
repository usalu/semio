/** ⚖️ Renderer behavioural-parity probe — ONE user journey, driven identically on the React host and the
 * wgpu host, producing a per-step action/outcome log, a structure delta and a screenshot per renderer, so
 * "the wgpu chrome behaves like React's" is MEASURED instead of eyeballed.
 *
 * 🪪️ The two renderers are addressed through the same vocabulary: a step names a CONTROL KEY
 * (`framework.panel.artifact`, `ui.introduction.skip`, `framework.window.<Window>.engagement.toggle`, …).
 *   - React publishes that key as an element `id`, so the driver resolves `[id="<key>"]` and clicks it.
 *   - wgpu publishes no DOM at all. It publishes the SAME keys through its chrome hit registry
 *     (`semioWgpuIntrospection.dumpChrome()` — added by this packet in
 *     `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, region `🎯️ChromeLedger`), one row per registered pointer
 *     target with its ABSOLUTE page rect, so the driver aims a real pointer at the rect's centre.
 *     A key some chrome mints with a prefix (wgpu's panel tabs are `shell.panel.tab.<anchor>.<key>`) is
 *     resolved by exact id, then by suffix, then by containment — and WHICH of the three matched is
 *     recorded, because an id that only matches loosely is itself a parity finding.
 * 🎬️ Both renderers publish a dispatched-action log in the same shape, which is what a step is judged on:
 *   - React: `globalThis.__semioInputLedger.recent` (the Input Causality Ledger, dev builds only).
 *   - wgpu: `dumpChrome().actions` (every action crossing the shell's one `dispatch_action` funnel).
 * Both are armed by `SEMIO_RUNTIME_DIAGNOSTICS`, which this probe writes into `localStorage` before load.
 *
 * 📤️ Writes `🗑️generated/<out>/steps.json` (per step, per renderer: dispatched actions, structure delta,
 * screenshot path, errors) and `🗑️generated/<out>/parity.md`, a table naming every step where the two
 * renderers disagree. Both are rewritten after EVERY step, so a run that dies half-way still leaves its
 * evidence. A renderer that is unreachable is recorded as skipped, never faked — so the React half can be
 * proven on its own and the wgpu half added after the coordinator's rebuild.
 *
 * Usage:
 *   cd <ticket> && SEMIO_PROBE_OUT=w5b-run-1 SEMIO_PROBE_TARGETS=react,wgpu \
 *     SEMIO_PROBE_REACT_URL=http://127.0.0.1:6313/?plugin=puzzle3d \
 *     SEMIO_PROBE_WGPU_URL=http://127.0.0.1:6213/?plugin=puzzle3d bun 🐍️parity-interact-probe.mjs
 * Env: SEMIO_PROBE_TARGETS (react,wgpu) · SEMIO_PROBE_OUT · SEMIO_PROBE_BOOT (s, default 240) ·
 *      SEMIO_PROBE_SETTLE (ms per step, default 1600) · SEMIO_PROBE_HEADED=1 · SEMIO_PROBE_ONLY=<step,step> ·
 *      SEMIO_PROBE_APPEARANCE (default dark) · SEMIO_PROBE_LANGUAGE (default de)
 */
import { chromium } from "playwright";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region ⚙️Configuration
const targets = (process.env.SEMIO_PROBE_TARGETS ?? "react,wgpu").split(",").map((name) => name.trim()).filter(Boolean);
const urls = { react: process.env.SEMIO_PROBE_REACT_URL ?? "http://127.0.0.1:6313/?plugin=puzzle3d", wgpu: process.env.SEMIO_PROBE_WGPU_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d" };
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w5b-parity");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 240);
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE ?? 1600);
const diagnosticsEnabled = process.env.SEMIO_PROBE_DIAGNOSTICS !== "0";
const onlySteps = (process.env.SEMIO_PROBE_ONLY ?? "").split(",").map((name) => name.trim()).filter(Boolean);
const viewport = { width: 1600, height: 1000 };
const mod = process.platform === "darwin" ? "Meta" : "Control";
const fullscreenChord = process.platform === "darwin" ? "Control+Meta+f" : "F11";
const settingsAppearanceProbeValue = process.env.SEMIO_PROBE_APPEARANCE ?? "dark";
const settingsLanguageProbeValue = process.env.SEMIO_PROBE_LANGUAGE ?? "de";
mkdirSync(outDir, { recursive: true });

/** 🪟️ The pane-chip keys a window publishes, matched as an id SUFFIX so the window instance in the middle
 * (`framework.window.<Window>.<chip>`) never has to be named by the journey. */
const PANE_CHIPS = ["engagement.toggle", "search.toggle", "utilityBar.unfold", "pane.fold", "measures.unfold"];
const GEOMETRY_CONTROLS = ["framework.panel.artifact", "framework.panel.catalogue", "framework.panel.inspection", "framework.panel.toolRun", "framework.chat", "ui.fullscreen.toggle", "playground.navbar.fixture", "playground.navbar.roles.editor", "playground.navbar.roles.viewer", "framework.category.display", "s-sync-status", "s-presence-peers", "framework.hub.signIn", "framework.settings", "framework.marketplace", "framework.panel.history", "os.task-manager", "framework.category.tool", "framework.category.command"];
const NONINTERACTIVE_GEOMETRY_LABELS = new Set(["s-presence-peers"]);

/** 🪪️ Control keys the two renderers genuinely spell differently. An alias is tried only after the exact /
 * suffix / containment ladder has failed, and the step records `resolved: "alias"` — so a divergent id shows
 * up in `parity.md` as a control-resolution difference instead of silently passing as "the same control".
 *
 * 🈳️ Deliberately EMPTY. It carried the tour chips (`shell.tour.*` against React's `ui.introduction.*`) and
 * the wgpu shell has since adopted React's spelling for those and for the five pane chips
 * (`framework.window.<segment>.…`, 📓️w9c-behaviour-parity-run-2.md). An alias here hides exactly the defect
 * this probe exists to measure, so a new entry needs a reason no fix can reach. */
const CONTROL_ALIASES = {};
//#endregion ⚙️Configuration

//#region 🧾️Report
const report = { startedAt: new Date().toISOString(), diagnosticsEnabled, cpuProfilesEnabled: process.env.SEMIO_PROBE_CPU_PROFILE === "1", urls, targets, steps: [] };
const flush = () => {
  writeFileSync(join(outDir, "steps.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "parity.md"), parityMarkdown());
  writeFileSync(join(outDir, "geometry.md"), geometryMarkdown());
  writeFileSync(join(outDir, "physical.md"), physicalMarkdown());
  writeFileSync(join(outDir, "cameras.md"), cameraMarkdown());
  writeFileSync(join(outDir, "latency.md"), latencyMarkdown());
};

function controlGeometry(state) {
  return Object.fromEntries(GEOMETRY_CONTROLS.map(key => {
    const { row, resolved } = resolveByLadder(state.controls ?? [], key, row => row.id ?? "");
    return [key, row ? { id: row.id, rect: row.rect, resolved } : null];
  }));
}

function geometryMarkdown() {
  const rows = [];
  for (const step of report.steps) {
    const react = step.renderers.react?.geometry;
    const wgpu = step.renderers.wgpu?.geometry;
    if (!react || !wgpu) continue;
    for (const key of GEOMETRY_CONTROLS) {
      const a = react[key]?.rect;
      const b = wgpu[key]?.rect;
      if (!a && !b) continue;
      const error = a && b ? Math.max(...a.map((value, i) => Math.abs(value - b[i]))) : null;
      rows.push(`| ${step.name} | ${key} | ${a?.join(", ") ?? "absent"} | ${b?.join(", ") ?? "absent"} | ${error === null ? "missing" : error <= 1 ? "within 1 px" : `${error.toFixed(1)} px`} |`);
    }
  }
  return ["# Chrome Geometry Comparison", "", `Viewport ${viewport.width} × ${viewport.height}, DPR 1. Rectangles are x, y, width, height in CSS pixels. Boot and dismiss-tour geometry is gated in physical.md at one CSS pixel, allowing subpixel rounding only.`, "", "| Step | Control | React | WGPU | Maximum Difference |", "| --- | --- | --- | --- | --- |", ...rows, ""].join("\n");
}

/** 📏️ Compare paired mounted identities and bounds independently of action-journal differences. */
function physicalDifferences(step) {
  const a = step.renderers.react;
  const b = step.renderers.wgpu;
  if (!a || !b || a.error || b.error || a.blockedByCrash || b.blockedByCrash) return null;
  const failures = [];
  let checks = 0;
  const rects = (label, left, right) => {
    checks += 1;
    if (!left || !right || left.length !== 4 || right.length !== 4 || !left.every(Number.isFinite) || !right.every(Number.isFinite)) failures.push(`${label}: missing finite rectangle`);
    else {
      const difference = Math.max(...left.map((value, index) => Math.abs(value - right[index])));
      if (difference > 1) failures.push(`${label}: ${difference.toFixed(3)} CSS px exceeds 1 CSS px`);
    }
  };
  if (["boot", "dismiss-tour"].includes(step.name)) {
    for (const key of GEOMETRY_CONTROLS) {
      const left = a.geometry?.[key]?.rect;
      const right = b.geometry?.[key]?.rect;
      if (NONINTERACTIVE_GEOMETRY_LABELS.has(key) && (!left || !right)) continue;
      if (left || right) rects(key, left, right);
    }
  }
  if (step.name !== "window-reopen" && (a.detail?.windowId || b.detail?.windowId)) {
    checks += 1;
    if (a.detail?.windowId !== b.detail?.windowId) failures.push(`target window: ${a.detail?.windowId} versus ${b.detail?.windowId}`);
  }
  for (const phase of ["beforeBodies", "afterBodies"]) {
    if (!a.detail?.[phase] && !b.detail?.[phase]) continue;
    const left = a.detail?.[phase] ?? {};
    const right = b.detail?.[phase] ?? {};
    for (const id of new Set([...Object.keys(left), ...Object.keys(right)])) rects(`${phase}/${id}`, left[id], right[id]);
  }
  for (const key of ["cap", "opening", "required", "visibleChildren", "targetKey", "dispatched", "restored", "physicalOutcome", "closed", "remaining", "beforeTabs", "afterTabs", "liveSceneBody"]) {
    if (a.detail?.[key] !== undefined || b.detail?.[key] !== undefined) {
      checks += 1;
      if (JSON.stringify(a.detail?.[key]) !== JSON.stringify(b.detail?.[key])) failures.push(`${key}: ${JSON.stringify(a.detail?.[key])} versus ${JSON.stringify(b.detail?.[key])}`);
    }
  }
  return checks ? failures : null;
}

function physicalMarkdown() {
  return ["# Physical Geometry and Target Acceptance", "", "Same 1600 × 1000 viewport at DPR 1; one CSS pixel admits subpixel layout rounding. Seeded window identities and their before/after bounds must agree. A new window uses a renderer-assigned identity, so its creation count and live scene body are compared. Failed or unpaired operations remain unmeasured. Static presence text has no WGPU hit target: its bounds remain unmeasured by this hit-based gate and require a text/pixel receipt; absence from hits does not establish missing paint.", "", "| Step | Verdict | Difference |", "| --- | --- | --- |", ...report.steps.map(step => {
    const failures = physicalDifferences(step);
    return `| ${step.name} | ${failures === null ? "unmeasured" : failures.length ? "differ" : "match"} | ${failures?.join("; ") ?? "no successful paired geometry or target receipt"} |`;
  }), ""].join("\n");
}

function cameraMarkdown() {
  const rows = [];
  const difference = (a, b) => Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every(Number.isFinite) && b.every(Number.isFinite) ? Math.max(...a.map((value, i) => Math.abs(value - b[i]))).toFixed(5) : "unavailable";
  const scalar = (a, b) => Number.isFinite(a) && Number.isFinite(b) ? Math.abs(a - b).toFixed(5) : "unavailable";
  const projection = camera => typeof camera?.projection === "string" ? camera.projection : camera?.projection?.mode?.kind;
  for (const step of report.steps) {
    const react = step.renderers.react?.diagnostics?.worlds ?? [];
    const diagnostics = step.renderers.wgpu?.diagnostics;
    const visible = new Set((diagnostics?.chrome?.surfaces ?? []).filter(row => row.level === "window").map(row => row.id));
    const wgpu = (diagnostics?.meshes?.surfaces ?? []).filter(row => visible.has(row.surfaceId));
    for (const windowId of new Set([...react.map(row => row.windowId), ...wgpu.map(row => row.surfaceId)])) {
      const a = react.find(row => row.windowId === windowId);
      const b = wgpu.find(row => row.surfaceId === windowId);
      const x = a?.liveCamera;
      const y = b?.liveCamera;
      rows.push(`| ${step.name} | ${windowId} | ${a && b ? "paired identity" : a ? "React only" : "WGPU only"} | ${projection(x) ?? "absent"} / ${projection(y) ?? "absent"} | ${difference(x?.position, y?.position)} | ${difference(x?.target, y?.target)} | ${difference(x?.up, y?.up)} | ${scalar(x?.zoom, y?.zoom)} | ${scalar(x?.fov, y?.fov)} | ${difference(a?.rect?.slice(2), b?.rect?.slice(2))} |`);
    }
  }
  return ["# Live Camera Comparison", "", "Values are absolute differences in the published live camera, paired only by the same window identity. Position, target, and up report the maximum component difference. Viewport size compares width/height in CSS pixels; WGPU local origins are not compared with React page origins. Only currently visible WGPU windows are admitted, so retired mesh diagnostics cannot substitute for a live scene. Generated identities that differ remain unpaired. Inspect the delivered seeds and interaction history in steps.json before attributing a pose difference to camera fitting.", "", "| Step | Window | Pairing | Projection React / WGPU | Position | Target | Up | Zoom | FOV | Viewport Size |", "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |", ...rows, ""].join("\n");
}

function latencyMarkdown() {
  const rows = [];
  let previous = new Map();
  let previousStep = "boot origin";
  for (const step of report.steps) {
    const stats = step.renderers.wgpu?.diagnostics?.frame?.frameLatency;
    if (!Array.isArray(stats?.stageTotals)) continue;
    const next = new Map();
    for (const stage of stats.stageTotals) {
      const key = `${stage.authorityDomain}/${stage.stage}`;
      next.set(key, stage);
      const before = previous.get(key);
      const count = stage.observationCount - (before?.observationCount ?? 0);
      const duration = stage.totalDurationUs - (before?.totalDurationUs ?? 0);
      const work = stage.totalWorkItems - (before?.totalWorkItems ?? 0);
      if (count < 0 || duration < 0 || work < 0) {
        rows.push(`| ${previousStep} → ${step.name} | ${key} | counter reset | — | — | — |`);
      } else if (count) {
        rows.push(`| ${previousStep} → ${step.name} | ${key} | ${count} | ${(duration / 1000).toFixed(3)} | ${(stage.maxDurationUs / 1000).toFixed(3)} | ${work} |`);
      }
    }
    previous = next;
    previousStep = step.name;
  }
  return ["# WGPU Measured Phase Totals", "", "Differences between successive WGPU diagnostic snapshots use explicit renderer-frame or browser-input-batch domains. They include background work and the intervening React operation. Nested scopes overlap, so stage durations must not be summed into elapsed time. The maximum is the lifetime maximum at that snapshot, not a per-gesture latency or GPU duration. Full recent phase envelopes, generation identities, summary evictions, and refusals remain in steps.json.", "", ...(rows.length ? ["| Snapshot Interval | Authority / Stage | Observations | Scope Total Delta (ms) | Lifetime Maximum (ms) | Work Delta |", "| --- | --- | --- | --- | --- | --- |", ...rows] : ["No aggregated stage totals were published. Older scalar-ring or disabled diagnostics cannot establish these phase totals."]), ""].join("\n");
}

/** ⚖️ Two renderers agree on a step when the same SET of action ids ran and the same surfaces moved in and
 * out. Deliberately a set, not a multiset: a guest lane that re-registers brush meshes once per settled
 * frame fired 61 times on one React step and 3 on the next for the same user gesture, so counts measure the
 * frame clock, not behaviour. Rects and timings stay out for the same reason — two layout engines. */
const actionIds = (side) => [...new Set((side?.actions ?? []).map((entry) => entry.action))].sort();
/** 🪪️ The controller is RECORDED but never compared: React names an instance-scoped controller
 * (`s.puzzle.puzzle3d@1/*#editor`) where the wgpu shell names its own host controller, and the difference is
 * an identity, not a behaviour. The bare verb is what both renderers must agree on. */
const actionCounts = (rows) => {
  const counts = {};
  for (const row of rows) counts[row.action] = (counts[row.action] ?? 0) + 1;
  return counts;
};
const actionControllers = (rows) => [...new Set(rows.map((row) => row.controller))].filter(Boolean);
const setText = (values) => (values?.length ? values.slice(0, 8).join(" ") : "—");
const sameList = (a, b) => a.length === b.length && a.every((value, index) => value === b[index]);
const observedChange = (side) => actionIds(side).length > 0 || ["surfacesAdded", "surfacesRemoved", "controlsAdded", "controlsRemoved"].some((key) => side.delta?.[key]?.length > 0) || side.delta?.fullscreenChanged === true;
function stepVerdict(step) {
  const react = step.renderers.react;
  const wgpu = step.renderers.wgpu;
  if (react?.blockedByCrash || wgpu?.blockedByCrash) return { verdict: "unmeasured", why: `not run after renderer crash: ${react?.blockedByCrash ?? wgpu?.blockedByCrash}` };
  if (!react || !wgpu) return { verdict: "unmeasured", why: `only ${react ? "react" : wgpu ? "wgpu" : "no renderer"} ran` };
  if (react.error || wgpu.error) return { verdict: "differ", why: `error react=${react.error ?? "-"} wgpu=${wgpu.error ?? "-"}` };
  if ((react.detail?.appliedSettings || wgpu.detail?.appliedSettings) && JSON.stringify(react.detail?.appliedSettings) !== JSON.stringify(wgpu.detail?.appliedSettings)) return { verdict: "differ", why: `applied settings react=${JSON.stringify(react.detail?.appliedSettings)} wgpu=${JSON.stringify(wgpu.detail?.appliedSettings)}` };
  if (!diagnosticsEnabled) return { verdict: "unmeasured", why: "diagnostics disabled: timing/outcome observation only" };
  if ((react.detail?.activated || wgpu.detail?.activated) && (react.detail?.activated?.commandId !== wgpu.detail?.activated?.commandId || react.detail?.activated?.label !== wgpu.detail?.activated?.label)) return { verdict: "differ", why: `activated command react=${JSON.stringify(react.detail?.activated)} wgpu=${JSON.stringify(wgpu.detail?.activated)}` };
  if (react.resolved === "absent" && wgpu.resolved === "absent") return { verdict: "unmeasured", why: "control absent in both renderers" };
  if (react.resolved !== wgpu.resolved) return { verdict: "differ", why: `control resolution react=${react.resolved} wgpu=${wgpu.resolved}` };
  if (!sameList(actionIds(react), actionIds(wgpu))) return { verdict: "differ", why: `actions react=[${setText(actionIds(react))}] wgpu=[${setText(actionIds(wgpu))}]` };
  if (!sameList(react.delta?.surfacesAdded ?? [], wgpu.delta?.surfacesAdded ?? []) || !sameList(react.delta?.surfacesRemoved ?? [], wgpu.delta?.surfacesRemoved ?? [])) {
    return { verdict: "differ", why: `surfaces react=+[${setText(react.delta?.surfacesAdded)}]-[${setText(react.delta?.surfacesRemoved)}] wgpu=+[${setText(wgpu.delta?.surfacesAdded)}]-[${setText(wgpu.delta?.surfacesRemoved)}]` };
  }
  if (step.name !== "boot" && !observedChange(react) && !observedChange(wgpu)) return { verdict: "unmeasured", why: "neither renderer published an observable action or UI transition" };
  if (observedChange(react) !== observedChange(wgpu)) return { verdict: "differ", why: "only one renderer published an observable action or UI transition" };
  if (react.delta?.fullscreenChanged !== wgpu.delta?.fullscreenChanged) return { verdict: "differ", why: "fullscreen transition differs" };
  return { verdict: "match", why: "" };
}

function parityMarkdown() {
  const rows = report.steps.map((step) => {
    const { verdict, why } = stepVerdict(step);
    const cell = (side) => (side ? `${actionIds(side).length ? setText(actionIds(side)) : "(none)"}${side.error ? ` ⚠️ ${side.error}` : ""}` : "(not run)");
    return `| \`${step.name}\` | ${verdict} | ${cell(step.renderers.react)} | ${cell(step.renderers.wgpu)} | ${why || ""} |`;
  });
  const differ = report.steps.filter((step) => stepVerdict(step).verdict === "differ");
  const measured = report.steps.filter((step) => stepVerdict(step).verdict !== "unmeasured");
  return [
    "# ⚖️ Renderer interaction parity",
    "",
    `React \`${urls.react}\` · wgpu \`${urls.wgpu}\` · targets \`${targets.join(",")}\` · ${report.startedAt}`,
    "",
    `Runtime diagnostics: ${diagnosticsEnabled ? "enabled" : "disabled; action-ledger equality is not an acceptance gate"}.`,
    "",
    `**${differ.length} of ${measured.length} measured steps differ** (${report.steps.length - measured.length} unmeasured). A step compares dispatched action sets, surface changes and fullscreen transitions. An interaction with no observed change on either renderer is unmeasured; geometry is reported separately.`,
    "",
    "| step | verdict | react actions | wgpu actions | why |",
    "| --- | --- | --- | --- | --- |",
    ...rows,
    "",
  ].join("\n");
}
//#endregion 🧾️Report

//#region 🌐️Drivers
/** 🔬️ Optional real worker CPU samples; disabled in ordinary interaction receipts. */
async function startWorkerProfiles(page, name, record) {
  if (process.env.SEMIO_PROBE_CPU_PROFILE !== "1" || name !== "wgpu") return null;
  const session = await page.context().newCDPSession(page);
  const pending = new Map();
  const workers = [];
  let nextId = 0;
  session.on("Target.receivedMessageFromTarget", event => {
    const message = JSON.parse(event.message);
    const key = `${event.sessionId}:${message.id}`;
    const command = pending.get(key);
    if (!command) return;
    pending.delete(key);
    clearTimeout(command.timer);
    message.error ? command.reject(new Error(message.error.message)) : command.resolve(message.result);
  });
  const send = (worker, method, params = {}) => new Promise((resolve, reject) => {
    const id = ++nextId;
    const key = `${worker}:${id}`;
    const timer = setTimeout(() => { pending.delete(key); reject(new Error(`CPU profile command timed out: ${method}`)); }, 10000);
    pending.set(key, { resolve, reject, timer });
    session.send("Target.sendMessageToTarget", { sessionId: worker, message: JSON.stringify({ id, method, params }) }).catch(error => {
      clearTimeout(timer);
      pending.delete(key);
      reject(error);
    });
  });
  session.on("Target.attachedToTarget", event => {
    const worker = { id: event.sessionId, url: event.targetInfo.url, started: false };
    workers.push(worker);
    worker.ready = (async () => {
      try {
        await send(worker.id, "Profiler.enable");
        await send(worker.id, "Profiler.setSamplingInterval", { interval: 1000 });
        await send(worker.id, "Profiler.start");
        worker.started = true;
      } catch (error) { record("profile", "error", String(error)); }
      finally { await send(worker.id, "Runtime.runIfWaitingForDebugger").catch(error => record("profile", "error", String(error))); }
    })();
  });
  await session.send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: true, flatten: false, filter: [{ type: "worker", exclude: false }, { exclude: true }] });
  return async () => {
    for (const [index, worker] of workers.entries()) {
      await worker.ready;
      if (!worker.started) continue;
      try {
        const { profile } = await send(worker.id, "Profiler.stop");
        const filename = `cpu-worker-${index + 1}.json`;
        writeFileSync(join(outDir, name, filename), JSON.stringify({ url: worker.url, profile }));
        const nodes = new Map(profile.nodes.map(node => [node.id, node]));
        const parents = new Map(profile.nodes.flatMap(node => (node.children ?? []).map(id => [id, node.id])));
        const totals = new Map();
        for (const [sampleIndex, id] of (profile.samples ?? []).entries()) {
          const duration = profile.timeDeltas?.[sampleIndex] ?? 0;
          let cursor = id;
          const seen = new Set();
          while (cursor !== undefined && !seen.has(cursor)) {
            seen.add(cursor);
            const node = nodes.get(cursor);
            if (!node) break;
            const key = `${node.callFrame.functionName || "(anonymous)"} ${node.callFrame.url}:${node.callFrame.lineNumber + 1}`;
            const row = totals.get(key) ?? { self: 0, inclusive: 0 };
            row.inclusive += duration;
            if (cursor === id) row.self += duration;
            totals.set(key, row);
            cursor = parents.get(cursor);
          }
        }
        const rows = [...totals].sort((a, b) => b[1].self - a[1].self).slice(0, 40).map(([key, row]) => `| ${key.replaceAll("|", "\\|")} | ${(row.self / 1000).toFixed(3)} | ${(row.inclusive / 1000).toFixed(3)} |`);
        writeFileSync(join(outDir, name, `cpu-worker-${index + 1}.md`), ["# Worker CPU Profile", "", `Actual Chrome CPU samples for ${worker.url}. Sampling interval1000µs. Profile collection adds overhead; this is attribution evidence, not an isolated latency benchmark. Inclusive durations overlap. Raw protocol profile: ${filename}.`, "", "| Function | Self Sampled Time (ms) | Inclusive Sampled Time (ms) |", "| --- | --- | --- |", ...rows, ""].join("\n"));
        record("profile", "complete", `worker=${index + 1} samples=${profile.samples?.length ?? 0}`);
      } catch (error) { record("profile", "error", String(error)); }
    }
    await session.send("Target.setAutoAttach", { autoAttach: false, waitForDebuggerOnStart: false }).catch(() => {});
    await session.detach();
  };
}

/** 🌐️ One renderer under the probe. `observe`/`actions`/`resolve` are the three seams the two targets
 * implement differently; everything else (stepping, deltas, screenshots) is shared above them. */
async function openDriver(browser, name, url) {
  const consoleLines = [];
  let fatalError = null;
  const page = await browser.newPage({ viewport, deviceScaleFactor: 1 });
  const t0 = Date.now();
  const record = (source, type, text) => {
    consoleLines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, 4000)}`);
    if (/wgpu-worker panicked:|wgpu renderer fault:|RuntimeError: unreachable|browser page crashed/.test(text)) fatalError ??= text.slice(0, 600);
  };
  page.on("console", (message) => record("page", message.type(), message.text()));
  page.on("pageerror", (error) => record("page", "pageerror", String(error)));
  page.on("crash", () => record("page", "crash", "browser page crashed"));
  page.on("worker", (worker) => worker.on("console", (message) => record("worker", message.type(), message.text())));
  const stopProfiles = await startWorkerProfiles(page, name, record);
  await page.addInitScript((enabled) => {
    try {
      globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", enabled ? "1" : "0");
    } catch {}
  }, diagnosticsEnabled);
  await page.goto(url, { waitUntil: "domcontentloaded" });
  const driver = name === "react" ? reactDriver(page, name, url, consoleLines) : wgpuDriver(page, name, url, consoleLines);
  driver.stopProfiles = stopProfiles;
  Object.defineProperty(driver, "fatalError", { get: () => fatalError });
  return driver;
}

const centre = (rect) => [rect[0] + rect[2] / 2, rect[1] + rect[3] / 2];

/** 🪪️ The shared resolution ladder: exact key, then `…​.<key>` suffix, then containment. The MATCH MODE is
 * reported, because a control only one renderer names exactly is a parity finding in itself. */
function resolveByLadder(rows, key, idOf) {
  for (const candidate of [key, ...(CONTROL_ALIASES[key] ?? [])]) {
    const exact = rows.find((row) => idOf(row) === candidate);
    if (exact) return { row: exact, resolved: candidate === key ? "exact" : "alias", matched: idOf(exact) };
    const suffix = rows.find((row) => idOf(row).endsWith(`.${candidate}`));
    if (suffix) return { row: suffix, resolved: candidate === key ? "suffix" : "alias", matched: idOf(suffix) };
    const contains = rows.find((row) => idOf(row).includes(candidate));
    if (contains) return { row: contains, resolved: candidate === key ? "contains" : "alias", matched: idOf(contains) };
  }
  return { row: null, resolved: "absent", matched: null };
}

function reactDriver(page, name, url, consoleLines) {
  const observe = () =>
    page
      .evaluate(() => {
        const rectOf = (element) => {
          const rect = element.getBoundingClientRect();
          return [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)];
        };
        const seen = new Map();
        const controls = [...document.querySelectorAll('[id], button, [role="tab"], [role="option"], [data-slot]')]
          .filter((element) => element.id || element.tagName === "BUTTON" || element.getAttribute("role") === "tab" || element.getAttribute("role") === "option" || element.hasAttribute("data-slot"))
          .map((element) => {
            const slot = element.getAttribute("data-slot");
            let id = element.id;
            if (!id) {
              const base = slot ?? element.getAttribute("role") ?? element.tagName.toLowerCase();
              const index = (seen.get(base) ?? 0) + 1;
              seen.set(base, index);
              id = index === 1 ? base : `${base}#${index}`;
            }
            return { id, kind: slot ?? element.getAttribute("role") ?? element.tagName.toLowerCase(), rect: rectOf(element), windowId: element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? undefined, pressed: element.getAttribute("aria-pressed") ?? element.getAttribute("data-state") ?? undefined };
          })
          .filter((control) => control.rect[2] > 0 && control.rect[3] > 0);
        // 🪟️ The shell's THREE surface levels, named the same way on both renderers: a window
        // instance, a docked panel (by the tab it shows) and "a dialog is up". Deliberately not
        // "every element carrying data-level": React puts that attribute on the chat panel's own
        // Send and Clear buttons and renders the tour as two dialog nodes, one of them anonymous, so
        // the raw DOM census measured React's composition rather than the shell's state and could
        // never be met by a canvas renderer. `data-surface-id` is dropped for the same reason — it
        // restates `window:<id>` as `surface:window:<id>` (📓️w9c-behaviour-parity-run-2.md).
        const panelTabPrefix = "framework.panelTab.";
        const surfaces = [
          ...[...document.querySelectorAll("[data-window-id]")].map((element) => `window:${element.getAttribute("data-window-id")}`),
          ...[...document.querySelectorAll('[data-level="panel"]')].filter((element) => element.id.startsWith(panelTabPrefix)).map((element) => `panel:${element.id.slice(panelTabPrefix.length)}`),
          ...(document.querySelector('[data-level="dialog"]') ? ["dialog"] : []),
        ];
        return {
          ready: document.documentElement.getAttribute("data-semio-os-ready"),
          error: document.documentElement.getAttribute("data-semio-os-error"),
          controls,
          surfaces: [...new Set(surfaces)].sort(),
          role: document.querySelector("[data-role]")?.getAttribute("data-role") ?? null,
          example: document.querySelector('[data-slot="select-value"]')?.textContent?.replace(/\s+/g, " ").trim() ?? null,
          introduction: document.documentElement.querySelector("[data-introduction-active]") !== null || document.querySelector("[data-introduction-active]") !== null,
          selected: [...document.querySelectorAll('[aria-selected="true"]')].map((element) => element.id || element.getAttribute("data-tab-id") || (element.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40)),
          canvases: document.querySelectorAll("canvas").length,
          fullscreen: document.fullscreenElement !== null,
        };
      })
      .catch((error) => ({ error: String(error).slice(0, 300), controls: [], surfaces: [] }));

  const actions = () =>
    page
      .evaluate(() => (globalThis.__semioInputLedger?.recent ?? []).map((row) => ({ seq: row.inputSeq, controller: row.controllerId ?? "?", action: row.action, windowId: row.windowId, origin: row.origin, outcome: row.outcome?.kind ?? "open", reason: row.outcome?.reason })))
      .catch(() => []);

  return {
    name,
    url,
    page,
    consoleLines,
    observe,
    actions,
    async diagnostics() {
      return page.evaluate(keys => ({ worlds: [...document.querySelectorAll("[data-viewport-camera-json]")].map(element => ({
        surfaceId: element.getAttribute("data-surface-id"),
        windowId: element.getAttribute("data-window-instance-id"),
        rect: (() => { const r = element.getBoundingClientRect(); return [r.x, r.y, r.width, r.height]; })(),
        camera: JSON.parse(element.getAttribute("data-camera-json") || "null"),
        liveCamera: JSON.parse(element.getAttribute("data-viewport-camera-json") || "null"),
        sun: JSON.parse(element.getAttribute("data-sun-json") || "null"),
        meshCount: JSON.parse(element.getAttribute("data-meshes-json") || "[]").length,
        instanceCount: JSON.parse(element.getAttribute("data-instances-json") || "[]").length,
      })), chromeStyle: keys.flatMap(key => [...document.querySelectorAll("[id]")].filter(element => element.id === key && element.getBoundingClientRect().width > 0).map(element => {
        const style = getComputedStyle(element);
        const rect = element.getBoundingClientRect();
        return { key, text: element.textContent?.trim(), rect: [rect.x, rect.y, rect.width, rect.height], font: style.font, fontFamily: style.fontFamily, fontSize: style.fontSize, fontWeight: style.fontWeight, letterSpacing: style.letterSpacing, lineHeight: style.lineHeight, gap: style.gap, padding: style.padding, borderWidth: style.borderWidth, icons: [...element.querySelectorAll("svg")].map(icon => { const r = icon.getBoundingClientRect(); return [r.x, r.y, r.width, r.height]; }) };
      })), treeStyle: [...document.querySelectorAll('[data-slot="tree-item-row"]')].filter(element => element.getBoundingClientRect().width > 0).slice(0, 8).map(element => {
        const style = getComputedStyle(element);
        const rect = element.getBoundingClientRect();
        return { id: element.id, rect: [rect.x, rect.y, rect.width, rect.height], font: style.font, lineHeight: style.lineHeight, padding: style.padding, minHeight: style.minHeight, height: style.height, flexShrink: style.flexShrink, boxSizing: style.boxSizing, icons: [...element.querySelectorAll("svg")].map(icon => { const r = icon.getBoundingClientRect(); return [r.x, r.y, r.width, r.height]; }), ancestors: [element.parentElement, element.parentElement?.parentElement].filter(Boolean).map(parent => { const s = getComputedStyle(parent); return { slot: parent.getAttribute("data-slot"), display: s.display, overflowY: s.overflowY, height: s.height, minHeight: s.minHeight, flexShrink: s.flexShrink, clientHeight: parent.clientHeight, scrollHeight: parent.scrollHeight }; }) };
      }), rootStyle: (() => { const style = getComputedStyle(document.documentElement); return { fontSize: style.fontSize, fontFamily: style.fontFamily, panel: style.getPropertyValue("--panel"), spacingSingle: style.getPropertyValue("--spacing-single"), textXs: style.getPropertyValue("--text-xs") }; })() }), GEOMETRY_CONTROLS);
    },
    // 🚦️ Booted means READY *and* QUIET: the readiness beacon plus a settled chrome census plus two
    // consecutive seconds in which the action ledger grew by nothing.
    //
    // 🩸️ Without the quiet term the probe measured a race, not a renderer. Both shells announce their
    // loaded collision meshes at boot (`registerBrushMesh`, one per window per mesh — React's
    // `BrushMeshRegistrar` off its GLB loader, wgpu's `step_world3d_brush_mesh_announce`). React's
    // GLBs resolve during the first render and its run is always over before the beacon's fifth
    // second; wgpu's decode finishes at t≈12.2 s and its run dispatched from 15.3 s to 20.5 s — right
    // across `dismiss-tour` and `panel-artifact`, which then differed by a boot announcement neither
    // step caused (`🗑️generated/w12c-parity-run-19/steps.json` steps 02/03, ticket 26/09/17 packet
    // W13c §1). Every step's own cursor is read at its start, so a row that lands inside this wait is
    // attributed to the boot it belongs to and to no step at all.
    async boot() {
      let quiet = 0;
      let seen = -1;
      for (let second = 0; second < bootSeconds; second += 1) {
        if (this.fatalError) return { booted: false, why: this.fatalError };
        await page.waitForTimeout(1000);
        const state = await observe();
        if (state.error) return { booted: false, why: `shell error ${state.error}` };
        if (!state.ready || state.controls.length <= 4 || second <= 4) continue;
        // 🔢️ The ledger is a ROLLING buffer, so its length saturates; the newest row's `inputSeq` is
        // the monotonic term and the only one that answers "did anything dispatch this second".
        const rows = await actions();
        const ledger = rows.length ? rows[rows.length - 1].seq : 0;
        quiet = ledger === seen ? quiet + 1 : 0;
        seen = ledger;
        if (quiet >= 2) return { booted: true, ready: state.ready, bootSeq: ledger, quietAfterSeconds: second };
      }
      return { booted: false, why: "never reported a quiet data-semio-os-ready" };
    },
    async resolve(key) {
      const state = await observe();
      const { row, resolved } = resolveByLadder(state.controls, key, (control) => control.id);
      return { rect: row?.rect ?? null, id: row?.id ?? null, resolved };
    },
    async click(key) {
      const { rect, id, resolved } = await this.resolve(key);
      if (!rect) return { resolved, clicked: false, why: `no control matches ${key}` };
      await this.verifyHit(id, rect);
      await page.mouse.click(...centre(rect));
      return { resolved, id, rect, clicked: true };
    },
    async pointer(kind, from, to, options) {
      if (kind === "wheel") {
        await page.mouse.move(...from);
        await page.mouse.wheel(0, options?.deltaY ?? -240);
        return { kind, from };
      }
      await page.mouse.move(...from);
      await page.mouse.down({ button: options?.button ?? "left" });
      if (options?.holdMs) await page.waitForTimeout(options.holdMs);
      for (let step = 1; step <= 8; step += 1) await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
      await page.mouse.up({ button: options?.button ?? "left" });
      return { kind, from, to };
    },
    /** 🎯️ A point INSIDE a pane's scene host and clear of its own overlays: the Actions/Search panels cover
     * the pane's left band and its utility bar the bottom, so the aim is 70 % across and 45 % down. */
    async surfacePoint() {
      const host = await page
        .evaluate(() => {
          const element = [...document.querySelectorAll('[data-slot="pane-host"], [data-slot="window-body"]')].sort((a, b) => b.getBoundingClientRect().width * b.getBoundingClientRect().height - a.getBoundingClientRect().width * a.getBoundingClientRect().height)[0];
          if (!element) return null;
          const rect = element.getBoundingClientRect();
          return [rect.x, rect.y, rect.width, rect.height];
        })
        .catch(() => null);
      return host ? [host[0] + host[2] * 0.7, host[1] + host[3] * 0.45] : [viewport.width / 2, viewport.height / 2];
    },
  };
}

function wgpuDriver(page, name, url, consoleLines) {
  const dump = (probe, windowId) =>
    page
      .evaluate(
        async ([which, id]) => {
          const beacon = globalThis.semioWgpuIntrospection;
          if (typeof beacon?.[which] !== "function") return { unavailable: which };
          try {
            const raw = await beacon[which](id);
            return raw ? JSON.parse(raw) : null;
          } catch (error) {
            return { error: String(error).slice(0, 300) };
          }
        },
        [probe, windowId],
      )
      .catch((error) => ({ error: String(error).slice(0, 300) }));

  const observe = async () => {
    const [chrome, structure] = await Promise.all([dump("dumpChrome"), dump("dumpStructure")]);
    if (chrome?.unavailable) return { error: "renderer exposes no dumpChrome — rebuild the wgpu wasm", controls: [], surfaces: [], chromeAvailable: false };
    const controls = (chrome?.hits ?? []).map((hit) => ({ id: hit.controlId, kind: hit.kind, rect: hit.rect, windowId: hit.windowId, action: hit.action }));
    // 🪟️ The shell's own census (`dumpChrome().surfaces`), NOT `dumpStructure().windowIds`: the wgpu
    // shell models a docked panel as a window instance, so the engine's window list reported every
    // open panel as a window where React reports a panel. The census carries each surface's level and
    // React's own element id (📓️w9c-behaviour-parity-run-2.md).
    const surfaces = [
      ...new Set(
        (chrome?.surfaces ?? []).map((surface) => {
          if (surface.level === "dialog") return "dialog";
          if (surface.level === "panel") return `panel:${surface.id}`;
          return `window:${surface.id}`;
        }),
      ),
    ].sort();
    return { ready: structure ? "wgpu" : null, error: chrome?.error ?? structure?.error, armed: chrome?.armed ?? false, generation: chrome?.generation ?? 0, controls, surfaces, chromeAvailable: true, fullscreen: await page.evaluate(() => document.fullscreenElement !== null) };
  };

  const actions = async () => {
    const chrome = await dump("dumpChrome");
    return (chrome?.actions ?? []).map((entry) => ({ seq: entry.seq, controller: entry.controllerId, action: entry.action, windowId: entry.windowId, origin: entry.origin ?? "shell", outcome: "dispatched", args: entry.args }));
  };

  return {
    name,
    url,
    page,
    consoleLines,
    observe,
    actions,
    async boot() {
      let signature = "";
      let quiet = 0;
      let firstGeneration = null;
      let lastState = null;
      for (let second = 0; second < bootSeconds; second += 1) {
        if (this.fatalError) return { booted: false, why: this.fatalError };
        await page.waitForTimeout(1000);
        const state = await observe();
        lastState = state;
        if (second % 15 === 0) {
          writeFileSync(join(outDir, name, "console.txt"), consoleLines.join("\n"));
          writeFileSync(join(outDir, name, "boot-progress.json"), JSON.stringify({second, quiet, firstGeneration, state}, null, 2));
          console.log(`[DEBUG] ${name} boot-progress second=${second} ready=${state.ready} hits=${state.controls.length} generation=${state.generation} quiet=${quiet} error=${state.error ?? "none"}`);
        }
        if (!state.chromeAvailable) continue;
        if (state.error) return { booted: false, why: state.error };
        if (!state.ready || state.controls.length <= 4) continue;
        firstGeneration ??= state.generation;
        const meshes = await dump("dumpMeshStats");
        const rows = await actions();
        const current = JSON.stringify({ controls: state.controls.map(row => [row.id, row.rect]), surfaces: state.surfaces, cameras: meshes?.surfaces?.map(row => [row.surfaceId, row.camera, row.liveCamera, row.meshes?.length, row.instances?.length]), action: rows.at(-1)?.seq ?? 0 });
        quiet = current === signature ? quiet + 1 : 0;
        signature = current;
        if (second < 10 || quiet < 3 || state.generation <= firstGeneration) continue;
        await page.locator("canvas").first().evaluate(element => element.focus?.());
        return { booted: true, ready: "wgpu", chromeAvailable: state.chromeAvailable, armed: state.armed, hits: state.controls.length, generation: state.generation, quietAfterSeconds: second };
      }
      return { booted: false, why: "WGPU camera, content, registry and action publication did not become quiet", lastState };
    },
    async settle(before, cursor, minimumMs) {
      const started = Date.now();
      let signature = "";
      let unchangedSince = started;
      let latest = before;
      let progressed = false;
      let firstPublicationAfterOperationMs = null;
      let firstSemanticChangeAfterOperationMs = null;
      while (Date.now() - started < 15000) {
        if (this.fatalError) break;
        await page.waitForTimeout(250);
        latest = await observe();
        const rows = await actions();
        const current = JSON.stringify({ controls: latest.controls.map(row => [row.id, row.rect]), surfaces: latest.surfaces, action: rows.at(-1)?.seq ?? 0, fullscreen: latest.fullscreen });
        if (current !== signature) { signature = current; unchangedSince = Date.now(); }
        progressed ||= latest.generation > before.generation;
        if (progressed) firstPublicationAfterOperationMs ??= Date.now() - started;
        const semanticChange = JSON.stringify(before.surfaces) !== JSON.stringify(latest.surfaces) || JSON.stringify(before.controls.map(row => [row.id, row.rect])) !== JSON.stringify(latest.controls.map(row => [row.id, row.rect])) || before.fullscreen !== latest.fullscreen;
        if (semanticChange) firstSemanticChangeAfterOperationMs ??= Date.now() - started;
        const changed = rows.some(row => row.seq > cursor) || JSON.stringify(before.surfaces) !== JSON.stringify(latest.surfaces) || before.controls.length !== latest.controls.length || before.fullscreen !== latest.fullscreen;
        if (Date.now() - started >= minimumMs && Date.now() - unchangedSince >= 1000 && progressed && changed) break;
      }
      return { elapsedMs: Date.now() - started, firstPublicationAfterOperationMs, firstSemanticChangeAfterOperationMs, generationBefore: before.generation, generationAfter: latest.generation, publicationAdvanced: progressed };
    },
    async diagnostics() {
      const chrome = await dump("dumpChrome");
      const structure = await dump("dumpStructure");
      const panelFrames = {};
      for (const surface of (chrome?.surfaces ?? []).filter(surface => surface.level === "panel")) panelFrames[surface.id] = await dump("dumpFrameStats", surface.id);
      const accessibilityMirror = await page.evaluate(() => [...document.querySelectorAll("#semio-wgpu-accessibility [data-node-key]")].map(element => ({ windowId: element.getAttribute("data-window"), key: element.getAttribute("data-node-key"), role: element.getAttribute("role") ?? element.tagName.toLowerCase(), label: element.getAttribute("aria-label"), hidden: element.getAttribute("aria-hidden"), expanded: element.getAttribute("aria-expanded"), focusable: element.getAttribute("data-focusable"), actionable: element.getAttribute("data-actionable") })));
      return { chrome, meshes: await dump("dumpMeshStats"), frame: await dump("dumpFrameStats"), panelFrames, structure, accessibility: await dump("dumpAccessibility"), accessibilityMirror };
    },
    async resolve(key) {
      const state = await observe();
      const controls = key === "framework.settings" ? state.controls.filter(control => control.id !== key || control.kind === "Toggle") : state.controls;
      const { row, resolved } = resolveByLadder(controls, key, (control) => control.id ?? "");
      return { rect: row?.rect ?? null, id: row?.id ?? null, resolved };
    },
    async click(key) {
      const { rect, id, resolved } = await this.resolve(key);
      if (!rect) return { resolved, clicked: false, why: `no hit target matches ${key}` };
      await page.mouse.click(...centre(rect));
      return { resolved, id, rect, clicked: true };
    },
    async pointer(kind, from, to, options) {
      if (kind === "wheel") {
        await page.mouse.move(...from);
        await page.mouse.wheel(0, options?.deltaY ?? -240);
        return { kind, from };
      }
      await page.mouse.move(...from);
      await page.mouse.down({ button: options?.button ?? "left" });
      if (options?.holdMs) await page.waitForTimeout(options.holdMs);
      for (let step = 1; step <= 8; step += 1) await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
      await page.mouse.up({ button: options?.button ?? "left" });
      return { kind, from, to };
    },
    async surfacePoint() {
      const state = await observe();
      const surface = state.controls.filter(control => control.kind === "World3d" && state.surfaces.includes(`window:${control.windowId}`)).sort((a,b) => b.rect[2] * b.rect[3] - a.rect[2] * a.rect[3])[0];
      if (!surface) throw new Error("No visible World3d surface has a live pointer registry");
      return [surface.rect[0] + surface.rect[2] * 0.7, surface.rect[1] + surface.rect[3] * 0.45];
    },
  };
}
//#endregion 🌐️Drivers

//#region 🚶️Journey
/** 🚶️ ONE journey, written once and run on both renderers. A step's `run` is handed the driver plus the
 * observation taken just before it, and answers whatever detail it wants recorded; the harness measures the
 * action log and the surface delta around it. */
const journey = [
  { name: "boot", run: async (driver) => driver.bootResult },
  { name: "dismiss-tour", run: (driver) => driver.click("ui.introduction.skip") },
  { name: "panel-artifact", run: (driver) => driver.click("framework.panel.artifact") },
  { name: "panel-catalogue", run: (driver) => driver.click("framework.panel.catalogue") },
  { name: "panel-inspection", run: (driver) => driver.click("framework.panel.inspection") },
  { name: "panel-tool-runs", run: (driver) => driver.click("framework.panel.toolRun") },
  { name: "panel-chat", run: (driver) => driver.click("framework.chat") },
  { name: "panel-chat-close", run: (driver) => driver.click("framework.chat") },
  { name: "panel-catalogue-close", run: (driver) => driver.closePanel("framework.panel.catalogue") },
  { name: "settings-open", run: (driver) => driver.click("framework.settings") },
  { name: "settings-app", run: async (driver) => {
    const keys = ["contact-tolerance", "proximity-radius", "chunk-size", "grid-spacing"].map(field => `puzzle3d-play-settings.${field}.control`);
    const published = state => keys.every(key => state.controls.some(control => control.id === key || control.id?.endsWith(`/${key}`)));
    const result = published(await driver.observe()) ? { resolved: "exact", alreadyOpen: true } : await driver.click("puzzle3d.panel.settings");
    await driver.waitUntil(published, "App settings did not publish all four numeric controls");
    return result;
  } },
  { name: "settings-general", run: async (driver) => {
    const result = await driver.click("framework.settings.general");
    await driver.waitUntil(state => ["framework.settings.appearance", "framework.settings.language"].every(key => state.controls.some(control => control.id === key || control.id?.endsWith(`/${key}`))), "General settings did not publish its Appearance and Language controls");
    return result;
  } },
  { name: "settings-appearance-open", run: (driver) => driver.openSelect("framework.settings.appearance", settingsAppearanceProbeValue) },
  { name: "settings-appearance-activate", run: (driver) => driver.activateSelectOption("framework.settings.appearance", settingsAppearanceProbeValue) },
  { name: "settings-language-open", run: (driver) => driver.openSelect("framework.settings.language", settingsLanguageProbeValue) },
  { name: "settings-language-activate", run: (driver) => driver.activateSelectOption("framework.settings.language", settingsLanguageProbeValue) },
  { name: "settings-drivers-open", run: async (driver) => {
    const keys = ["labels", "labelTier", "drag", "chrome", "gumball", "tooltips", "hotkeys"].map(axis => `framework.settings.driver.${axis}`);
    const has = (state, key) => state.controls.some(control => control.id === key || control.id?.endsWith(`/${key}`));
    const before = await driver.observe();
    if (keys.some(key => has(before, key))) throw new Error("Default-closed Drivers section exposes child hit targets");
    const result = await driver.click("framework.settings.driver.editor");
    await driver.waitUntil(state => keys.every(key => has(state, key)), "Opening Drivers did not publish all seven axis controls");
    return result;
  } },
  { name: "settings-drivers-close", run: async (driver) => {
    const keys = ["labels", "labelTier", "drag", "chrome", "gumball", "tooltips", "hotkeys"].map(axis => `framework.settings.driver.${axis}`);
    const result = await driver.click("framework.settings.driver.editor");
    await driver.waitUntil(state => keys.every(key => !state.controls.some(control => control.id === key || control.id?.endsWith(`/${key}`))), "Closing Drivers left child hit targets visible");
    return result;
  } },
  { name: "settings-close", run: async (driver) => {
    const result = await driver.click("framework.settings");
    await driver.waitUntil(state => !state.surfaces.some(surface => surface.startsWith("panel:framework.settings.") || surface === "panel:puzzle3d.panel.settings"), "Closing Settings left its active panel visible");
    return result;
  } },
  ...PANE_CHIPS.flatMap((chip) => [
    { name: `pane-chip-${chip.replace(/[^a-z]+/gi, "-").toLowerCase()}`, run: (driver, before) => driver.clickPaneChip(before, chip) },
    ...(chip === "engagement.toggle" ? [{ name: "pane-actions-scroll", run: (driver) => driver.exerciseActionTree() }] : []),
    { name: `pane-chip-${chip.replace(/[^a-z]+/gi, "-").toLowerCase()}-close`, run: (driver, before) => driver.clickPaneChip(before, chip.replace(/\.unfold$/, ".fold")) },
  ]),
  { name: "split-gutter-drag", run: (driver, before) => driver.dragGutter(before) },
  { name: "window-cap-focus", run: (driver, before) => driver.clickWindowCap(before, "focus") },
  { name: "window-cap-unfocus", run: (driver, before) => driver.clickWindowCap(before, "focus") },
  { name: "window-cap-refocus", run: (driver, before) => driver.clickWindowCap(before, "focus") },
  { name: "window-cap-close", run: (driver, before) => driver.clickWindowCap(before, "close") },
  { name: "window-close-last", run: (driver) => driver.closeAllWindows() },
  { name: "window-reopen", run: (driver, before) => driver.reopenWindow(before) },
  // 🕰️ The five scene gestures settle LONGER than the chrome steps, on both renderers. React's own
  // `CAMERA_SYNC_DEBOUNCE_MS` is 120 ms, so its rows land inside the default window either way; the wgpu
  // host answers a synthesized 8-move drag through the frame worker's intent queue and its
  // `noteWorldNavigation`/`setCamera` settle landed up to ~1.3 s after the release — measured in
  // `🗑️generated/w11a-parity-run-14/wgpu/console.txt` (orbit release t=76797 ms, `setCamera` produced at
  // t=77130 ms with the ledger read already taken), where the rows fell into the gap BETWEEN this step's
  // read and the next step's cursor and were counted for neither. Widening the window equally for both
  // renderers measures the journal instead of the ingress latency, which is reported separately.
  { name: "orbit-drag", settle: 4000, run: async (driver) => { const at = await driver.surfacePoint(); return driver.pointer("drag", at, [at[0] + 180, at[1] + 90]); } },
  { name: "pan-drag", settle: 4000, run: async (driver) => { const at = await driver.surfacePoint(); return driver.pointer("drag", at, [at[0] - 140, at[1] + 40], { button: "middle" }); } },
  { name: "zoom-wheel", settle: 4000, run: async (driver) => { const at = await driver.surfacePoint(); return driver.pointer("wheel", at, at, { deltaY: -360 }); } },
  { name: "pick-instance", settle: 4000, run: async (driver) => { const at = await driver.surfacePoint(); await driver.page.mouse.click(at[0], at[1]); return { at, button: "left" }; } },
  { name: "context-menu", settle: 4000, run: async (driver) => { const at = await driver.surfacePoint(); await driver.page.mouse.click(at[0], at[1], { button: "right" }); return { at, button: "right" }; } },
  { name: "context-menu-dismiss", settle: 4000, run: (driver) => driver.chord("Escape") },
  { name: "command-palette-activate", settle: 4000, run: (driver) => driver.exerciseCommandPalette(settingsLanguageProbeValue) },
  { name: "command-palette-keyboard", settle: 4000, run: (driver) => driver.exerciseCommandPalette(settingsLanguageProbeValue, true) },
  { name: "chord-escape", run: (driver) => driver.chord("Escape") },
  { name: "chord-undo", run: (driver) => driver.chord(`${mod}+z`) },
  { name: "chord-redo", run: (driver) => driver.chord(`${mod}+Shift+z`) },
  { name: "chord-fullscreen", run: (driver) => driver.toggleFullscreen() },
  { name: "chord-fullscreen-exit", run: (driver) => driver.toggleFullscreen() },
  { name: "chord-panel-anchor-left", run: (driver) => driver.chord(`${mod}+b`) },
  { name: "chord-panel-anchor-right", run: (driver) => driver.chord(`${mod}+Shift+b`) },
  // 🪟️ The picker and the role switch come LAST and in this order: an open picker overlay swallows the next
  // step's click, and `viewer` makes the whole shell read-only — measured on the 2026-09-18 React run, where
  // an early role switch turned every later step into `refused: viewer-read-only`.
  { name: "example-picker-open", settle: 2600, run: (driver) => driver.click("playground.navbar.fixture") },
  { name: "example-switch", settle: 6000, run: (driver, before) => driver.pickExample(before) },
  { name: "example-picker-dismiss", run: (driver) => driver.chord("Escape") },
  { name: "role-viewer", settle: 4000, run: (driver) => driver.click("playground.navbar.roles.viewer") },
  { name: "role-editor", settle: 4000, run: (driver) => driver.click("playground.navbar.roles.editor") },
];
/** 🧰️ The journey verbs a step needs that are not a plain click, shared by both drivers so a step never
 * learns which renderer it is running on. */
const windowTabs = (state) => (state.controls ?? []).filter((row) => /^mode-dock-tab-(?:root|[0-9])/.test(row.id ?? "") || /^dock\.tab\.[^.]*\.[^.]+$/.test(row.id ?? ""));

/** 🪟️ Visible physical bodies indexed by the shell's concrete window identity. */
function windowBodies(state) {
  const bodies = new Map();
  for (const surface of state.surfaces.filter(surface => surface.startsWith("window:"))) {
    const id = surface.slice("window:".length);
    const candidates = state.controls.filter(control => control.id === id && control.rect?.[2] > 0 && control.rect?.[3] > 0);
    const body = candidates.sort((a, b) => b.rect[2] * b.rect[3] - a.rect[2] * a.rect[3])[0];
    if (body) bodies.set(id, body.rect);
  }
  return bodies;
}

/** 📐️ The rectangle occupied by a set of physical window bodies. */
function bodyUnion(bodies) {
  const rects = [...bodies.values()];
  const x = Math.min(...rects.map(rect => rect[0]));
  const y = Math.min(...rects.map(rect => rect[1]));
  return [x, y, Math.max(...rects.map(rect => rect[0] + rect[2])) - x, Math.max(...rects.map(rect => rect[1] + rect[3])) - y];
}

function attachJourneyVerbs(driver) {
  driver.settingsState = () => driver.page.evaluate(renderer => {
    if (renderer === "react") {
      const root = [...document.querySelectorAll('.semio-scope[data-shell-id]')].find(element => element.getBoundingClientRect().width > 0);
      return { appearance: root?.dataset.uiAppearance ?? null, locale: root?.lang || document.documentElement.lang || null };
    }
    const value = id => [...document.querySelectorAll('#semio-wgpu-accessibility [role="combobox"][data-node-key]')].find(element => {
      const key = element.getAttribute("data-node-key");
      return key === id || key?.endsWith(`/${id}`);
    })?.getAttribute("aria-valuetext") ?? null;
    return { appearance: value("framework.settings.appearance"), locale: value("framework.settings.language") };
  }, driver.name);
  driver.waitUntil = async (predicate, failure, timeoutMs = 15000) => {
    const deadline = Date.now() + timeoutMs;
    do {
      if (driver.fatalError) throw new Error(driver.fatalError);
      const state = await driver.observe();
      if (predicate(state)) return state;
      await driver.page.waitForTimeout(150);
    } while (Date.now() < deadline);
    throw new Error(failure);
  };
  driver.closePanel = async (key) => {
    const before = await driver.observe();
    return before.surfaces.includes(`panel:${key}`) ? driver.click(key) : { resolved: "exact", alreadyClosed: true };
  };
  driver.verifyHit = async (id, rect) => {
    if (driver.name !== "react") return;
    const obstruction = await driver.page.evaluate(({ id, at }) => {
      const hit = document.elementFromPoint(...at);
      const exact = [...document.querySelectorAll("[id]")].filter((element) => element.id === id);
      const slot = id.replace(/#\d+$/, "");
      const candidates = exact.length ? exact : [...document.querySelectorAll("[data-slot]")].filter((element) => element.getAttribute("data-slot") === slot);
      if (!candidates.length || candidates.some((element) => element === hit || element.contains(hit))) return null;
      return hit?.closest("[id]")?.id || hit?.getAttribute("data-slot") || hit?.tagName || "outside viewport";
    }, { id, at: centre(rect) });
    if (obstruction) throw new Error(`Control ${id} is covered by ${obstruction}`);
  };
  driver.chord = async (chord) => {
    if (process.env.SEMIO_PROBE_KEYS === "1") await driver.page.evaluate(() => {
      const events = [];
      const listener = event => events.push({ key: event.key, ctrl: event.ctrlKey, meta: event.metaKey, target: event.target?.tagName, targetId: event.target?.id, active: document.activeElement?.tagName, activeId: document.activeElement?.id, trusted: event.isTrusted });
      document.addEventListener("keydown", listener, true);
      globalThis.__parityKeyProbe = { events, listener };
    });
    await driver.page.keyboard.press(chord);
    if (process.env.SEMIO_PROBE_KEYS === "1") {
      const receipt = await driver.page.evaluate(() => {
        const probe = globalThis.__parityKeyProbe;
        document.removeEventListener("keydown", probe.listener, true);
        delete globalThis.__parityKeyProbe;
        return { events: probe.events, platform: navigator.userAgentData?.platform ?? navigator.platform, activation: navigator.userActivation.isActive, active: document.activeElement?.tagName, activeId: document.activeElement?.id };
      });
      console.log("[DEBUG] keyboard receipt " + JSON.stringify({ url: driver.page.url(), chord, ...receipt }));
    }
    return { chord };
  };
  driver.exerciseCommandPalette = async (languageHint, keyboard = false) => {
    await driver.chord(`${mod}+p`);
    await driver.waitUntil(
      state => state.surfaces.includes("dialog") && state.controls.some(control => control.id === "ui.search.input"),
      "Command palette did not publish its dialog and authored search input",
    );
    const inputSelector = '#ui\\.search\\.input, #ui.search.input, [data-node-key="ui.search.input"]';
    await driver.page.waitForFunction(selector => {
      const input = document.querySelector(selector);
      return input instanceof HTMLInputElement && document.activeElement === input;
    }, inputSelector, { timeout: 15000 });
    const opened = await driver.page.evaluate(selector => {
      const input = document.querySelector(selector);
      if (!(input instanceof HTMLInputElement)) return null;
      return {
        id: input.id || input.dataset.nodeKey,
        label: input.getAttribute("aria-label"),
        placeholder: input.placeholder,
        value: input.value,
        focused: document.activeElement === input,
        rendererFocused: input.dataset.focused === "true" || undefined,
      };
    }, inputSelector);
    if (!opened?.focused || opened.value !== "") throw new Error(`Command palette input did not open focused and empty: ${JSON.stringify(opened)}`);
    let keyboardNavigation;
    if (keyboard) {
      const selection = () => driver.page.evaluate(selector => {
        const input = document.querySelector(selector);
        const list = document.getElementById(input?.getAttribute("aria-controls") ?? "");
        const activeId = input?.getAttribute("aria-activedescendant");
        const active = activeId ? document.getElementById(activeId) : null;
        return input?.getAttribute("role") === "combobox" && list?.getAttribute("role") === "listbox" && list.contains(active) && active?.getAttribute("aria-selected") === "true" ? activeId : null;
      }, inputSelector);
      const initial = await selection();
      if (!initial) throw new Error("Palette combobox lacks an owned listbox and selected active descendant");
      await driver.chord("ArrowDown");
      await driver.page.waitForFunction(({ selector, initial }) => document.querySelector(selector)?.getAttribute("aria-activedescendant") !== initial, { selector: inputSelector, initial }, { timeout: 15000 });
      const next = await selection();
      if (!next || next === initial) throw new Error("ArrowDown did not advance the palette selection");
      await driver.chord("ArrowUp");
      await driver.page.waitForFunction(({ selector, initial }) => document.querySelector(selector)?.getAttribute("aria-activedescendant") === initial, { selector: inputSelector, initial }, { timeout: 15000 });
      keyboardNavigation = { initial, next, restored: await selection() === initial };
    }
    const targetLabel = await driver.page.evaluate(() => {
      const row = document.querySelector('[role="option"][data-command-item-id="command.os.os.setThemeId"]');
      return row?.getAttribute("aria-label") ?? row?.textContent?.replace(/\s+/g, " ").trim() ?? null;
    });
    if (!targetLabel?.endsWith("…")) throw new Error(`Canonical Set Theme command lacks its localized staged label: ${JSON.stringify(targetLabel)}`);
    const query = targetLabel.slice(0, -1);
    await driver.page.keyboard.type(query);
    await driver.page.waitForFunction(({ selector, targetLabel }) => {
      const input = document.querySelector(selector);
      const rows = [...document.querySelectorAll('[role="option"][data-command-item-id], #semio-wgpu-accessibility [data-node-key^="ui.search.item."]')];
      const labelOf = row => row.getAttribute("aria-label") ?? row.textContent?.replace(/\s+/g, " ").trim();
      return input instanceof HTMLInputElement && input.value === targetLabel.slice(0, -1) && document.activeElement === input && rows.length === 1 && labelOf(rows[0]) === targetLabel;
    }, { selector: inputSelector, targetLabel }, { timeout: 15000 });
    const resolveFilteredRow = async () => {
      const deadline = Date.now() + 15000;
      let previous = "";
      let filtered;
      do {
        filtered = await driver.page.evaluate(async targetLabel => {
      const beacon = globalThis.semioWgpuIntrospection;
      const [projection, chrome] = beacon ? await Promise.all([beacon.dumpAccessibility().then(JSON.parse), beacon.dumpChrome().then(JSON.parse)]) : [null, null];
      const hitFor = row => {
        if (!beacon) return row.id;
        const node = projection.windows.flatMap(window => window.nodes).find(node => node.key === row.getAttribute("data-node-key"));
        return chrome.hits.find(hit => hit.controlId?.startsWith("ui.search.item.") && node?.rect?.every((value, index) => Math.abs(value - hit.rect[index]) < 0.01))?.controlId;
      };
      const rows = [...document.querySelectorAll('[role="option"][data-command-item-id], #semio-wgpu-accessibility [data-node-key^="ui.search.item."]')].map(row => ({
        hitId: hitFor(row),
        commandId: row.getAttribute("data-command-item-id"),
        label: row.getAttribute("aria-label") ?? row.textContent?.replace(/\s+/g, " ").trim(),
        selected: row.getAttribute("aria-selected"),
        rect: beacon ? chrome.hits.find(hit => hit.controlId === hitFor(row))?.rect : (() => { const rect = row.getBoundingClientRect(); return [rect.x, rect.y, rect.width, rect.height]; })(),
      }));
      return { rows, target: rows.find(row => row.label === targetLabel) ?? null };
    }, targetLabel);
        const signature = JSON.stringify(filtered);
        const target = filtered.target;
        if (filtered.rows.length === 1 && target?.hitId && target.commandId === "command.os.os.setThemeId" && target.rect?.[2] > 0 && signature === previous) return filtered;
        previous = signature;
        await driver.page.waitForTimeout(150);
      } while (Date.now() < deadline);
      throw new Error(`Filtered palette did not publish a stable canonical Set Theme hit: ${JSON.stringify(filtered)}`);
    };
    let filtered = await resolveFilteredRow();
    await driver.waitUntil(state => state.controls.some(control => control.id === filtered.target.hitId), `Filtered palette row ${filtered.target.hitId} has no physical hit target`);
    let pointerDismissals;
    const reopenQuery = async () => {
      await driver.chord(`${mod}+p`);
      await driver.page.waitForFunction(({ selector, query }) => {
        const input = document.querySelector(selector);
        return input instanceof HTMLInputElement && document.activeElement === input && input.value === query;
      }, { selector: inputSelector, query }, { timeout: 15000 });
      filtered = await resolveFilteredRow();
    };
    if (!keyboard) {
      await driver.page.waitForTimeout(400);
      const dialog = (await driver.observe()).controls.find(control => control.id === "ui.search.dialog" || control.kind === "dialog-content");
      if (!dialog?.rect) throw new Error("Palette dialog has no physical bounds");
      const interior = [dialog.rect[0] + 5, dialog.rect[1] + dialog.rect[3] * 0.5];
      await driver.verifyHit(dialog.id, [interior[0], interior[1], 0, 0]);
      await driver.page.mouse.click(...interior);
      await driver.page.waitForTimeout(400);
      if (!(await driver.observe()).surfaces.includes("dialog")) throw new Error("Palette interior padding dismissed the dialog");
      const row = await driver.resolve(filtered.target.hitId);
      const input = await driver.resolve("ui.search.input");
      if (!row.rect || !input.rect) throw new Error("Palette row or input has no physical bounds");
      await driver.page.mouse.move(...centre(row.rect));
      await driver.page.mouse.down();
      await driver.page.waitForTimeout(200);
      await driver.page.mouse.move(...centre(input.rect));
      await driver.page.mouse.up();
      await driver.page.waitForTimeout(400);
      if (!(await driver.observe()).surfaces.includes("dialog")) throw new Error("Palette drag release activated a different physical target");
      const close = (await driver.observe()).controls.find(control => control.id === "ui.search.close" || control.kind === "dialog-close");
      if (!close?.rect) throw new Error("Palette omitted its physical Close control");
      await driver.click(close.id);
      await driver.waitUntil(state => !state.surfaces.includes("dialog"), "Palette Close did not dismiss the dialog");
      await reopenQuery();
      await driver.page.mouse.click(4, 4);
      await driver.waitUntil(state => !state.surfaces.includes("dialog"), "Palette outside click did not dismiss the dialog");
      await reopenQuery();
      pointerDismissals = { interiorRetained: true, dragReleaseCancelled: true, closePreservedQuery: true, outsidePreservedQuery: true };
    } else {
      await driver.chord("Escape");
      await driver.waitUntil(state => !state.surfaces.includes("dialog") && !state.controls.some(control => control.id === "ui.search.input"), "Escape from the focused palette input did not dismiss it");
      await reopenQuery();
    }
    filtered = await resolveFilteredRow();
    await driver.verifyHit(filtered.target.hitId, filtered.target.rect);
    const activation = keyboard ? await driver.chord("Enter") : await driver.click(filtered.target.hitId);
    const formControls = ["command.category.appearance.form", "command-os.os.setThemeId-execute", "command-os.os.setThemeId-reset"];
    const consequence = await driver.waitUntil(
      state => !state.surfaces.includes("dialog")
        && !state.controls.some(control => control.id === "ui.search.input" || control.id?.startsWith("ui.search.item."))
        && state.surfaces.includes("panel:command.category.appearance")
        && formControls.every(key => state.controls.some(control => control.id === key || control.id?.endsWith(`/${key}`))),
      "Set Theme palette activation did not retire the dialog and publish its staged Appearance form with Execute and Reset",
    );
    return {
      chord: `${mod}+p`,
      languageHint,
      opened,
      keyboardNavigation,
      pointerDismissals,
      query,
      filteredRows: filtered.rows,
      activated: filtered.target,
      activation,
      consequence: { dialogRetired: true, panel: "command.category.appearance", formControls, controls: consequence.controls.length },
    };
  };
  driver.toggleFullscreen = async () => {
    const before = (await driver.observe()).fullscreen;
    await driver.chord(fullscreenChord);
    await driver.waitUntil(state => state.fullscreen !== before, "Fullscreen shortcut did not change the browser fullscreen state");
    return { chord: fullscreenChord, entered: !before };
  };
  /** 🪟️ A window cap chip. React renders it id-less with `data-slot="mode-dock-tab-<cap>"`, wgpu registers it
   * as `dock.tab.<path>.<windowId>.<cap>` — both END in the cap verb, which is what this matches on. */
  driver.clickWindowCap = async (_before, cap) => {
    // 🖱️ HOVER the dock tab first. React renders a window cap chip only while its tab is hovered
    // (`mode-dock-tab-<cap>` appears under the pointer), so without this the step resolved `absent` on
    // React and `contains` on wgpu — and the wgpu shell then CLOSED both world panes while React closed
    // none, so every later step compared two different applications: with no window left, the wgpu
    // chord ladder journalled nothing at all for `chord-escape`/`-undo`/`-redo`
    // (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`). One journey must drive one state.
    const tabbed = await driver.observe();
    const tab = windowTabs(tabbed)[0];
    if (tab) await driver.page.mouse.move(...centre(tab.rect));
    await driver.page.waitForTimeout(400);
    const before = await driver.observe();
    if (!tab) throw new Error("No dock tab is available for a window-cap operation");
    const windowId = tab.windowId ?? tab.id.match(/^dock\.tab\.[^.]*\.(.+)$/)?.[1];
    if (!windowId) throw new Error(`Dock tab ${tab.id} exposes no concrete window identity`);
    const key = cap === "focus" ? "focus" : cap === "close" ? "close" : "maximize";
    const control = (before.controls ?? []).find((row) => new RegExp(`(^|[.\\-])${key}$`, "i").test(row.id ?? "") && (row.windowId === windowId || row.id === `${tab.id}.${key}`));
    if (!control) throw new Error(`Window ${windowId} publishes no ${key} cap`);
    await driver.verifyHit(control.id, control.rect);
    const priorBodies = windowBodies(before);
    await driver.page.mouse.click(...centre(control.rect));
    let state;
    if (cap === "close") {
      state = await driver.waitUntil(snapshot => !snapshot.surfaces.includes(`window:${windowId}`) && !windowTabs(snapshot).some(row => row.windowId === windowId || row.id.match(/^dock\.tab\.[^.]*\.(.+)$/)?.[1] === windowId), `Closing ${windowId} left its dock tab or physical body visible`);
      driver.focusRestore = undefined;
    } else if (cap === "focus" && driver.focusRestore) {
      const restore = driver.focusRestore;
      state = await driver.waitUntil(snapshot => {
        const bodies = windowBodies(snapshot);
        return bodies.size === restore.size && [...restore].every(([id, rect]) => bodies.has(id) && bodies.get(id).every((value, index) => Math.abs(value - rect[index]) <= 3));
      }, `Unfocusing ${windowId} did not restore the previous window bodies and bounds`);
      driver.focusRestore = undefined;
    } else if (cap === "focus") {
      if (priorBodies.size < 2) throw new Error("Focus acceptance requires at least two visible window bodies");
      const bounds = bodyUnion(priorBodies);
      state = await driver.waitUntil(snapshot => {
        const bodies = windowBodies(snapshot);
        return bodies.size === 1 && bodies.has(windowId) && bodies.get(windowId).every((value, index) => Math.abs(value - bounds[index]) <= 3);
      }, `Focusing ${windowId} did not expand its body and hide sibling bodies`);
      driver.focusRestore = priorBodies;
    }
    return { resolved: "contains", id: control.id, rect: control.rect, cap, windowId, beforeBodies: Object.fromEntries(priorBodies), afterBodies: Object.fromEntries(windowBodies(state ?? await driver.observe())), physicalOutcome: true };
  };
  driver.closeAllWindows = async () => {
    let count = 0;
    for (; count < 16; count += 1) {
      const before = await driver.observe();
      const tabs = windowTabs(before);
      if (!tabs.length) return { resolved: "exact", closed: count, remaining: 0 };
      const result = await driver.clickWindowCap(before, "close");
      if (result.resolved === "absent") throw new Error("A remaining window has no close target");
    }
    throw new Error("Window-close journey exceeded its bounded fixture size");
  };
  driver.reopenWindow = async () => {
    let state = await driver.observe();
    const tabCount = windowTabs(state).length;
    const priorWindows = new Set(state.surfaces.filter(surface => surface.startsWith("window:")));
    const template = (snapshot) => snapshot.controls.find((row) => (row.id ?? "").includes("framework.display.windows.") && (row.id ?? "").endsWith(".kind") && !(row.id ?? "").startsWith("tree.drag."));
    const sectionOf = snapshot => snapshot.controls.find(row => /(?:^|[./])framework\.display\.windows\.[^.]+$/.test(row.id ?? "") && !/\.(?:panel|empty|scroll)$/.test(row.id ?? ""));
    if (!template(state)) {
      const display = await driver.click("framework.category.display");
      if (!display.clicked) throw new Error("Display category is missing");
      state = await driver.waitUntil(snapshot => template(snapshot) || sectionOf(snapshot), "Display exposes no window-template section");
    }
    if (!template(state)) {
      const section = sectionOf(state);
      if (!section) throw new Error("Display exposes no window-template section");
      await driver.page.mouse.click(...centre(section.rect));
      state = await driver.waitUntil(snapshot => template(snapshot), "Expanded Display section exposes no draggable window kind");
    }
    const row = template(state);
    if (!row) throw new Error("Expanded Display section exposes no draggable window kind");
    const to = [viewport.width / 2, viewport.height / 2];
    const handleId = `tree.drag.transfer.${row.id.replace(/^tree\.label\./, "")}`;
    const handleRect = driver.name === "wgpu" ? state.controls.find((control) => control.id === handleId)?.rect : await driver.page.evaluate((id) => {
      const handle = document.getElementById(id)?.querySelector('[data-slot="drag-handle"]');
      if (!handle) return null;
      const rect = handle.getBoundingClientRect();
      return rect.width && rect.height ? [rect.x, rect.y, rect.width, rect.height] : null;
    }, row.id);
    if (!handleRect) throw new Error(`Window template ${row.id} exposes no transfer handle`);
    if (driver.name === "react") await driver.verifyHit(row.id, handleRect);
    const from = centre(handleRect);
    const dragSource = "handle";
    await driver.pointer("drag", from, to, { holdMs: 650 });
    const deadline = Date.now() + 8_000;
    do {
      await driver.page.waitForTimeout(150);
      state = await driver.observe();
      if (windowTabs(state).length === tabCount + 1) {
        const windowId = state.surfaces.find(surface => surface.startsWith("window:") && !priorWindows.has(surface))?.slice("window:".length);
        if (!windowId) throw new Error("The new tab publishes no new window identity");
        await driver.waitUntil(asyncState => driver.name === "wgpu" ? asyncState.controls.some(control => control.kind === "World3d" && control.windowId === windowId) : asyncState.controls.some(control => control.id === windowId && control.kind === "window"), "New window tab never published its live scene body");
        if (driver.name === "react") await driver.page.waitForFunction(id => [...document.querySelectorAll("[data-viewport-camera-json]")].some(element => element.getAttribute("data-window-instance-id") === id && element.querySelector("canvas")), windowId, {timeout:15000});
        return { resolved: "exact", id: row.id, rect: row.rect, from, to, dragSource, beforeTabs: tabCount, afterTabs: windowTabs(state).length, windowId, liveSceneBody: true };
      }
    } while (Date.now() < deadline);
    throw new Error("Dragging a Display template did not create exactly one window");
  };
  driver.dragGutter = async () => {
    const before = await driver.observe();
    const gutter = (before.controls ?? []).find((row) => /resizable-handle|separator|PanelResize|DockSplit/i.test(`${row.id ?? ""} ${row.kind ?? ""}`));
    if (!gutter) throw new Error("No split gutter is published");
    const at = centre(gutter.rect);
    const axis = gutter.rect[3] > gutter.rect[2] ? 0 : 1;
    const to = [...at];
    to[axis] += 120;
    const priorBodies = windowBodies(before);
    if (priorBodies.size < 2) throw new Error("Split-drag acceptance requires at least two visible window bodies");
    await driver.pointer("drag", at, to);
    const state = await driver.waitUntil(snapshot => {
      const current = snapshot.controls.filter(row => row.kind === gutter.kind && (row.rect[3] > row.rect[2] ? 0 : 1) === axis && Math.abs(centre(row.rect)[1 - axis] - at[1 - axis]) <= 4).sort((a, b) => Math.abs(centre(a.rect)[axis] - to[axis]) - Math.abs(centre(b.rect)[axis] - to[axis]))[0];
      const bodies = windowBodies(snapshot);
      return current && Math.abs(centre(current.rect)[axis] - to[axis]) <= 4 && bodies.size === priorBodies.size && [...priorBodies].every(([id]) => bodies.has(id)) && [...priorBodies].some(([id, rect]) => Math.abs(bodies.get(id)[axis + 2] - rect[axis + 2]) >= 100);
    }, `Dragging ${gutter.id} by 120 pixels did not move the divider and resize neighboring window bodies`);
    return { resolved: "contains", id: gutter.id, kind: gutter.kind, rect: gutter.rect, to, beforeBodies: Object.fromEntries(priorBodies), afterBodies: Object.fromEntries(windowBodies(state)), physicalOutcome: true };
  };
  driver.clickPaneChip = async (_before, chip) => {
    const before = await driver.observe();
    const control = (before.controls ?? []).find((row) => (row.id ?? "").endsWith(`.${chip}`));
    if (!control) throw new Error(`No pane chip ${chip} is published`);
    const body = [...windowBodies(before)].find(([, rect]) => {
      const at = centre(control.rect);
      return at[0] >= rect[0] && at[0] <= rect[0] + rect[2] && at[1] >= rect[1] && at[1] <= rect[1] + rect[3];
    });
    if (!body) throw new Error(`Pane chip ${control.id} belongs to no visible window body`);
    const required = chip.startsWith("utilityBar.") ? ["transform", "brush"] : chip.startsWith("measures.") ? ["puzzle3d-play-grid-visible", "puzzle3d-measure-sun-enabled"] : chip === "pane.fold" ? ["orthographic", "perspective", "parallel"] : ["action.clearSelection", "action.selectAll"];
    const visible = snapshot => {
      const rect = windowBodies(snapshot).get(body[0]);
      if (!rect) return [];
      return required.filter(key => snapshot.controls.some(row => {
        const id = row.id ?? "";
        const at = centre(row.rect);
        return (id === key || id.endsWith(`/${key}`) || id.endsWith(`.${key}`) || id.endsWith(`::${key}`)) && at[0] >= rect[0] && at[0] <= rect[0] + rect[2] && at[1] >= rect[1] && at[1] <= rect[1] + rect[3];
      }));
    };
    const prior = visible(before);
    if (prior.length !== 0 && prior.length !== required.length) throw new Error(`Pane ${chip} exposes an incomplete child-control set before interaction: ${prior.join(", ")}`);
    const opening = prior.length === 0;
    await driver.verifyHit(control.id, control.rect);
    await driver.page.mouse.click(...centre(control.rect));
    const state = await driver.waitUntil(snapshot => visible(snapshot).length === (opening ? required.length : 0), `${opening ? "Opening" : "Closing"} pane ${chip} did not ${opening ? "publish" : "retire"} its required child controls`);
    if (opening && chip === "engagement.toggle") driver.actionTreeWindowId = body[0];
    return { resolved: "suffix", id: control.id, rect: control.rect, windowId: body[0], opening, required, visibleChildren: visible(state), physicalOutcome: true };
  };
  driver.exerciseActionTree = async () => {
    const windowId = driver.actionTreeWindowId;
    if (!windowId) throw new Error("Actions Tree acceptance requires the preceding pane-open step");
    const visibleRow = (state, key) => {
      const body = windowBodies(state).get(windowId);
      if (!body) return null;
      return state.controls.find(row => {
        const at = centre(row.rect);
        return (row.id === key || row.id?.endsWith(`/${key}`)) && at[0] >= body[0] && at[0] <= body[0] + body[2] && at[1] >= body[1] && at[1] <= body[1] + body[3];
      }) ?? null;
    };
    const before = await driver.observe();
    const clear = visibleRow(before, "action.clearSelection");
    const all = visibleRow(before, "action.selectAll");
    if (!clear || !all) throw new Error("Actions Tree omitted its first two physical rows");
    const pitch = all.rect[1] - clear.rect[1];
    if ([clear.rect[3], all.rect[3], pitch].some(value => Math.abs(value - 24) > 1)) throw new Error(`Actions Tree rows must retain 24-pixel height and pitch: ${JSON.stringify({ clear: clear.rect, all: all.rect, pitch })}`);
    const targetKey = "action.engagementAbort";
    if (visibleRow(before, targetKey)) throw new Error("The offscreen Actions command is already visible before scrolling");
    const at = centre(all.rect);
    await driver.pointer("wheel", at, at, { deltaY: 720 });
    const scrolled = await driver.waitUntil(state => visibleRow(state, targetKey), "Scrolling Actions did not expose the offscreen Abort command");
    const target = visibleRow(scrolled, targetKey);
    await driver.verifyHit(target.id, target.rect);
    const cursor = (await driver.actions()).at(-1)?.seq ?? 0;
    await driver.page.mouse.click(...centre(target.rect));
    let dispatched = false;
    for (let turn = 0; turn < 75; turn += 1) {
      dispatched = (await driver.actions()).some(action => action.seq > cursor && action.action === "engagementAbort");
      if (dispatched) break;
      await driver.page.waitForTimeout(200);
    }
    if (!dispatched) throw new Error("The revealed Abort row did not dispatch its own action after a physical click");
    await driver.pointer("wheel", centre(target.rect), centre(target.rect), { deltaY: -720 });
    await driver.waitUntil(state => {
      const first = visibleRow(state, "action.clearSelection");
      return first && Math.abs(first.rect[1] - clear.rect[1]) <= 1 && !visibleRow(state, targetKey);
    }, "Scrolling Actions back did not restore the original rows and clip the offscreen command");
    return { windowId, rowHeight: clear.rect[3], rowPitch: pitch, targetKey, scrolledTargetRect: target.rect, dispatched, restored: true, physicalOutcome: true };
  };
  driver.openSelect = async (controlKey, optionValue) => {
    const state = await driver.observe();
    const { row, resolved } = resolveByLadder(state.controls.filter(control => control.kind === (driver.name === "react" ? "select-trigger" : "Select")), controlKey, control => control.id ?? "");
    const trigger = { rect: row?.rect, id: row?.id, resolved };
    if (!trigger.rect) throw new Error(`Select ${controlKey} is missing`);
    await driver.verifyHit(trigger.id, trigger.rect);
    await driver.page.mouse.click(...centre(trigger.rect));
    let optionRect;
    if (driver.name === "react") {
      await driver.page.waitForFunction(({ controlKey, optionValue }) => {
        const trigger = [...document.querySelectorAll('[role="combobox"]')].find(element => element.id === controlKey);
        return trigger?.getAttribute("aria-expanded") === "true" && [...document.querySelectorAll('[role="option"][data-value]')].some(option => option.getAttribute("data-value") === optionValue && option.getBoundingClientRect().width > 0);
      }, { controlKey, optionValue }, { timeout: 15000 });
      optionRect = await driver.page.evaluate(optionValue => {
        const option = [...document.querySelectorAll('[role="option"][data-value]')].find(candidate => candidate.getAttribute("data-value") === optionValue);
        if (!option) return null;
        const rect = option.getBoundingClientRect();
        return [rect.x, rect.y, rect.width, rect.height];
      }, optionValue);
    } else {
      const state = await driver.waitUntil(snapshot => snapshot.controls.some(control => control.id === optionValue && control.kind === "Button"), `Select ${controlKey} did not publish option ${optionValue}`);
      optionRect = state.controls.find(control => control.id === optionValue && control.kind === "Button")?.rect;
    }
    if (!optionRect) throw new Error(`Select ${controlKey} published no physical option rect for ${optionValue}`);
    return { resolved: "exact", controlKey, optionValue, triggerRect: trigger.rect, optionRect, popupOpen: true };
  };
  driver.activateSelectOption = async (controlKey, optionValue) => {
    let optionRect;
    let optionLabel;
    if (driver.name === "react") {
      optionLabel = await driver.page.evaluate(optionValue => [...document.querySelectorAll('[role="option"][data-value]')].find(candidate => candidate.getAttribute("data-value") === optionValue)?.textContent?.replace(/\s+/g, " ").trim(), optionValue);
      optionRect = await driver.page.evaluate(optionValue => {
        const option = [...document.querySelectorAll('[role="option"][data-value]')].find(candidate => candidate.getAttribute("data-value") === optionValue);
        if (!option) return null;
        const rect = option.getBoundingClientRect();
        return [rect.x, rect.y, rect.width, rect.height];
      }, optionValue);
    } else {
      const state = await driver.observe();
      optionRect = state.controls.find(control => control.id === optionValue && control.kind === "Button")?.rect;
    }
    if (!optionRect) throw new Error(`Open Select ${controlKey} has no option ${optionValue}`);
    await driver.page.mouse.click(...centre(optionRect));
    if (driver.name === "react") {
      await driver.page.waitForFunction(({ controlKey, optionValue }) => [...document.querySelectorAll('[role="combobox"]')].find(element => element.id === controlKey)?.getAttribute("aria-expanded") === "false" && ![...document.querySelectorAll('[role="option"][data-value]')].some(option => option.getAttribute("data-value") === optionValue), { controlKey, optionValue }, { timeout: 15000 });
    } else {
      await driver.waitUntil(snapshot => !snapshot.controls.some(control => control.id === optionValue && control.kind === "Button"), `Selecting ${optionValue} did not retire ${controlKey}'s popup`);
    }
    await driver.page.waitForFunction(({ controlKey, optionValue, optionLabel, renderer }) => {
      if (renderer === "react") return [...document.querySelectorAll('[role="combobox"]')].find(element => element.id === controlKey)?.textContent?.replace(/\s+/g, " ").trim() === optionLabel;
      return [...document.querySelectorAll('#semio-wgpu-accessibility [role="combobox"][data-node-key]')].some(element => {
        const key = element.getAttribute("data-node-key");
        return (key === controlKey || key?.endsWith(`/${controlKey}`)) && element.getAttribute("aria-valuetext") === optionValue;
      });
    }, { controlKey, optionValue, optionLabel, renderer: driver.name }, { timeout: 15000 });
    let appliedSettings;
    if (controlKey === "framework.settings.appearance" || controlKey === "framework.settings.language") {
      const field = controlKey === "framework.settings.appearance" ? "appearance" : "locale";
      const deadline = Date.now() + 15000;
      do {
        if (driver.fatalError) throw new Error(driver.fatalError);
        appliedSettings = await driver.settingsState();
        if (appliedSettings[field] === optionValue) break;
        await driver.page.waitForTimeout(150);
      } while (Date.now() < deadline);
      if (appliedSettings[field] !== optionValue) throw new Error(`Selected ${field} did not reach the mounted shell: ${JSON.stringify(appliedSettings)}`);
      if (field === "locale" && appliedSettings.appearance !== settingsAppearanceProbeValue) throw new Error(`The mounted appearance changed during locale selection: ${JSON.stringify(appliedSettings)}`);
    }
    return { resolved: "exact", controlKey, optionValue, optionLabel, optionRect, popupOpen: false, selectedValueConfirmed: true, appliedSettings };
  };
  /** 🎚️ Switch the example to the SECOND entry the open picker offers. React renders the options as DOM
   * (`[role="option"]`/`[data-slot=select-item]`); wgpu registers them as `DropdownItem` hits — one verb,
   * two publications, which is exactly the parity this step measures. */
  driver.pickExample = async () => {
    const before = await driver.observe();
    const selector = '[role="option"], [data-slot="select-item"], [data-slot="dropdown-menu-item"], [data-semio-portal-layer] [role], [data-semio-portal-layer] button';
    const options = await driver.page
      .evaluate((query) =>
        [...document.querySelectorAll(query)]
          .filter((element) => element.querySelector(query) === null)
          .map((element) => {
            const rect = element.getBoundingClientRect();
            return { text: (element.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 60), rect: [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)] };
          })
          .filter((option) => option.text.length > 0 && option.rect[2] > 0 && option.rect[3] > 0),
        selector,
      )
      .catch(() => []);
    const target = options.find((option) => option.text !== before.example && !/^no /i.test(option.text));
    if (target) {
      await driver.page.mouse.click(...centre(target.rect));
      return { resolved: "exact", via: "dom-option", was: before.example, picked: target.text, options: options.map((option) => option.text).slice(0, 12) };
    }
    const rows = (before.controls ?? []).filter((control) => /DropdownItem|ContextMenu/i.test(control.kind ?? ""));
    if (rows.length > 1) {
      await driver.page.mouse.click(...centre(rows[1].rect));
      return { resolved: "exact", via: "hit-registry", picked: rows[1].id, options: rows.map((row) => row.id).slice(0, 12) };
    }
    const portal = await driver.page.evaluate(() => [...document.querySelectorAll('[data-semio-portal-layer], [data-radix-popper-content-wrapper]')].map((element) => (element.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 200))).catch(() => []);
    return { resolved: "absent", why: "the example picker published no options", domOptions: options.length, hitOptions: rows.length, portal };
  };
}
//#endregion 🚶️Journey

//#region 🏃️Run
const diff = (before, after) => ({
  fullscreenChanged: before.fullscreen !== after.fullscreen,
  surfacesAdded: (after.surfaces ?? []).filter((id) => !(before.surfaces ?? []).includes(id)),
  surfacesRemoved: (before.surfaces ?? []).filter((id) => !(after.surfaces ?? []).includes(id)),
  controlsAdded: (after.controls ?? []).map((control) => control.id).filter((id) => !(before.controls ?? []).some((control) => control.id === id)).slice(0, 40),
  controlsRemoved: (before.controls ?? []).map((control) => control.id).filter((id) => !(after.controls ?? []).some((control) => control.id === id)).slice(0, 40),
  controlCount: [(before.controls ?? []).length, (after.controls ?? []).length],
});

if (process.env.SEMIO_PROBE_REPLAY) {
  Object.assign(report, JSON.parse(readFileSync(process.env.SEMIO_PROBE_REPLAY, "utf8")));
  const provenance = `\nRecomputed from recorded evidence: ${process.env.SEMIO_PROBE_REPLAY}. No browser was run.\n`;
  writeFileSync(join(outDir, "cameras.md"), cameraMarkdown() + provenance);
  writeFileSync(join(outDir, "latency.md"), latencyMarkdown() + provenance);
  writeFileSync(join(outDir, "physical.md"), physicalMarkdown() + provenance);
  const seed = report.steps.find(step => step.name === "window-cap-focus");
  if (seed) {
    if (physicalDifferences(seed)?.length !== 0) throw new Error("Recorded focus receipt must provide the passing physical mutation baseline");
    for (const [name, mutate] of [
      ["wrong-window", step => { step.renderers.wgpu.detail.windowId = "wrong-window"; }],
      ["wrong-body", step => { Object.values(step.renderers.wgpu.detail.afterBodies)[0][2] += 10; }],
      ["missing-body", step => { delete step.renderers.wgpu.detail.afterBodies[Object.keys(step.renderers.wgpu.detail.afterBodies)[0]]; }],
      ["missing-outcome", step => { delete step.renderers.wgpu.detail.physicalOutcome; }],
    ]) {
      const mutated = structuredClone(seed);
      mutate(mutated);
      if (!physicalDifferences(mutated)?.length) throw new Error(`Physical receipt falsely accepted ${name}`);
    }
  }
  console.log(`[DEBUG] Recomputed camera/phase/physical reports for ${report.steps.length} recorded steps; ${seed ? "4 physical receipt mutations rejected; " : ""}no runtime test executed`);
  process.exit(0);
}

const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
const drivers = {};
for (const name of targets) {
  mkdirSync(join(outDir, name), { recursive: true });
  try {
    const driver = await openDriver(browser, name, urls[name]);
    attachJourneyVerbs(driver);
    driver.bootResult = await driver.boot();
    writeFileSync(join(outDir, name, "console.txt"), driver.consoleLines.join("\n"));
    if (driver.bootResult.booted) drivers[name] = driver;
    else {
      report.steps.push({name: "boot-failed", renderers: {[name]: {error: driver.bootResult.why, detail: driver.bootResult}}});
      flush();
      await driver.page.screenshot({path: join(outDir, name, "boot-failed.png"), timeout: 10000}).catch(() => {});
      await driver.stopProfiles?.();
      await driver.page.close();
    }
    console.log(`[DEBUG] ${name} boot ${JSON.stringify(driver.bootResult)}`);
  } catch (error) {
    console.log(`[DEBUG] ${name} unreachable ${String(error).slice(0, 200)}`);
    report.steps.push({ name: "open", renderers: { [name]: { error: `unreachable: ${String(error).slice(0, 200)}` } } });
  }
}

let index = 0;
for (const step of journey) {
  if (onlySteps.length && !onlySteps.includes(step.name)) continue;
  index += 1;
  const entry = { name: step.name, index, renderers: {} };
  for (const [name, driver] of Object.entries(drivers)) {
    if (driver.fatalError) {
      entry.renderers[name] = { blockedByCrash: driver.fatalError, resolved: "not-run", actions: [], delta: {}, state: { ready: null } };
      continue;
    }
    const before = await driver.observe();
    const actionsBefore = await driver.actions();
    const cursor = actionsBefore.length ? actionsBefore[actionsBefore.length - 1].seq : 0;
    let detail = null;
    let error = null;
    const operationStarted = Date.now();
    try {
      detail = await step.run(driver, before);
    } catch (failure) {
      error = String(failure).slice(0, 300);
    }
    const operationMs = Date.now() - operationStarted;
    const settling = driver.settle ? await driver.settle(before, cursor, step.settle ?? settleMs) : await driver.page.waitForTimeout(step.settle ?? settleMs);
    const after = await driver.observe();
    const actionsAfter = await driver.actions();
    const stepActions = actionsAfter.filter((action) => action.seq > cursor);
    const shot = join(name, `${String(index).padStart(2, "0")}-${step.name}.png`);
    await driver.page.screenshot({ path: join(outDir, shot), type: "png" }).catch(() => {});
    entry.renderers[name] = {
      detail,
      operationMs,
      settling,
      controls: after.controls,
      diagnostics: driver.diagnostics ? await driver.diagnostics() : undefined,
      error: driver.fatalError ?? error ?? after.error ?? null,
      resolved: detail?.resolved ?? (detail?.clicked === false ? "absent" : "n/a"),
      actions: stepActions.slice(0, 60),
      actionCounts: actionCounts(stepActions),
      actionControllers: actionControllers(stepActions),
      delta: diff(before, after),
      state: { ready: after.ready ?? null, surfaces: (after.surfaces ?? []).length, controls: (after.controls ?? []).length, role: after.role, example: after.example, armed: after.armed },
      geometry: controlGeometry(after),
      screenshot: shot,
    };
    writeFileSync(join(outDir, name, "console.txt"), driver.consoleLines.join("\n"));
  }
  report.steps.push(entry);
  flush();
  const line = Object.entries(entry.renderers).map(([name, side]) => `${name}=${side.resolved}/${actionIds(side).length}v${side.error ? `/err` : ""}`).join(" ");
  console.log(`[DEBUG] ${String(index).padStart(2, "0")} ${step.name} ${line} ${stepVerdict(entry).verdict}`);
}

for (const [name, driver] of Object.entries(drivers)) {
  await driver.stopProfiles?.();
  writeFileSync(join(outDir, name, "console.txt"), driver.consoleLines.join("\n"));
}
report.finishedAt = new Date().toISOString();
flush();
const differ = report.steps.filter((step) => stepVerdict(step).verdict === "differ").length;
const failures = report.steps.flatMap((step) => Object.values(step.renderers)).filter((side) => side.error || side.detail?.booted === false).length;
const physicalFailures = report.steps.filter(step => physicalDifferences(step)?.length).length;
console.log(`[DEBUG] DONE steps=${report.steps.length} differ=${differ} failures=${failures} physicalFailures=${physicalFailures} out=${outDir}`);
if (failures || differ || physicalFailures) process.exitCode = 1;
await browser.close();
//#endregion 🏃️Run
