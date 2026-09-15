/** 🔋 React end-to-end battery for the generation3d playground on 6018.
 *
 * Runs every React probe in this ticket SEQUENTIALLY — one browser at a time, against ONE restage of
 * the tree — and writes a single scoreboard so "everything works end to end" is one artifact rather
 * than fifteen lane reports each proven on its own restage.
 *
 * Each probe lands in `🗑️generated/react-verify/<probe>/`; the battery then reads that probe's OWN
 * published evidence (its `results.json` / `wire-result.json` / `verdict.json` / console) and turns it
 * into pass-fail steps. A probe is never called green from its exit code alone: every verdict below
 * names the number it read.
 *
 * Usage:
 *   cd <ticket> && bun 🐍️react-battery.mjs                    # everything
 *   cd <ticket> && bun 🐍️react-battery.mjs --only=journey,gaps # a subset, comma separated
 *   cd <ticket> && bun 🐍️react-battery.mjs --list
 *
 * Env: SEMIO_BATTERY_URL (default http://127.0.0.1:6018/?plugin=generation3d),
 *      SEMIO_BATTERY_ROOT (default react-verify) — the `🗑️generated/<root>/` folder this run owns.
 * @see 🐍️journey-probe.mjs, 🐍️react-gap-probe.mjs, 📓️react-end-to-end-verification-2026-09-13.md
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const T = import.meta.dir;
const GEN = join(T, "🗑️generated");
/** 🗂️ Which folder under `🗑️generated` this run owns. Two lanes running two ports at once must not
 * write into each other's evidence, so the root is an env knob and only its DEFAULT is `react-verify`. */
const ROOT_NAME = process.env.SEMIO_BATTERY_ROOT ?? "react-verify";
const ROOT = join(GEN, ROOT_NAME);
/** 📦️ A probe's `SEMIO_PROBE_OUT` under this run's root; `outUp` for the probes that nest it one level deeper. */
const out = (sub) => `${ROOT_NAME}/${sub}`;
const outUp = (sub) => `../${ROOT_NAME}/${sub}`;
const URL = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const BASE = URL.split("/?")[0];
const argv = process.argv.slice(2);
const only = (argv.find((a) => a.startsWith("--only=")) ?? "").slice("--only=".length).split(",").filter(Boolean);

/** 🧯 Lines that mean the shell hurt itself, whichever probe produced them. */
const FAULT_RE = /pageerror|Unhandled|no actor for instance|unknown kind|unknown merge|retained document permit failed|watchdog|panicked|RuntimeError|command page invalid|refreshUi failed|thunk failed|action failed/i;

const readJson = (path) => { try { return JSON.parse(readFileSync(path, "utf8")); } catch { return null; } };
const readText = (path) => { try { return readFileSync(path, "utf8"); } catch { return ""; } };
/** 🪶 The `[DEBUG]`/fault lines a reader needs, deduplicated and capped so the scoreboard stays legible. */
const faultLines = (text, cap = 12) => [...new Set(text.split("\n").filter((l) => FAULT_RE.test(l)).map((l) => l.trim().slice(0, 300)))].slice(0, cap);
const countPageErrors = (text) => text.split("\n").filter((l) => /\bpageerror\b/.test(l)).length;

/** 📋 One row per probe: how to run it, where its evidence lands, and how to read a verdict out of it.
 * `out`/`outUp` build what the probe's own `SEMIO_PROBE_OUT` has to be so its artifacts land under this run's root
 * — the probes nest that value under different fixed parents, hence the `../` in some rows. */
