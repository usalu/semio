/** 🔋 wgpu end-to-end battery for the generation3d playground on 6118 — the twin of `🐍️react-battery.mjs`.
 *
 * Runs every wgpu probe in this ticket SEQUENTIALLY — one headless browser at a time, against ONE
 * build of the renderer wasm + frame worker + staged guest — and writes a single scoreboard, so
 * "procedural 3d works end to end on wgpu" is one artifact rather than a dozen lane reports each
 * proven on its own build.
 *
 * Each probe lands in `🗑️generated/wgpu-verify/<probe>/`; the battery then reads that probe's OWN
 * published evidence (`results.json` / `verdict.json` / `verdicts.json` / `report.json` / `console.txt`)
 * and turns it into pass-fail steps. A probe is never called green from its exit code alone: every
 * verdict below names the number it read.
 *
 * ⚡️ The wgpu host tick is INPUT-DRIVEN, so every probe it runs nudges the pointer rather than
 * sleeping; nothing in this file waits on wall clock beyond a probe's own budget.
 *
 * Usage:
 *   cd <ticket> && bun 🐍️wgpu-battery.mjs                      # everything
 *   cd <ticket> && bun 🐍️wgpu-battery.mjs --only=boot,examples  # a subset, comma separated
 *   cd <ticket> && bun 🐍️wgpu-battery.mjs --list
 *
 * Env: SEMIO_BATTERY_URL (default http://127.0.0.1:6118/?plugin=generation3d).
 * @see 🐍️react-battery.mjs, 📓️wgpu-end-to-end-verification-2026-09-14.md
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const T = import.meta.dir;
const GEN = join(T, "🗑️generated");
const ROOT = join(GEN, "wgpu-verify");
const URL = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const ORIGIN = URL.split("/?")[0];
const argv = process.argv.slice(2);
const only = (argv.find((a) => a.startsWith("--only=")) ?? "").slice("--only=".length).split(",").filter(Boolean);

/** 🧯 Lines that mean the wgpu shell hurt itself, whichever probe produced them. `unmapped effect` is
 * in because that is exactly how this target drops a verb silently (`spawn-job`, `request-file-open`). */
const FAULT_RE = /pageerror|panicked|wgpu-worker panicked|RuntimeError|surface fault|frame fault recorded|quarantin|watchdog|authority faulted|deferred action failed|unmapped effect|ItemCredits|BoundedActionFault|graph move fault|refreshUi failed/i;

const readJson = (path) => { try { return JSON.parse(readFileSync(path, "utf8")); } catch { return null; } };
const readText = (path) => { try { return readFileSync(path, "utf8"); } catch { return ""; } };
/** 🪶 The fault lines a reader needs, deduplicated and capped so the scoreboard stays legible. */
const faultLines = (text, cap = 10) => [...new Set(text.split("\n").filter((l) => FAULT_RE.test(l)).map((l) => l.trim().slice(0, 260)))].slice(0, cap);
const countPageErrors = (text) => text.split("\n").filter((l) => /\bpageerror\b/.test(l)).length;
const consoleOf = (dir, ...extra) => [join(dir, "console.txt"), ...extra].map(readText).join("\n");

/** 📚️ The eight bundled examples of `s.procedural.generation3d@1/*`, plus React's own `No example` lane. */
const EXAMPLES = ["hexagonal-mushroom-column", "rectangle-extrude-volume", "rectangle-wire-preview", "box-shell-preview", "box-fillet-preview", "sphere-cut-with-torus", "sphere-box-fuse", "face-sweep-extrude"];

