/** 🎚️ Does the preview FOLLOW a dragged slider? Drags each of the three slider kinds with real
 * pointer events at ~60 Hz for one second across its range and reports what the user actually sees.
 *
 * 🧾 The three kinds and who owns each dispatch:
 *   graph     — the node-graph inline slider (`Profile Radius`, `Column Height`, `Side Count`),
 *               painted by `GraphSliderOverlays` over the wasm flow canvas
 *               (`📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`'s `FlowGraphCanvasHost`)
 *   inspector — the Inspection panel's number field for the selected `InputSlider` widget
 *               (`✏️editor/📌️panels/🔍️inspection/🦀️.rs` → `patchFlowWidgets`)
 *   form      — a generate-mode Form `slider` question
 *               (`🫀️core/🖼️semantic-ui/🦀️.rs` → `updateGenerationValues`)
 *
 * 🩺️ What each number means. Butter-smooth is FOUR properties at once, so all four are measured:
 *   latency      — from a VALUE change to the first MESH change that follows it (median/p95, ms).
 *                  Sampled in-page every 50 ms, so the floor of the reading is one sample period.
 *   coalescing   — evaluations DISPATCHED (`toolRunStart`) per distinct value change. A gesture that
 *                  queues one evaluation per tick is not smooth however fast each one is: the user
 *                  watches a queue of stale geometry drain after they let go. Target ≤ 1, and at most
 *                  two evaluations live at any instant (one in flight + one owed with the latest value).
 *   stale tail   — after the LAST value change the mesh must change at least once more and then hold.
 *                  A preview that stops one value short is the defect the user reported.
 *   round trip   — dragging back to the starting value must republish the STARTING digest. This is the
 *                  only cheap oracle for "the final mesh matches the final value" that does not need a
 *                  second evaluator: identical inputs, identical published payload.
 *   history      — one entry per RELEASE, never one per tick (`data-history-json`).
 *
 * The dispatch ledger is read from an in-page `console.log` hook installed before any app code runs,
 * so the `performInvocation` lines carry the same clock as the samples and survive a reload
 * (`📓️feedback-browser-console-buffer-survives-reload`).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d \
 *          SEMIO_PROBE_OUT=slider/run1 SEMIO_PROBE_KINDS=graph,inspector,form bun 🐍️slider-live-preview-probe.mjs
 * @see 🐍️generate-mode-probe.mjs, 🐍️react-hop-cost-probe.mjs, 🐍️preview-rearm-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { loadExampleOracles } from "./🐍️example-oracle.mjs";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column").split(",").map((entry) => entry.trim()).filter(Boolean);
const kinds = (process.env.SEMIO_PROBE_KINDS ?? "graph,inspector,form").split(",").map((kind) => kind.trim()).filter(Boolean);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "slider/run");
const dragMs = Number(process.env.SEMIO_PROBE_DRAG_MS ?? 1000);
const dragSteps = Number(process.env.SEMIO_PROBE_DRAG_STEPS ?? 60);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 40);
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 180);
const latencyBudgetMs = Number(process.env.SEMIO_PROBE_LATENCY_BUDGET ?? 150);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const exampleOracles = loadExampleOracles();
const report = { url, examples, kinds, budgets: { latencyBudgetMs, evaluationsPerChange: 1, liveEvaluations: 2 }, rows: [] };
const flush = () => {
  writeFileSync(join(outDir, "results.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

//#region 🪝️InPageLedger
/** 🪝️ One hook, installed before the app's first module, that timestamps every `performInvocation`
 * line on the SAME `performance.now()` clock the samples use. Parsing the Node-side console buffer
 * instead would date each line by its delivery over CDP, which is exactly the quantity under test. */
