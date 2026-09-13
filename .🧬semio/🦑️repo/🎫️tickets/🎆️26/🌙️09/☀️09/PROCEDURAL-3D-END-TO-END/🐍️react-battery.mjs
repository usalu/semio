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
 * Env: SEMIO_BATTERY_URL (default http://127.0.0.1:6018/?plugin=generation3d).
 * @see 🐍️journey-probe.mjs, 🐍️react-gap-probe.mjs, 📓️react-end-to-end-verification-2026-09-13.md
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const T = import.meta.dir;
const GEN = join(T, "🗑️generated");
const ROOT = join(GEN, "react-verify");
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
 * `out` is what the probe's own `SEMIO_PROBE_OUT` has to be so its artifacts land under `react-verify/`
 * — the probes nest that value under different fixed parents, hence the `../` in some rows. */
const PROBES = [
  {
    name: "boot", script: "🐍️console-dump-probe.mjs", dir: "boot", minutes: 5,
    env: { SEMIO_PROBE_OUT: "react-verify/boot", SEMIO_PROBE_SECONDS: "100" },
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
    env: { SEMIO_PROBE_OUT: "react-verify/journey", SEMIO_PROBE_MESH_WAIT: "120" },
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
    name: "interact", script: "🔍️browser-probe.ts", dir: "interact", minutes: 12,
    args: ["--mode=interact", "--steps=example,hover,select,orbit", "--label=react-verify-interact", "--settle=150", "--example=box-shell-preview"],
    /** 📦️ This probe writes to a timestamped dir plus `<GENERATED>/<label>-*` files; collect both. */
    collect: (dir) => {
      for (const entry of readdirSync(GEN)) {
        if (entry.startsWith("react-verify-interact-")) renameSync(join(GEN, entry), join(dir, entry.replace("react-verify-interact-", "")));
        else if (entry.startsWith("probe-react-verify-interact-")) renameSync(join(GEN, entry), join(dir, "run"));
      }
    },
    verdict: (dir) => {
      const summary = readJson(join(dir, "summary.json")) ?? {};
      const console_ = readText(join(dir, "console.jsonl")) + readText(join(dir, "run", "console.jsonl"));
      const has = (re) => re.test(console_);
      const hover = /hoverTarget|interactionHover/.test(console_);
      const select = /selectedIds|interactionSelect/.test(console_);
      const orbit = /setCamera/.test(console_);
      return {
        steps: [
          { step: "meshes before gesture", ok: (summary.meshCount ?? 0) > 0, detail: { meshCount: summary.meshCount ?? 0, example: summary.exampleLabel } },
          { step: "hover publishes a target", ok: hover, detail: { evidence: "hoverTarget|interactionHover" } },
          { step: "click publishes a selection", ok: select, detail: { evidence: "selectedIds|interactionSelect" } },
          { step: "drag publishes a camera", ok: orbit, detail: { evidence: "setCamera" } },
          { step: "no unknown merge", ok: !has(/unknown merge/), detail: {} },
        ],
        key: { meshCount: summary.meshCount ?? 0, windows: (summary.windowIds ?? []).length, diagnosis: summary.diagnosis },
        pageerrors: (readText(join(dir, "run", "page-faults.jsonl")).trim().split("\n").filter(Boolean)).length,
        faults: faultLines(console_),
      };
    },
  },
  {
    name: "generate-mode", script: "🐍️generate-mode-probe.mjs", dir: "generate-mode", minutes: 18,
    env: { SEMIO_PROBE_OUT: "react-verify/generate-mode" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r?.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "flow-window", script: "🐍️flow-window-probe.mjs", dir: "flow-window", minutes: 10,
    env: { SEMIO_PROBE_MODE: "default", SEMIO_PROBE_OUT: "../react-verify/flow-window", SEMIO_PROBE_SECONDS: "70" },
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
    env: { SEMIO_PROBE_MODE: "wire", SEMIO_PROBE_OUT: "../react-verify/flow-wire", SEMIO_PROBE_SECONDS: "70" },
    verdict: (dir) => {
      const r = readJson(join(dir, "wire-result.json"));
      const console_ = readText(join(dir, "console.txt"));
      return {
        steps: [
          { step: "graph publishes wires", ok: (r?.before?.length ?? 0) > 0, detail: { before: r?.before ?? null } },
          { step: "cut removes the wire", ok: Boolean(r?.cutRemovedTheWire), detail: { afterCut: r?.afterCut ?? null } },
          { step: "redraw restores the wire", ok: Boolean(r?.redrawRestoredTheWire), detail: { afterRedraw: r?.afterRedraw ?? r?.after ?? null } },
        ],
        key: { before: r?.before?.length ?? 0, afterCut: r?.afterCut?.length ?? null, preview: r?.preview ?? null },
        pageerrors: countPageErrors(console_), faults: faultLines(console_),
      };
    },
  },
  {
    name: "flow-reorganize", script: "🐍️flow-window-probe.mjs", dir: "flow-reorganize", minutes: 8,
    env: { SEMIO_PROBE_MODE: "reorganize", SEMIO_PROBE_OUT: "../react-verify/flow-reorganize", SEMIO_PROBE_SECONDS: "120" },
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
    env: { SEMIO_PROBE_OUT: "../react-verify/cancel-preview" },
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
    name: "keyboard-verbs", script: "🐍️editor-verbs-keyboard-probe.mjs", dir: "keyboard-verbs", minutes: 12,
    env: { SEMIO_PROBE_OUT: "../react-verify/keyboard-verbs" },
    /** ⌨️ A chord is green when the shell INVOKED something for it. `cancel-preview-eval` is exempt:
     * the probe presses it on a quiet shell, where refusing to cancel nothing is the correct answer. */
    verdict: (dir) => {
      const rows = readJson(join(dir, "results.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      /** 🪪️ `cancel-preview-eval` is pressed on a QUIET shell, where cancelling nothing is correct.
       * `add-generation` is declared on the generations window, which only generate mode mounts, so an
       * edit-mode press reaching nothing is the scoping — the `gaps` probe's `generate-chord` step
       * presses it in the mode that owns it and is the one that decides it. */
      const optional = { baseline: "not a chord", "cancel-preview-eval": "pressed on a quiet shell: no eval to cancel", "add-generation": "scoped to the generations window (generate mode) — decided by gaps/generate-chord" };
      const steps = rows.map((r) => ({ step: r.label, ok: r.label in optional ? undefined : (r.invoked ?? []).length > 0, detail: { chord: r.chord, invoked: r.invoked, history: r.history, cancelButton: r.cancelButton, ...(r.label in optional ? { undecided: optional[r.label] } : {}) } }));
      return { steps, key: { rows: rows.length, chords: Object.fromEntries(rows.map((r) => [r.label, (r.invoked ?? []).join(",") || "(nothing)"])), historyJsonPublished: rows.some((r) => r.history) }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "io-surface", script: "🐍️io-surface-probe.mjs", dir: "io-surface", minutes: 14,
    env: { SEMIO_PROBE_OUT: "react-verify/io-surface" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const rows = r?.results ?? [];
      const steps = rows.map((x) => ({ step: x.label, ok: x.ok !== undefined ? Boolean(x.ok) : !x.error, detail: x }));
      return { steps, key: { rows: rows.length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "export-encoding", script: "🐍️export-encoding-probe.mjs", dir: "export-encoding", minutes: 14,
    env: { SEMIO_PROBE_OUT: "react-verify/export-encoding", SEMIO_PROBE_FORMATS: "dwg,stl" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json"));
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r?.results ?? []).map((x) => ({ step: `export ${x.format}`, ok: !x.error && (x.bytes ?? 0) > 0, detail: { bytes: x.bytes, filename: x.filename, magicAscii: x.magicAscii, error: x.error } }));
      return { steps, key: { booted: r?.booted, formats: (r?.results ?? []).map((x) => `${x.format}:${x.bytes ?? "err"}`) }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "status-parity", script: "🐍️status-parity-probe.mjs", dir: "status-parity", minutes: 14,
    env: { SEMIO_PROBE_OUT: "react-verify/status-parity" },
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
    env: { SEMIO_PROBE_OUT: "react-verify/role-switch" },
    verdict: (dir) => {
      const s = readJson(join(dir, "summary.json")) ?? {};
      const rows = readJson(join(dir, "steps.json")) ?? [];
      const console_ = readText(join(dir, "console.txt"));
      const steps = rows.map((r) => ({ step: r.label, ok: (r.windows ?? []).length > 0, detail: { windows: r.windows, roles: r.roles, modes: r.modes } }));
      steps.push({ step: "no `no actor for instance`", ok: (s.noActor ?? 0) === 0, detail: { noActor: s.noActor, sample: s.noActorSample } });
      const pageErrors = Array.isArray(s.pageErrors) ? s.pageErrors.length : Number(s.pageErrors ?? 0);
      return { steps, key: { noActor: s.noActor ?? 0, pageErrors, traces: (s.traces ?? []).length }, pageerrors: pageErrors || countPageErrors(console_), faults: faultLines(console_) };
    },
  },
  {
    name: "i18n-a11y", script: "🐍️i18n-a11y-customization-probe.mjs", dir: "i18n-a11y", minutes: 22,
    env: { SEMIO_PROBE_OUT: "../react-verify/i18n-a11y", SEMIO_PROBE_BASE: BASE },
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
    env: { SEMIO_PROBE_OUT: "../react-verify/customization" },
    verdict: (dir) => {
      const v = readJson(join(dir, "verdict.json")) ?? {};
      return {
        steps: [
          { step: "locale survives a reload", ok: Boolean(v.localeKept), detail: { lsKeys: v.lsKeysReloaded, picked: v.okLang } },
          { step: "appearance survives a reload", ok: Boolean(v.appearanceKept), detail: { picked: v.okAppearance, appearanceBefore: v.appearanceBefore, appearanceAfter: v.appearanceAfter, appearanceReloaded: v.appearanceReloaded } },
        ],
        key: { localeKept: v.localeKept, appearanceKept: v.appearanceKept }, pageerrors: 0, faults: [],
      };
    },
  },
  {
    name: "gaps", script: "🐍️react-gap-probe.mjs", dir: "gaps", minutes: 30,
    env: { SEMIO_PROBE_OUT: "react-verify/gaps" },
    verdict: (dir) => {
      const r = readJson(join(dir, "results.json")) ?? {};
      const console_ = readText(join(dir, "console.txt"));
      const steps = (r.steps ?? []).map((s) => ({ step: s.step, ok: Boolean(s.ok), detail: s.detail }));
      return { steps, key: { steps: steps.length, ok: steps.filter((s) => s.ok).length }, pageerrors: countPageErrors(console_), faults: faultLines(console_) };
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
  const decided = v.steps.filter((s) => s.ok !== undefined);
  const ok = run.status === 0 && !verdictError && decided.length > 0 && decided.every((s) => s.ok);
  board.probes.push({
    probe: probe.name, script: probe.script, out: `🗑️generated/react-verify/${probe.dir}/`,
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