/** 📋 One row per probe: how to run it, where its evidence lands, and how to read a verdict out of it. */
const PROBES = [
  {
    name: "boot", script: "🐍️wgpu-journey-probe.mjs", dir: "boot", minutes: 8,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/boot", SEMIO_PROBE_ONLY: "boot", SEMIO_PROBE_BOOT: "150" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const boot = r.boot ?? {};
      const census = r.consoleCensus ?? {};
      return {
        steps: [
          { step: "the shell reaches a live window", ok: Boolean(boot.booted), detail: { seconds: boot.seconds, windowIds: boot.windowIds } },
          { step: "the dock plans its windows", ok: (census.dockPlan ?? 0) > 0, detail: { dockPlan: census.dockPlan, renderBegin: census.renderBegin } },
          /** 🩺️ The probe's own `failureLines` greps the bare word `faulted`, which the World3d census
           * line carries as `apply-faulted=false` on EVERY healthy frame — so the real predicate is the
           * fault TRACES themselves, not that grep. */
          { step: "no surface fault", ok: !/wgpu-shell surface fault surface=|renderDocument promise failed|\bpanicked\b/.test(console_), detail: { faults: faultLines(console_, 3) } },
        ],
        key: { bootSeconds: boot.seconds, windows: (boot.windowIds ?? []).length, pointerHits: census.pointerHits, world3dSurface: census.world3dSurface },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "examples", script: "🐍️wgpu-example-matrix-probe.mjs", dir: "examples", minutes: 60,
    /** ⏲️ `MIN_SECONDS` is the settle FLOOR this lane added: convergence answers the first sample that
     * carries a drawable, and generation3d publishes its port-preview WIRES ~7 s before the guest
     * finishes tessellating the solid, so a run that stops at first convergence reports a census the
     * user has not finished seeing (hex measured `meshes 2 → 3` at 30.2 s → 37.1 s). */
    env: { SEMIO_PROBE_OUT: "wgpu-verify/examples", SEMIO_PROBE_ORIGIN: ORIGIN, SEMIO_PROBE_LANES: "edit,viewer", SEMIO_PROBE_EXAMPLES: EXAMPLES.join(","), SEMIO_PROBE_BUDGET: "200", SEMIO_PROBE_MIN_SECONDS: "75" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const rows = r.results ?? [];
      const meshes = (row) => (row.meshSurfaces ?? []).reduce((n, s) => Math.max(n, s.meshes ?? 0), 0);
      const steps = rows.map((row) => ({
        step: `${row.lane}: ${row.example}`, ok: row.verdict === "pass",
        detail: { seconds: row.timeToMeshSeconds, meshes: meshes(row), instances: (row.meshSurfaces ?? [])[0]?.instances ?? 0, lines: (row.meshSurfaces ?? [])[0]?.lines ?? 0, body: row.geometry?.body ?? null, alert: row.alert },
      }));
      return {
        steps,
        key: {
          rows: rows.length, passed: rows.filter((row) => row.verdict === "pass").length,
          perExampleSeconds: Object.fromEntries(rows.map((row) => [`${row.lane}:${row.example}`, row.timeToMeshSeconds])),
          meshesByLane: Object.fromEntries(rows.map((row) => [`${row.lane}:${row.example}`, meshes(row)])),
        },
        pageerrors: 0, faults: [],
      };
    },
  },
  {
    name: "no-example", script: "🐍️wgpu-example-matrix-probe.mjs", dir: "no-example", minutes: 10,
    /** 🕳️ React's own `No example` row: the plugin booted with no `?example=` at all must paint a
     * preview that carries NOTHING. The matrix probe's `converged` predicate demands a drawable, so
     * here the PASS is its refusal — inverted below, never read as a failure. */
    env: { SEMIO_PROBE_OUT: "wgpu-verify/no-example", SEMIO_PROBE_ORIGIN: ORIGIN, SEMIO_PROBE_LANES: "edit,viewer", SEMIO_PROBE_EXAMPLES: "(none)", SEMIO_PROBE_BUDGET: "60" },
    verdict: (dir) => {
      const rows = (readJson(join(dir, "results.json")) ?? {}).results ?? [];
      return {
        /** 📐️ "Nothing" is `geometry === null` — no published mesh whose `positions`/`edgePositions`
         * actually start with a number — and no instance. The preview still CARRIES two entries with
         * empty arrays (generation3d always ships its `@wire#0` companions), so counting entries would
         * score an empty preview as a painted one. */
        steps: rows.map((row) => ({
          step: `${row.lane}: No example paints no body`, ok: row.verdict === "fail" && row.geometry === null && (row.meshSurfaces ?? []).every((s) => (s.instances ?? 0) === 0) && !row.alert && !row.bootExampleTrace,
          detail: { geometry: row.geometry, meshSurfaces: row.meshSurfaces, bootExample: row.bootExampleTrace, alert: row.alert },
        })),
        key: { rows: rows.length }, pageerrors: 0, faults: [],
      };
    },
  },
  {
    name: "generate-add", script: "🐍️wgpu-add-generation-probe.mjs", dir: "generate-add", minutes: 20,
    /** 🔤️ `SEMIO_PROBE_ROW` is the PATH needle, and the published path spells the row
     * `procedural3d-play-generate.add-generation` — the probe's own `addGeneration` default matches
     * neither that path nor the row's (absent) text. */
    env: { SEMIO_PROBE_OUT: "wgpu-verify/generate-add", SEMIO_PROBE_ORIGIN: ORIGIN, SEMIO_PROBE_ROW: "add-generation", SEMIO_PROBE_EXAMPLES: "hexagonal-mushroom-column,box-shell-preview", SEMIO_PROBE_SETTLE: "60", SEMIO_PROBE_AFTER: "120" },
    verdict: (dir) => {
      const rows = (readJson(join(dir, "results.json")) ?? {}).results ?? [];
      const console_ = rows.map((row) => readText(join(dir, row.example, "console.txt"))).join("\n");
      return {
        steps: rows.map((row) => ({
          step: `Add Generation → mesh: ${row.example}`, ok: row.verdict === "pass",
          detail: { verdict: row.verdict, dispatched: row.dispatched, seconds: row.timeToMeshSeconds, after: row.after?.meshy ?? null, target: row.target?.path ?? null },
        })),
        key: { rows: rows.length, passed: rows.filter((row) => row.verdict === "pass").length },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "generation-roster", script: "🐍️wgpu-generation-publication-probe.mjs", dir: "generation-roster", minutes: 14,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/generation-roster", SEMIO_PROBE_ORIGIN: ORIGIN, SEMIO_PROBE_GUEST_DIAGNOSTICS: "1", SEMIO_PROBE_JOURNEY: "1" },
    verdict: (dir) => {
      const v = readJson(join(dir, "verdicts.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const world = Object.values(v.world3d ?? {})[0] ?? {};
      const journey = v.journey ?? {};
      return {
        steps: [
          { step: "the retained row dispatches addGeneration", ok: (v.retainedPress ?? 0) > 0 && (v.typedOperation ?? 0) > 0, detail: { retainedPress: v.retainedPress, typedOperation: v.typedOperation } },
          { step: "the roster grows a generation", ok: Boolean(v.rosterGrew), detail: { before: (v.rosterBefore ?? []).length, after: (v.rosterAfter ?? []).length } },
          { step: "the Form binds that generation's inputs", ok: (v.formAfter ?? []).length > (v.formBefore ?? []).length, detail: { formBefore: (v.formBefore ?? []).length, formAfter: (v.formAfter ?? []).length } },
          { step: "the generate preview carries a mesh", ok: (world.stateMeshes ?? 0) > 0 && ((world.instances ?? 0) > 0 || (world.lines ?? 0) > 0), detail: world },
          { step: "a second Add lands a second row, and selecting it moves the editor", ok: (journey.rosterAfterSecondAdd ?? []).length > (journey.rosterAfterFirstAdd ?? []).length, detail: { first: journey.rosterAfterFirstAdd, second: journey.rosterAfterSecondAdd } },
        ],
        key: { rosterAfter: (v.rosterAfter ?? []).length, formAfter: (v.formAfter ?? []).length, scenePassesAfter: v.scenePassesAfter, world3d: v.world3d },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "deferred-commit", script: "🐍️wgpu-deferred-commit-probe.mjs", dir: "deferred-commit", minutes: 20,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/deferred-commit", SEMIO_PROBE_ORIGIN: ORIGIN, SEMIO_PROBE_SETTLE: "60" },
    verdict: (dir) => {
      const v = readJson(join(dir, "verdicts.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const rename = v.rename ?? {};
      const slider = v.slider ?? {};
      return {
        steps: [
          { step: "rename commits and the guest runs it", ok: Boolean(rename.committed) && (rename.hops?.guestRanTheCommand ?? 0) > 0, detail: { shown: rename.shown, hops: rename.hops } },
          { step: "a Form slider drag reaches the guest", ok: Boolean(slider.found) && (slider.hops?.guestRanTheCommand ?? slider.hops?.admittedByTheGuest ?? 0) > 0, detail: { path: slider.path, hops: slider.hops } },
          { step: "the preview PIXELS move with the slider", ok: (slider.previewPixels?.fraction ?? 0) > 0.05 && (slider.previewPixelsIdleControl?.fraction ?? 1) < 0.01, detail: { moved: slider.previewPixels, idleControl: slider.previewPixelsIdleControl, censusChanged: slider.censusChanged } },
        ],
        key: { renameHops: rename.hops, sliderHops: slider.hops, sliderPixelFraction: slider.previewPixels?.fraction ?? null },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "world3d-editor", script: "🐍️wgpu-world3d-interaction-probe.mjs", dir: "world3d-editor", minutes: 16,
    /** ⏳️ SETTLE is 60 s, not the probe's 15 s default, and the example is on the url. Boot now leaves
     * in ~6 s where the lane that wrote this probe measured ~27 s, so a short settle starts gesturing at
     * a preview whose guest has published NO geometry yet (measured: `lanes.meshes=2` bytes,
     * `Pick[Hover hit=false]`) and every pick legitimately misses. Edit-mode time-to-mesh is 10-36 s. */
    env: { SEMIO_PROBE_OUT: "wgpu-verify/world3d-editor", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&example=hexagonal-mushroom-column`, SEMIO_PROBE_BOOT: "150", SEMIO_PROBE_SETTLE: "60" },
    verdict: (dir) => world3dVerdict(dir),
  },
  {
    name: "world3d-viewer", script: "🐍️wgpu-world3d-interaction-probe.mjs", dir: "world3d-viewer", minutes: 16,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/world3d-viewer", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&role=viewer&example=hexagonal-mushroom-column`, SEMIO_PROBE_BOOT: "150", SEMIO_PROBE_SETTLE: "60" },
    verdict: (dir) => world3dVerdict(dir),
  },
  {
    name: "spawn-job", script: "🐍️wgpu-spawn-job-probe.mjs", dir: "spawn-job", minutes: 16,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/spawn-job", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&example=hexagonal-mushroom-column`, SEMIO_PROBE_BOOT: "150", SEMIO_PROBE_SETTLE: "60" },
    /** 🧵️ A selection is delivered as `Effect::SpawnJob{framework.reserved.tool}`; the verdict is that
     * the bridge PUMPED it (`wgpu-bridge spawn-job …`) and dropped none, not that a wire line exists. */
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = Object.entries(r.steps ?? {}).filter(([name]) => name.startsWith("s")).map(([name, step]) => ({
        step: name, ok: (step.newPumps ?? []).length > 0 || (step.newActions ?? []).length > 0,
        detail: { pumps: (step.newPumps ?? []).length, drops: step.newDrops ?? 0, actions: (step.newActions ?? []).length, hover: step.guestHoverTarget, selected: step.guestSelected },
      }));
      const counts = r.steps?.counts ?? {};
      steps.push({ step: "no dropped spawn-job effect", ok: (counts['unmapped effect "spawn-job"'] ?? 0) === 0 && (counts["unmapped effect"] ?? 0) === 0, detail: { unmapped: counts["unmapped effect"] } });
      steps.push({ step: "no panic, no dispatch failure", ok: (counts.panicked ?? 0) === 0 && (counts["wgpu-worker panicked"] ?? 0) === 0 && (counts["frame deferred action failed"] ?? 0) === 0, detail: counts });
      return { steps, key: counts, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "frame-loop", script: "🐍️wgpu-frame-loop-probe.mjs", dir: "frame-loop", minutes: 25,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/frame-loop", SEMIO_PROBE_URL: URL, SEMIO_PROBE_GESTURES: "20", SEMIO_PROBE_BOOT: "150", SEMIO_PROBE_GESTURE_SETTLE: "4" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps_ = r.steps ?? [];
      const last = steps_.at(-1) ?? {};
      const gestures = steps_.filter((s) => /^g\d+_/.test(s.name ?? ""));
      const quarantines = steps_.reduce((n, s) => n + (s.wire?.quarantines ?? []).length, 0);
      return {
        steps: [
          { step: `${gestures.length} gestures run`, ok: gestures.length >= 20, detail: { gestures: gestures.length } },
          { step: "the frame wire never quarantines", ok: quarantines === 0, detail: { quarantines } },
          /** 📬️ The final sample is taken while the wire is still turning, so ONE batch may legitimately
           * be in flight when it lands (`batches 6779 / frames 6778`). A STOP is the shape the
           * frame-loop lane named — a batch with no answer AND no further batch — so the predicate is
           * that the wire is at most one frame behind and carries no fault, not that the last byte
           * happened to be a reply. */
          { step: "the frame wire is still answering", ok: (last.wire?.batches ?? 0) - (last.wire?.frames ?? 0) <= 1 && (last.wire?.faults ?? []).length === 0, detail: { batches: last.wire?.batches, frames: last.wire?.frames, answered: last.wire?.lastBatchAnswered, faults: (last.wire?.faults ?? []).length } },
          { step: "the loop keeps publishing actions", ok: (last.actions ?? 0) > 0, detail: { actions: last.actions, admits: last.admits, sweeps: last.sweeps } },
        ],
        key: { batches: last.wire?.batches, frames: last.wire?.frames, actions: last.actions, admits: last.admits, sweeps: last.sweeps, quarantines },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "node-gestures", script: "🐍️wgpu-node-gestures-probe.mjs", dir: "node-gestures", minutes: 25,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/node-gestures", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&example=hexagonal-mushroom-column` },
    verdict: (dir) => {
      const v = readJson(join(dir, "verdict.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const c = v.counts ?? {};
      return {
        steps: [
          { step: "the graph publishes its geometry census", ok: (v.census?.nodesTotal ?? 0) > 0, detail: { nodes: v.census?.nodesTotal, onScreen: v.census?.nodesOnScreen, ports: Object.keys(v.found?.ports ?? {}).length } },
          { step: "a node is selected", ok: (c["action=interactionSelect"] ?? 0) > 0, detail: { interactionSelect: c["action=interactionSelect"] } },
          { step: "a node is dragged / a wire is drawn or cut", ok: (c["action=nodeGraphEdit"] ?? 0) > 0, detail: { nodeGraphEdit: c["action=nodeGraphEdit"], connected: c["dag edge connected"], removed: c["dag edge removed"] } },
          { step: "a wire is connected", ok: (c["dag edge connected"] ?? 0) > 0, detail: { connected: c["dag edge connected"] } },
          { step: "a wire is cut", ok: (c["dag edge removed"] ?? 0) > 0, detail: { removed: c["dag edge removed"] } },
          { step: "the minimap moves the camera", ok: (c["action=nodeGraphViewport"] ?? 0) > 0, detail: { viewport: c["action=nodeGraphViewport"] } },
          { step: "Fit graph publishes a camera", ok: (c["shell node-graph fit"] ?? 0) > 0, detail: { fit: c["shell node-graph fit"] } },
          { step: "the dense sweep never faults", ok: (c.ItemCredits ?? 0) === 0 && (c.BoundedActionFault ?? 0) === 0 && (c["graph move fault"] ?? 0) === 0 && (c.panicked ?? 0) === 0, detail: { ItemCredits: c.ItemCredits, BoundedActionFault: c.BoundedActionFault, graphMoveFault: c["graph move fault"], panicked: c.panicked } },
        ],
        key: { ...c, sweepPoints: v.sweep?.points, acknowledged: v.sweep?.acknowledgedMoves, seconds: v.seconds },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "chrome", script: "🐍️wgpu-input-deliverables-probe.mjs", dir: "chrome", minutes: 16,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/chrome", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&example=box-fillet-preview`, SEMIO_PROBE_BOOT: "150", SEMIO_PROBE_SETTLE: "45" },
    /** 🖱️ A mode/role chord is a SHELL state transition, not a guest command, so the honest witness is
     * the dock REPLAN it causes (`📓️wgpu-input-hit-runtime-2026-09-13.md` §10.4), never a settled command. */
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const s = r.steps ?? {};
      const planKey = (plan) => Object.keys(plan ?? {}).sort().join(",");
      const chords = s.d6a_chords_idle?.chordWitness ?? [];
      const controls = [...new Set((console_.match(/hit=Some\(\([A-Za-z]+, Some\("([^"]+)"\)/g) ?? []).map((m) => /Some\("([^"]+)"\)/.exec(m)?.[1]).filter(Boolean))];
      return {
        steps: [
          { step: "the navbar answers its own control ids", ok: controls.some((id) => id.startsWith("playground.navbar.")), detail: { controls: controls.slice(0, 14) } },
          { step: "the example picker row is reachable", ok: controls.some((id) => id.includes("navbar.fixture") || id.startsWith("shell.example.")), detail: { picker: controls.filter((id) => id.includes("fixture") || id.startsWith("shell.example.")).slice(0, 6) } },
          { step: "the mode/role buttons are reachable", ok: controls.some((id) => id.includes("navbar.modes.")) && controls.some((id) => id.includes("navbar.roles.")), detail: { modes: controls.filter((id) => id.includes("navbar.modes.")), roles: controls.filter((id) => id.includes("navbar.roles.")) } },
          { step: "a chord replans the dock", ok: chords.length > 0 && chords.some((c) => (c.renderDelta ?? 0) > 0 || (c.sessionSwitchDelta ?? 0) > 0 || (c.newCommands ?? []).length > 0), detail: { chords } },
          { step: "the world3d cancel control is declared", ok: /shell\.world3d\.cancel|world3d cancel/.test(console_) || Boolean(s.d5_cancel ?? s.cancel), detail: { observed: /shell\.world3d\.cancel/.test(console_) } },
        ],
        key: { controls: controls.length, chords: chords.length, previewLanes: s.preview?.lanes ?? null },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "catalogue", script: "🐍️wgpu-catalogue-probe.mjs", dir: "catalogue", minutes: 8,
    /** 🛍️ This probe nests its own run under `wgpu-catalogue/`, so its `SEMIO_PROBE_OUT` is a leaf name
     * and the collected evidence is copied back into the battery's own folder below. */
    env: { SEMIO_PROBE_OUT: "verify", SEMIO_PROBE_URL: URL, SEMIO_PROBE_SECONDS: "110" },
    from: join(GEN, "wgpu-catalogue", "verify"),
    verdict: (dir) => {
      const r = readJson(join(dir, "report.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      return {
        steps: [
          { step: "the app-static catalogue parses", ok: (r.ready ?? []).length > 0 && (r.parseFailed ?? []).length === 0, detail: { ready: (r.ready ?? [])[0] ?? null, parseFailed: (r.parseFailed ?? []).slice(0, 2) } },
          { step: "no UiFixedMap refusal", ok: (r.fixedMapRefusals ?? []).length === 0, detail: { refusals: (r.fixedMapRefusals ?? []).slice(0, 2) } },
          { step: "the catalogue panel renders", ok: (r.catalogueRenders ?? 0) > 0, detail: { renders: r.catalogueRenders } },
        ],
        key: { bytes: /bytes=(\d+)/.exec((r.ready ?? [])[0] ?? "")?.[1] ?? null, renders: r.catalogueRenders, lines: r.lines },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "status-a11y-i18n", script: "🐍️wgpu-status-a11y-i18n-probe.mjs", dir: "status-a11y-i18n", minutes: 22,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/status-a11y-i18n", SEMIO_PROBE_URL: URL, SEMIO_PROBE_BOOT: "200", SEMIO_PROBE_SETTLE: "60" },
    verdict: (dir) => {
      const r = readJson(join(dir, "report.json")) ?? {};
      const german = readJson(join(dir, "german.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: s.verdict === "pass", detail: { ...s, step: undefined, verdict: s.verdict } }));
      return {
        steps,
        key: {
          ariaNodes: (r.steps ?? []).find((s) => s.step === "accessibility:mirror")?.nodeCount ?? null,
          germanViaPalette: (german.viaPalette?.germanFound ?? []).length,
          germanViaBoot: (german.viaBoot?.germanFound ?? []).length,
          pillPhases: (r.steps ?? []).find((s) => s.step === "status:pill-while-computing")?.phases ?? null,
        },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "io", script: "🐍️wgpu-io-probe.mjs", dir: "io", minutes: 14,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/io", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&example=hexagonal-mushroom-column`, SEMIO_PROBE_BOOT: "150" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const step = (name) => (r.steps ?? []).find((s) => s.step === name) ?? {};
      const exportSecond = step("export:second-chord").hops ?? {};
      const importRequest = step("import:chord").hops?.request ?? {};
      const leftover = (console_.match(/effects leftover \d+ tags=[^\n]*/g) ?? []).slice(-3);
      const importStep = step("import:chord");
      const anchors = r.witness?.anchors ?? [];
      // 📄️ A real ASCII STL starts `solid `. The default export format of this artifact is `stl`, so the
      // magic is the file's own, not the probe's expectation of one — a base64-text save would read
      // `c29saWQ…` instead, which is exactly what the encoding lane found on the other renderer.
      const stlAnchor = anchors.find((anchor) => String(anchor.hex ?? "").startsWith("73 6f 6c 69 64"));
      return {
        steps: [
          { step: "mod+shift+e reaches the guest as exportDocument", ok: (exportSecond.admittedByTheGuest ?? 0) > 0, detail: exportSecond },
          { step: "mod+o reaches the guest as importDocumentRequest", ok: (importRequest.admittedByTheGuest ?? 0) > 0, detail: importRequest },
          { step: "no dispatch failed on either verb", ok: (exportSecond.dispatchFailed ?? 0) === 0 && (importRequest.dispatchFailed ?? 0) === 0, detail: { export: exportSecond.dispatchFailed, import: importRequest.dispatchFailed } },
          // 👤️ The USER-FACING half. Both verbs crossing to the guest is the OLD verdict and it hid two
          // whole holes: the download was stashed in the bridge and the picker never existed, so this
          // lane read green while neither journey reached a person
          // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-io-effects-2026-09-14.md`).
          { step: "the export hands the user real bytes", ok: Boolean(stlAnchor) && (stlAnchor?.bytes ?? 0) > 0, detail: { anchors: anchors.map((anchor) => ({ name: anchor.download, type: anchor.type, bytes: anchor.bytes, hex: anchor.hex })), downloads: (r.downloads ?? []).length } },
          { step: "the import opens a real file picker", ok: (r.witness?.inputs ?? []).length > 0 || (r.fileChoosers ?? []).length > 0, detail: { inputs: r.witness?.inputs ?? [], choosers: (r.fileChoosers ?? []).length } },
          { step: "the picked file REPLACES the graph", ok: (importStep.importedNodes ?? []).length === 3, detail: { importedNodes: importStep.importedNodes ?? [], graphChanged: importStep.graphChanged } },
        ],
        key: { counts: r.counts, downloads: (r.downloads ?? []).length, fileChoosers: (r.fileChoosers ?? []).length, anchors: anchors.length, fileInputs: (r.witness?.inputs ?? []).length, leftover, requestFileOpenDropped: (console_.match(/unmapped effect "request-file-open"/g) ?? []).length },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "port-fit", script: "🐍️port-fit-probe.mjs", dir: "port-fit", minutes: 14,
    env: { SEMIO_PROBE_OUT: "wgpu-verify/port-fit", SEMIO_PROBE_TARGET: "wgpu", SEMIO_PROBE_SETTLE: "60", SEMIO_PROBE_URL: `${ORIGIN}/?plugin=generation3d&example=hexagonal-mushroom-column` },
    verdict: (dir) => {
      const findings = readJson(join(dir, "findings.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      return {
        steps: findings.map((f) => ({ step: f.id, ok: Boolean(f.ok), detail: { evidence: String(f.detail ?? "").slice(0, 240) } })),
        key: { findings: findings.length, green: findings.filter((f) => f.ok).length },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
];

/** 🕹️ Both world3d lanes read the same nine hops: a gesture is green when the frame PUBLISHED an
 * action for it, which is the authority's own `[DEBUG] frame input action` line and nothing weaker. */
function world3dVerdict(dir) {
  const r = readJson(join(dir, "results.json")) ?? {};
  const console_ = readText(join(dir, "console.txt"));
  const s = r.steps ?? {};
  const acted = (name, needle) => {
    const step = s[name];
    if (!step) return { step: name, ok: false, detail: { missing: true } };
    const actions = (step.newActions ?? []).map((line) => String(line).split("/args=")[0].split("/").at(-1));
    return { step: name, ok: actions.some((action) => action === needle), detail: { actions: [...new Set(actions)], camera: step.authorityAfter?.camera ?? null, hover: step.authorityAfter?.hover ?? null } };
  };
  const counts = s.counts ?? {};
  const steps = [
    acted("h1_hover_centre", "interactionHover"),
    acted("h2_hover_empty", "interactionHover"),
    acted("h3_click_select", "interactionSelect"),
    /** 🔶️ A shift-click lands ADDITIVELY unless the click point is on the gumball a previous selection
     * put there, in which case the shell publishes that gumball's own verb — so this hop is green for
     * either, and the gumball answer is recorded rather than scored as a miss. */
    (() => {
      const additive = acted("h4_shift_add", "interactionSelect");
      const gumball = acted("h4_shift_add", "translateSelection");
      return additive.ok ? additive : { ...gumball, step: "h4_shift_add" };
    })(),
    acted("h5_empty_clear", "interactionSelect"),
    acted("h6_marquee", "interactionSelect"),
    acted("h7_wheel_zoom", "setCamera"),
    acted("h8_right_drag_orbit", "setCamera"),
    acted("h9_right_drag_pan", "setCamera"),
    { step: "no authority fault, no panic, no dropped effect", ok: (counts["world3d retained interaction authority faulted"] ?? 0) === 0 && (counts.panicked ?? 0) === 0 && (counts["wgpu-worker panicked"] ?? 0) === 0 && (counts["frame deferred action failed"] ?? 0) === 0 && (counts["unmapped effect"] ?? 0) === 0, detail: counts },
  ];
  return { steps, key: { ...counts, previewLanes: s.preview?.lanes ?? null, bootMs: s.boot?.bootShellLeaveMs ?? null }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
}

if (argv.includes("--list")) { console.log(PROBES.map((p) => p.name).join("\n")); process.exit(0); }

const selected = only.length ? PROBES.filter((p) => only.includes(p.name)) : PROBES;
if (only.length && selected.length !== only.length) { console.error(`unknown probe(s): ${only.filter((n) => !PROBES.some((p) => p.name === n)).join(", ")}`); process.exit(2); }

mkdirSync(ROOT, { recursive: true });
const scoreboardPath = join(ROOT, "scoreboard.json");
const previous = readJson(scoreboardPath);
const board = { schema: "semio.procedural3d.wgpu-battery/1", url: URL, startedAt: new Date().toISOString(), probes: [] };
if (previous?.probes && only.length) board.probes = previous.probes.filter((p) => !only.includes(p.probe));

const summarize = () => ({
  probes: board.probes.length,
  green: board.probes.filter((p) => p.ok).length,
  red: board.probes.filter((p) => !p.ok).map((p) => p.probe),
  totalSeconds: Math.round(board.probes.reduce((n, p) => n + p.seconds, 0)),
  pageerrors: board.probes.reduce((n, p) => n + (Number(p.pageerrors) || 0), 0),
});
const flush = () => writeFileSync(scoreboardPath, JSON.stringify({ ...board, finishedAt: new Date().toISOString(), summary: summarize() }, null, 2));

for (const probe of selected) {
  const dir = join(ROOT, probe.dir);
  rmSync(dir, { recursive: true, force: true });
  mkdirSync(dir, { recursive: true });
  const started = Date.now();
  console.log(`\n[DEBUG] battery ▶ ${probe.name} (${probe.script}) budget=${probe.minutes}m`);
  const run = spawnSync("bun", [probe.script, ...(probe.args ?? [])], {
    cwd: T, encoding: "utf8", timeout: probe.minutes * 60_000, maxBuffer: 256 * 1024 * 1024,
    env: { ...process.env, SEMIO_PROBE_URL: URL, ...(probe.env ?? {}) },
  });
  const seconds = Math.round((Date.now() - started) / 1000);
  const stdout = `${run.stdout ?? ""}\n${run.stderr ?? ""}`;
  writeFileSync(join(dir, "run.txt"), stdout);
  if (probe.from && existsSync(probe.from)) spawnSync("cp", ["-R", `${probe.from}/.`, dir]);
  let v = { steps: [], key: {}, pageerrors: 0, faults: [] };
  let verdictError = null;
  try { v = probe.verdict(dir, stdout); } catch (e) { verdictError = String(e).slice(0, 400); }
  const decided = v.steps.filter((s) => s.ok !== undefined);
  const ok = !verdictError && decided.length > 0 && decided.every((s) => s.ok);
  board.probes.push({
    probe: probe.name, script: probe.script, out: `🗑️generated/wgpu-verify/${probe.dir}/`,
    seconds, exit: run.status, timedOut: run.signal === "SIGTERM" || Boolean(run.error && /ETIMEDOUT|timed/i.test(String(run.error))),
    ok, verdictError,
    steps: v.steps, key: v.key,
    pageerrors: v.pageerrors ?? 0,
    faults: (v.faults ?? []).length ? v.faults : faultLines(stdout),
  });
  console.log(`[DEBUG] battery ■ ${probe.name} ok=${ok} exit=${run.status} ${seconds}s steps=${decided.filter((s) => s.ok).length}/${decided.length} pageerrors=${v.pageerrors ?? 0}${verdictError ? ` verdictError=${verdictError}` : ""}`);
  for (const s of v.steps) console.log(`[DEBUG]   ${s.ok === undefined ? "·" : s.ok ? "✓" : "✗"} ${s.step} ${JSON.stringify(s.detail ?? {}).slice(0, 240)}`);
  flush();
}

flush();
const sum = summarize();
console.log(`\n[DEBUG] BATTERY DONE green=${sum.green}/${sum.probes} red=${JSON.stringify(sum.red)} ${sum.totalSeconds}s pageerrors=${sum.pageerrors}`);
console.log(`[DEBUG] scoreboard ${scoreboardPath}`);
process.exit(sum.red.length ? 1 : 0);