const PROBES = [
  {
    name: "boot", script: "🐍️console-dump-probe.mjs", dir: "boot", minutes: 5,
    env: { SEMIO_PROBE_OUT: out("boot"), SEMIO_PROBE_SECONDS: "100" },
    verdict: (dir, stdout) => {
      const console_ = readText(join(dir, "console.txt"));
      const hosts = readJson(join(dir, "hosts.json")) ?? [];
      const m = /hosts\s+(\[.*)/s.exec(stdout);
      const parsed = hosts.length ? hosts : (() => { try { return JSON.parse(m?.[1] ?? "[]"); } catch { return []; } })();
      const surfaces = parsed.map((h) => h.id ?? h.surfaceId).filter(Boolean);
      const pageerrors = countPageErrors(console_ + stdout);
      return {
        steps: [
          { step: "surfaces mount", ok: surfaces.length >= 2, detail: { surfaces } },
          { step: "no page errors", ok: pageerrors === 0, detail: { pageerrors } },
        ],
        key: { surfaces: surfaces.length, consoleLines: console_.split("\n").length },
        pageerrors, faults: faultLines(console_),
      };
    },
  },
  {
    name: "journey", script: "🐍️journey-probe.mjs", dir: "journey", minutes: 45,
    env: { SEMIO_PROBE_OUT: out("journey"), SEMIO_PROBE_MESH_WAIT: "120" },
    verdict: (dir) => {
      const rows = readJson(join(dir, "results.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const steps = rows.map((r) => ({ step: r.label, ok: Boolean(r.converged), detail: { seconds: Math.round(r.seconds ?? 0), meshes: r.meshes, example: r.example, faults: r.faults ?? [] } }));
      const edit = rows.filter((r) => r.label?.startsWith("edit:"));
      const view = rows.filter((r) => r.label?.startsWith("view:"));
      return {
        steps,
        key: {
          rows: rows.length, converged: rows.filter((r) => r.converged).length,
          editExamples: edit.length, viewExamples: view.length,
          perExampleSeconds: Object.fromEntries(rows.map((r) => [r.label, Math.round(r.seconds ?? 0)])),
          meshesByLabel: Object.fromEntries(rows.map((r) => [r.label, r.meshes])),
        },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    /** 🎯 Gestures over ALL EIGHT examples, each hop decided by the id/camera VALUE the pane
     * publishes. The old lane ran `🔍️browser-probe.ts` against the single hardcoded
     * `box-shell-preview` and graded it with regexes over the whole run's console dump, which could
     * not say which target was hit — or that anything was hit at all
     * (`📓️example-oracle-strength-audit-2026-09-14.md` §1). */
    name: "interact", script: "🐍️interaction-matrix-probe.mjs", dir: "interact", minutes: 40,
    env: { SEMIO_PROBE_OUT: out("interact"), SEMIO_PROBE_MESH_WAIT: "120" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const rows = r.results ?? [];
      const steps = rows.map((row) => ({ step: `${row.example} · ${row.step}`, ok: Boolean(row.ok), detail: row.detail }));
      return {
        steps,
        key: { examples: (r.examples ?? []).length, rows: rows.length, green: rows.filter((row) => row.ok).length, byStep: Object.fromEntries(["payload", "selection-reset", "hover", "select", "inspector", "orbit", "fit"].map((step) => [step, `${rows.filter((row) => row.step === step && row.ok).length}/${rows.filter((row) => row.step === step).length}`])) },
        pageerrors: (r.faults ?? []).length, faults: faultLines(console_),
      };
    },
  },
  {
    // 🎚️ Does the preview FOLLOW a dragged slider? One row per slider kind per example: the gate is
    // zero dropped actions, ONE history entry per press, a coalesced dispatch count, and geometry that
    // ends on the released value (`📓️slider-preview-update-2026-09-15.md`).
    name: "slider-live-preview", script: "🐍️slider-live-preview-probe.mjs", dir: "slider-live-preview", minutes: 30,
    env: { SEMIO_PROBE_OUT: out("slider-live-preview"), SEMIO_PROBE_KINDS: "graph", SEMIO_PROBE_SETTLE: "25", SEMIO_PROBE_EXAMPLES: "hexagonal-mushroom-column,sphere-cut-with-torus,box-shell-preview,box-fillet-preview,rectangle-extrude-volume,sphere-box-fuse,face-sweep-extrude,rectangle-wire-preview" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const rows = (r?.rows ?? []).map((row) => ({
        example: row.example, kind: row.kind, ok: Boolean(row.ok),
        dropped: (row.droppedActions ?? []).length,
        history: row.history?.delta ?? null,
        starts: row.drag?.starts ?? null,
        changes: row.drag?.valueChanges ?? null,
        medianMs: row.drag?.latencyMedianMs ?? null,
        settledPhase: row.drag?.settledPhase ?? null,
        settledMeshes: row.drag?.settledMeshes ?? null,
      }));
      return {
        rows,
        key: {
          rows: rows.length,
          reached: rows.filter((row) => row.changes != null).length,
          noDroppedActions: rows.filter((row) => row.dropped === 0).length,
          oneHistoryEntryPerPress: rows.filter((row) => row.history === 1).length,
          coalesced: rows.filter((row) => row.starts != null && row.changes != null && row.starts <= row.changes).length,
        },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "generate-mode", script: "🐍️generate-mode-probe.mjs", dir: "generate-mode", minutes: 18,
    env: { SEMIO_PROBE_OUT: out("generate-mode") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r?.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "flow-window", script: "🐍️flow-window-probe.mjs", dir: "flow-window", minutes: 10,
    env: { SEMIO_PROBE_MODE: "default", SEMIO_PROBE_OUT: outUp("flow-window"), SEMIO_PROBE_SECONDS: "70" },
    verdict: (dir) => {
      const console_ = readText(join(dir, "console.txt"));
      const geo = /\[DEBUG\] geometry.*|graph host.*|dag draw lod=(\w+) zoom=([0-9.]+)/.exec(console_);
      const draws = [...console_.matchAll(/dag draw lod=(\w+) zoom=([0-9.]+)/g)];
      const nodes = /nodes=(\d+)/.exec(console_);
      return {
        steps: [
          { step: "node graph paints", ok: draws.length > 0, detail: { draws: draws.length, lastLod: draws.at(-1)?.[1] ?? null, lastZoom: draws.at(-1)?.[2] ?? null } },
          { step: "no page errors", ok: countPageErrors(console_) === 0, detail: { pageerrors: countPageErrors(console_) } },
        ],
        key: { draws: draws.length, nodes: nodes?.[1] ?? null, geo: geo?.[0]?.slice(0, 120) ?? null },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "flow-wire", script: "🐍️flow-window-probe.mjs", dir: "flow-wire", minutes: 14,
    env: { SEMIO_PROBE_MODE: "wire", SEMIO_PROBE_OUT: outUp("flow-wire"), SEMIO_PROBE_SECONDS: "70" },
    verdict: (dir) => {
      const r = readJson(join(dir, "wire-result.json"));
      const console_ = readText(join(dir, "console.txt"));
      return {
        steps: [
          { step: "graph publishes wires", ok: (r?.before?.length ?? 0) > 0, detail: { before: r?.before ?? null } },
          { step: "cut removes the wire", ok: Boolean(r?.cutRemovedTheWire), detail: { afterCut: r?.afterCut ?? null } },
          /** 🎯️ `redrawPressLanded`/`staleAim` say whether the press the verdict is about ever entered a
           * wire draw. A press that missed the port the HOST published is stale published geometry
           * (`dagIntroductionResolver`'s undated cache), not a graph that refuses to be rewired, and it
           * used to be graded as the latter. */
          { step: "redraw restores the wire", ok: Boolean(r?.redrawRestoredTheWire), detail: { afterRedraw: r?.afterRedraw ?? r?.after ?? null, pressLanded: r?.redrawPressLanded ?? null, staleAim: (r?.staleAim ?? []).length } },
        ],
        key: { before: r?.before?.length ?? 0, afterCut: r?.afterCut?.length ?? null, preview: r?.preview ?? null, redrawPressLanded: r?.redrawPressLanded ?? null, staleAim: (r?.staleAim ?? []).length },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "flow-reorganize", script: "🐍️flow-window-probe.mjs", dir: "flow-reorganize", minutes: 8,
    // ⏱️ 45 s, not 120: this mode only needs the boot and one keypress, and the extra idle minute was
    // long enough for a dev hot-swap to retire the app instance under the probe — after which the Flow
    // window has no node-graph host at all and the reorganize lands on nothing.
    env: { SEMIO_PROBE_MODE: "reorganize", SEMIO_PROBE_OUT: outUp("flow-reorganize"), SEMIO_PROBE_SECONDS: "45" },
    verdict: (dir) => {
      const r = readJson(join(dir, "reorganize-result.json"));
      const console_ = readText(join(dir, "console.txt"));
      return {
        steps: [{ step: "mod+alt+L moves widgets", ok: (r?.moved?.length ?? 0) > 0, detail: { moved: r?.moved ?? [] } }],
        key: { moved: r?.moved?.length ?? 0 }, pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "cancel-preview", script: "🐍️cancel-preview-probe.mjs", dir: "cancel-preview", minutes: 12,
    env: { SEMIO_PROBE_OUT: outUp("cancel-preview") },
    verdict: (dir) => {
      const rows = readJson(join(dir, "results.json")) ?? [];
      const frames = readJson(join(dir, "status-frames.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const cancellable = frames.some((f) => f?.cancellable === true || f?.status?.cancellable === true);
      const button = rows.some((r) => r.cancelButton || r.cancelSlot || /cancel/i.test(JSON.stringify(r)) && r.cancelPresent);
      return {
        steps: [
          { step: "a cancellable frame is published", ok: cancellable, detail: { frames: frames.length } },
          { step: "the cancel affordance exists", ok: button || cancellable, detail: { rows: rows.length } },
        ],
        key: { frames: frames.length, rows: rows.length }, pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "keyboard-verbs", script: "🐍️editor-verbs-keyboard-probe.mjs", dir: "keyboard-verbs", minutes: 24,
    env: { SEMIO_PROBE_OUT: outUp("keyboard-verbs") },
    /** ⌨️ Every row is decided by the probe's OWN oracle sentence — the probe presses each chord in the
     * state that owns it and reads back the effect that chord must have, so no step is exempt here.
     * A row that reaches this verdict without an `ok` of its own is a probe that did not finish its
     * step, and is red rather than silently skipped. */
    verdict: (dir) => {
      const rows = readJson(join(dir, "results.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const steps = rows.map((r) => ({ step: r.label, ok: typeof r.ok === "boolean" ? r.ok : false, detail: { chord: r.chord, oracle: r.oracle ?? "(the probe published no oracle for this row)", invoked: r.invoked, evidence: r.evidence, history: r.history, cancelButton: r.cancelButton } }));
      return { steps, key: { rows: rows.length, chords: Object.fromEntries(rows.map((r) => [r.label, (r.invoked ?? []).join(",") || "(nothing)"])), historyJsonPublished: rows.some((r) => r.history) }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "io-surface", script: "🐍️io-surface-probe.mjs", dir: "io-surface", minutes: 14,
    env: { SEMIO_PROBE_OUT: out("io-surface") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const rows = r?.results ?? [];
      /** 🚪️ The probe states its own `ok` per row (`🐍️io-surface-probe.mjs` `record`). It used to
       * state none at all, and `x.ok !== undefined ? … : !x.error` then read `undefined`/`undefined`
       * on EVERY row — so the P0 import/export gate was structurally incapable of failing whatever
       * the probe saw (`📓️window-coverage-audit-2026-09-14.md` §3). A row that carries no verdict is
       * now red, not green. */
      const steps = rows.map((x) => ({ step: x.label, ok: Boolean(x.ok), detail: x }));
      return { steps, key: { rows: rows.length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "export-encoding", script: "🐍️export-encoding-probe.mjs", dir: "export-encoding", minutes: 14,
    env: { SEMIO_PROBE_OUT: out("export-encoding"), SEMIO_PROBE_FORMATS: "dwg,stl" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r?.results ?? []).map((x) => ({ step: `export ${x.format}`, ok: !x.error && (x.bytes ?? 0) > 0, detail: { bytes: x.bytes, filename: x.filename, magicAscii: x.magicAscii, error: x.error } }));
      return { steps, key: { booted: r?.booted, formats: (r?.results ?? []).map((x) => `${x.format}:${x.bytes ?? "err"}`) }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "status-parity", script: "🐍️status-parity-probe.mjs", dir: "status-parity", minutes: 14,
    env: { SEMIO_PROBE_OUT: out("status-parity") },
    verdict: (dir) => {
      const rows = readJson(join(dir, "results.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const steps = rows.map((r) => {
        const hosts = r.hosts ?? [];
        const preview = hosts.filter((h) => (h.surfaceId ?? "").includes("preview"));
        return { step: r.label, ok: hosts.length > 0 && preview.length > 0 && preview.every((h) => h.status !== undefined && h.status !== null), detail: { hosts: hosts.map((h) => [h.surfaceId, h.meshes, h.status?.phase ?? (h.status ? "node-map" : null)]) } };
      });
      return { steps, key: { rows: rows.length, surfaces: [...new Set(rows.flatMap((r) => (r.hosts ?? []).map((h) => h.surfaceId)))] }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "role-switch", script: "🐍️role-switch-runtime-probe.mjs", dir: "role-switch", minutes: 16,
    env: { SEMIO_PROBE_OUT: out("role-switch") },
    verdict: (dir) => {
      const s = readJson(join(dir, "summary.json")) ?? {};
      const rows = readJson(join(dir, "steps.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      /** 🔀️ The probe's OWN wait predicate is the verdict: `mid-chain`/`post-viewer` wait for the
       * viewer role to be pressed, `post-editor` for the editor one, `generate` for generate mode.
       * Deciding on `windows.length > 0` instead scored a switch that never happened as a pass
       * (`📓️window-coverage-audit-2026-09-14.md` §3). */
      const expected = { boot: (r) => r.roles?.editor === "true", "mid-chain": (r) => r.roles?.viewer === "true", "post-editor": (r) => r.roles?.editor === "true", "post-viewer": (r) => r.roles?.viewer === "true", generate: (r) => r.modes?.generate === "true" };
      const steps = rows.map((r) => ({ step: r.label, ok: (r.windows ?? []).length > 0 && (expected[r.label] ? expected[r.label](r) : true), detail: { windows: r.windows, roles: r.roles, modes: r.modes, decidedOn: expected[r.label] ? "role/mode pressed state" : "windows mounted" } }));
      steps.push({ step: "no `no actor for instance`", ok: (s.noActor ?? 0) === 0, detail: { noActor: s.noActor, sample: s.noActorSample } });
      const pageErrors = Array.isArray(s.pageErrors) ? s.pageErrors.length : Number(s.pageErrors ?? 0);
      return { steps, key: { noActor: s.noActor ?? 0, pageErrors, traces: (s.traces ?? []).length }, pageerrors: pageErrors || countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "i18n-a11y", script: "🐍️i18n-a11y-customization-probe.mjs", dir: "i18n-a11y", minutes: 22,
    env: { SEMIO_PROBE_OUT: outUp("i18n-a11y"), SEMIO_PROBE_BASE: BASE },
    /** 🇩🇪️ A German tag is green when every label that tag's MODE actually paints is German. Edit mode
     * never paints the generate windows and generate mode never paints the graph outline, so demanding
     * all 14 strings on every tag scored an absent window as an untranslated one. */
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const rows = r.steps ?? [];
      const APPLICABLE = {
        "2-edit-de": ["window_flow", "window_preview", "graph_nodes", "graph_wires", "graph_input_port", "graph_output_port", "status_ok"],
        "4-generate-de": ["window_generations", "window_generate_form", "window_preview"],
        "5-viewer-de": ["window_preview"],
        "6-after-reload": ["window_flow", "window_preview", "graph_nodes", "graph_wires"],
      };
      const steps = rows.map((s) => {
        const want = APPLICABLE[s.tag] ?? [];
        const missing = want.filter((k) => !(s.germanHits ?? {})[k]);
        const a11yOk = s.surfaceShells > 0 && s.namedShells === s.surfaceShells && s.focusableShells === s.surfaceShells;
        return { step: s.tag, ok: a11yOk && missing.length === 0 && (!want.length || s.htmlLang === "de"), detail: { shells: s.surfaceShells, named: s.namedShells, focusable: s.focusableShells, live: s.liveShells, lang: s.htmlLang, appearance: s.appearance, applicable: want, missing, absentElsewhere: Object.entries(s.germanHits ?? {}).filter(([k, h]) => !h && !want.includes(k)).map(([k]) => k) } };
      });
      return { steps, key: { tags: rows.length, appearanceAfterReload: rows.find((s) => s.tag === "6-after-reload")?.appearance ?? null, langAfterReload: rows.find((s) => s.tag === "6-after-reload")?.htmlLang ?? null }, pageerrors: (r.errors ?? []).length, faults: (r.errors ?? []).slice(0, 12) };
    },
  },
  {
    name: "customization-persistence", script: "🐍️customization-persistence-probe.mjs", dir: "customization", minutes: 12,
    env: { SEMIO_PROBE_OUT: outUp("customization") },
    verdict: (dir) => {
      const v = readJson(join(dir, "verdict.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      return {
        steps: [
          { step: "locale survives a reload", ok: Boolean(v.localeKept), detail: { lsKeys: v.lsKeysReloaded, picked: v.okLang } },
          { step: "appearance survives a reload", ok: Boolean(v.appearanceKept), detail: { picked: v.okAppearance, appearanceBefore: v.appearanceBefore, appearanceAfter: v.appearanceAfter, appearanceReloaded: v.appearanceReloaded } },
        ],
        key: { localeKept: v.localeKept, appearanceKept: v.appearanceKept }, pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "gaps", script: "🐍️react-gap-probe.mjs", dir: "gaps", minutes: 30,
    env: { SEMIO_PROBE_OUT: out("gaps") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "preview-chrome", script: "🐍️preview-chrome-probe.mjs", dir: "preview-chrome", minutes: 20,
    env: { SEMIO_PROBE_OUT: out("preview-chrome") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "status-states", script: "🐍️status-states-probe.mjs", dir: "status-states", minutes: 30,
    env: { SEMIO_PROBE_OUT: out("status-states") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length, statesEn: Object.keys(r.passes?.en?.seen ?? {}), statesDe: Object.keys(r.passes?.de?.seen ?? {}) }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "menus", script: "🐍️menu-dump-probe.mjs", dir: "menus", minutes: 16,
    env: { SEMIO_PROBE_OUT: out("menus") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "viewer-actions", script: "🐍️viewer-actions-probe.mjs", dir: "viewer-actions", minutes: 22,
    env: { SEMIO_PROBE_OUT: out("viewer-actions") },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "flow-scroll", script: "🐍️flow-scroll-render-perf-probe.mjs", dir: "flow-scroll", minutes: 14,
    env: { SEMIO_PROBE_OUT: out("flow-scroll") },
    /** 🖱️ Butter-smooth is four numbers per gesture, not an impression: the board repaints within a
     * frame of each tick, the tail stays inside a slow frame, the plugin hears NOTHING while the
     * gesture runs, and it hears the settled camera exactly once. A gesture that publishes per tick
     * reads as "scrolling takes seconds to render" — the user's own words, 2026-09-15 13:03. */
    verdict: (dir) => {
      const rows = readJson(join(dir, "scroll.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const steps = rows.flatMap((row) => [
        { step: `${row.example} ${row.gesture} · paint median ≤ 16 ms`, ok: row.paintMedianMs !== null && row.paintMedianMs <= 16, detail: { medianMs: row.paintMedianMs, paints: row.paints, events: row.events } },
        { step: `${row.example} ${row.gesture} · paint p95 ≤ 50 ms`, ok: row.paintP95Ms !== null && row.paintP95Ms <= 50, detail: { p95Ms: row.paintP95Ms, maxMs: row.paintMaxMs, longTasks: row.longTasks } },
        { step: `${row.example} ${row.gesture} · 0 guest hops during`, ok: row.guestInvocationsDuring === 0, detail: { during: row.guestInvocationsDuringByAction } },
        { step: `${row.example} ${row.gesture} · 1 camera publication at settle`, ok: row.viewportPublicationsAtSettle === 1, detail: { atSettle: row.guestInvocationsAtSettleByAction, refresh: `${row.refreshPassesDuring}/${row.refreshPassesAtSettle}`, commits: `${row.reactCommitsDuring}/${row.reactCommitsAtSettle}` } },
      ]);
      return { steps, key: { gestures: rows.length, ok: steps.filter((step) => step.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "graph-keyboard", script: "🐍️graph-keyboard-nav-probe.mjs", dir: "graph-keyboard", minutes: 16,
    env: { SEMIO_PROBE_OUT: outUp("graph-keyboard") },
    /** ⌨️ Each traversal chord must reach its OWN verb, not merely "something was invoked": the four
     * arrows and `Enter` are the whole keyboard route through the graph, and a chord that resolves to
     * a neighbour's action is exactly the defect. `baseline` presses nothing and cannot be decided. */
    verdict: (dir) => {
      const rows = readJson(join(dir, "results.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const want = {
        "arrow-down-1": "selectNextNode", "arrow-down-2": "selectNextNode", "arrow-down-3": "selectNextNode",
        "arrow-right-downstream": "selectDownstreamNode", "arrow-left-upstream": "selectUpstreamNode",
        "arrow-up-previous": "selectPreviousNode", "enter-activate": "activateSelection",
        "arrow-right-after-activate": "selectDownstreamNode", "arrow-down-reenter": "selectNextNode",
      };
      const steps = rows.map((r) => ({
        step: r.label,
        ok: want[r.label] === undefined ? undefined : (r.invoked ?? []).includes(want[r.label]),
        detail: { key: r.key, want: want[r.label] ?? null, invoked: r.invoked, activeRole: r.activeRole, activeLabel: r.activeLabel, outlineSelected: r.outlineSelected, inspection: r.inspection },
      }));
      /** 🔍️ A chord that DISPATCHES is not a chord that WORKS. The Inspection panel is fed by the
       * framework-owned `graph` selection, so the node it names is the user-visible proof that an
       * arrow key moved the selection and not merely an action id through the console. */
      const inspections = rows.filter((r) => r.key).map((r) => r.inspection).filter(Boolean);
      const distinct = [...new Set(inspections)];
      steps.push({ step: "the Inspection panel is mounted", ok: rows.some((r) => (r.inspectorPresent ?? 0) > 0), detail: { inspectorPresent: rows.map((r) => r.inspectorPresent ?? 0) } });
      steps.push({ step: "traversal moves the selection the Inspection panel shows", ok: distinct.length > 1, detail: { distinct: distinct.slice(0, 12), samples: inspections.length } });
      return { steps, key: { rows: rows.length, chords: Object.fromEntries(rows.map((r) => [r.label, (r.invoked ?? []).join(",") || "(nothing)"])) }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "outline-selection", script: "🐍️outline-selection-probe.mjs", dir: "outline-selection", minutes: 12,
    env: { SEMIO_PROBE_OUT: out("outline-selection"), SEMIO_BATTERY_URL: URL },
    /** 🌳️ F1's DOM half, as a battery row. The probe states its own verdict per step — the outline
     * paints rows, a traversal marks exactly one of them, `Escape` retires the mark and a traversal
     * after it re-marks exactly one — so a row without an `ok` is red rather than skipped. It was
     * written by lane `window-gaps-followup` and never registered anywhere, which is a gate that
     * measures nothing (`📓️window-gaps-followup-2026-09-14.md` §6). */
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.verdicts ?? []).map((v) => ({ step: v.step, ok: Boolean(v.ok), detail: v.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "panel-i18n", script: "🐍️panel-i18n-probe.mjs", dir: "panel-i18n", minutes: 22,
    env: { SEMIO_PROBE_OUT: `../${ROOT_NAME}/panel-i18n`, SEMIO_PROBE_BASE: BASE },
    /** 🇩🇪️ One step per `Generation3dLabels` field this probe can reach; a field whose German never
     * appears on any surface is a red row naming the field, not a silent omission. */
    verdict: (dir) => {
      const r = readJson(join(dir, "result.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const seen = r.seen ?? {};
      const fields = [...Object.keys(seen), ...(r.unreached ?? [])].sort();
      const steps = fields.map((field) => ({ step: `de:${field}`, ok: Boolean(seen[field]), detail: { surface: seen[field] ?? null, identicalByDesign: (r.identicalByDesign ?? []).includes(field) } }));
      steps.unshift({ step: "locale switched to German", ok: Boolean(r.german) && r.lang === "de", detail: { german: r.german, lang: r.lang } });
      return { steps, key: { reached: r.reached, checkedHere: r.checkedHere, total: r.total, unreached: r.unreached, ownedElsewhere: r.ownedElsewhere }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "inspection-i18n", script: "🐍️inspection-i18n-probe.mjs", dir: "inspection-i18n", minutes: 16,
    env: { SEMIO_PROBE_OUT: `../${ROOT_NAME}/inspection-i18n`, SEMIO_PROBE_BASE: BASE },
    verdict: (dir) => {
      const r = readJson(join(dir, "result.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const seen = r.seen ?? {};
      const fields = [...Object.keys(seen), ...(r.missing ?? [])].sort();
      const steps = fields.map((field) => ({ step: `de:${field}`, ok: Boolean(seen[field]), detail: { surface: seen[field] ?? null } }));
      return { steps, key: { seen, missing: r.missing, catalogueItems: (r.catalogueItems ?? []).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
];

if (argv.includes("--list")) { console.log(PROBES.map((p) => p.name).join("\n")); process.exit(0); }

const selected = only.length ? PROBES.filter((p) => only.includes(p.name)) : PROBES;
if (only.length && selected.length !== only.length) { console.error(`unknown probe(s): ${only.filter((n) => !PROBES.some((p) => p.name === n)).join(", ")}`); process.exit(2); }

mkdirSync(ROOT, { recursive: true });
const scoreboardPath = join(ROOT, "scoreboard.json");
const previous = readJson(scoreboardPath);
const board = { schema: "semio.procedural3d.react-battery/1", url: URL, startedAt: new Date().toISOString(), probes: [] };
if (previous?.probes && only.length) board.probes = previous.probes.filter((p) => !only.includes(p.probe));

const flush = () => writeFileSync(scoreboardPath, JSON.stringify({ ...board, finishedAt: new Date().toISOString(), summary: summarize() }, null, 2));
const summarize = () => ({
  probes: board.probes.length,
  green: board.probes.filter((p) => p.ok).length,
  red: board.probes.filter((p) => !p.ok).map((p) => p.probe),
  totalSeconds: Math.round(board.probes.reduce((n, p) => n + p.seconds, 0)),
  pageerrors: board.probes.reduce((n, p) => n + (Number(p.pageerrors) || 0), 0),
});

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
  if (probe.collect) { try { probe.collect(dir); } catch (e) { writeFileSync(join(dir, "collect-error.txt"), String(e)); } }
  let v = { steps: [], key: {}, pageerrors: 0, faults: [] };
  let verdictError = null;
  try { v = probe.verdict(dir, stdout); } catch (e) { verdictError = String(e).slice(0, 400); }
  /** 🧯 EVERY probe is gated on a clean page, centrally — two rows appended to whatever its own
   * verdict decided. Before this, only `boot` and `flow-window` turned page errors into a verdict,
   * so a run carrying a recurring `TypeError … addEventListener` and a `FlowMessageRejected` still
   * reported `green=16/16` (`📓️window-coverage-audit-2026-09-14.md` §3, §5 item 3). A shell that
   * hurt itself is not a passing shell, whichever surface the probe was pointed at. */
  v.steps = [
    ...v.steps.filter((s) => s.step !== "no page errors" && s.step !== "no shell faults"),
    { step: "no page errors", ok: (Number(v.pageerrors) || 0) === 0, detail: { pageerrors: Number(v.pageerrors) || 0 } },
    { step: "no shell faults", ok: (v.faults ?? []).length === 0, detail: { faults: (v.faults ?? []).slice(0, 6) } },
  ];
  const decided = v.steps.filter((s) => s.ok !== undefined);
  const ok = run.status === 0 && !verdictError && decided.length > 0 && decided.every((s) => s.ok);
  board.probes.push({
    probe: probe.name, script: probe.script, out: `🗑️generated/${ROOT_NAME}/${probe.dir}/`,
    seconds, exit: run.status, timedOut: run.signal === "SIGTERM" || Boolean(run.error && /ETIMEDOUT|timed/i.test(String(run.error))),
    ok, verdictError,
    steps: v.steps, key: v.key,
    pageerrors: v.pageerrors ?? 0,
    faults: (v.faults ?? []).length ? v.faults : faultLines(stdout),
  });
  console.log(`[DEBUG] battery ■ ${probe.name} ok=${ok} exit=${run.status} ${seconds}s steps=${decided.filter((s) => s.ok).length}/${decided.length} pageerrors=${v.pageerrors ?? 0}${verdictError ? ` verdictError=${verdictError}` : ""}`);
  for (const s of v.steps) console.log(`[DEBUG]   ${s.ok === undefined ? "·" : s.ok ? "✓" : "✗"} ${s.step} ${JSON.stringify(s.detail ?? {}).slice(0, 260)}`);
  flush();
}

flush();
const sum = summarize();
console.log(`\n[DEBUG] BATTERY DONE green=${sum.green}/${sum.probes} red=${JSON.stringify(sum.red)} ${sum.totalSeconds}s pageerrors=${sum.pageerrors}`);
console.log(`[DEBUG] scoreboard ${scoreboardPath}`);
process.exit(sum.red.length ? 1 : 0);
