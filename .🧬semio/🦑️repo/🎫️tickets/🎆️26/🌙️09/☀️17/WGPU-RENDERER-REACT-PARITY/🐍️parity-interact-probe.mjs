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
 *      SEMIO_PROBE_SETTLE (ms per step, default 1600) · SEMIO_PROBE_HEADED=1 · SEMIO_PROBE_ONLY=<step,step>
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region ⚙️Configuration
const targets = (process.env.SEMIO_PROBE_TARGETS ?? "react,wgpu").split(",").map((name) => name.trim()).filter(Boolean);
const urls = { react: process.env.SEMIO_PROBE_REACT_URL ?? "http://127.0.0.1:6313/?plugin=puzzle3d", wgpu: process.env.SEMIO_PROBE_WGPU_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d" };
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w5b-parity");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 240);
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE ?? 1600);
const onlySteps = (process.env.SEMIO_PROBE_ONLY ?? "").split(",").map((name) => name.trim()).filter(Boolean);
const viewport = { width: 1600, height: 1000 };
const mod = process.platform === "darwin" ? "Meta" : "Control";
mkdirSync(outDir, { recursive: true });

/** 🪟️ The pane-chip keys a window publishes, matched as an id SUFFIX so the window instance in the middle
 * (`framework.window.<Window>.<chip>`) never has to be named by the journey. */
const PANE_CHIPS = ["engagement.toggle", "search.toggle", "windowControls", "utilityBar.unfold", "projection.toggle", "windowOptions.toggle"];

/** 🪪️ Control keys the two renderers genuinely spell differently. An alias is tried only after the exact /
 * suffix / containment ladder has failed, and the step records `resolved: "alias"` — so a divergent id shows
 * up in `parity.md` as a control-resolution difference instead of silently passing as "the same control".
 * Measured 2026-09-18: the wgpu tour chips are `shell.tour.{skip,next,back}` where React's are
 * `ui.introduction.{skip,next,back}`. */
const CONTROL_ALIASES = {
  "ui.introduction.skip": ["shell.tour.skip"],
  "ui.introduction.next": ["shell.tour.next"],
  "ui.introduction.back": ["shell.tour.back"],
};
//#endregion ⚙️Configuration