await page.addInitScript(() => {
  const ledger = [];
  const drops = [];
  window.__sliderLedger = ledger;
  window.__sliderDrops = drops;
  for (const channel of ["log", "warn", "info", "debug", "error"]) {
    const original = console[channel].bind(console);
    console[channel] = (...args) => {
      try {
        const raw = args.map((value) => (typeof value === "string" ? value : "")).join(" ");
        if (raw.includes("performInvocation")) {
          const actionId = (raw.match(/"actionId":"([^"]+)"/) ?? [])[1] ?? null;
          if (actionId) ledger.push({ t: performance.now(), actionId, settled: raw.includes("settled") });
        }
        if (raw.includes("dropped action")) drops.push({ t: performance.now(), action: (raw.match(/dropped action "([^"]+)"/) ?? [])[1] ?? null });
      } catch {}
      original(...args);
    };
  }
});
//#endregion 🪝️InPageLedger

//#region 📏️Reading
/** 🖼️ The edit/generate preview surfaces as the DOM publishes them, with a cheap stable digest of the
 * mesh payload — length plus a 32-bit FNV over the bytes, which is enough to separate two payloads and
 * cheap enough to run every 50 ms inside the page. */
const SAMPLER = () => {
  const fnv = (text) => {
    let hash = 0x811c9dc5;
    for (let index = 0; index < text.length; index += 1) {
      hash ^= text.charCodeAt(index);
      hash = Math.imul(hash, 0x01000193) >>> 0;
    }
    return hash.toString(16);
  };
  const parse = (text) => {
    try {
      return JSON.parse(text ?? "");
    } catch {
      return null;
    }
  };
  window.__sliderSample = (surfaceId, valueSelector) => {
    const surface = document.querySelector(`[data-surface-id="${surfaceId}"]`);
    const payload = surface?.getAttribute("data-meshes-json") ?? "";
    const status = parse(surface?.getAttribute("data-status-json"));
    const meshes = parse(payload);
    const box = { lo: [0, 0, 0], hi: [0, 0, 0], points: 0 };
    if (Array.isArray(meshes)) {
      let started = false;
      for (const mesh of meshes) {
        const data = mesh?.data ?? mesh ?? {};
        const positions = (Array.isArray(data.positions) && data.positions.length > 0 ? data.positions : data.edgePositions) ?? [];
        for (let index = 0; index + 2 < positions.length; index += 3) {
          const point = [positions[index], positions[index + 1], positions[index + 2]];
          box.points += 1;
          if (!started) {
            box.lo = [...point];
            box.hi = [...point];
            started = true;
            continue;
          }
          for (let axis = 0; axis < 3; axis += 1) {
            if (point[axis] < box.lo[axis]) box.lo[axis] = point[axis];
            if (point[axis] > box.hi[axis]) box.hi[axis] = point[axis];
          }
        }
      }
    }
    const control = valueSelector ? document.querySelector(valueSelector) : null;
    const value = control ? (control.getAttribute("aria-valuenow") ?? control.value ?? null) : null;
    const history = parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json"));
    return {
      t: performance.now(),
      value,
      bytes: payload.length,
      digest: `${payload.length}:${fnv(payload)}`,
      meshes: Array.isArray(meshes) ? meshes.length : 0,
      bbox: box.points > 0 ? box.lo.concat(box.hi).map((entry) => Math.round(entry * 1000) / 1000) : null,
      phase: status?.phase ?? null,
      ratio: status?.progress?.ratio ?? null,
      fault: status?.fault?.code ?? null,
      // 🚦️ What the surface NAMES when it delivers nothing: a per-node evaluation error, or a typed
      // kernel diagnostic. An empty payload with neither is the silent vanish this probe grades.
      named: (() => {
        const said = [];
        for (const [widget, message] of Object.entries(status?.widgetErrors ?? {})) {
          if (typeof message === "string" && message.trim()) said.push(`${widget}: ${message}`);
        }
        if (typeof status?.error === "string" && status.error.trim()) said.push(`graph: ${status.error}`);
        if (typeof status?.fault?.faultMessage === "string" && status.fault.faultMessage.trim()) said.push(`${status.fault.extensionId ?? "extension"}: ${status.fault.faultMessage}`);
        // 🩺️ A typed validate-gate diagnostic is published as `{ handle, issues: [{ entity, code, message }] }`.
        for (const entry of Array.isArray(status?.diagnostics) ? status.diagnostics : []) {
          for (const issue of Array.isArray(entry?.issues) ? entry.issues : []) {
            const text = typeof issue?.message === "string" && issue.message.trim() ? issue.message : typeof issue?.code === "string" ? issue.code : "";
            if (text.trim()) said.push(`${issue?.entity ?? entry?.handle ?? "kernel"}: ${text}`);
          }
        }
        return said;
      })(),
      history: history ? { entries: history.entries ?? history.length ?? null, index: history.index ?? history.cursor ?? null, canUndo: history.canUndo ?? null } : null,
    };
  };
  // ⏱️ Two cadences, because the two quantities cost differently: the CONTROL's value is one attribute
  // read and is polled at 10 ms so a 60 Hz gesture is not undersampled, while the MESH payload must be
  // hashed and is polled at 50 ms. A single 50 ms cadence would hide five of every six value changes.
  window.__sliderStart = (surfaceId, valueSelector) => {
    window.__sliderSamples = [];
    window.__sliderValues = [];
    window.__sliderTimer = setInterval(() => {
      try {
        window.__sliderSamples.push(window.__sliderSample(surfaceId, valueSelector));
      } catch {}
    }, 50);
    window.__sliderValueTimer = setInterval(() => {
      try {
        const control = document.querySelector(valueSelector);
        const value = control ? (control.getAttribute("aria-valuenow") ?? control.value ?? null) : null;
        window.__sliderValues.push({ t: performance.now(), value });
      } catch {}
    }, 10);
  };
  window.__sliderStop = () => {
    clearInterval(window.__sliderTimer);
    clearInterval(window.__sliderValueTimer);
    return { samples: window.__sliderSamples ?? [], values: window.__sliderValues ?? [] };
  };
};

const surfaces = () =>
  page.evaluate(() =>
    [...document.querySelectorAll("[data-status-json]")].map((el) => {
      let meshes = 0;
      try {
        meshes = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]").length;
      } catch {}
      let status = null;
      try {
        status = JSON.parse(el.getAttribute("data-status-json"));
      } catch {}
      return { id: el.getAttribute("data-surface-id"), meshes, bytes: (el.getAttribute("data-meshes-json") ?? "").length, phase: status?.phase };
    }),
  );

const untilSurface = async (pred, seconds) => {
  let value = await surfaces();
  for (let index = 0; index < seconds && !pred(value); index += 1) {
    await page.waitForTimeout(1000);
    value = await surfaces();
  }
  return value;
};
//#endregion 📏️Reading

//#region 🫳️Gesture
/** 🫳️ One drag: press on the thumb, walk the track at ~60 Hz for `dragMs`, release. Real pointer
 * events, because the whole question is what a DRAG costs — a keyboard step or a programmatic
 * `onValueChange` never exercises the gesture the user complained about. */
const dragAcross = async (handle, fraction) => {
  const box = await handle.boundingBox();
  if (!box) return null;
  const track = await (await handle.evaluateHandle((el) => el.closest('[data-slot="slider"]') ?? el.parentElement ?? el)).asElement().boundingBox();
  const rail = track && track.width > box.width ? track : box;
  const y = box.y + box.height / 2;
  const from = box.x + box.width / 2;
  const to = rail.x + rail.width * fraction;
  await page.mouse.move(from, y);
  await page.mouse.down();
  const perStep = dragMs / dragSteps;
  const pressedAt = Date.now();
  for (let step = 1; step <= dragSteps; step += 1) {
    await page.mouse.move(from + ((to - from) * step) / dragSteps, y);
    const owed = pressedAt + step * perStep - Date.now();
    if (owed > 0) await page.waitForTimeout(owed);
  }
  await page.mouse.up();
  return { from, to, y, rail: { x: Math.round(rail.x), width: Math.round(rail.width) } };
};

/** ⌨️ The number field's twin of a drag: the value stream a held spinner produces, at the same
 * ~60 Hz. Driven through the native value setter plus a bubbled `input` event, which is exactly the
 * event React's own `onChange` listens for — so the product's `commitValue` → `dispatchTrigger` →
 * invocation chain runs unchanged. A Playwright `focus`+`press` cannot be used here: the Inspection
 * body is republished on every guest turn, so the input node is replaced faster than an actionability
 * check can settle and every `focus` times out. A `fill` would be ONE change and could never show a
 * coalescing defect.
 * @see https://react.dev/reference/react-dom/components/input */
const stepValue = async (selector, from, direction, step) => {
  const perStep = dragMs / dragSteps;
  const startedAt = Date.now();
  const applied = [];
  for (let index = 1; index <= dragSteps; index += 1) {
    const value = await page.evaluate(
      ([sel, next]) => {
        const element = document.querySelector(sel);
        if (!element) return null;
        const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value")?.set;
        setter?.call(element, String(next));
        element.dispatchEvent(new Event("input", { bubbles: true }));
        return next;
      },
      [selector, Math.round((from + direction * step * index) * 1000) / 1000],
    );
    if (value != null) applied.push(value);
    const owed = startedAt + index * perStep - Date.now();
    if (owed > 0) await page.waitForTimeout(owed);
  }
  return { kind: "value-stream", steps: applied.length, from: applied[0] ?? null, to: applied.at(-1) ?? null };
};

//#endregion 🫳️Gesture

//#region 📊️Metrics
const quantile = (values, q) => {
  if (values.length === 0) return null;
  const sorted = [...values].sort((a, b) => a - b);
  const position = (sorted.length - 1) * q;
  const low = Math.floor(position);
  const high = Math.ceil(position);
  return Math.round(low === high ? sorted[low] : sorted[low] + (sorted[high] - sorted[low]) * (position - low));
};

/** 📐️ Turns one sampled gesture into the five readings. `changes` are the instants the CONTROL moved,
 * `publications` the instants the PAYLOAD moved; a latency pairs each change with the first
 * publication strictly after it, and a change with no publication after it at all is a DROP. */
const measure = (samples, values, ledger, window_) => {
  const changes = [];
  const publications = [];
  for (let index = 1; index < values.length; index += 1) {
    if (values[index].value !== values[index - 1].value && values[index].value != null) changes.push({ t: values[index].t, value: values[index].value });
  }
  for (let index = 1; index < samples.length; index += 1) {
    if (samples[index].digest !== samples[index - 1].digest) publications.push({ t: samples[index].t, digest: samples[index].digest, meshes: samples[index].meshes, bbox: samples[index].bbox });
  }
  const latencies = [];
  let dropped = 0;
  for (const change of changes) {
    const answer = publications.find((publication) => publication.t > change.t);
    if (answer) latencies.push(answer.t - change.t);
    else dropped += 1;
  }
  const inWindow = ledger.filter((entry) => entry.t >= window_.from && entry.t <= window_.to && !entry.settled);
  const byAction = {};
  for (const entry of inWindow) byAction[entry.actionId] = (byAction[entry.actionId] ?? 0) + 1;
  const lastChange = changes.at(-1) ?? null;
  const afterLast = lastChange ? publications.filter((publication) => publication.t > lastChange.t) : [];
  const final = samples.at(-1) ?? null;
  return {
    valueChanges: changes.length,
    firstValue: values.find((entry) => entry.value != null)?.value ?? null,
    lastValue: [...values].reverse().find((entry) => entry.value != null)?.value ?? null,
    publications: publications.length,
    droppedChanges: dropped,
    latencyMedianMs: quantile(latencies, 0.5),
    latencyP95Ms: quantile(latencies, 0.95),
    latencyMaxMs: latencies.length ? Math.round(Math.max(...latencies)) : null,
    evaluations: byAction,
    evaluationsTotal: inWindow.length,
    starts: byAction.toolRunStart ?? 0,
    ticks: byAction.flowEvalTick ?? 0,
    evaluationsPerChange: changes.length > 0 ? Math.round(((byAction.toolRunStart ?? 0) / changes.length) * 100) / 100 : null,
    publicationsAfterLastChange: afterLast.length,
    staleTail: afterLast.length === 0,
    settledDigest: final?.digest ?? null,
    settledBbox: final?.bbox ?? null,
    settledMeshes: final?.meshes ?? null,
    settledPhase: final?.phase ?? null,
    settledNamed: final?.named ?? [],
    fault: final?.fault ?? null,
  };
};
//#endregion 📊️Metrics

//#region 🎬️Run
/** 🚀️ A fresh load, converged, with the example's OWN node graph in place — the node graph publishes
 * a fallback two-widget fixture first, and a slider located before that swap is the wrong slider. */
let example = examples[0];
const boot = async () => {
  await page.goto(`${url}&example=${example}`, { waitUntil: "domcontentloaded" });
  const booted = await untilSurface((state) => state.some((entry) => entry.id === "window:procedural-preview" && entry.meshes > 0 && entry.phase === "idle"), bootSeconds);
  for (let second = 0; second < bootSeconds; second += 1) {
    if (await page.locator('[role="slider"]:not([aria-label="Number"])').count()) break;
    await page.waitForTimeout(1000);
  }
  report.boot = booted;
  flush();
  return booted;
};

/** 🎬️ One measured gesture on one located slider: baseline, drag, settle, drag back, settle. */
const runKind = async (kind, locate) => {
  const row = { kind, example, ok: false };
  // 🧼️ Every kind starts from a fresh load: one shell carries the previous kind's open panels, mode
  // and selection, and a locator that answers differently per predecessor measures the predecessor.
  await boot();
  const located = await locate().catch((error) => {
    row.locateError = String(error).slice(0, 300);
    return null;
  });
  if (!located) {
    row.detail = "slider not reachable";
    report.rows.push(row);
    flush();
    return row;
  }
  const { selector, surfaceId, control, reacquire } = located;
  // 🔁️ Re-resolved AND re-acquired before every gesture: a panel body is republished on each guest
  // turn and its selection can be pruned, so a handle captured at locate time can be detached — and
  // the control it addressed can be gone from the DOM entirely — by the time the gesture runs.
  const handleOf = () => page.locator(control ?? selector).first();
  const ensure = async () => {
    for (let attempt = 0; attempt < 4; attempt += 1) {
      if (await handleOf().count()) return true;
      if (!reacquire) break;
      await reacquire();
    }
    return (await handleOf().count()) > 0;
  };
  row.reacquired = await ensure();
  if (!row.reacquired) {
    row.detail = "control vanished before the gesture";
    row.domAtFailure = await page.evaluate(() => [...document.querySelectorAll('[id*="procedural-play-inspector"], [id*="generate.form."], [role="slider"]')].map((el) => el.id || el.getAttribute("aria-label")).slice(0, 40));
    await page.screenshot({ path: join(outDir, `${example}-${kind}-vanished.png`) });
    report.rows.push(row);
    flush();
    return row;
  }
  await page.evaluate(SAMPLER);
  const baseline = await page.evaluate(([surface, value]) => window.__sliderSample(surface, value), [surfaceId, selector]);
  row.surfaceId = surfaceId;
  row.selector = selector;
  row.baseline = { value: baseline.value, digest: baseline.digest, meshes: baseline.meshes, bbox: baseline.bbox };

  // 📐️ Where the baseline value sits on the rail, so the return gesture lands on the SAME value
  // rather than on an arbitrary fraction — an identity round trip needs an identical input.
  const rail = await page.evaluate((value) => {
    const control = document.querySelector(value);
    const min = Number(control?.getAttribute("aria-valuemin"));
    const max = Number(control?.getAttribute("aria-valuemax"));
    const now = Number(control?.getAttribute("aria-valuenow"));
    return Number.isFinite(min) && Number.isFinite(max) && max > min && Number.isFinite(now) ? { min, max, fraction: (now - min) / (max - min) } : null;
  }, selector);
  row.rail = rail;

  await page.evaluate(([surface, value]) => window.__sliderStart(surface, value), [surfaceId, selector]);
  const beforeDrag = await page.evaluate(() => performance.now());
  if (!(await ensure())) {
    row.detail = "control vanished between the baseline reading and the gesture";
    row.domAtFailure = await page.evaluate(() => [...document.querySelectorAll('[id*="procedural-play-inspector"], [id*="generate.form."], [role="slider"]')].map((el) => el.id || el.getAttribute("aria-label")).slice(0, 40));
    await page.screenshot({ path: join(outDir, `${example}-${kind}-vanished.png`) });
    report.rows.push(row);
    flush();
    return row;
  }
  row.gesture = rail ? await dragAcross(handleOf(), Number(process.env.SEMIO_PROBE_TARGET_FRACTION ?? 0.92)) : await stepValue(selector, Number(baseline.value), 1, Number(process.env.SEMIO_PROBE_STEP ?? 0.05));
  const releasedAt = await page.evaluate(() => performance.now());
  for (let second = 0; second < settleSeconds; second += 1) {
    await page.waitForTimeout(1000);
    const quiet = await page.evaluate(() => {
      const samples = window.__sliderSamples ?? [];
      const tail = samples.slice(-12);
      return tail.length >= 12 && tail.every((entry) => entry.digest === tail[0].digest) && (tail.at(-1).phase === "idle" || tail.at(-1).phase === null);
    });
    if (quiet) break;
  }
  const { samples, values } = await page.evaluate(() => window.__sliderStop());
  const settledAt = await page.evaluate(() => performance.now());
  const ledger = await page.evaluate(() => window.__sliderLedger ?? []);
  row.drag = measure(samples, values, ledger, { from: beforeDrag, to: settledAt });
  row.droppedActions = (await page.evaluate(() => window.__sliderDrops ?? [])).filter((entry) => entry.t >= beforeDrag && entry.t <= settledAt).map((entry) => entry.action);
  row.releaseToSettleMs = Math.round(settledAt - releasedAt);
  row.samples = samples.length;
  writeFileSync(join(outDir, `${example}-${kind}-samples.json`), JSON.stringify({ samples, values }, null, 2));

  const historyBefore = baseline.history;
  const historyAfter = samples.at(-1)?.history ?? null;
  row.history = { before: historyBefore, after: historyAfter, delta: historyBefore?.entries != null && historyAfter?.entries != null ? historyAfter.entries - historyBefore.entries : null };

  // 🔁️ Identical input, identical published payload: the only oracle for "the final mesh matches the
  // final value" that needs no second evaluator.
  await page.evaluate(([surface, value]) => window.__sliderStart(surface, value), [surfaceId, selector]);
  await ensure();
  if (rail) await dragAcross(handleOf(), rail.fraction);
  else await stepValue(selector, Number(baseline.value) + Number(process.env.SEMIO_PROBE_STEP ?? 0.05) * dragSteps, -1, Number(process.env.SEMIO_PROBE_STEP ?? 0.05));
  for (let second = 0; second < settleSeconds; second += 1) {
    await page.waitForTimeout(1000);
    const quiet = await page.evaluate(() => {
      const samples = window.__sliderSamples ?? [];
      const tail = samples.slice(-12);
      return tail.length >= 12 && tail.every((entry) => entry.digest === tail[0].digest) && (tail.at(-1).phase === "idle" || tail.at(-1).phase === null);
    });
    if (quiet) break;
  }
  const backStopped = await page.evaluate(() => window.__sliderStop());
  const back = backStopped.samples.at(-1) ?? null;
  const backValue = [...backStopped.values].reverse().find((entry) => entry.value != null)?.value ?? null;
  row.returned = { value: backValue, digest: back?.digest ?? null, bbox: back?.bbox ?? null, meshes: back?.meshes ?? null };
  row.roundTrip = Boolean(back && backValue === baseline.value && back.digest === baseline.digest);
  row.movedAtAll = row.drag.publications > 0;

  // 🧿️ What the RELEASED value must actually deliver, graded against the committed example oracle
  // (`🐍️example-oracle.mjs`, the same fixture `cargo test --test example-geometry` reads). A slider
  // moves sizes, never topology, so the published mesh COUNT is the value-independent number the
  // oracle commits; the extent must exist and be finite; and the chain must rest in `idle`. A value
  // the kernel genuinely cannot build is not exempt — it owes a NAMED fault instead of an empty
  // payload, which is exactly the "settles empty under idle" defect this row exists to catch
  // (`📓️slider-reevaluation-correctness-2026-09-15.md`).
  const oracle = exampleOracles[example] ?? null;
  const finiteBox = (box) => Array.isArray(box) && box.length === 6 && box.every((entry) => Number.isFinite(entry));
  const releaseVerdict = (settledMeshes, settledBbox, settledPhase, named) => {
    if (Array.isArray(named) && named.length > 0) return { ok: true, why: `named fault: ${named[0]}` };
    if (!oracle) return { ok: false, why: `no committed oracle for ${example}` };
    if (!settledMeshes) return { ok: false, why: `settled EMPTY under phase ${settledPhase} and named nothing` };
    if (settledMeshes !== oracle.meshes) return { ok: false, why: `published ${settledMeshes} meshes, the example commits ${oracle.meshes}` };
    if (!finiteBox(settledBbox)) return { ok: false, why: `published no finite extent (${JSON.stringify(settledBbox)})` };
    if (settledPhase !== "idle" && settledPhase !== null) return { ok: false, why: `carries the geometry but rests in phase ${settledPhase}` };
    return { ok: true, why: "" };
  };
  row.releasedDelivery = releaseVerdict(row.drag.settledMeshes, row.drag.settledBbox, row.drag.settledPhase, row.drag.settledNamed);
  row.returnedDelivery = releaseVerdict(row.returned.meshes, row.returned.bbox, back?.phase ?? null, back?.named ?? []);
  row.oracle = oracle ? { meshes: oracle.meshes, boundingBoxMin: oracle.boundingBoxMin, boundingBoxMax: oracle.boundingBoxMax } : null;

  row.ok = Boolean(
    row.drag.valueChanges >= 2 &&
      row.movedAtAll &&
      row.releasedDelivery.ok &&
      row.returnedDelivery.ok &&
      row.drag.staleTail === false &&
      row.drag.latencyP95Ms != null &&
      row.drag.latencyP95Ms <= latencyBudgetMs &&
      row.drag.evaluationsPerChange != null &&
      row.drag.evaluationsPerChange <= 1 &&
      row.roundTrip &&
      row.history.delta === 1 &&
      row.droppedActions.length === 0,
  );
  report.rows.push(row);
  flush();
  console.log(`[DEBUG] ${example}/${kind} ok=${row.ok} released=${JSON.stringify(row.releasedDelivery)} returned=${JSON.stringify(row.returnedDelivery)} ${JSON.stringify({ ...row.drag, samples: undefined })}`);
  await page.screenshot({ path: join(outDir, `${example}-${kind}.png`) });
  return row;
};
//#endregion 🎬️Run

//#region 🧭️Locators
/** 🕸️ Any inline slider the example's own graph paints. The label is example-specific (`Column
 * Height`, `Torus Major Radius`, …), so the locator addresses the OVERLAY (`graph-slider-…`) and
 * takes the first knob that is not the fallback fixture's bare `Number`. */
const graphSlider = async () => {
  const wanted = process.env.SEMIO_PROBE_GRAPH_SLIDER;
  const label = await page.evaluate((preferred) => {
    const knobs = [...document.querySelectorAll('[role="slider"]')].filter((el) => (el.closest("[id]")?.id ?? "").startsWith("graph-slider-"));
    const chosen = (preferred ? knobs.find((el) => el.getAttribute("aria-label") === preferred) : undefined) ?? knobs.find((el) => el.getAttribute("aria-label") !== "Number") ?? knobs[0];
    return chosen?.getAttribute("aria-label") ?? null;
  }, wanted ?? null);
  if (!label) throw new Error("this example paints no inline graph slider");
  const selector = `[role="slider"][aria-label="${label.replace(/"/g, '\\"')}"]`;
  const handle = page.locator(selector).first();
  await handle.waitFor({ state: "visible", timeout: 30000 });
  await handle.scrollIntoViewIfNeeded().catch(() => {});
  return { handle, selector, surfaceId: "window:procedural-preview" };
};

/** 🔍️ The inspector's number field for one `InputSlider` widget. The Artifact panel's own graph row
 * is what selects the widget, and selecting it is what fills the Inspection body — clicking the
 * Inspection TAB afterwards folds the tab away again, which is how an earlier reading found nothing.
 * The rendered DOM id is namespaced by its panel (`panel:<panelId>/<authored id>`). */
const inspectorField = async () => {
  const widgetId = process.env.SEMIO_PROBE_WIDGET ?? "height";
  const selector = '[id$="procedural-play-inspector.value.input"]';
  await page.locator('[id="framework.panel.artifact"]').first().click({ timeout: 8000 }).catch(() => {});
  await page.waitForTimeout(1800);
  for (let attempt = 0; attempt < 4; attempt += 1) {
    if (await page.locator(selector).count()) break;
    await page.locator(`[data-slot="panel"] [id="panel:procedural-play-graph/${widgetId}"]`).first().click({ timeout: 8000 }).catch(() => {});
    await page.waitForTimeout(3000);
  }
  const handle = page.locator(selector).first();
  await handle.waitFor({ state: "visible", timeout: 20000 });
  return {
    handle,
    selector,
    surfaceId: "window:procedural-preview",
    reacquire: async () => {
      await page.locator(`[data-slot="panel"] [id="panel:procedural-play-graph/${widgetId}"]`).first().click({ timeout: 8000 }).catch(() => {});
      await page.waitForTimeout(3000);
    },
  };
};

/** 🧬️ A generate-mode Form `slider` question. The Form renders its controls only once a generation
 * exists and is the selected one, so the roster is polled rather than waited on by a fixed sleep. */
const formSlider = async () => {
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await untilSurface((state) => state.some((entry) => entry.id === "window:generation3d-generate-preview"), 40);
  const rosterRows = () => page.evaluate(() => [...document.querySelectorAll('[id*="procedural3d-play-generate.generation."]')].filter((el) => !el.id.endsWith(".rename")).map((el) => el.id));
  const controls = () => page.locator('[id$=".slider"] [role="slider"]');
  for (let attempt = 0; attempt < 3; attempt += 1) {
    if (await controls().count()) break;
    if ((await rosterRows()).length === 0) {
      const add = page.locator(':text-is("Add Generation")').first();
      if (await add.count()) await add.click({ timeout: 6000 }).catch(() => {});
      for (let second = 0; second < 90; second += 1) {
        await page.waitForTimeout(1000);
        if ((await rosterRows()).length > 0) break;
      }
    }
    const rows = await rosterRows();
    if (rows.at(-1)) {
      await page.locator(`[id="${rows.at(-1)}"]`).first().click({ timeout: 6000 }).catch(() => {});
      for (let second = 0; second < 60; second += 1) {
        await page.waitForTimeout(1000);
        if (await controls().count()) break;
      }
    }
  }
  await untilSurface((state) => state.some((entry) => entry.id === "window:generation3d-generate-preview" && entry.meshes > 0), 120);
  const handle = controls().first();
  await handle.waitFor({ state: "visible", timeout: 30000 });
  const owner = await handle.evaluate((el) => el.closest('[id$=".slider"]')?.id ?? null);
  return { handle, selector: owner ? `[id="${owner}"] [role="slider"]` : '[role="slider"]', surfaceId: "window:generation3d-generate-preview" };
};
//#endregion 🧭️Locators

const locators = { graph: graphSlider, inspector: inspectorField, form: formSlider };
for (const entry of examples) {
  example = entry;
  for (const kind of kinds) {
    if (!locators[kind]) continue;
    await runKind(kind, locators[kind]).catch((error) => {
      report.rows.push({ kind, example, ok: false, runError: String(error).slice(0, 400) });
      flush();
    });
  }
}

report.faults = lines.filter((line) => /pageerror|SemioFaultError|Rejected|dispatch failed|command failed/i.test(line)).slice(0, 40);
report.green = report.rows.filter((row) => row.ok).length;
report.red = report.rows.filter((row) => !row.ok).map((row) => `${row.example}/${row.kind}`);
flush();
console.log(`SLIDER LIVE PREVIEW DONE green=${report.green}/${report.rows.length} red=${JSON.stringify(report.red)} faults=${report.faults.length}`);
await browser.close();