//#region 🧾️Report
const report = { startedAt: new Date().toISOString(), urls, targets, steps: [] };
const flush = () => {
  writeFileSync(join(outDir, "steps.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "parity.md"), parityMarkdown());
};

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
function stepVerdict(step) {
  const react = step.renderers.react;
  const wgpu = step.renderers.wgpu;
  if (!react || !wgpu) return { verdict: "unmeasured", why: `only ${react ? "react" : wgpu ? "wgpu" : "no renderer"} ran` };
  if (react.error || wgpu.error) return { verdict: "differ", why: `error react=${react.error ?? "-"} wgpu=${wgpu.error ?? "-"}` };
  if (react.resolved !== wgpu.resolved) return { verdict: "differ", why: `control resolution react=${react.resolved} wgpu=${wgpu.resolved}` };
  if (!sameList(actionIds(react), actionIds(wgpu))) return { verdict: "differ", why: `actions react=[${setText(actionIds(react))}] wgpu=[${setText(actionIds(wgpu))}]` };
  if (!sameList(react.delta?.surfacesAdded ?? [], wgpu.delta?.surfacesAdded ?? []) || !sameList(react.delta?.surfacesRemoved ?? [], wgpu.delta?.surfacesRemoved ?? [])) {
    return { verdict: "differ", why: `surfaces react=+[${setText(react.delta?.surfacesAdded)}]-[${setText(react.delta?.surfacesRemoved)}] wgpu=+[${setText(wgpu.delta?.surfacesAdded)}]-[${setText(wgpu.delta?.surfacesRemoved)}]` };
  }
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
    `**${differ.length} of ${measured.length} measured steps differ** (${report.steps.length - measured.length} unmeasured — only one renderer ran). A step matches when both renderers dispatched the same SET of action ids and moved the same surfaces; counts, rects and timings are out of the verdict by design.`,
    "",
    "| step | verdict | react actions | wgpu actions | why |",
    "| --- | --- | --- | --- | --- |",
    ...rows,
    "",
  ].join("\n");
}
//#endregion 🧾️Report

//#region 🌐️Drivers
/** 🌐️ One renderer under the probe. `observe`/`actions`/`resolve` are the three seams the two targets
 * implement differently; everything else (stepping, deltas, screenshots) is shared above them. */
async function openDriver(browser, name, url) {
  const consoleLines = [];
  const page = await browser.newPage({ viewport, deviceScaleFactor: 1 });
  const t0 = Date.now();
  const record = (source, type, text) => consoleLines.push(`${Date.now() - t0} ${source} ${type} ${text.slice(0, 4000)}`);
  page.on("console", (message) => record("page", message.type(), message.text()));
  page.on("pageerror", (error) => record("page", "pageerror", String(error)));
  page.on("worker", (worker) => worker.on("console", (message) => record("worker", message.type(), message.text())));
  await page.addInitScript(() => {
    try {
      globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
    } catch {}
  });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  return name === "react" ? reactDriver(page, name, url, consoleLines) : wgpuDriver(page, name, url, consoleLines);
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
            return { id, kind: slot ?? element.getAttribute("role") ?? element.tagName.toLowerCase(), rect: rectOf(element), pressed: element.getAttribute("aria-pressed") ?? element.getAttribute("data-state") ?? undefined };
          })
          .filter((control) => control.rect[2] > 0 && control.rect[3] > 0);
        const surfaces = [
          ...[...document.querySelectorAll("[data-window-id]")].map((element) => `window:${element.getAttribute("data-window-id")}`),
          ...[...document.querySelectorAll('[data-level="panel"]')].map((element) => `panel:${element.id || element.getAttribute("data-tab-id") || element.getAttribute("data-anchor") || "?"}`),
          ...[...document.querySelectorAll('[data-level="dialog"]')].map((element) => `dialog:${element.id || "?"}`),
          ...[...document.querySelectorAll("[data-surface-id]")].map((element) => `surface:${element.getAttribute("data-surface-id")}`),
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
    async boot() {
      for (let second = 0; second < bootSeconds; second += 1) {
        await page.waitForTimeout(1000);
        const state = await observe();
        if (state.error) return { booted: false, why: `shell error ${state.error}` };
        if (state.ready && state.controls.length > 4 && second > 4) return { booted: true, ready: state.ready };
      }
      return { booted: false, why: "never reported data-semio-os-ready" };
    },
    async resolve(key) {
      const state = await observe();
      const { row, resolved } = resolveByLadder(state.controls, key, (control) => control.id);
      return { rect: row?.rect ?? null, id: row?.id ?? null, resolved };
    },
    async click(key) {
      const { rect, id, resolved } = await this.resolve(key);
      if (!rect) return { resolved, clicked: false, why: `no control matches ${key}` };
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
    const surfaces = [...new Set([...(structure?.windowIds ?? []).map((id) => `window:${id}`), ...controls.filter((control) => control.windowId).map((control) => `surface:${control.windowId}`)])].sort();
    return { ready: structure ? "wgpu" : null, error: chrome?.error ?? structure?.error, armed: chrome?.armed ?? false, generation: chrome?.generation ?? 0, controls, surfaces, chromeAvailable: true };
  };

  const actions = async () => {
    const chrome = await dump("dumpChrome");
    return (chrome?.actions ?? []).map((entry) => ({ seq: entry.seq, controller: entry.controllerId, action: entry.action, windowId: entry.windowId, origin: "shell", outcome: "dispatched", args: entry.args }));
  };

  return {
    name,
    url,
    page,
    consoleLines,
    observe,
    actions,
    async boot() {
      for (let second = 0; second < bootSeconds; second += 1) {
        await page.waitForTimeout(1000);
        await page.mouse.move(4 + (second % 3), 4 + (second % 3)).catch(() => {});
        const structure = await dump("dumpStructure");
        if (structure && !structure.unavailable && !structure.error && second > 4) {
          await page.locator("canvas").first().click({ position: { x: 4, y: 4 }, timeout: 4000 }).catch(() => {});
          const state = await observe();
          return { booted: true, ready: "wgpu", chromeAvailable: state.chromeAvailable, armed: state.armed, hits: state.controls.length };
        }
      }
      return { booted: false, why: "dumpStructure never answered" };
    },
    async resolve(key) {
      const state = await observe();
      const { row, resolved } = resolveByLadder(state.controls, key, (control) => control.id ?? "");
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
      for (let step = 1; step <= 8; step += 1) await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 8, from[1] + ((to[1] - from[1]) * step) / 8);
      await page.mouse.up({ button: options?.button ?? "left" });
      return { kind, from, to };
    },
    async surfacePoint() {
      const meshes = await dump("dumpMeshStats");
      const surface = (meshes?.surfaces ?? [])[0];
      if (surface?.rect) return [surface.rect[0] + surface.rect[2] * 0.7, surface.rect[1] + surface.rect[3] * 0.45];
      const state = await observe();
      const window = state.controls.find((control) => control.kind === "Window" || control.kind === "ScrollRegion");
      return window ? centre(window.rect) : [viewport.width / 2, viewport.height / 2];
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
  ...PANE_CHIPS.map((chip) => ({ name: `pane-chip-${chip.replace(/[^a-z]+/gi, "-").toLowerCase()}`, run: (driver, before) => driver.clickPaneChip(before, chip) })),
  { name: "split-gutter-drag", run: (driver, before) => driver.dragGutter(before) },
  { name: "window-cap-focus", run: (driver, before) => driver.clickWindowCap(before, "focus") },
  { name: "window-cap-close", run: (driver, before) => driver.clickWindowCap(before, "close") },
  { name: "window-reopen", run: (driver, before) => driver.reopenWindow(before) },
  { name: "orbit-drag", run: async (driver) => { const at = await driver.surfacePoint(); return driver.pointer("drag", at, [at[0] + 180, at[1] + 90]); } },
  { name: "pan-drag", run: async (driver) => { const at = await driver.surfacePoint(); return driver.pointer("drag", at, [at[0] - 140, at[1] + 40], { button: "middle" }); } },
  { name: "zoom-wheel", run: async (driver) => { const at = await driver.surfacePoint(); return driver.pointer("wheel", at, at, { deltaY: -360 }); } },
  { name: "pick-instance", run: async (driver) => { const at = await driver.surfacePoint(); await driver.page.mouse.click(at[0], at[1]); return { at, button: "left" }; } },
  { name: "context-menu", run: async (driver) => { const at = await driver.surfacePoint(); await driver.page.mouse.click(at[0], at[1], { button: "right" }); return { at, button: "right" }; } },
  { name: "context-menu-dismiss", run: (driver) => driver.chord("Escape") },
  { name: "chord-command-palette", run: (driver) => driver.chord(`${mod}+k`) },
  { name: "chord-escape", run: (driver) => driver.chord("Escape") },
  { name: "chord-undo", run: (driver) => driver.chord(`${mod}+z`) },
  { name: "chord-redo", run: (driver) => driver.chord(`${mod}+Shift+z`) },
  { name: "chord-fullscreen", run: (driver) => driver.chord(`${mod}+Shift+f`) },
  { name: "chord-fullscreen-exit", run: (driver) => driver.chord(`${mod}+Shift+f`) },
  { name: "chord-panel-anchor-left", run: (driver) => driver.chord(`${mod}+Alt+1`) },
  { name: "chord-panel-anchor-right", run: (driver) => driver.chord(`${mod}+Alt+2`) },
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
function attachJourneyVerbs(driver) {
  driver.chord = async (chord) => {
    await driver.page.keyboard.press(chord).catch(() => {});
    return { chord };
  };
  /** 🪟️ A window cap chip. React renders it id-less with `data-slot="mode-dock-tab-<cap>"`, wgpu registers it
   * as `dock.tab.<path>.<windowId>.<cap>` — both END in the cap verb, which is what this matches on. */
  driver.clickWindowCap = async (_before, cap) => {
    const before = await driver.observe();
    const key = cap === "focus" ? "focus" : cap === "close" ? "close" : "maximize";
    const control = (before.controls ?? []).find((row) => new RegExp(`(^|[.\\-])${key}$`, "i").test(row.id ?? ""));
    if (!control) return { resolved: "absent", why: `no window cap chip named ${key}`, sampled: (before.controls ?? []).map((row) => row.id).filter((id) => /tab|cap|window/i.test(id ?? "")).slice(0, 10) };
    await driver.page.mouse.click(...centre(control.rect));
    return { resolved: "contains", id: control.id, rect: control.rect, cap };
  };
  driver.reopenWindow = async () => {
    const before = await driver.observe();
    const control = (before.controls ?? []).find((row) => /mode-dock-tab|dock\.tab|appSwitcher|window.*tab/i.test(row.id ?? ""));
    if (!control) return { resolved: "absent", why: "no app-switcher / dock tab to reopen from" };
    await driver.page.mouse.click(...centre(control.rect));
    return { resolved: "contains", id: control.id, rect: control.rect };
  };
  driver.dragGutter = async () => {
    const before = await driver.observe();
    const gutter = (before.controls ?? []).find((row) => /resizable-handle|separator|PanelResize|DockSplit/i.test(`${row.id ?? ""} ${row.kind ?? ""}`));
    if (!gutter) return { resolved: "absent", why: "no split gutter registered" };
    const at = centre(gutter.rect);
    await driver.pointer("drag", at, [at[0] + 120, at[1]]);
    return { resolved: "contains", id: gutter.id, kind: gutter.kind, rect: gutter.rect, to: [at[0] + 120, at[1]] };
  };
  driver.clickPaneChip = async (_before, chip) => {
    const before = await driver.observe();
    const control = (before.controls ?? []).find((row) => (row.id ?? "").endsWith(`.${chip}`));
    if (!control) return { resolved: "absent", why: `no pane chip ${chip}` };
    await driver.page.mouse.click(...centre(control.rect));
    return { resolved: "suffix", id: control.id, rect: control.rect };
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
  surfacesAdded: (after.surfaces ?? []).filter((id) => !(before.surfaces ?? []).includes(id)),
  surfacesRemoved: (before.surfaces ?? []).filter((id) => !(after.surfaces ?? []).includes(id)),
  controlsAdded: (after.controls ?? []).map((control) => control.id).filter((id) => !(before.controls ?? []).some((control) => control.id === id)).slice(0, 40),
  controlsRemoved: (before.controls ?? []).map((control) => control.id).filter((id) => !(after.controls ?? []).some((control) => control.id === id)).slice(0, 40),
  controlCount: [(before.controls ?? []).length, (after.controls ?? []).length],
});

const browser = await chromium.launch({ headless: process.env.SEMIO_PROBE_HEADED !== "1", args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const drivers = {};
for (const name of targets) {
  mkdirSync(join(outDir, name), { recursive: true });
  try {
    const driver = await openDriver(browser, name, urls[name]);
    attachJourneyVerbs(driver);
    driver.bootResult = await driver.boot();
    drivers[name] = driver;
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
    const before = await driver.observe();
    const actionsBefore = await driver.actions();
    const cursor = actionsBefore.length ? actionsBefore[actionsBefore.length - 1].seq : 0;
    let detail = null;
    let error = null;
    try {
      detail = await step.run(driver, before);
    } catch (failure) {
      error = String(failure).slice(0, 300);
    }
    await driver.page.waitForTimeout(step.settle ?? settleMs);
    const after = await driver.observe();
    const actionsAfter = await driver.actions();
    const stepActions = actionsAfter.filter((action) => action.seq > cursor);
    const shot = join(name, `${String(index).padStart(2, "0")}-${step.name}.png`);
    await driver.page.screenshot({ path: join(outDir, shot), type: "png" }).catch(() => {});
    entry.renderers[name] = {
      detail,
      error: error ?? after.error ?? null,
      resolved: detail?.resolved ?? (detail?.clicked === false ? "absent" : "n/a"),
      actions: stepActions.slice(0, 60),
      actionCounts: actionCounts(stepActions),
      actionControllers: actionControllers(stepActions),
      delta: diff(before, after),
      state: { ready: after.ready ?? null, surfaces: (after.surfaces ?? []).length, controls: (after.controls ?? []).length, role: after.role, example: after.example, armed: after.armed },
      screenshot: shot,
    };
  }
  report.steps.push(entry);
  flush();
  const line = Object.entries(entry.renderers).map(([name, side]) => `${name}=${side.resolved}/${actionIds(side).length}v${side.error ? `/err` : ""}`).join(" ");
  console.log(`[DEBUG] ${String(index).padStart(2, "0")} ${step.name} ${line} ${stepVerdict(entry).verdict}`);
}

for (const [name, driver] of Object.entries(drivers)) writeFileSync(join(outDir, name, "console.txt"), driver.consoleLines.join("\n"));
report.finishedAt = new Date().toISOString();
flush();
const differ = report.steps.filter((step) => stepVerdict(step).verdict === "differ").length;
console.log(`[DEBUG] DONE steps=${report.steps.length} differ=${differ} out=${outDir}`);
await browser.close();
//#endregion 🏃️Run
