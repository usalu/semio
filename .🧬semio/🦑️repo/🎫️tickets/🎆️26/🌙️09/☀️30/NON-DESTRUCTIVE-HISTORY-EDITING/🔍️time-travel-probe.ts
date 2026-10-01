/** 🔍️ Time-travel end-to-end probe for the puzzle 2d React serve on `127.0.0.1:<port>` (ticket 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING, work package W3-E2E). One headless Chromium, one tab at a time, one fresh browser
 * context per UI locale (`en`, then `de`, chosen through `navigator.language`), nine numbered steps per locale:
 *
 * 1. boot: `data-board-fixture-parsed` first, then a board with nodes and the locale of the history panel; then the
 *    document is bound to a fresh local folder through the sync card (`--folder-at=1`, the default), so every edit persists;
 * 2. select two nodes, drag them by (+80,+40) world units → exactly one new document row labelled from its
 *    `drag-selection` mutation, `data-board-positions-json` moved by the offset; then one downstream drag of a third
 *    node, so "downstream is not applied" is observable;
 * 3. Edit the drag mutation (`historyEditBegin`) → the React band (`role=status`) in `editing`, the preview = state
 *    before the drag + the draft (design §4), the downstream drag not applied, the editor's dx/dy steppers (snap step)
 *    and the targets reference list;
 * 4. dx → 120 through the stepper (keyboard: fill, Enter, ArrowUp, ArrowDown) → preview; Accept → replay → review
 *    `ready` → head at +120 with the downstream drag re-applied;
 * 5. Finalize → dialog (destructive Overwrite, New alternative + name) → Overwrite → band gone, an
 *    "History edited — overwrite" row; after step 8 a page reload plus a re-attach of the same folder must restore the head
 *    and every document row;
 * 6. Undo (chord, then button) → +80; Redo → +120; the undone/redone rows;
 * 7. a second session on dy → New alternative with a name → the alternative row, the head and the Alternatives section
 *    (`framework.history.alternative.<id>`); switching to the trunk and back when the trunk is listed, else a second
 *    alternative and switching between the two;
 * 8. fatal path: duplicate a node, drag the clone, withdraw the clone's `create-node` → review `blocked`, the drag row
 *    reads Error "Target missing", Finalize disabled; Next problem → Withdraw the drag → `ready`; Exit → zero trace;
 * 9. console: uncaught page errors fail the run; errors, warnings, hard faults and `[DEBUG] ` lines are digested.
 *
 * Every control is the real one, reached the way a user reaches it: the windowed history body is scrolled until the
 * section holds the row, steppers take keyboard input.
 *
 * `--renderer=wgpu` drives the puzzle 2d wgpu shell (default port 6112) through the same steps: rows, buttons, inputs and the
 * dialog through its ARIA mirror (`#semio-wgpu-accessibility`), chrome through `dumpChrome` hit rects, the board with the
 * pointer and keyboard on the canvas at the positions `semioWgpuIntrospection.dumpBoard2d` publishes (region `🔖️Wgpu`).
 *
 * Run: `bun 🔍️time-travel-probe.ts [--renderer=react|wgpu] [--port=6012|6112] [--only=1,2,3] [--locales=en,de] [--chords=de]
 * [--folder-at=1|5] [--explore]`.
 * `--chords=<locales>` drives Accept/Exit through the band chords (`alt+enter`, `alt+shift+backspace`) in those
 * locales and through the band buttons elsewhere. `--explore` boots each locale, opens the history panel, dumps the DOM
 * inventory and exits. Outputs: `🗑️generated/e2e/probe-<stamp>.md` (step lines, verdicts, notes, timeline, console
 * digest), `probe-<stamp>.ndjson` (one record per verdict/note/dump) and `probe-<stamp>-<locale>-s<step>-<tag>.png|json`.
 * @see ./📋️design.md
 * @see ./📓️w2-a-report.md
 * @see ./📓️w2-b-report.md
 * @see ../../☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts */
import { chromium, type BrowserContext, type ConsoleMessage, type Page } from "playwright";
import { appendFileSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🔖️Arguments
type Locale = "en" | "de";
type Positions = Record<string, [number, number]>;
type Point = { x: number; y: number };

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated", "e2e");
mkdirSync(OUT, { recursive: true });
const argument = (name: string) => process.argv.find((entry) => entry.startsWith(`--${name}=`))?.slice(name.length + 3);
const renderer: "react" | "wgpu" = argument("renderer") === "wgpu" ? "wgpu" : "react";
const port = argument("port") ?? (renderer === "wgpu" ? "6112" : "6012");
const only = argument("only") ? new Set(argument("only")!.split(",").map((entry) => Number(entry.trim())).filter(Number.isFinite)) : null;
const locales = (argument("locales") ?? "en,de").split(",").map((entry) => entry.trim()).filter((entry): entry is Locale => entry === "en" || entry === "de");
const chordLocales = new Set((argument("chords") ?? "de").split(",").map((entry) => entry.trim()));
const explore = process.argv.includes("--explore");
const folderAt = Number(argument("folder-at") ?? "1");
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const base = `probe-${renderer === "wgpu" ? "wgpu-" : ""}${stamp}`;
const ndjsonPath = join(OUT, `${base}.ndjson`);
writeFileSync(ndjsonPath, "");
const OVERVIEW = "2d-overview";
const HISTORY_TAB = "framework.panel.history";
const t0 = Date.now();
//#endregion 🔖️Arguments

//#region 🔖️Report
type VerdictRow = { locale: Locale; step: number; name: string; ok: boolean; detail: Record<string, unknown> };
const verdicts: VerdictRow[] = [];
const notes: string[] = [];
const timeline: string[] = [];
let currentLocale: Locale = "en";
let currentStep = 0;

/** 🧾️ One ndjson record, stamped with the run clock, the locale and the step. */
const emit = (record: Record<string, unknown>) => appendFileSync(ndjsonPath, `${JSON.stringify({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, ...record })}\n`);

/** 🕰️ One timeline line, printed and kept for the markdown report. */
const log = (message: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${currentLocale}/${currentStep} ${message}`;
  timeline.push(row);
  console.log(row);
};

/** ⚖️ One PASS/FAIL verdict of the current locale and step. */
const verdict = (name: string, ok: boolean, detail: Record<string, unknown> = {}) => {
  verdicts.push({ locale: currentLocale, step: currentStep, name, ok, detail });
  emit({ kind: "verdict", name, verdict: ok ? "PASS" : "FAIL", ...detail });
  log(`${ok ? "PASS" : "FAIL"} ${name} ${JSON.stringify(detail).slice(0, 600)}`);
  return ok;
};

/** 📝️ An observation that is neither PASS nor FAIL (timing, a design reading, an inventory). */
const note = (name: string, detail: Record<string, unknown> = {}) => {
  const row = `${currentLocale}/${currentStep} ${name} ${JSON.stringify(detail).slice(0, 900)}`;
  notes.push(row);
  emit({ kind: "note", name, ...detail });
  log(`NOTE ${name} ${JSON.stringify(detail).slice(0, 600)}`);
};
//#endregion 🔖️Report

//#region 🔖️Copy
/** 🌍️ The localized texts every verdict compares against, byte-equal with their Rust/TS producers:
 * drag/create labels (`✋️drag-selection`, `🌱create-node`), history-edit rows (`SupersedeLedger::label`), the finalize
 * dialog (`history_edit_finalize_dialog`), the panel copy (`HistoryPanelText`) and the review lines (`TimeTravelLabel`). */
const COPY = {
  en: {
    commands: "Commands",
    items: (count: number) => (count === 1 ? "1 item" : `${count} items`),
    drag: (count: number, dx: string, dy: string) => `Drag ${COPY.en.items(count)} by (${dx}, ${dy})`,
    createNode: (id: string) => `Create node "${id}"`,
    row: (role: "edit" | "undo" | "redo", scope: string | null, count: number) =>
      `${{ edit: "History edited", undo: "History edit undone", redo: "History edit redone" }[role]} — ${scope === null ? "overwrite" : `alternative ${scope}`}: ${count === 1 ? "1 mutation" : `${count} mutations`}`,
    edit: /^(Edit)$/,
    withdraw: /^(Withdraw)$/,
    pending: "Not applied while editing",
    targetMissing: "Error: Target missing",
    severityError: /\b(Error|Fatal)\b/,
    stageEditing: "Editing a mutation",
    reviewReady: "Ready to finalize",
    reviewBlocked: "Errors must be fixed or withdrawn before finalizing",
    dialogTitle: "Finish editing history",
    overwrite: "Overwrite",
    overwriteDescription: "Replaces the edited mutations in every alternative that contains them.",
    newAlternative: "New alternative",
    defaultName: "Edited history",
    tourButtons: [/^\s*(x\s*)?skip\s*$/i, /^\s*close\s*$/i],
  },
  de: {
    commands: "Befehle",
    items: (count: number) => (count === 1 ? "1 Element" : `${count} Elemente`),
    drag: (count: number, dx: string, dy: string) => `${COPY.de.items(count)} um (${dx}; ${dy}) ziehen`,
    createNode: (id: string) => `Knoten "${id}" erstellen`,
    row: (role: "edit" | "undo" | "redo", scope: string | null, count: number) =>
      `${{ edit: "Verlauf bearbeitet", undo: "Verlaufsbearbeitung rückgängig", redo: "Verlaufsbearbeitung wiederhergestellt" }[role]} — ${scope === null ? "überschrieben" : `Alternative ${scope}`}: ${count === 1 ? "1 Mutation" : `${count} Mutationen`}`,
    edit: /^(Bearbeiten)$/,
    withdraw: /^(Zurückziehen)$/,
    pending: "Beim Bearbeiten nicht angewendet",
    targetMissing: "Fehler: Ziel fehlt",
    severityError: /\b(Fehler|Kritisch)\b/,
    stageEditing: "Mutation wird bearbeitet",
    reviewReady: "Bereit zum Abschließen",
    reviewBlocked: "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden",
    dialogTitle: "Verlaufsbearbeitung abschließen",
    overwrite: "Überschreiben",
    overwriteDescription: "Ersetzt die bearbeiteten Mutationen in jeder Alternative, die sie enthält.",
    newAlternative: "Neue Alternative",
    defaultName: "Bearbeiteter Verlauf",
    tourButtons: [/^\s*(x\s*)?(skip|überspringen)\s*$/i, /^\s*(close|schließen)\s*$/i],
  },
} as const;

/** 🔢️ `puzzle2d_selection_number`: two decimals, trailing zeros and a negative zero dropped, a decimal comma in German. */
const labelNumber = (value: number, locale: Locale) => {
  const rounded = Math.round(value * 100) / 100;
  const text = (rounded === 0 ? 0 : rounded).toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
  return locale === "de" ? text.replace(".", ",") : text;
};
const dragLabel = (count: number, dx: number, dy: number, locale = currentLocale) => COPY[locale].drag(count, labelNumber(dx, locale), labelNumber(dy, locale));
//#endregion 🔖️Copy

//#region 🔖️Console
type ConsoleRow = { t: number; locale: Locale; step: number; source: "page" | "worker"; type: string; text: string };
const consoleRows: ConsoleRow[] = [];
const pageErrors: { locale: Locale; step: number; text: string }[] = [];
const hardFaults: { locale: Locale; step: number; text: string }[] = [];
const BENIGN_RE = /contributions document sources|staged plugin module|\[stale\]|activate-puzzle2d-react-dev|transform freshness|Download the React DevTools/i;
const HARD_FAULT_RE =
  /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked/i;

/** 📥️ Keeps one console line (page or worker) in the ring and classifies it as a hard fault. */
const keepConsole = (source: ConsoleRow["source"], message: ConsoleMessage) => {
  const row: ConsoleRow = { t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source, type: message.type(), text: message.text().slice(0, 2000) };
  consoleRows.push(row);
  if (consoleRows.length > 20000) consoleRows.shift();
  if (HARD_FAULT_RE.test(row.text) && !BENIGN_RE.test(row.text)) hardFaults.push({ locale: row.locale, step: row.step, text: row.text.slice(0, 400) });
};

/** 🎧️ Subscribes one page and every worker it spawns: page lines that belong to a worker are left to that worker's
 * own listener, so no line is counted twice. */
const listen = (target: Page) => {
  target.on("console", (message) => {
    if (message.worker?.()) return;
    keepConsole("page", message);
  });
  target.on("worker", (worker) => worker.on("console", (message) => keepConsole("worker", message)));
  target.on("response", (response) => {
    if (response.url().includes("/semio-backbone")) consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "backbone", text: `HTTP ${response.status()} ${response.request().method()} ${decodeURIComponent(response.url()).slice(0, 400)}` });
    if (response.status() < 400) return;
    consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "http", text: `HTTP ${response.status()} ${response.request().method()} ${response.url().slice(0, 300)}` });
  });
  target.on("pageerror", (error) => {
    const text = String(error?.stack ?? error).slice(0, 1200);
    pageErrors.push({ locale: currentLocale, step: currentStep, text });
    consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "pageerror", text });
  });
};
//#endregion 🔖️Console

//#region 🔖️Page
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer"] });
let context: BrowserContext;
let page: Page;
let mod = "Meta";
let navigations = 0;
let expectedReloads = 0;

/** 🧭️ One `page.evaluate` that never takes the run down (a reload race answers `fallback`). */
const evalSafe = async <T, A = undefined>(fn: (arg: A) => T | Promise<T>, fallback: T, arg?: A): Promise<T> => {
  try {
    return (await page.evaluate(fn as (value: unknown) => T | Promise<T>, arg as unknown)) as T;
  } catch {
    return fallback;
  }
};
const sleep = (ms: number) => page.waitForTimeout(ms);

/** ⏳️ Polls `read` until `settled` holds or `timeoutMs` passes; answers the last value and the wait. */
const waitUntil = async <T,>(read: () => Promise<T>, settled: (value: T) => boolean, timeoutMs = 20000, everyMs = 250) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < timeoutMs) {
    await sleep(everyMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: settled(value) };
};

/** 📸️ One screenshot of the current step. */
const shot = async (tag: string) => {
  const path = join(OUT, `${base}-${currentLocale}-s${currentStep}-${tag}.png`);
  await page.screenshot({ path }).catch(() => {});
  return path;
};

/** 🧰️ Writes one diagnostic JSON beside the screenshots and returns its path. */
const dumpJson = (tag: string, value: unknown) => {
  const path = join(OUT, `${base}-${currentLocale}-s${currentStep}-${tag}.json`);
  writeFileSync(path, JSON.stringify(value, null, 1));
  return path;
};
//#endregion 🔖️Page

//#region 🔖️Wgpu
/** 🧊️ The wgpu shell paints on a canvas and publishes no DOM of its own. What a user (or an assistive technology) can
 * reach is: the ARIA mirror `#semio-wgpu-accessibility` (one element per projected node, `data-node-key` = the node key or
 * chrome control id, `data-window` = its window; a synthetic `click` on an actionable element is an `accessibility-activate`,
 * typing into a mirrored input an `accessibility-value`), the chrome hit registry (`semioWgpuIntrospection.dumpChrome()`:
 * every pointer target with its page rect, and the dispatched-action ledger when `SEMIO_RUNTIME_DIAGNOSTICS` is armed), and
 * the canvas itself for pointer and keyboard input. The board's positions, camera and selection are read from
 * `semioWgpuIntrospection.dumpBoard2d(windowId)`, the wgpu twin of React's `data-board-*` vitals.
 * @see ../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts
 * @see ../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs */
type MirrorNode = { key: string; window: string; role: string; label: string; description: string; disabled: boolean; expanded: string | null; pressed: string | null; selected: string | null; value: string | null; valueNow: string | null; valueMax: string | null; actionable: boolean; live: string | null; busy: boolean; shortcut: string | null; tag: string };
type Board2dSurface = { surfaceId: string; windowId: string; rect: [number, number, number, number]; camera: { x: number; y: number; zoom: number }; positions: Positions; selection: string[]; nodes: number; edges: number; handles: number; parsed: boolean };

/** 🪞️ Every mirrored node in reading order, with the ARIA state the mirror stamps. */
const mirror = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).map((el) => {
        const describedBy = el.getAttribute("aria-describedby");
        return {
          key: el.dataset.nodeKey ?? "",
          window: el.dataset.window ?? "",
          role: el.getAttribute("role") ?? el.tagName.toLowerCase(),
          label: el.getAttribute("aria-label") ?? el.textContent ?? "",
          description: describedBy ? (document.getElementById(describedBy)?.textContent ?? "") : "",
          disabled: el.getAttribute("aria-disabled") === "true",
          expanded: el.getAttribute("aria-expanded"),
          pressed: el.getAttribute("aria-pressed"),
          selected: el.getAttribute("aria-selected"),
          value: el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement ? el.value : null,
          valueNow: el.getAttribute("aria-valuenow"),
          valueMax: el.getAttribute("aria-valuemax"),
          actionable: el.dataset.actionable === "true",
          live: el.getAttribute("aria-live"),
          busy: el.getAttribute("aria-busy") === "true",
          shortcut: el.getAttribute("aria-keyshortcuts"),
          tag: el.tagName.toLowerCase(),
        };
      }),
    [] as MirrorNode[],
  );

/** 🔎️ The mirrored node an authored key names: exact, then a `/`- or `.`-delimited suffix (a chrome control id prefixes
 * the tab id: `shell.panel.tab.<anchor>.<tabId>`), never a bare substring. */
const mirrorFind = (nodes: MirrorNode[], authored: string) =>
  nodes.find((node) => node.key === authored) ?? nodes.find((node) => !node.key.includes("::") && (node.key.endsWith(`/${authored}`) || node.key.endsWith(`.${authored}`)));

/** 🧷️ Stamps the mirrored element of `key` (in `window` when given) with a probe token and answers its locator. */
const mirrorLocator = async (key: string, windowId?: string) => {
  const token = markToken();
  const found = await evalSafe(
    (arg) => {
      const el = Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).find((node) => node.dataset.nodeKey === arg.key && (!arg.windowId || node.dataset.window === arg.windowId));
      el?.setAttribute("data-probe-target", arg.token);
      return Boolean(el);
    },
    false,
    { key, windowId: windowId ?? null, token },
  );
  return found ? page.locator(`[data-probe-target="${token}"]`).first() : null;
};

/** 👆️ Activates one mirrored node the way an assistive technology does: a `click` on its mirror element, which the mirror
 * forwards as `accessibility-activate` to the renderer. */
const mirrorActivate = async (key: string, windowId?: string) => {
  const target = await mirrorLocator(key, windowId);
  if (!target) return false;
  await target.dispatchEvent("click").catch(() => {});
  return true;
};

/** 🎯️ Focuses one mirrored node (the mirror forwards `accessibility-focus`, so keyboard input then reaches that node). */
const mirrorFocus = async (key: string) => {
  const target = await mirrorLocator(key);
  if (!target) return false;
  await target.focus().catch(() => {});
  return true;
};

/** 🎯️ The chrome hit registry: every pointer target with its page rect (CSS px). */
const chromeHits = async () => {
  const json = await evalSafe(async () => (await (window as unknown as { semioWgpuIntrospection?: { dumpChrome: () => Promise<string> } }).semioWgpuIntrospection?.dumpChrome()) ?? "", "");
  try {
    return (JSON.parse(json || "{}") as { hits?: { controlId: string; kind: string; rect: [number, number, number, number]; windowId?: string; action?: string }[]; actions?: { seq: number; controllerId: string; action: string; origin: string; args?: string }[] });
  } catch {
    return {};
  }
};

/** 🖱️ Presses a chrome control with the real pointer at the centre of its registered hit rect. */
const pointerPress = async (authored: string) => {
  const hits = (await chromeHits()).hits ?? [];
  const hit = hits.find((row) => row.controlId === authored) ?? hits.find((row) => row.controlId.endsWith(`.${authored}`) || row.controlId.endsWith(`/${authored}`));
  if (!hit) return null;
  await page.mouse.click(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
  return hit.controlId;
};

/** 👆️ Presses `authored` through the mirror (activation), else with the pointer on its chrome hit rect. */
const wgpuPress = async (authored: string) => {
  const node = mirrorFind(await mirror(), authored);
  if (node && (await mirrorActivate(node.key, node.window))) return `mirror:${node.key}`;
  const hit = await pointerPress(authored);
  return hit ? `pointer:${hit}` : null;
};

/** 🎲️ The overview board as `dumpBoard2d` publishes it, or null while the renderer lacks the export (a named prerequisite). */
const wgpuBoard = async () => {
  const json = await evalSafe(
    async (windowId) => {
      const introspection = (window as unknown as { semioWgpuIntrospection?: { dumpBoard2d?: (windowId?: string) => Promise<string> } }).semioWgpuIntrospection;
      return typeof introspection?.dumpBoard2d === "function" ? await introspection.dumpBoard2d(windowId) : null;
    },
    null as string | null,
    OVERVIEW,
  );
  if (!json) return null;
  try {
    const dump = JSON.parse(json) as { surfaces?: Board2dSurface[] };
    return dump.surfaces?.find((surface) => surface.windowId === OVERVIEW) ?? dump.surfaces?.[0] ?? null;
  } catch {
    return null;
  }
};

/** 📜️ The contract `dumpBoard2d` is asked to answer — the wgpu twin of React's Board2dHost `data-board-*` vitals. */
const WGPU_BOARD_CONTRACT = "semioWgpuIntrospection.dumpBoard2d(windowId?) → {surfaces:[{surfaceId, windowId, rect:[x,y,w,h] (page CSS px of the board canvas), camera:{x,y,zoom}, positions:{nodeId:[x,y]} (the published fixture), selection:[id], nodes, edges, handles, parsed}]} — read-only, from the Board2d scene the frame worker already holds (fixture_json, camera_json, selection_json)";

/** 🧊️ Band controls as the wgpu shell names them (`⏪️time-travel` `TimeTravelControl::control_id`). */
const WGPU_BAND_CONTROLS: readonly (readonly [string, string])[] = [
  ["accept", "ui.timeTravel.accept"],
  ["discard", "ui.timeTravel.discard"],
  ["cancelReplay", "shell.time-travel.cancel-replay"],
  ["rerun", "shell.time-travel.rerun"],
  ["finalize", "shell.time-travel.finalize"],
  ["back", "shell.time-travel.back"],
  ["exit", "ui.timeTravel.exit"],
];

/** 🪧️ Stage and review words of the band's announced message (`TimeTravelLabel`, en and de), in match order. */
const BAND_WORDS: readonly (readonly [string, RegExp])[] = [
  ["replaying", /Replaying later mutations|Spätere Mutationen werden neu angewendet/],
  ["reviewing", /Reviewing the edited history|Bearbeiteter Verlauf wird geprüft/],
  ["choosing", /Choose how to finalize|Art des Abschlusses wählen/],
  ["finalizing", /Finalizing the history edit|Verlaufsbearbeitung wird abgeschlossen/],
  ["editing", /Editing a mutation|Mutation wird bearbeitet/],
];
const REVIEW_WORDS: readonly (readonly [string, RegExp])[] = [
  ["ready", /Ready to finalize|Bereit zum Abschließen/],
  ["blocked", /Errors must be fixed or withdrawn before finalizing|Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden/],
  ["needsReplay", /Replay needed|Neu anwenden nötig/],
  ["noChanges", /No changes: showing the current history|Keine Änderungen: aktueller Verlauf wird angezeigt/],
];

/** 📣️ The band as the wgpu shell announces it: the live `shell.time-travel.status` node (its message joins stage · target ·
 * progress · review · worst · fault · accepted) and the band controls the mirror carries. */
const wgpuBand = async (): Promise<Band | null> => {
  const nodes = await mirror();
  const status = nodes.find((node) => node.key === "shell.time-travel.status");
  if (!status) return null;
  const text = status.label;
  const controls = WGPU_BAND_CONTROLS.map(([control, key]) => [control, nodes.find((node) => node.key === key)] as const)
    .filter((entry): entry is readonly [string, MirrorNode] => Boolean(entry[1]))
    .map(([control, node]) => ({ control, disabled: node.disabled, title: node.description || null, keys: node.shortcut, text: node.label }));
  return {
    stage: BAND_WORDS.find(([, pattern]) => pattern.test(text))?.[0] ?? "",
    role: status.role,
    live: status.live,
    generation: null,
    text,
    review: REVIEW_WORDS.find(([, pattern]) => pattern.test(text))?.[0] ?? null,
    outcome: /Worst outcome: (\w+)|Schwerstes Ergebnis: (\w+)/.exec(text)?.slice(1).find(Boolean)?.toLowerCase() ?? null,
    fault: null,
    target: text,
    progress: status.valueNow !== null && status.valueMax !== null ? `${status.valueNow}/${status.valueMax}` : null,
    controls,
  };
};
//#endregion 🔖️Wgpu

//#region 🔖️Board
type Vitals = { surface: string; nodes: number; edges: number; handles: number; parsed: string; selection: string; camera: string; transform: string; utility: string };

/** 🩺️ The overview pane's board vitals, read without a guest round trip: React's `data-board-*` attributes, wgpu's
 * `dumpBoard2d`. */
const vitals = async (): Promise<Vitals | null> => {
  if (renderer === "wgpu") {
    const board = await wgpuBoard();
    return board ? { surface: board.surfaceId, nodes: board.nodes, edges: board.edges, handles: board.handles, parsed: String(board.parsed), selection: JSON.stringify(board.selection), camera: JSON.stringify(board.camera), transform: "", utility: "" } : null;
  }
  return reactVitals();
};

/** 🩺️ React's Board2dHost `data-board-*` vitals of the overview pane. */
const reactVitals = () =>
  evalSafe(
    (surface) => {
      const el = document.querySelector(`[data-surface-id="${surface}"]`);
      if (!el) return null;
      return {
        surface,
        nodes: Number(el.getAttribute("data-board-nodes") ?? "-1"),
        edges: Number(el.getAttribute("data-board-edges") ?? "-1"),
        handles: Number(el.getAttribute("data-board-handles") ?? "-1"),
        parsed: el.getAttribute("data-board-fixture-parsed") ?? "",
        selection: el.getAttribute("data-board-selection-json") ?? "",
        camera: el.getAttribute("data-board-camera-json") ?? "",
        transform: el.getAttribute("data-board-transform-json") ?? "",
        utility: el.getAttribute("data-board-active-utility") ?? "",
      };
    },
    null as Vitals | null,
    `window:${OVERVIEW}`,
  );

/** 📍️ Every node position the overview pane publishes (React `data-board-positions-json`, wgpu `dumpBoard2d`). */
const positions = async (): Promise<Positions> => (renderer === "wgpu" ? ((await wgpuBoard())?.positions ?? {}) : reactPositions());
const reactPositions = async () => JSON.parse(await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-positions-json") ?? "{}", "{}", `window:${OVERVIEW}`)) as Positions;
const selectionIds = (v: Vitals | null) => {
  try {
    return JSON.parse(v?.selection || "[]") as string[];
  } catch {
    return [] as string[];
  }
};
const cameraOf = (v: Vitals | null) => {
  try {
    const camera = JSON.parse(v?.camera || "{}") as { x?: number; y?: number; zoom?: number };
    return { x: camera.x ?? 0, y: camera.y ?? 0, zoom: camera.zoom && camera.zoom > 0 ? camera.zoom : 1 };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
};
const paneBox = async () => {
  if (renderer === "wgpu") {
    const rect = (await wgpuBoard())?.rect;
    return rect ? { x: rect[0], y: rect[1], width: rect[2], height: rect[3] } : { x: 0, y: 0, width: 1, height: 1 };
  }
  return (await page.locator(`[data-surface-id="window:${OVERVIEW}"] canvas`).first().boundingBox().catch(() => null)) ?? { x: 0, y: 0, width: 1, height: 1 };
};

/** 🎯️ World → screen through the published camera: the pane centre is the camera position. */
const toScreen = (world: [number, number], camera: { x: number; y: number; zoom: number }, box: { x: number; y: number; width: number; height: number }): Point => ({
  x: box.x + box.width / 2 + (world[0] - camera.x) * camera.zoom,
  y: box.y + box.height / 2 + (world[1] - camera.y) * camera.zoom,
});
const inside = (point: Point, box: { x: number; y: number; width: number; height: number }, margin: number) => point.x > box.x + margin && point.x < box.x + box.width - margin && point.y > box.y + margin && point.y < box.y + box.height - margin;

/** 🗺️ The screen layout of every node: its point, whether it is inside the pane and its nearest neighbour in pixels. */
const layout = async () => {
  const [v, p, box] = [await vitals(), await positions(), await paneBox()];
  const camera = cameraOf(v);
  const points = Object.entries(p).map(([id, world]) => ({ id, world, at: toScreen(world, camera, box) }));
  const rows = points.map((row) => ({ ...row, clearance: Math.min(...points.filter((other) => other.id !== row.id).map((other) => Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y)), 1e9) }));
  return { camera, box, rows };
};

/** 🔭️ Zooms out at the pane centre until at least `minimum` nodes are inside the pane with some clearance. */
const frameBoard = async (minimum: number) => {
  for (let attempt = 0; attempt < 6; attempt++) {
    const { box, rows } = await layout();
    const usable = rows.filter((row) => inside(row.at, box, 90) && row.clearance >= 14);
    if (usable.length >= minimum) return usable.length;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(1200);
  }
  return 0;
};

/** 🎯️ Picks `count` distinct nodes that are inside the pane, clickable (≥ 14 px from any other node), whose
 * `offsets` (world units) keep them inside the pane, and that are ≥ `spread` px apart from each other. */
const pickNodes = async (count: number, offsets: [number, number][], exclude: string[], spread = 40) => {
  const { camera, box, rows } = await layout();
  const picked: typeof rows = [];
  const candidates = rows
    .filter((row) => !exclude.includes(row.id) && row.clearance >= 14 && inside(row.at, box, 90))
    .filter((row) => offsets.every(([dx, dy]) => inside({ x: row.at.x + dx * camera.zoom, y: row.at.y + dy * camera.zoom }, box, 60)))
    .sort((a, b) => b.clearance - a.clearance);
  for (const row of candidates) {
    if (picked.length === count) break;
    if (picked.every((other) => Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y) >= spread && Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y) <= 420)) picked.push(row);
  }
  return { camera, box, picked };
};

/** 🧬️ A node to duplicate whose clone (`duplicate_selection_in_fixture` offsets it by +24,+24 world units) and a drag
 * destination for that clone are both ≥ 55 world units from every other node — the press on the clone lands on the
 * clone, never on a neighbour's free handle. Zooms out until one is inside the pane. */
const pickCloneSource = async (exclude: string[]) => {
  const moves: [number, number][] = [[70, 70], [90, 0], [0, 90], [-90, 0], [0, -90], [70, -70], [-70, 70], [-70, -70]];
  for (let attempt = 0; attempt < 6; attempt++) {
    const { camera, box, rows } = await layout();
    const clear = (world: [number, number], skip: string[]) => rows.every((other) => skip.includes(other.id) || Math.hypot(other.world[0] - world[0], other.world[1] - world[1]) >= 55);
    for (const row of rows) {
      if (exclude.includes(row.id) || row.clearance < 14 || !inside(row.at, box, 90)) continue;
      const clone: [number, number] = [row.world[0] + 24, row.world[1] + 24];
      if (!clear(clone, [row.id])) continue;
      const move = moves.find(([dx, dy]) => {
        const destination: [number, number] = [clone[0] + dx, clone[1] + dy];
        return clear(destination, []) && Math.hypot(destination[0] - row.world[0], destination[1] - row.world[1]) >= 55 && inside(toScreen(destination, camera, box), box, 60);
      });
      if (move) return { row, move, zoom: camera.zoom };
    }
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(1200);
  }
  return null;
};

/** 🖱️ A press–move–release of `steps` pointer moves from `from` by `(dx, dy)` screen pixels. */
const dragBy = async (from: Point, dx: number, dy: number, steps = 12) => {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  for (let index = 1; index <= steps; index++) await page.mouse.move(from.x + (dx * index) / steps, from.y + (dy * index) / steps);
  await page.mouse.up();
};

/** 🧮️ `after[id] - before[id]`, or null when either is missing. */
const offsetOf = (before: Positions, after: Positions, id: string): [number, number] | null => (before[id] && after[id] ? [after[id][0] - before[id][0], after[id][1] - before[id][1]] : null);
const near = (a: number, b: number, eps: number) => Math.abs(a - b) <= eps;
/** 📐️ Whether `id` sits at `origin + (dx, dy)` within `eps`. */
const placed = (p: Positions, id: string, origin: [number, number] | undefined, dx: number, dy: number, eps = 0.02) => Boolean(origin && p[id] && near(p[id][0], origin[0] + dx, eps) && near(p[id][1], origin[1] + dy, eps));
/** 🧾️ The ids whose positions differ between two readings (beyond `eps`), plus ids present in only one. */
const movedIds = (a: Positions, b: Positions, eps = 1e-6) => {
  const ids = new Set([...Object.keys(a), ...Object.keys(b)]);
  return [...ids].filter((id) => !a[id] || !b[id] || !near(a[id][0], b[id][0], eps) || !near(a[id][1], b[id][1], eps));
};
//#endregion 🔖️Board

//#region 🔖️Chrome
/** 🪪️ The live DOM id of an authored id: exact, or namespaced `…/<authored>` (panel bodies, window measures). */
const resolveDomId = (authored: string) =>
  evalSafe(
    (target) => {
      for (const el of Array.from(document.querySelectorAll("[id]"))) if (el.id === target || el.id.endsWith(`/${target}`)) return el.id;
      return null;
    },
    null as string | null,
    authored,
  );
const byId = (id: string) => page.locator(`[id="${id.replace(/"/g, '\\"')}"]`).first();

/** 🔎️ The live handle of an authored key whichever renderer answers: React's DOM id, wgpu's mirror key. */
const presentKey = async (authored: string) => (renderer === "wgpu" ? (mirrorFind(await mirror(), authored)?.key ?? null) : resolveDomId(authored));

/** 🪟️ The per-window time-travel indicators as `stage-or-role:accessible name` (React `[data-semio-time-travel-indicator]`,
 * wgpu `framework.window.<id>.timeTravel.indicator`, a `note`). */
const indicators = async () =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /framework\.window\..+\.timeTravel\.indicator$/.test(node.key)).map((node) => `${node.role}:${node.label}`)
    : evalSafe(() => Array.from(document.querySelectorAll("[data-semio-time-travel-indicator]")).map((el) => `${el.getAttribute("data-semio-time-travel-indicator")}:${el.getAttribute("aria-label")}`), [] as string[]);

/** 👥️ The presence roster's accessible text (React `#s-presence-peers`, wgpu the `s-presence-peers` status node). */
const presenceText = async () =>
  renderer === "wgpu"
    ? (mirrorFind(await mirror(), "s-presence-peers")?.label ?? null)
    : evalSafe(() => {
        const el = document.getElementById("s-presence-peers");
        return el ? `${el.getAttribute("aria-label") ?? ""} ${(el as HTMLElement).innerText}`.trim() : null;
      }, null as string | null);

/** 📝️ The visible text of an authored key: React's `innerText`, wgpu's accessible name and description. */
const textOfKey = async (authored: string) => {
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), authored);
    return node ? `${node.label} ${node.description}`.trim() : "";
  }
  const id = await resolveDomId(authored);
  return id ? await byId(id).innerText().catch(() => "") : "";
};

/** 🪟️ The dock panels that are open, by tab id (wgpu: the chrome tab `shell.panel.tab.<anchor>.<tabId>` reads pressed). */
const openPanelTabIds = async () =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /^shell\.panel\.tab\./.test(node.key) && (node.pressed === "true" || node.selected === "true")).map((node) => node.key.replace(/^shell\.panel\.tab\.[^.]+\./, ""))
    : reactOpenPanelTabIds();
const reactOpenPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((el) => (el as HTMLElement).offsetParent !== null).map((el) => el.id.replace(/^framework\.panelTab\./, "")), [] as string[]);

/** 🪟️ Opens (never toggles) the panel tab `id`. */
const openTab = async (id: string) => {
  if ((await openPanelTabIds()).includes(id)) return true;
  if (renderer === "wgpu") {
    const via = await wgpuPress(id);
    return via !== null && (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
  }
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  if (!(await tab.count().catch(() => 0))) return false;
  await tab.click({ timeout: 4000 }).catch(() => {});
  return (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
};

/** 🪟️ Closes every open dock panel (they overlay the board, and a pointer gesture there lands on the panel). */
const closePanels = async () => {
  for (const id of await openPanelTabIds()) {
    if (renderer === "wgpu") await wgpuPress(id);
    else await page.locator(`[data-slot="panel-tab-button"][id="${id}"]`).first().click({ timeout: 3000 }).catch(() => {});
    await sleep(300);
  }
  return openPanelTabIds();
};

/** 🙈️ Dismisses a boot tour dialog by its Skip/Close button (en or de), scoped to dialogs only. */
const dismissTour = async () => {
  if (renderer === "wgpu") {
    if (mirrorFind(await mirror(), "ui.introduction.skip") && (await wgpuPress("ui.introduction.skip"))) log("dismissed tour ui.introduction.skip");
    return;
  }
  for (const pattern of COPY[currentLocale].tourButtons) {
    const button = page.locator('[role="dialog"] button', { hasText: pattern }).first();
    if (await button.count().catch(() => 0)) {
      await button.click({ timeout: 2000 }).catch(() => {});
      await sleep(600);
      log(`dismissed tour ${pattern}`);
    }
  }
};

/** 🥾️ Waits for three board windows and three canvases, dismissing a tour on the way. */
const waitForBoot = async (label: string, polls = 100) => {
  for (let index = 0; index < polls; index++) {
    await sleep(3000);
    const shape = renderer === "wgpu" ? await wgpuBootShape() : await evalSafe(() => ({ windows: document.querySelectorAll('[data-slot="window"]').length, canvases: document.querySelectorAll("canvas").length, dialogs: document.querySelectorAll('[role="dialog"]').length, body: document.body?.innerText.slice(0, 160) ?? "" }), { windows: 0, canvases: 0, dialogs: 0, body: "" });
    if (shape.dialogs) await dismissTour();
    if (shape.windows >= 3 && shape.canvases >= 1) {
      log(`${label} booted after ${((index + 1) * 3).toFixed(0)} s: ${JSON.stringify(shape).slice(0, 200)}`);
      await dismissTour();
      return true;
    }
    if (index % 5 === 4) log(`${label} waiting… ${JSON.stringify(shape).slice(0, 200)}`);
  }
  return false;
};

/** 🥾️ The wgpu boot shape: the introspection shim is attached only once the frame worker booted, its structure dump
 * names the live windows, and the ARIA mirror holds nodes once the first frame was projected. */
const wgpuBootShape = async () => {
  const shape = await evalSafe(
    async () => {
      const introspection = (window as unknown as { semioWgpuIntrospection?: { dumpStructure: () => Promise<string> } }).semioWgpuIntrospection;
      let windowIds: string[] = [];
      try {
        windowIds = introspection ? ((JSON.parse((await introspection.dumpStructure()) || "{}") as { windowIds?: string[] }).windowIds ?? []) : [];
      } catch {
        windowIds = [];
      }
      const mirrorRoot = document.getElementById("semio-wgpu-accessibility");
      return { booted: Boolean(introspection), windowIds, mirrorNodes: Number(mirrorRoot?.dataset.nodeCount ?? "0"), canvases: document.querySelectorAll("canvas").length };
    },
    { booted: false, windowIds: [] as string[], mirrorNodes: 0, canvases: 0 },
  );
  const boards = shape.windowIds.filter((id) => id.startsWith("2d-")).length;
  return { windows: boards, canvases: shape.canvases, dialogs: mirrorFind(await mirror(), "ui.introduction.skip") ? 1 : 0, body: JSON.stringify(shape).slice(0, 160) };
};

/** 🧷️ Marks the first element `find` answers with `data-probe-target=<token>` in page, so Playwright can click the
 * exact element a structural search found. */
const markToken = (() => {
  let next = 0;
  return () => `probe-${++next}`;
})();
//#endregion 🔖️Chrome

//#region 🔖️History
type HistoryRow = { id: string; kind: "entry" | "mutation"; key: string; label: string; text: string; expandable: boolean; expanded: boolean; parent: string | null; index: number };

/** 🕰️ Every materialised history row (`framework.history.entry.<seq>` and `framework.history.mutation.<id>`) in reading
 * order; a mutation row belongs to the entry row above it. wgpu answers from the ARIA mirror (label plus description). */
const readHistory = async (): Promise<HistoryRow[]> => {
  if (renderer !== "wgpu") return reactReadHistory();
  let parent: string | null = null;
  return (await mirror())
    .filter((node) => !node.key.includes("::") && /(^|\/)framework\.history\.(entry\.\d+$|mutation\.)/.test(node.key))
    .map((node, index) => {
      const match = node.key.match(/framework\.history\.(entry|mutation)\.(.+)$/)!;
      const kind = match[1] as "entry" | "mutation";
      if (kind === "entry") parent = node.key;
      return { id: node.key, kind, key: match[2], label: node.label.replace(/\s+/g, " ").trim(), text: `${node.label} ${node.description}`.replace(/\s+/g, " ").trim().slice(0, 400), expandable: node.expanded !== null, expanded: node.expanded === "true", parent: kind === "mutation" ? parent : null, index };
    });
};
const reactReadHistory = () =>
  evalSafe(
    () => {
      const rows = Array.from(document.querySelectorAll("[id]")).filter((el) => /(^|\/)framework\.history\.(entry\.\d+$|mutation\.)/.test(el.id));
      let parent: string | null = null;
      return rows.map((el, index) => {
        const match = el.id.match(/framework\.history\.(entry|mutation)\.(.+)$/)!;
        const labelEl = el.querySelector('[data-slot="tree-label"]') as HTMLElement | null;
        const first = labelEl?.querySelector("span") as HTMLElement | null;
        const kind = match[1] as "entry" | "mutation";
        if (kind === "entry") parent = el.id;
        return {
          id: el.id,
          kind,
          key: match[2],
          label: (first?.innerText ?? labelEl?.innerText ?? "").replace(/\s+/g, " ").trim(),
          text: [(el as HTMLElement).innerText, el.getAttribute("title") ?? "", ...Array.from(el.querySelectorAll("[title]")).map((node) => node.getAttribute("title") ?? "")].join(" ").replace(/\s+/g, " ").trim().slice(0, 400),
          expandable: el.hasAttribute("aria-expanded"),
          expanded: el.getAttribute("aria-expanded") === "true",
          parent: kind === "mutation" ? parent : null,
          index,
        };
      });
    },
    [] as HistoryRow[],
  );

/** 📜️ Scrolls the history panel's scroll container to its start or end (the Commands section is windowed). wgpu: a wheel
 * over the panel's registered scroll region. */
const scrollHistory = async (where: "start" | "end") => {
  if (renderer !== "wgpu") return reactScrollHistory(where);
  const region = ((await chromeHits()).hits ?? []).find((hit) => /scroll/i.test(hit.kind) && /history/i.test(`${hit.controlId} ${hit.windowId ?? ""}`));
  if (!region) return false;
  await page.mouse.move(region.rect[0] + region.rect[2] / 2, region.rect[1] + region.rect[3] / 2);
  await page.mouse.wheel(0, where === "end" ? 20000 : -20000);
  return true;
};
const reactScrollHistory = (where: "start" | "end") =>
  evalSafe(
    (edge) => {
      const row = Array.from(document.querySelectorAll("[id]")).find((el) => /framework\.history\.(entry\.\d+|commands)$/.test(el.id));
      let node = row?.parentElement ?? null;
      while (node) {
        const style = getComputedStyle(node);
        if ((style.overflowY === "auto" || style.overflowY === "scroll") && node.scrollHeight > node.clientHeight + 4) {
          node.scrollTop = edge === "end" ? node.scrollHeight : 0;
          return true;
        }
        node = node.parentElement;
      }
      return false;
    },
    false,
    where,
  );

/** 🕰️ Opens the History panel and waits for its body (`framework.history.actions`). */
const openHistory = async () => {
  const opened = await openTab(HISTORY_TAB);
  const body = await waitUntil(() => presentKey("framework.history.actions"), (id) => id !== null, 15000);
  return opened && body.ok;
};

/** 🔎️ All history rows after scrolling the windowed list to the end (newest rows) and to the start, merged by id. */
const allHistoryRows = async () => {
  await openHistory();
  const merged = new Map<string, HistoryRow>();
  for (const edge of ["end", "start", "end"] as const) {
    const scrolled = await scrollHistory(edge);
    if (scrolled) await sleep(700);
    for (const row of await readHistory()) merged.set(row.id, row);
    if (!scrolled) break;
  }
  return [...merged.values()];
};

/** 🧮️ The entry rows that are document or history-edit rows: expandable rows (they carry mutation children) plus
 * rows labelled as a history edit. Chrome rows (panel toggles, tool arming) are neither. */
const documentEntries = (rows: HistoryRow[]) => rows.filter((row) => row.kind === "entry" && (row.expandable || /History edit|Verlauf bearbeitet|Verlaufsbearbeitung/.test(row.label)));

/** 🧬️ The edit ids the document rows carry: each expandable row is expanded and its mutation children's keys
 * (`<editId>#<op>`) read — the label-independent identity of what history holds. */
const documentEditIds = async (rows: HistoryRow[]) => {
  const ids = new Set<string>();
  for (const entry of documentEntries(rows).filter((row) => row.expandable).slice(0, 24)) {
    for (const mutation of (await expandEntry(entry.id)).mutations) ids.add(mutation.key.replace(/#\d+$/, ""));
  }
  return [...ids].sort();
};

/** 🔢️ The newest entry sequence among `rows` (entry rows are keyed by their command-log seq). The Commands list is
 * windowed, so "new rows" are the ones NEWER than this, never the ones missing from an earlier read. */
const newestEntrySeq = (rows: HistoryRow[]) => Math.max(-1, ...rows.filter((row) => row.kind === "entry").map((row) => Number(row.key)).filter(Number.isFinite));

/** 🌳️ Expands one entry row through its disclosure button (wgpu: its activation, else focus + ArrowRight) and waits for
 * its mutation children. */
const expandEntry = async (entryId: string) => {
  if (renderer === "wgpu") {
    const row = (await mirror()).find((node) => node.key === entryId);
    let state = !row ? "absent" : row.expanded === "true" ? "open" : "activate";
    if (state === "activate") {
      await mirrorActivate(entryId);
      if (!(await waitUntil(readHistory, (rows) => rows.some((entry) => entry.parent === entryId), 4000)).ok) {
        state = "arrow";
        await mirrorFocus(entryId);
        await page.keyboard.press("ArrowRight").catch(() => {});
      }
    }
    const children = await waitUntil(readHistory, (rows) => rows.some((entry) => entry.parent === entryId), 10000);
    return { state, mutations: children.value.filter((entry) => entry.parent === entryId) };
  }
  const state = await evalSafe(
    (id) => {
      const row = document.getElementById(id);
      if (!row) return "absent";
      if (row.getAttribute("aria-expanded") === "true") return "open";
      const chevron = row.querySelector("button[aria-expanded]") as HTMLButtonElement | null;
      if (chevron) chevron.click();
      else (row as HTMLElement).click();
      return chevron ? "chevron" : "row";
    },
    "absent",
    entryId,
  );
  const children = await waitUntil(readHistory, (rows) => rows.some((row) => row.parent === entryId), 10000);
  return { state, mutations: children.value.filter((row) => row.parent === entryId) };
};

/** 🖱️ Presses the row action of `rowId` whose accessible name matches `name` (hovering first, the action strip is a
 * reveal region); falls back to the row's own label, which is the row's activation. */
const pressRowAction = async (rowId: string, name: RegExp) => {
  if (renderer === "wgpu") {
    const action = (await mirror()).find((node) => node.key.startsWith(`${rowId}::row-action::`) && name.test(node.label.trim()));
    if (action) return (await mirrorActivate(action.key, action.window)) ? "action" : "absent";
    return (await mirrorActivate(rowId)) ? "label" : "absent";
  }
  const token = markToken();
  await byId(rowId).hover({ timeout: 3000 }).catch(() => {});
  const via = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      if (!row) return "absent";
      const pattern = new RegExp(arg.source, arg.flags);
      const candidates = Array.from(row.querySelectorAll('button, [role="button"], [data-slot="action"]'));
      const hit = candidates.find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? (el as HTMLElement).innerText ?? "").trim()));
      if (hit) {
        hit.setAttribute("data-probe-target", arg.token);
        return "action";
      }
      const label = row.querySelector('[data-slot="tree-label"]');
      (label ?? row).setAttribute("data-probe-target", arg.token);
      return "label";
    },
    "absent",
    { rowId, source: name.source, flags: name.flags, token },
  );
  if (via !== "absent") await page.locator(`[data-probe-target="${token}"]`).first().click({ force: true, timeout: 4000 }).catch((error) => log(`pressRowAction ${rowId} click failed ${String(error).split("\n")[0]}`));
  return via;
};

/** 🔘️ Clicks a panel-body button by its authored id (`framework.history.editor.withdraw`, …). */
const pressAuthored = async (authored: string) => {
  const section = authored.match(/framework\.history\.(editor|timeTravel)/)?.[0];
  if (section) {
    await revealHistory(section);
    await revealHistory(`${authored}.row`);
  }
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = mirrorFind(nodes, authored) ?? mirrorFind(nodes, `${authored}.row`);
    if (!node) return { present: false, disabled: null as boolean | null, id: null as string | null };
    await mirrorActivate(node.key, node.window);
    return { present: true, disabled: node.disabled as boolean | null, id: node.key as string | null };
  }
  const id = (await resolveDomId(authored)) ?? (await resolveDomId(`${authored}.row`));
  if (!id) return { present: false, disabled: null as boolean | null, id: null as string | null };
  const disabled = await evalSafe((target) => {
    const el = document.getElementById(target);
    const button = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
    if (button) return button.disabled;
    const flag = el?.getAttribute("aria-disabled") ?? el?.getAttribute("data-disabled");
    return flag === null || flag === undefined ? null : flag === "true" || flag === "";
  }, null as boolean | null, id);
  const target = page.locator(`[id="${id}"]`).first();
  const tag = await target.evaluate((el) => el.tagName.toLowerCase()).catch(() => "");
  const inner = tag === "button" ? 1 : await target.locator("button").count().catch(() => 0);
  await (tag === "button" ? target : inner ? target.locator("button").first() : target.locator('[data-slot="tree-label"]').first()).click({ force: true, timeout: 4000 }).catch(() => target.click({ force: true, timeout: 4000 }).catch(() => {}));
  return { present: true, disabled, id };
};

/** 🧾️ The history-panel inventory for diagnosis: every id under `framework.history` (wgpu: every mirrored history, band,
 * dialog and indicator node), with role and state. */
const historyInventory = async (): Promise<Record<string, unknown>[]> =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /framework\.history|shell\.time-travel|shell\.dialog|ui\.timeTravel|timeTravel\.indicator|framework\.sync|s-presence-peers|shell\.panel\.tab/.test(node.key)).slice(0, 400).map((node) => ({ id: node.key, window: node.window, role: node.role, expanded: node.expanded, disabled: node.disabled, aria: node.label, title: node.description, value: node.value ?? node.valueNow, text: node.label }))
    : reactHistoryInventory();
const reactHistoryInventory = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /framework\.history|framework\.panel\.history|ui\.dialog/.test(el.id))
        .slice(0, 400)
        .map((el) => ({
          id: el.id,
          tag: el.tagName.toLowerCase(),
          slot: el.getAttribute("data-slot"),
          role: el.getAttribute("role"),
          expanded: el.getAttribute("aria-expanded"),
          disabled: (el as HTMLButtonElement).disabled ?? null,
          aria: el.getAttribute("aria-label"),
          title: el.getAttribute("title"),
          value: (el as HTMLInputElement).value ?? null,
          text: (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 140) ?? "",
        })),
    [] as Record<string, unknown>[],
  );
//#endregion 🔖️History

//#region 🔖️Band
type BandControl = { control: string; disabled: boolean; title: string | null; keys: string | null; text: string };
type Band = { stage: string; role: string | null; live: string | null; generation: string | null; text: string; review: string | null; outcome: string | null; fault: string | null; target: string | null; progress: string | null; controls: BandControl[] };

/** 📣️ The time-travel band, or null when no session is live: React's `[data-semio-time-travel]`, wgpu's announced
 * `shell.time-travel.status` with its mirrored controls. */
const band = (): Promise<Band | null> => (renderer === "wgpu" ? wgpuBand() : reactBand());
const reactBand = () =>
  evalSafe(
    () => {
      const el = document.querySelector("[data-semio-time-travel]");
      if (!el) return null;
      const attr = (selector: string, name: string) => el.querySelector(selector)?.getAttribute(name) ?? null;
      const progress = el.querySelector("progress");
      return {
        stage: el.getAttribute("data-semio-time-travel") ?? "",
        role: el.getAttribute("role"),
        live: el.getAttribute("aria-live"),
        generation: el.getAttribute("data-time-travel-generation"),
        text: (el as HTMLElement).innerText.replace(/\s+/g, " ").trim(),
        review: attr("[data-semio-time-travel-review]", "data-semio-time-travel-review"),
        outcome: attr("[data-semio-time-travel-outcome]", "data-semio-time-travel-outcome"),
        fault: attr("[data-semio-time-travel-fault]", "data-semio-time-travel-fault"),
        target: (el.querySelector("[data-semio-time-travel-target]") as HTMLElement | null)?.innerText ?? null,
        progress: progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : null,
        controls: Array.from(el.querySelectorAll("[data-semio-time-travel-control]")).map((button) => ({
          control: button.getAttribute("data-semio-time-travel-control") ?? "",
          disabled: (button as HTMLButtonElement).disabled,
          title: button.getAttribute("title"),
          keys: button.getAttribute("aria-keyshortcuts"),
          text: (button as HTMLElement).innerText.trim(),
        })),
      };
    },
    null as Band | null,
  );

/** 🎞️ Installs an in-page MutationObserver that records every distinct band state (stage | review | progress) with
 * its time, so a stage React rendered for one frame is still seen. Re-install after every reload. */
const installBandTrace = () =>
  renderer === "wgpu"
    ? evalSafe(
        (words) => {
          const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver };
          host.__probeBandTrace = [];
          host.__probeBandObserver?.disconnect();
          let last = "";
          const read = () => {
            const el = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
            const text = el?.getAttribute("aria-label") ?? "";
            const stage = words.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const review = words.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const progress = el?.getAttribute("aria-valuenow") !== null && el?.getAttribute("aria-valuenow") !== undefined ? `${el?.getAttribute("aria-valuenow")}/${el?.getAttribute("aria-valuemax")}` : "";
            const key = el ? `${stage}|${review}|${progress}` : "none";
            if (key === last) return;
            last = key;
            host.__probeBandTrace!.push({ t: Math.round(performance.now()), key });
            if (host.__probeBandTrace!.length > 400) host.__probeBandTrace!.shift();
          };
          const root = document.getElementById("semio-wgpu-accessibility") ?? document.body;
          host.__probeBandObserver = new MutationObserver(read);
          host.__probeBandObserver.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-label", "aria-valuenow", "aria-valuemax"] });
          read();
          return true;
        },
        false,
        { stages: BAND_WORDS.map(([stage, pattern]) => [stage, pattern.source] as [string, string]), reviews: REVIEW_WORDS.map(([review, pattern]) => [review, pattern.source] as [string, string]) },
      )
    : reactInstallBandTrace();
const reactInstallBandTrace = () =>
  evalSafe(() => {
    const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver };
    host.__probeBandTrace = [];
    host.__probeBandObserver?.disconnect();
    let last = "";
    const read = () => {
      const el = document.querySelector("[data-semio-time-travel]");
      const progress = el?.querySelector("progress");
      const key = el ? `${el.getAttribute("data-semio-time-travel")}|${el.querySelector("[data-semio-time-travel-review]")?.getAttribute("data-semio-time-travel-review") ?? ""}|${progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : ""}` : "none";
      if (key === last) return;
      last = key;
      host.__probeBandTrace!.push({ t: Math.round(performance.now()), key });
      if (host.__probeBandTrace!.length > 400) host.__probeBandTrace!.shift();
    };
    host.__probeBandObserver = new MutationObserver(read);
    host.__probeBandObserver.observe(document.body, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-semio-time-travel", "data-semio-time-travel-review", "value", "max"] });
    read();
    return true;
  }, false);
/** 📣️ Records every transient notice code React shows (`[data-notice-code]`) for the page's life; re-install after a reload. */
const installNoticeTrace = () =>
  renderer === "wgpu"
    ? Promise.resolve(false)
    : evalSafe(() => {
        const host = window as unknown as { __probeNoticeCodes?: string[]; __probeNoticeObserver?: MutationObserver };
        host.__probeNoticeCodes ??= [];
        host.__probeNoticeObserver?.disconnect();
        const read = () => {
          for (const el of Array.from(document.querySelectorAll("[data-notice-code]"))) {
            const code = el.getAttribute("data-notice-code") ?? "";
            if (code && !host.__probeNoticeCodes!.includes(code)) host.__probeNoticeCodes!.push(code);
          }
        };
        host.__probeNoticeObserver = new MutationObserver(read);
        host.__probeNoticeObserver.observe(document.body, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-notice-code"] });
        read();
        return true;
      }, false);
const noticeCodes = () => evalSafe(() => ((window as unknown as { __probeNoticeCodes?: string[] }).__probeNoticeCodes ?? []).slice(), [] as string[]);
const bandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: { t: number; key: string }[] }).__probeBandTrace ?? []).slice(), [] as { t: number; key: string }[]);
const clearBandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: unknown[] }).__probeBandTrace = []).length, 0);

/** ⌨️ Puts keyboard focus where the shell admits a chord: React listens on the document, so the active field is blurred;
 * the wgpu wire admits keys only from its canvas or a mirror element, so the largest focusable canvas takes focus. */
const prepareChord = async () => {
  if (renderer !== "wgpu") {
    await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
    return "blurred";
  }
  return evalSafe(() => {
    const canvases = Array.from(document.querySelectorAll("canvas")).sort((left, right) => right.width * right.height - left.width * left.height);
    for (const canvas of canvases) {
      canvas.focus();
      if (document.activeElement === canvas) return "canvas";
    }
    const status = document.querySelector<HTMLElement>('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
    status?.focus();
    return document.activeElement === status ? "status" : "none";
  }, "none");
};

/** 🔘️ Presses one band control by its button, or by its chord in a chord locale (`accept`, `discard`, `exit`). */
const pressBand = async (control: string) => {
  const chords: Record<string, string> = { accept: "Alt+Enter", discard: "Alt+Backspace", exit: "Alt+Shift+Backspace" };
  if (chordLocales.has(currentLocale) && chords[control]) {
    await prepareChord();
    await page.keyboard.press(chords[control]).catch(() => {});
    return `chord ${chords[control]}`;
  }
  if (renderer === "wgpu") {
    const id = WGPU_BAND_CONTROLS.find(([name]) => name === control)?.[1];
    return (id && (await wgpuPress(id))) ?? "absent";
  }
  await page.locator(`[data-semio-time-travel-control="${control}"]`).first().click({ timeout: 4000 }).catch((error) => log(`band ${control} click failed ${String(error).split("\n")[0]}`));
  return "button";
};

type DialogButton = { text: string; destructive: string | null; tone: string | null; disabled: boolean | null; description: string | null };
type FinalizeDialog = { title: string; text: string; overwrite: DialogButton | null; submit: DialogButton | null; cancel: { text: string } | null; name: { id: string; value: string } | null };

/** 🪟️ The wgpu finalize dialog through the mirror: `shell.dialog.<id>` (title) with `.choice.overwrite`, `.confirm` (New
 * alternative), `.cancel` and `.field.name`; the chrome projects only the dialog's nodes while it is open. */
const wgpuFinalizeDialog = async (): Promise<FinalizeDialog | null> => {
  const nodes = await mirror();
  const root = nodes.find((node) => /^shell\.dialog\.[^.]+$/.test(node.key) && nodes.some((other) => other.key === `${node.key}.confirm`));
  if (!root) return null;
  const button = (suffix: string): DialogButton | null => {
    const node = nodes.find((other) => other.key === `${root.key}.${suffix}`);
    return node ? { text: node.label, destructive: null, tone: null, disabled: node.disabled, description: node.description || null } : null;
  };
  const name = nodes.find((node) => node.key === `${root.key}.field.name`);
  return { title: root.label, text: nodes.filter((node) => node.key.startsWith(root.key)).map((node) => node.label).join(" ").slice(0, 500), overwrite: button("choice.overwrite"), submit: button("confirm"), cancel: button("cancel"), name: name ? { id: name.key, value: name.value ?? name.label } : null };
};

/** ✍️ Types the alternative name into the open finalize dialog; answers what the field then holds. */
const dialogFillName = async (value: string) => {
  if (renderer === "wgpu") {
    const dialog = await wgpuFinalizeDialog();
    const field = dialog?.name ? await mirrorLocator(dialog.name.id) : null;
    if (!field) return null;
    await field.fill(value, { force: true, timeout: 4000 }).catch(() => {});
    return field.inputValue({ timeout: 2000 }).catch(() => null);
  }
  const field = page.locator('[role="dialog"] input[id="name"], [role="dialog"] input[type="text"]').first();
  await field.fill(value, { timeout: 4000 }).catch(() => {});
  return field.inputValue({ timeout: 2000 }).catch(() => null);
};

/** 🔀️ Presses the finalize dialog's destructive Overwrite choice or its New alternative submit. */
const dialogChoose = async (choice: "overwrite" | "submit") => {
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const root = nodes.find((node) => /^shell\.dialog\.[^.]+$/.test(node.key) && nodes.some((other) => other.key === `${node.key}.confirm`));
    return root ? mirrorActivate(`${root.key}.${choice === "overwrite" ? "choice.overwrite" : "confirm"}`) : false;
  }
  return page.locator(`[id="${choice === "overwrite" ? "ui.dialog.choice.overwrite" : "ui.dialog.submit"}"]`).first().click({ timeout: 4000 }).then(() => true).catch((error) => {
    log(`dialog ${choice} click failed ${String(error).split("\n")[0]}`);
    return false;
  });
};

/** 🪟️ The finalize dialog (`finalizeHistoryEdit`): title, the destructive Overwrite choice, the name field, submit. */
const finalizeDialog = (): Promise<FinalizeDialog | null> => (renderer === "wgpu" ? wgpuFinalizeDialog() : reactFinalizeDialog());
const reactFinalizeDialog = () =>
  evalSafe(
    () => {
      const dialog = Array.from(document.querySelectorAll('[role="dialog"]')).find((el) => el.querySelector('[id="ui.dialog.submit"]')) as HTMLElement | undefined;
      if (!dialog) return null;
      const button = (id: string) => {
        const el = dialog.querySelector(`[id="${id}"]`);
        const control = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
        const describedBy = (control ?? el)?.getAttribute("aria-describedby");
        return el ? { text: (el as HTMLElement).innerText.trim(), destructive: el.getAttribute("data-destructive") ?? control?.getAttribute("data-destructive") ?? null, tone: el.getAttribute("data-tone") ?? control?.getAttribute("data-tone") ?? null, disabled: control?.disabled ?? null, description: describedBy ? (document.getElementById(describedBy)?.textContent ?? null) : null } : null;
      };
      const name = (dialog.querySelector('input[id="name"]') ?? dialog.querySelector('input[type="text"]')) as HTMLInputElement | null;
      return {
        title: (dialog.querySelector("h2") as HTMLElement | null)?.innerText.trim() ?? dialog.innerText.split("\n")[0],
        text: dialog.innerText.replace(/\s+/g, " ").slice(0, 500),
        overwrite: button("ui.dialog.choice.overwrite"),
        submit: button("ui.dialog.submit"),
        cancel: button("ui.dialog.cancel"),
        name: name ? { id: name.id, value: name.value } : null,
      };
    },
    null as FinalizeDialog | null,
  );

type Editor = { heading: string | null; status: string | null; dx: StepperRead | null; dy: StepperRead | null; targets: { id: string; chips: string[]; useSelection: boolean; text: string } | null; inputs: string[] };
type StepperRead = { id: string; tag: string; stepper: boolean; value: string | null; step: string | null; min: string | null; max: string | null; plus: boolean; minus: boolean; role: string | null };

/** ✏️ The draft editor section: its heading, the Rust band status line, the dx/dy controls and the targets list. */
const editor = (): Promise<Editor | null> => (renderer === "wgpu" ? wgpuEditor() : reactEditor());

/** ✏️ The draft editor through the wgpu mirror; a number input is a `spinbutton` (or `slider`) whose value the mirror
 * carries as the input value / `aria-valuenow`. */
const wgpuEditor = async (): Promise<Editor | null> => {
  const nodes = await mirror();
  const number = (pointer: string): StepperRead | null => {
    const node = mirrorFind(nodes, `framework.history.editor.input.${pointer}`);
    if (!node) return null;
    const steppers = nodes.filter((other) => other.key.startsWith(`${node.key}::`) || other.key.startsWith(`${node.key}.`));
    return { id: node.key, tag: node.tag, stepper: node.role === "spinbutton", value: node.value ?? node.valueNow, step: null, min: null, max: null, plus: steppers.some((other) => /increase|erhöhen|\+/i.test(other.label)), minus: steppers.some((other) => /decrease|verringern|−|-/i.test(other.label)), role: node.role };
  };
  const targets = mirrorFind(nodes, "framework.history.editor.input.targets");
  const chips = nodes.filter((node) => /framework\.history\.editor\.input\.targets\.chip\.\d+$/.test(node.key)).map((node) => node.label);
  return {
    heading: mirrorFind(nodes, "framework.history.editor.target")?.label ?? null,
    status: mirrorFind(nodes, "framework.history.timeTravel.status")?.label ?? null,
    dx: number("dx"),
    dy: number("dy"),
    targets: targets ? { id: targets.key, chips, useSelection: nodes.some((node) => /use selection|auswahl verwenden/i.test(node.label) && node.key.includes("framework.history.editor")), text: targets.label } : null,
    inputs: nodes.filter((node) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(node.key)).map((node) => `${node.key.replace(/^.*framework\.history\.editor\.input/, "")}=${node.label.slice(0, 60)}`),
  };
};
const reactEditor = () =>
  evalSafe(
    () => {
      const find = (authored: string) => Array.from(document.querySelectorAll("[id]")).find((el) => el.id === authored || el.id.endsWith(`/${authored}`)) as HTMLElement | undefined;
      const number = (pointer: string) => {
        const el = find(`framework.history.editor.input.${pointer}`);
        if (!el) return null;
        const input = (el.tagName === "INPUT" ? el : el.querySelector("input")) as HTMLInputElement | null;
        const group = el.closest('[data-slot="stepper-group"]') ?? el.querySelector('[data-slot="stepper-group"]');
        return {
          id: el.id,
          tag: el.tagName.toLowerCase(),
          stepper: input?.getAttribute("data-stepper-input") === "true",
          value: input?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? null,
          step: input?.getAttribute("step") ?? null,
          min: input?.getAttribute("min") ?? null,
          max: input?.getAttribute("max") ?? null,
          plus: Boolean(group?.querySelector('[data-slot="stepper-plus"]')),
          minus: Boolean(group?.querySelector('[data-slot="stepper-minus"]')),
          role: el.getAttribute("role") ?? el.querySelector('[role="slider"]')?.getAttribute("role") ?? null,
        };
      };
      const targetsEl = find("framework.history.editor.input.targets");
      const chips = Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.targets\.chip\.\d+$/.test(el.id)).map((el) => (el as HTMLElement).innerText.replace(/\s+/g, " ").trim());
      return {
        heading: find("framework.history.editor.target")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        status: find("framework.history.timeTravel.status")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        dx: number("dx"),
        dy: number("dy"),
        targets: targetsEl ? { id: targetsEl.id, chips, useSelection: Array.from(targetsEl.querySelectorAll("button")).some((b) => /use selection|auswahl verwenden/i.test(`${b.innerText} ${b.getAttribute("aria-label") ?? ""}`)), text: targetsEl.innerText.replace(/\s+/g, " ").trim().slice(0, 200) } : null,
        inputs: Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(el.id)).map((el) => `${el.id.replace(/^.*framework\.history\.editor\.input/, "")}=${(el as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)}`),
      };
    },
    null as Editor | null,
  );

/** 🔭️ Scrolls the history body so the section (or row) `authored` names sits at the top of the panel — what a user does to
 * reach it; the body is windowed, so rows below the viewport are not in the document until they scroll in. */
const revealHistory = async (authored: string) => {
  if (renderer === "wgpu") {
    let node = mirrorFind(await mirror(), authored);
    if (!node && (await scrollHistory("end"))) {
      await sleep(700);
      node = mirrorFind(await mirror(), authored);
    }
    if (node) await mirrorFocus(node.key);
    await sleep(node ? 500 : 0);
    return Boolean(node);
  }
  const found = await evalSafe(
    (target) => {
      const el = Array.from(document.querySelectorAll("[id]")).find((node) => node.id === target || node.id.endsWith(`/${target}`)) as HTMLElement | undefined;
      el?.scrollIntoView({ block: "start" });
      return Boolean(el);
    },
    false,
    authored,
  );
  await sleep(found ? 500 : 0);
  return found;
};

/** ✏️ {@link editor} after revealing the editor's inputs section (`framework.history.editor.inputs`). */
const readEditor = async () => {
  await openHistory();
  if (!(await revealHistory("framework.history.editor.inputs"))) await revealHistory("framework.history.editor");
  return editor();
};

/** ⌨️ Types `value` into the editor's number control at `pointer` (one `fill` = one input event = one draft), then
 * Enter to commit and blur; answers the value the field held before Enter. */
const typeEditorNumber = async (pointer: string, value: number) => {
  await revealHistory("framework.history.editor.inputs");
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null };
    await field.fill(String(value), { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await mirrorFocus(node.key);
    await page.keyboard.press("Enter").catch(() => {});
    return { present: true, typed };
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return { present: false, typed: null as string | null };
  const host = page.locator(`[id="${id}"]`).first();
  const input = (await host.evaluate((el) => el.tagName === "INPUT").catch(() => false)) ? host : host.locator("input").first();
  await input.click({ timeout: 4000 }).catch(() => {});
  await input.fill(String(value), { timeout: 4000 }).catch(() => {});
  const typed = await input.inputValue({ timeout: 2000 }).catch(() => null);
  await input.press("Enter").catch(() => {});
  return { present: true, typed };
};

/** ⌨️ Presses one key in the editor's number control at `pointer` (ArrowUp/ArrowDown step by the control's step). */
const keyEditorNumber = async (pointer: string, key: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    if (!node || !(await mirrorFocus(node.key))) return false;
    await page.keyboard.press(key).catch(() => {});
    return true;
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return false;
  const host = page.locator(`[id="${id}"]`).first();
  const input = (await host.evaluate((el) => el.tagName === "INPUT").catch(() => false)) ? host : host.locator("input").first();
  await input.focus().catch(() => {});
  await input.press(key).catch(() => {});
  return true;
};
//#endregion 🔖️Band

//#region 🔖️Steps
type Ctx = {
  a?: string;
  b?: string;
  c?: string;
  p0?: Positions;
  p1?: Positions;
  pc?: Positions;
  dx?: number;
  dy?: number;
  cOffset?: [number, number];
  dragEntry?: string;
  dragMutation?: string;
  cMutation?: string;
  afterOverwrite?: Positions;
  dyEdited?: number;
  alternative?: string;
  folder?: string;
};

/** 🧭️ Opens the drag's entry row and presses Edit on its `drag-selection` mutation; answers the band once `editing`. */
const beginDragEdit = async (ctx: Ctx) => {
  await openHistory();
  const rows = await allHistoryRows();
  const prefix = COPY[currentLocale].drag(2, "", "").split("(")[0];
  const candidates = [...rows.filter((row) => row.id === ctx.dragEntry), ...documentEntries(rows).filter((row) => row.id !== ctx.dragEntry && row.label.startsWith(prefix))];
  let entry: HistoryRow | undefined;
  let mutation: HistoryRow | undefined;
  for (const candidate of candidates.slice(0, 6)) {
    const expanded = await expandEntry(candidate.id);
    mutation = expanded.mutations.find((row) => row.key === ctx.dragMutation);
    if (mutation) {
      entry = candidate;
      break;
    }
  }
  if (!entry || !mutation) return { entry: null, via: "absent", band: null as Band | null, rows, candidates: candidates.map((row) => `${row.id}=${row.label.slice(0, 40)}`) };
  if (entry.id !== ctx.dragEntry) log(`drag row moved ${ctx.dragEntry} → ${entry.id}`);
  const via = await pressRowAction(mutation.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  return { entry, mutation, via, band: settled.value, waitedMs: settled.waitedMs, rows };
};

/** 🥾️ Step 1 — the board is parsed and populated, and the history panel speaks the locale under test. */
const step1 = async (ctx: Ctx) => {
  const parsed = await waitUntil(vitals, (v) => v?.parsed === "true", 60000);
  verdict("fixture-parsed", parsed.ok, { parsed: parsed.value?.parsed, waitedMs: parsed.waitedMs });
  const populated = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
  const p = await positions();
  verdict("board-has-nodes", (populated.value?.nodes ?? 0) > 0 && Object.keys(p).length > 0, { nodes: populated.value?.nodes, edges: populated.value?.edges, positions: Object.keys(p).length, camera: populated.value?.camera });
  const opened = await openHistory();
  const commands = await presentKey("framework.history.commands");
  const commandsText = await textOfKey("framework.history.commands");
  verdict("history-panel-speaks-the-locale", opened && commandsText.toLowerCase().includes(COPY[currentLocale].commands.toLowerCase()), { opened, commands, commandsText: commandsText.slice(0, 80), expected: COPY[currentLocale].commands, lang: await evalSafe(() => navigator.language, "") });
  emit({ kind: "inventory", tag: "boot-history", rows: await historyInventory() });
  await shot("boot");
  await closePanels();
  if (folderAt === 1) {
    ctx.folder = join(OUT, `folder-${stamp}-${currentLocale}`);
    mkdirSync(ctx.folder, { recursive: true });
    const before = await positions();
    const attached = await attachFolder(ctx.folder);
    const after = await positions();
    verdict("local-folder-attach-keeps-the-document", attached.typed === ctx.folder && attached.attachButton && movedIds(before, after, 1e-6).length === 0, { attached, drift: movedIds(before, after, 1e-6).length, reading: "the document is bound to a local folder (sync card → Folder → Attach, `persistedLocalOnly`) before any edit, so every later edit is persisted; step 5's reload check re-attaches it" });
    await closePanels();
  }
};

/** ✋️ Step 2 — select two nodes, drag them by (+80,+40) world units, one new row; then a downstream drag of a third node. */
const step2 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const { camera, picked } = await pickNodes(3, [[80, 40], [-60, 30]], [], 50);
  if (picked.length < 3) {
    verdict("three-clickable-nodes", false, { picked: picked.map((row) => row.id), camera });
    return;
  }
  const [a, b, c] = picked;
  ctx.a = a.id;
  ctx.b = b.id;
  ctx.c = c.id;
  log(`picked a=${a.id}@${JSON.stringify(a.at)} b=${b.id}@${JSON.stringify(b.at)} c=${c.id}@${JSON.stringify(c.at)} zoom=${camera.zoom}`);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await page.mouse.click(a.at.x, a.at.y);
  const first = await waitUntil(vitals, (v) => selectionIds(v).includes(a.id), 15000);
  await page.keyboard.down("Shift");
  await page.mouse.click(b.at.x, b.at.y);
  await page.keyboard.up("Shift");
  const both = await waitUntil(vitals, (v) => selectionIds(v).includes(a.id) && selectionIds(v).includes(b.id), 15000);
  verdict("two-nodes-selected", first.ok && both.ok && selectionIds(both.value).length === 2, { selection: selectionIds(both.value), transform: both.value?.transform?.slice(0, 200) });
  ctx.p0 = await positions();
  const zoom = cameraOf(await vitals()).zoom;
  await dragBy(a.at, 80 * zoom, 40 * zoom);
  const moved = await waitUntil(positions, (p) => offsetOf(ctx.p0!, p, a.id)?.some((v) => Math.abs(v) > 0.5) === true && offsetOf(ctx.p0!, p, b.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  ctx.p1 = moved.value;
  const offA = offsetOf(ctx.p0, ctx.p1, a.id);
  const offB = offsetOf(ctx.p0, ctx.p1, b.id);
  ctx.dx = offA?.[0] ?? 0;
  ctx.dy = offA?.[1] ?? 0;
  const others = movedIds(ctx.p0, ctx.p1).filter((id) => id !== a.id && id !== b.id);
  verdict("both-nodes-moved-by-one-common-offset", moved.ok && Boolean(offA && offB && near(offA[0], offB[0], 1e-6) && near(offA[1], offB[1], 1e-6)), { offA, offB, zoom, waitedMs: moved.waitedMs });
  verdict("offset-is-80-40-world-units", Boolean(offA && near(offA[0], 80, 0.005) && near(offA[1], 40, 0.005)), { offA, screenDelta: [80 * zoom, 40 * zoom], note: "screen delta = world offset × zoom; a pixel-quantised pointer makes this inexact" });
  verdict("no-other-node-moved", others.length === 0, { others: others.slice(0, 8) });
  const expected = dragLabel(2, ctx.dx, ctx.dy);
  const grew = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 1, 30000, 1000);
  const added = grew.value;
  verdict("exactly-one-new-history-row", added.length === 1, { added: added.map((row) => `${row.id}=${row.label}`), waitedMs: grew.waitedMs });
  const row = added.find((entry) => entry.label.startsWith(expected) || entry.text.includes(expected)) ?? added[0];
  verdict("row-labelled-from-the-drag-mutation", Boolean(row && (row.label === expected || row.label.startsWith(expected))), { expected, label: row?.label, text: row?.text?.slice(0, 160) });
  if (row) {
    ctx.dragEntry = row.id;
    const expanded = await expandEntry(row.id);
    const mutation = expanded.mutations.find((entry) => entry.label.startsWith(expected)) ?? expanded.mutations[0];
    ctx.dragMutation = mutation?.key;
    verdict("row-expands-to-its-drag-selection-mutation", Boolean(mutation && mutation.label.startsWith(expected)), { state: expanded.state, mutations: expanded.mutations.map((entry) => `${entry.key}=${entry.label}`).slice(0, 8) });
  }
  await shot("dragged");
  await closePanels();
  const beforeC = await positions();
  const cAt = toScreen(beforeC[c.id], cameraOf(await vitals()), await paneBox());
  await dragBy(cAt, -60 * zoom, 30 * zoom);
  const cMoved = await waitUntil(positions, (p) => offsetOf(beforeC, p, c.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  ctx.pc = cMoved.value;
  ctx.cOffset = offsetOf(beforeC, ctx.pc, c.id) ?? [0, 0];
  const cOthers = movedIds(beforeC, ctx.pc).filter((id) => id !== c.id);
  verdict("downstream-drag-moves-only-c", cMoved.ok && cOthers.length === 0, { cOffset: ctx.cOffset, others: cOthers.slice(0, 6) });
  const cRows = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((entry) => Number(entry.key) > newestBefore && entry.id !== ctx.dragEntry), (rows) => rows.length >= 1, 30000, 1000);
  const cRow = cRows.value[0];
  if (cRow) {
    const expanded = await expandEntry(cRow.id);
    ctx.cMutation = expanded.mutations[0]?.key;
    verdict("downstream-drag-is-its-own-row", cRows.value.length === 1 && cRow.label.startsWith(dragLabel(1, ctx.cOffset[0], ctx.cOffset[1])), { label: cRow.label, expected: dragLabel(1, ctx.cOffset[0], ctx.cOffset[1]), mutation: ctx.cMutation });
  } else verdict("downstream-drag-is-its-own-row", false, { rows: cRows.value.length });
  await closePanels();
};

/** ✏️ Step 3 — Edit the drag: band `editing`, preview = state before + draft, downstream pending, dx/dy steppers and targets. */
const step3 = async (ctx: Ctx) => {
  if (!ctx.dragEntry || !ctx.dragMutation || !ctx.p0 || !ctx.a || !ctx.b || !ctx.c || !ctx.pc) {
    verdict("precondition-step-2", false, { ctx: Object.keys(ctx) });
    return;
  }
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  const b = begun.band;
  verdict("edit-opens-the-band-in-editing", b?.stage === "editing", { via: begun.via, stage: b?.stage, waitedMs: begun.waitedMs, text: b?.text?.slice(0, 200) });
  verdict("band-is-a-polite-status-region", b?.role === "status" && b?.live === "polite", { role: b?.role, live: b?.live });
  verdict("band-names-stage-and-target", Boolean(b && b.text.includes(COPY[currentLocale].stageEditing) && (b.target ?? "").includes(dragLabel(2, ctx.dx!, ctx.dy!))), { text: b?.text?.slice(0, 200), target: b?.target });
  verdict("band-offers-accept-discard-exit", JSON.stringify(b?.controls.map((c) => c.control)) === JSON.stringify(["accept", "discard", "exit"]), { controls: b?.controls });
  const indicator = await indicators();
  verdict("windows-wear-the-time-travel-indicator", indicator.length > 0, { indicator: indicator.slice(0, 4) });
  const roster = await presenceText();
  verdict("presence-roster-shows-no-editing-peer", roster === null || !/⏪|is editing|bearbeitet .* in der Zeitreise/.test(roster), { roster, reading: "one tab, no peer: the ⏪ badge and the \"is editing\" notes (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers`) must stay absent; a live second peer needs a presence transport (hub), which this serve does not run" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], ctx.dx!, ctx.dy!) && placed(p, ctx.c!, ctx.p0![ctx.c!], 0, 0), 15000);
  verdict("preview-is-state-before-target-plus-draft", placed(preview.value, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(preview.value, ctx.b, ctx.p0[ctx.b], ctx.dx!, ctx.dy!), {
    a: preview.value[ctx.a],
    aBefore: ctx.p0[ctx.a],
    draft: [ctx.dx, ctx.dy],
    design: "§4: the preview is the document as of the edited mutation with the DRAFT applied (Begin drafts the original input), so the dragged nodes read pre-drag + draft",
  });
  verdict("downstream-not-applied-while-editing", placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { c: preview.value[ctx.c], cBeforeItsDrag: ctx.p0[ctx.c], cHead: ctx.pc[ctx.c], waitedMs: preview.waitedMs });
  const pendingRows = await waitUntil(readHistory, (rows) => rows.some((row) => row.kind === "mutation" && row.key === ctx.cMutation && row.text.includes(COPY[currentLocale].pending)), 10000);
  const cRow = pendingRows.value.find((row) => row.kind === "mutation" && row.key === ctx.cMutation);
  verdict("downstream-row-reads-not-applied", Boolean(cRow?.text.includes(COPY[currentLocale].pending)), { cRow: cRow?.text?.slice(0, 160), expected: COPY[currentLocale].pending });
  const e = await waitUntil(readEditor, (value) => Boolean(value?.dx && value?.dy && value?.targets), 15000);
  const ed = e.value;
  const steppers = renderer === "wgpu" ? Boolean(ed?.dx?.stepper && ed?.dy?.stepper) : Boolean(ed?.dx?.stepper && ed?.dy?.stepper && ed.dx.step === "1" && ed.dy.step === "1" && ed.dx.plus && ed.dx.minus);
  verdict("editor-shows-dx-dy-steppers-with-grid-snap-step", steppers, { dx: ed?.dx, dy: ed?.dy, note: renderer === "wgpu" ? "the mirror announces a spinbutton; its snap step is proven by ArrowUp/ArrowDown in step 4" : "snapSource {config: gridFactor} resolves to the grid factor (default 1) as the stepper step" });
  verdict("editor-dx-dy-read-the-original-input", Boolean(ed?.dx && ed?.dy && near(Number(ed.dx.value), ctx.dx!, 0.005) && near(Number(ed.dy.value), ctx.dy!, 0.005)), { dx: ed?.dx?.value, dy: ed?.dy?.value, expected: [ctx.dx, ctx.dy] });
  verdict("editor-shows-the-targets-reference-list", Boolean(ed?.targets && ed.targets.chips.length === 2 && ed.targets.chips.some((chip) => chip.includes(ctx.a!)) && ed.targets.chips.some((chip) => chip.includes(ctx.b!)) && ed.targets.useSelection), { targets: ed?.targets, inputs: ed?.inputs });
  emit({ kind: "inventory", tag: "editing", rows: await historyInventory(), editor: ed, band: b });
  await shot("editing");
};

/** 🎚️ Step 4 — dx → 120 through the stepper (keyboard), Accept, replay, review `ready`, head at +120 with downstream. */
const step4 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || (await band())?.stage !== "editing") {
    verdict("precondition-editing-session", false, { band: (await band())?.stage ?? null });
    return;
  }
  const typed = await typeEditorNumber("dx", 120);
  verdict("dx-stepper-control-reachable", typed.present && typed.typed === "120", { typed, authored: "framework.history.editor.input.dx" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.b!, ctx.p0![ctx.b!], 120, ctx.dy!), 20000);
  verdict("dx-120-updates-the-preview", preview.ok && placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { typed, a: preview.value[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dy!], c: preview.value[ctx.c], waitedMs: preview.waitedMs });
  if (typed.present) {
    await keyEditorNumber("dx", "ArrowUp");
    const up = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 121, ctx.dy!), 15000);
    await keyEditorNumber("dx", "ArrowDown");
    const down = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!), 15000);
    verdict("arrow-keys-step-dx-by-the-snap-step", up.ok && down.ok, { up: up.value[ctx.a], down: down.value[ctx.a], dxField: (await editor())?.dx?.value });
  }
  const accepted = (await editor())?.dx?.value ?? null;
  await clearBandTrace();
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  const trace = await bandTrace();
  const stages = [...new Set(trace.map((row) => row.key.split("|")[0]))];
  const progress = trace.filter((row) => row.key.split("|")[2]);
  verdict("accept-replays-then-reviews-ready", review.value?.stage === "reviewing" && review.value.review === "ready", { via, dxField: accepted, band: review.value?.text?.slice(0, 200), review: review.value?.review, waitedMs: review.waitedMs });
  if (stages.includes("replaying") || progress.length) verdict("replay-stage-rendered", true, { trace: trace.slice(0, 20) });
  else note("replay-stage-not-rendered", { trace: trace.slice(0, 20), reading: "the two-mutation replay finished inside one reactor turn, so the band went editing → reviewing without a replaying frame (progress is throttled ≥ 100 ms / ≥ 5 %)" });
  verdict("review-line-reads-ready", Boolean(review.value?.text.includes(COPY[currentLocale].reviewReady)), { text: review.value?.text?.slice(0, 200) });
  const finalize = review.value?.controls.find((c) => c.control === "finalize");
  verdict("finalize-enabled-when-ready", finalize?.disabled === false, { controls: review.value?.controls });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.c!, ctx.pc![ctx.c!], 0, 0), 15000);
  verdict("reviewed-head-at-plus-120-with-downstream-reapplied", head.ok && placed(head.value, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!), { a: head.value[ctx.a], b: head.value[ctx.b], c: head.value[ctx.c], cHead: ctx.pc?.[ctx.c] });
  await shot("reviewing");
};

/** 🏁️ Step 5 — Finalize → dialog → Overwrite → band gone, overwrite row, head persists across a reload. */
const step5 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || (await band())?.review !== "ready") {
    verdict("precondition-ready-review", false, { band: await band() });
    return;
  }
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  const d = dialog.value;
  const copy = COPY[currentLocale];
  verdict("finalize-opens-the-dialog", Boolean(d && d.title.includes(copy.dialogTitle)), { title: d?.title, waitedMs: dialog.waitedMs });
  verdict("dialog-offers-destructive-overwrite", Boolean(d?.overwrite && d.overwrite.text.includes(copy.overwrite) && (renderer === "wgpu" || d.overwrite.destructive === "true") && (d.overwrite.description ?? "").includes(copy.overwriteDescription)), { overwrite: d?.overwrite, expectedDescription: copy.overwriteDescription, reading: renderer === "wgpu" ? "the mirror carries no tone; the destructive choice is told by its description" : "data-destructive + the choice description" });
  verdict("dialog-offers-new-alternative-with-a-name-field", Boolean(d?.submit && d.submit.text.includes(copy.newAlternative) && d.name && d.name.value === copy.defaultName), { submit: d?.submit, name: d?.name });
  if (renderer === "wgpu") note("band-while-the-dialog-is-open", { band: (await band())?.stage ?? null, reading: "the wgpu chrome projects only the modal dialog's nodes while it is open (`chrome_accessibility_nodes`), so the band's status is not announced then" });
  else verdict("band-reads-choosing-while-the-dialog-is-open", (await band())?.stage === "choosing", { band: (await band())?.stage });
  await shot("dialog");
  await dialogChoose("overwrite");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict("overwrite-closes-the-session", gone.ok && (await finalizeDialog()) === null, { waitedMs: gone.waitedMs, band: gone.value?.stage ?? null });
  const expected = copy.row("edit", null, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("overwrite-row-appears", rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  ctx.afterOverwrite = await positions();
  verdict("head-keeps-plus-120-after-overwrite", placed(ctx.afterOverwrite, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.c, ctx.pc![ctx.c], 0, 0), { a: ctx.afterOverwrite[ctx.a], c: ctx.afterOverwrite[ctx.c] });
  await shot("overwritten");
  note("reload-check-deferred", { reason: "the reload is taken after step 8 and before step 9 (attributed to step 5), so steps 6–8 keep the document it would otherwise replace and step 9 digests its console" });
};

/** 📂️ Picks the sync card's Folder kind: the sync utilities render as one grouped toggle (`ui.utilities.group.sync`)
 * whose menu lists File / Folder / Remote. */
const pickSyncFolder = async () => {
  if (renderer === "wgpu") {
    if (!mirrorFind(await mirror(), "framework.sync.folder")) await wgpuPress("ui.utilities.group.sync");
    await sleep(700);
    return (await wgpuPress("framework.sync.folder")) ?? "absent";
  }
  const direct = page.locator('[id="framework.sync.folder"]').first();
  if (!(await direct.count().catch(() => 0))) {
    await page.locator('[id="ui.utilities.group.sync"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(700);
  }
  if (await direct.count().catch(() => 0)) {
    await direct.click({ timeout: 4000 }).catch(() => {});
    return "id";
  }
  const item = page.locator('[role="menuitem"], [role="menuitemradio"], [role="option"], button').filter({ hasText: /^\s*(folder|ordner)\s*$/i }).first();
  if (await item.count().catch(() => 0)) {
    await item.click({ timeout: 4000 }).catch(() => {});
    return "text";
  }
  return "absent";
};

/** 🗂️ Attaches the session document to the local folder `path` through the sync chip (`s-sync-status` →
 * `framework.sync.folder` → `framework.sync.folder.path` → Attach): `openSyncTarget` opens it with a
 * `persistedLocalOnly` folder binding the backbone worker reads and writes through the dev serve's `/semio-backbone`. */
const attachFolder = async (path: string) => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await sleep(600);
    await pickSyncFolder();
    const shown = await waitUntil(async () => mirrorFind(await mirror(), "framework.sync.folder.path") ?? null, (node) => node !== null, 8000);
    const field = shown.value ? await mirrorLocator(shown.value.key, shown.value.window) : null;
    await field?.fill(path, { force: true, timeout: 4000 }).catch(() => {});
    const typed = field ? await field.inputValue({ timeout: 2000 }).catch(() => null) : null;
    await sleep(400);
    const attached = await wgpuPress("framework.sync.attach");
    await sleep(1500);
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return { card: shown.ok, typed, attachButton: attached !== null };
  }
  const chip = page.locator('[id="s-sync-status"]').first();
  if (!(await openPanelTabIds()).includes("s-sync-status")) await chip.click({ timeout: 4000 }).catch(() => {});
  await sleep(600);
  await pickSyncFolder();
  const field = page.locator('[id="framework.sync.folder.path"]').first();
  const shown = await waitUntil(() => field.count().catch(() => 0), (count) => count > 0, 8000);
  await field.fill(path, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  const attach = page.locator("button").filter({ hasText: /^\s*(attach|verbinden)\s*$/i }).first();
  const attachCount = await attach.count().catch(() => 0);
  if (attachCount) await attach.click({ timeout: 4000 }).catch(() => {});
  else await field.press("Enter").catch(() => {});
  await sleep(1500);
  await page.keyboard.press("Escape").catch(() => {});
  return { card: shown.ok, typed, attachButton: attachCount > 0 };
};

/** 🔁️ Step 5's reload half, taken after step 8. A `?plugin=` playground opens no space, so
 * `resolveDocumentOpeningBindings` (`🏛️ShellHost/🧭️opening/🟦️.ts`) answers no binding and a bare reload re-runs Set
 * Active Example; the one local-only route is the sync card's folder attach, remembered per device in
 * `os.config.local-folders` by the document's store id. The folder is attached at boot (`--folder-at=1`) or here with one
 * edit to trigger the write (`--folder-at=5`); then: see the archive on disk, reload, take the reconnect band's offer
 * (`🏛️ShellHost/📁️local-folders`, fallback: attach again by hand), compare head, rows and edit ids with the ones before the
 * reload, and finally detach and reload once more — a forgotten folder must not be offered. */
const reloadCheck = async (ctx: Ctx) => {
  if (!ctx.afterOverwrite || !ctx.a) return;
  const early = ctx.folder !== undefined;
  const folder = ctx.folder ?? join(OUT, `folder-${stamp}-${currentLocale}`);
  mkdirSync(folder, { recursive: true });
  const beforeAttach = await positions();
  await closePanels();
  const attached = early ? { card: true, typed: folder, attachButton: true } : await attachFolder(folder);
  const afterAttach = await positions();
  if (!early) verdict("folder-attach-keeps-the-head", movedIds(beforeAttach, afterAttach, 1e-6).length === 0, { attached, drift: movedIds(beforeAttach, afterAttach, 1e-6).slice(0, 6) });
  const files = () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => `${entry.parentPath ?? ""}/${entry.name}`.replace(folder, ""));
  if (!early && ctx.c && afterAttach[ctx.c]) {
    await closePanels();
    const zoom = cameraOf(await vitals()).zoom;
    await dragBy(toScreen(afterAttach[ctx.c], cameraOf(await vitals()), await paneBox()), 10 * zoom, 10 * zoom);
    await waitUntil(positions, (p) => offsetOf(afterAttach, p, ctx.c!)?.some((v) => Math.abs(v) > 0.5) === true, 15000);
  }
  const written = await waitUntil(async () => files(), (list) => list.length > 0, 30000, 1000);
  verdict("folder-attach-writes-the-document-archive", attached.typed === folder && written.ok, { files: written.value.slice(0, 8), folder, waitedMs: written.waitedMs, reading: "the archive is written on the bound document's outbound mutations (`archivePersistence` in `bindDocumentBackbone.send`), so one edit follows the attach" });
  const before = await positions();
  const historyBefore = await allHistoryRows();
  const rowsBefore = documentEntries(historyBefore).map((row) => row.label.slice(0, 60));
  const editsBefore = await documentEditIds(historyBefore);
  emit({ kind: "rows", tag: "before-reload", editIds: editsBefore, rows: historyBefore.filter((row) => row.kind === "entry").map((row) => ({ key: row.key, label: row.label, text: row.text.slice(0, 160), expandable: row.expandable })) });
  const warned = historyBefore.find((row) => row.kind === "entry" && /Set Active Example|Beispiel/i.test(row.label) && /Warning|Warnung/.test(row.text));
  if (warned) {
    const expanded = await expandEntry(warned.id);
    note("example-row-warning-after-attach", { label: warned.text.slice(0, 120), flagged: expanded.mutations.filter((row) => /Warning|Warnung/.test(row.text)).slice(0, 8).map((row) => row.text.slice(0, 200)), shown: expanded.mutations.length });
  }
  await closePanels();
  expectedReloads += 1;
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebooted = await waitForBoot("reload", 60);
  await installBandTrace();
  await installNoticeTrace();
  const bare = await positions();
  note("bare-reload-reruns-the-example", { driftCount: movedIds(before, bare, 1e-6).length, route: await evalSafe(() => location.search, "") });
  const example = (await allHistoryRows()).filter((row) => row.kind === "entry" && /Set Active Example|Beispiel/i.test(row.label));
  if (example[0]) {
    const expanded = await expandEntry(example[0].id);
    note("post-reload-example-row", { label: example[0].label.slice(0, 120), text: example[0].text.slice(0, 200), flagged: expanded.mutations.filter((row) => /Warning|Warnung|Error|Fehler/.test(row.text)).slice(0, 6).map((row) => row.text.slice(0, 160)) });
  }
  await closePanels();
  const folderName = folder.split("/").pop() ?? folder;
  const offer = await waitUntil(reconnectBand, (state) => state?.state === "offered", 20000, 500);
  const offered = offer.value;
  verdict("folder-reconnect-offered", Boolean(offered && offered.state === "offered" && offered.role === "status" && offered.live === "polite" && offered.message.includes(folderName) && offered.reconnect && offered.forget), { band: offered, folderName, waitedMs: offer.waitedMs });
  let reattached: Record<string, unknown>;
  if (offered?.reconnect) {
    const via = renderer === "wgpu" ? await wgpuPress("s-folder-reconnect") : await page.locator("#s-folder-reconnect").click({ timeout: 4000 }).then(() => "button").catch(() => null);
    const busy = await waitUntil(reconnectBand, (state) => state === null || state.state === "reconnecting", 10000, 100);
    const settled = await waitUntil(reconnectBand, (state) => state === null, 60000, 500);
    reattached = { via, sawBusy: busy.value?.state === "reconnecting", bandGone: settled.ok, waitedMs: settled.waitedMs };
    verdict("folder-reconnect-attaches-and-closes-the-band", Boolean(via) && settled.ok, reattached);
  } else {
    reattached = { fallback: await attachFolder(folder) };
    note("folder-reconnect-fallback-manual-attach", reattached);
  }
  const restored = await waitUntil(positions, (p) => movedIds(before, p, 1e-6).length === 0, 60000, 1000);
  const drift = movedIds(before, restored.value, 1e-6);
  verdict("positions-persist-after-reload", rebooted && restored.ok, { rebooted, reattached, a: restored.value[ctx.a], expected: before[ctx.a], driftCount: drift.length, drift: drift.slice(0, 6), folder });
  note("folder-backbone-requests", { requests: consoleRows.filter((row) => row.locale === currentLocale && row.type === "backbone").slice(-30).map((row) => `${row.t}s ${row.text}`), sessions: await evalSafe(() => Array.from(document.querySelectorAll("[data-surface-id]")).map((el) => el.getAttribute("data-surface-id")).slice(0, 3), [] as (string | null)[]) });
  const hydrated = await waitUntil(allHistoryRows, (all) => documentEntries(all).length >= rowsBefore.length, 20000, 1000);
  const historyAfter = hydrated.value;
  const editsAfter = await documentEditIds(historyAfter);
  emit({ kind: "rows", tag: "after-reload-reattach", editIds: editsAfter, rows: historyAfter.filter((row) => row.kind === "entry").map((row) => ({ key: row.key, label: row.label, text: row.text.slice(0, 160), expandable: row.expandable })) });
  verdict("edit-ids-survive-the-reload", editsBefore.length > 0 && editsBefore.every((id) => editsAfter.includes(id)), { before: editsBefore.length, after: editsAfter.length, lost: editsBefore.filter((id) => !editsAfter.includes(id)).slice(0, 12) });
  const labels = documentEntries(historyAfter).map((row) => row.label.slice(0, 60));
  const descriptionless = /^(Duplicate Selection|Set Active Example|Auswahl duplizieren|Aktives Beispiel festlegen)$/;
  const lost = rowsBefore.filter((label) => !labels.includes(label) && !descriptionless.test(label));
  const relabelled = rowsBefore.filter((label) => !labels.includes(label) && descriptionless.test(label));
  verdict("document-rows-survive-the-reload", rowsBefore.length > 0 && lost.length === 0 && labels.length >= rowsBefore.length, { before: rowsBefore.slice(0, 14), after: labels.slice(0, 14), lost: lost.slice(0, 12), counts: [rowsBefore.length, labels.length], waitedMs: hydrated.waitedMs });
  if (relabelled.length) note("descriptionless-rows-come-back-as-op-text", { relabelled, reading: "known W2-B gap: commands whose app emits no `description` (Duplicate Selection, Set Active Example) reload with op-text labels; counted, not compared by label" });
  const overwrite = COPY[currentLocale].row("edit", null, 1);
  if (rowsBefore.some((label) => label.startsWith(overwrite))) verdict("overwrite-row-survives-the-reload", labels.some((label) => label.startsWith(overwrite)), { labels: labels.slice(0, 10) });
  else note("overwrite-row-absent-before-the-reload", { before: rowsBefore.slice(0, 12), reading: "the overwrite row was no longer in the document when the reload check ran (a dev reload reset it earlier)" });
  await shot("reloaded");
  await closePanels();
  const detached = await detachFolder();
  expectedReloads += 1;
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebootedAgain = await waitForBoot("reload-after-detach", 60);
  await installBandTrace();
  await installNoticeTrace();
  const stray = await waitUntil(reconnectBand, (state) => state !== null, 15000, 500);
  verdict("a-forgotten-folder-is-not-offered-after-reload", rebootedAgain && !stray.ok, { detached, band: stray.value, waitedMs: stray.waitedMs, reading: "Detach on the sync card forgets `os.config.local-folders`' binding, so the next load offers nothing" });
};

type ReconnectBand = { state: string; role: string | null; live: string | null; label: string | null; message: string; reconnect: boolean; forget: boolean };

/** 📁️ The folder-reconnect band a reload offers for a document bound to a local folder (`🏛️ShellHost/📁️local-folders`):
 * `role=status`, polite, `data-semio-folder-reconnect=offered|reconnecting`, its message and the Reconnect / Forget buttons
 * (wgpu: the same ids through the mirror). */
const reconnectBand = async (): Promise<ReconnectBand | null> => {
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const reconnect = nodes.find((node) => node.key === "s-folder-reconnect");
    if (!reconnect) return null;
    const message = nodes.find((node) => node.key === "s-folder-reconnect-message");
    return { state: reconnect.disabled ? "reconnecting" : "offered", role: "status", live: "polite", label: null, message: message?.label ?? "", reconnect: true, forget: nodes.some((node) => node.key === "s-folder-forget") };
  }
  return evalSafe(() => {
    const el = document.querySelector("[data-semio-folder-reconnect]");
    if (!el) return null;
    return {
      state: el.getAttribute("data-semio-folder-reconnect") ?? "",
      role: el.getAttribute("role"),
      live: el.getAttribute("aria-live"),
      label: el.getAttribute("aria-label"),
      message: document.getElementById("s-folder-reconnect-message")?.textContent ?? "",
      reconnect: Boolean(document.getElementById("s-folder-reconnect")),
      forget: Boolean(document.getElementById("s-folder-forget")),
    };
  }, null as ReconnectBand | null);
};

/** ✂️ Detaches the document from its folder through the sync card (Detach / Trennen; wgpu `framework.sync.detach`). */
const detachFolder = async () => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await sleep(600);
    await pickSyncFolder();
    await sleep(600);
    const via = await wgpuPress("framework.sync.detach");
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return via;
  }
  if (!(await openPanelTabIds()).includes("s-sync-status")) await page.locator('[id="s-sync-status"]').first().click({ timeout: 4000 }).catch(() => {});
  await sleep(600);
  await pickSyncFolder();
  const detach = page.locator("button").filter({ hasText: /^\s*(detach|trennen)\s*$/i }).first();
  const present = await detach.count().catch(() => 0);
  if (present) await detach.click({ timeout: 4000 }).catch(() => {});
  await sleep(1000);
  await page.keyboard.press("Escape").catch(() => {});
  return present ? "button" : null;
};

/** ↩️ Step 6 — Undo takes the finalize back (+80), Redo re-authors it (+120); chord first, then the panel button. */
const step6 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.afterOverwrite) {
    verdict("precondition-step-5", false, {});
    return;
  }
  const isAt = (dx: number) => async () => placed(await positions(), ctx.a!, ctx.p0![ctx.a!], dx, ctx.dy!);
  const attempt = async (verb: "undo" | "redo", target: number) => {
    const chord = verb === "undo" ? `${mod}+z` : `${mod}+Shift+z`;
    await prepareChord();
    await page.keyboard.press(chord).catch(() => {});
    const byChord = await waitUntil(isAt(target), (ok) => ok, 12000, 400);
    verdict(`${verb}-chord-${verb === "undo" ? "takes-back" : "re-applies"}-the-finalize`, byChord.ok, { chord, a: (await positions())[ctx.a!], expected: [ctx.p0![ctx.a!][0] + target, ctx.p0![ctx.a!][1] + ctx.dy!], waitedMs: byChord.waitedMs });
    if (byChord.ok) return 0;
    const presses: Record<string, unknown>[] = [];
    await openHistory();
    for (let press = 1; press <= 4; press++) {
      const before = await positions();
      const panelsBefore = await openPanelTabIds();
      if (!panelsBefore.includes(HISTORY_TAB)) await openHistory();
      const pressed = await pressAuthored(`framework.history.${verb}`);
      const settled = await waitUntil(isAt(target), (ok) => ok, 8000, 400);
      presses.push({ press, pressed, reached: settled.ok, moved: movedIds(before, await positions()).length, panels: await openPanelTabIds() });
      if (settled.ok) {
        verdict(`${verb}-button-reaches-the-finalize`, press === 1, { presses });
        return press;
      }
    }
    verdict(`${verb}-button-reaches-the-finalize`, false, { presses });
    return -1;
  };
  await closePanels();
  await attempt("undo", ctx.dx!);
  const undone = await positions();
  verdict("undo-restores-plus-80", placed(undone, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(undone, ctx.b!, ctx.p0[ctx.b!], ctx.dx!, ctx.dy!), { a: undone[ctx.a], b: undone[ctx.b!] });
  await closePanels();
  await attempt("redo", 120);
  const redone = await positions();
  verdict("redo-restores-plus-120", placed(redone, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(redone, ctx.b!, ctx.p0[ctx.b!], 120, ctx.dy!), { a: redone[ctx.a] });
  const copy = COPY[currentLocale];
  const rows = await allHistoryRows();
  const labels = documentEntries(rows).map((row) => row.label);
  verdict("undone-and-redone-rows-appear", labels.some((label) => label.startsWith(copy.row("undo", null, 1))) && labels.some((label) => label.startsWith(copy.row("redo", null, 1))), { expected: [copy.row("undo", null, 1), copy.row("redo", null, 1)], labels: labels.slice(-8) });
  await shot("undo-redo");
  await closePanels();
};

/** 🌿️ Step 7 — a second session on dy → New alternative with a name → its row and head; then switch alternatives. */
const step7 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.dragEntry) {
    verdict("precondition-step-2", false, {});
    return;
  }
  const dy0 = ctx.dy!;
  ctx.dyEdited = Math.round(dy0 + 30);
  ctx.alternative = `probe ${currentLocale} ${stamp.slice(11, 19)}`;
  const first = await editDragAsAlternative(ctx, ctx.dyEdited, ctx.alternative, "");
  if (!first) return;
  await shot("alternative");
  const listed = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(ctx.alternative!)), 15000, 1000);
  verdict("alternatives-section-lists-the-new-alternative", listed.ok, { rows: listed.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${row.active ? " [active]" : ""}`), expectedSelector: "framework.history.alternatives / framework.history.alternative.<id>" });
  if (!listed.value.length) {
    verdict("alternative-switcher-present", false, { inventory: (await historyInventory()).filter((row) => /alternative/i.test(String(row.id))).slice(0, 12), note: "no `framework.history.alternative.<id>` rows in this build" });
    await closePanels();
    return;
  }
  const trunk = listed.value.find((row) => !row.text.includes(ctx.alternative!));
  const expectAt = (dy: number) => async () => placed(await positions(), ctx.a!, ctx.p0![ctx.a!], 120, dy) && placed(await positions(), ctx.b!, ctx.p0![ctx.b!], 120, dy);
  if (trunk) {
    const via = await switchAlternative(trunk);
    const back = await waitUntil(expectAt(dy0), (ok) => ok, 20000, 500);
    verdict("switching-to-the-trunk-shows-original-positions", back.ok, { via, trunk: trunk.text.slice(0, 60), a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + dy0] });
    const mine = (await readAlternatives()).find((row) => row.text.includes(ctx.alternative!));
    const forth = mine ? await switchAlternative(mine) : "absent";
    const edited = await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    verdict("switching-back-shows-the-edited-positions", edited.ok, { via: forth, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
  } else {
    verdict("trunk-listed-after-new-alternative", false, { rows: listed.value.map((row) => row.text.slice(0, 60)), pending: "W1-G: after finalize-as-new-alternative the original trunk is not yet listed — expected-pending" });
    const second = `${ctx.alternative} b`;
    const dy2 = Math.round(dy0 + 60);
    const made = await editDragAsAlternative(ctx, dy2, second, "-2");
    if (!made) return;
    const both = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(second)) && rows.some((row) => row.text.includes(ctx.alternative!) && !row.text.includes(second)), 15000, 1000);
    verdict("alternatives-section-lists-both-alternatives", both.ok, { rows: both.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${row.active ? " [active]" : ""}`) });
    const firstRow = both.value.find((row) => row.text.includes(ctx.alternative!) && !row.text.includes(second));
    const viaFirst = firstRow ? await switchAlternative(firstRow) : "absent";
    const atFirst = await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    verdict("switching-to-the-first-alternative-shows-its-positions", atFirst.ok, { via: viaFirst, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
    const secondRow = (await readAlternatives()).find((row) => row.text.includes(second));
    const viaSecond = secondRow ? await switchAlternative(secondRow) : "absent";
    const atSecond = await waitUntil(expectAt(dy2), (ok) => ok, 20000, 500);
    verdict("switching-to-the-second-alternative-shows-its-positions", atSecond.ok, { via: viaSecond, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + dy2] });
    const back = firstRow ? await switchAlternative((await readAlternatives()).find((row) => row.key === firstRow.key) ?? firstRow) : "absent";
    await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    note("left-on-the-first-alternative", { via: back });
  }
  await shot("switched");
  await closePanels();
};

type AlternativeRow = { id: string; key: string; text: string; active: boolean };

/** 🌿️ The Alternatives section's rows (`framework.history.alternative.<id>`), with whether each reads active. */
const readAlternatives = async (): Promise<AlternativeRow[]> => {
  await openHistory();
  for (const edge of ["start", "end"] as const) await scrollHistory(edge);
  if (renderer === "wgpu") return (await mirror()).filter((node) => !node.key.includes("::") && /(^|\/)framework\.history\.alternative\.[^/]+$/.test(node.key)).map((node) => ({ id: node.key, key: node.key.replace(/^.*framework\.history\.alternative\./, ""), text: `${node.label} ${node.description}`.trim(), active: node.selected === "true" || node.pressed === "true" }));
  return evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /(^|\/)framework\.history\.alternative\.[^/]+$/.test(el.id))
        .map((el) => ({
          id: el.id,
          key: el.id.replace(/^.*framework\.history\.alternative\./, ""),
          text: (el as HTMLElement).innerText.replace(/\s+/g, " ").trim(),
          active: el.getAttribute("aria-selected") === "true" || el.getAttribute("aria-current") !== null || el.getAttribute("data-selected") === "true",
        })),
    [] as AlternativeRow[],
  );
};

/** 🔀️ Switches to one alternative through its Switch row action, else the row's own activation. */
const switchAlternative = async (row: AlternativeRow) => pressRowAction(row.id, /^(Switch|Switch to|Switch alternative|Wechseln|Umschalten|Alternative wechseln)$/i);

/** 🌿️ One history-edit session on the drag's dy through the real stepper → Accept → Finalize → New alternative `name`;
 * verdicts carry `suffix` so a second alternative reads apart from the first. Answers whether the session committed. */
const editDragAsAlternative = async (ctx: Ctx, dyValue: number, name: string, suffix: string) => {
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  verdict(`session-opens${suffix}`, begun.band?.stage === "editing", { via: begun.via, stage: begun.band?.stage });
  if (begun.band?.stage !== "editing") return false;
  const ed = (await waitUntil(readEditor, (value) => Boolean(value?.dx && value?.dy), 15000)).value;
  verdict(`editor-reads-the-effective-input${suffix}`, Boolean(ed?.dx && near(Number(ed.dx.value), 120, 0.005)), { dx: ed?.dx?.value, dy: ed?.dy?.value, note: "the drag's effective input in the current alternative carries the overwritten dx=120" });
  if (!ed?.dx) {
    dumpJson(`no-editor${suffix}`, { band: await band(), panels: await openPanelTabIds(), inventory: (await historyInventory()).filter((row) => !/entry\.|mutation\./.test(String(row.id))), rows: (await readHistory()).slice(0, 40) });
    await shot(`no-editor${suffix}`);
  }
  const typed = await typeEditorNumber("dy", dyValue);
  verdict(`dy-stepper-control-reachable${suffix}`, typed.present && typed.typed === String(dyValue), { typed, authored: "framework.history.editor.input.dy" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, dyValue), 20000);
  verdict(`dy-edit-updates-the-preview${suffix}`, preview.ok, { a: preview.value[ctx.a!], expected: [ctx.p0![ctx.a!][0] + 120, ctx.p0![ctx.a!][1] + dyValue] });
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict(`review-ready${suffix}`, review.value?.review === "ready", { via, review: review.value?.review, text: review.value?.text?.slice(0, 160) });
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  const nameValue = await dialogFillName(name);
  verdict(`name-field-takes-the-alternative-name${suffix}`, dialog.ok && nameValue === name, { nameValue, submit: dialog.value?.submit });
  await dialogChoose("submit");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict(`new-alternative-closes-the-session${suffix}`, gone.ok, { band: gone.value?.stage ?? null, fault: gone.value?.fault ?? null });
  const expected = COPY[currentLocale].row("edit", name, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict(`alternative-row-appears${suffix}`, rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(0, 6) });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, dyValue), 15000);
  verdict(`head-is-on-the-edited-alternative${suffix}`, head.ok, { a: head.value[ctx.a!], expected: [ctx.p0![ctx.a!][0] + 120, ctx.p0![ctx.a!][1] + dyValue] });
  return gone.ok;
};

/** 💥️ Step 8 — duplicate a node, drag the clone, withdraw the clone's `create-node` → blocked review with an Error row,
 * Finalize disabled; Next problem → withdraw the drag → ready; Exit → zero trace. */
const step8 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  const d = spot?.row;
  if (!spot || !d) {
    verdict("clone-source-with-free-space", false, { reason: "no node whose clone (+24,+24) and drag destination are ≥ 55 world units from every other node" });
    return;
  }
  log(`clone source ${d.id} at ${JSON.stringify(d.world)} drag ${JSON.stringify(spot.move)} zoom=${spot.zoom}`);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(d.at.x, d.at.y);
  const selected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === d.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  const grew = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  verdict("duplicate-adds-one-clone", selected.ok && grew.ok && Boolean(clone), { d: d.id, clone, nodes: [nodesBefore, grew.value?.nodes] });
  if (!clone) return;
  const reselected = await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  const cleared = await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  note("clone-deselected-before-its-drag", { cleared: cleared.ok, selection: selectionIds(cleared.value), reading: "board `sole_indirect_handle_hit_idle_selected_node`: while a node with a sole free handle is the only selection, a press inside it resolves to that handle and starts a link drag" });
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  const grab = { x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 };
  await dragBy(grab, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const cloneOffset = offsetOf(beforeDrag, dragged.value, clone);
  verdict("clone-drag-moves-only-the-clone", dragged.ok && movedIds(beforeDrag, dragged.value).every((id) => id === clone), { reselected: reselected.ok, cloneOffset, moved: movedIds(beforeDrag, dragged.value).slice(0, 4), selection: selectionIds(await vitals()), grab, cloneAt });
  const head = dragged.value;
  const added = (await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 2, 30000, 1000)).value;
  const copy = COPY[currentLocale];
  let createRow: HistoryRow | undefined;
  let dragRow: HistoryRow | undefined;
  for (const entry of added) {
    const expanded = await expandEntry(entry.id);
    createRow ??= expanded.mutations.find((row) => row.label.startsWith(copy.createNode(clone)));
    if (cloneOffset) dragRow ??= expanded.mutations.find((row) => row.label.startsWith(dragLabel(1, cloneOffset[0], cloneOffset[1])));
  }
  verdict("duplicate-and-drag-rows-carry-their-mutations", Boolean(createRow && dragRow), { added: added.map((row) => row.label), create: createRow?.label, drag: dragRow?.label, expectedCreate: copy.createNode(clone) });
  if (!createRow || !dragRow) return;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  await installBandTrace();
  const via = await pressRowAction(createRow.id, copy.edit);
  const editing = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  const preview = await waitUntil(positions, (p) => placed(p, clone, beforeDrag[clone], 0, 0), 15000);
  verdict("editing-the-create-node-previews-the-clone-before-its-drag", editing.ok && preview.ok, { via, clone: preview.value[clone], beforeDrag: beforeDrag[clone], head: head[clone], waitedMs: preview.waitedMs });
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  verdict("withdraw-control-reachable", withdraw.present && withdraw.disabled !== true, { withdraw, authored: "framework.history.editor.withdraw(.row)" });
  const withdrawn = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("withdraw-previews-the-document-without-the-clone", withdrawn.ok, { withdraw, hasClone: Boolean(withdrawn.value[clone]) });
  await pressBand("accept");
  const blocked = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("replay-review-is-blocked", blocked.value?.review === "blocked", { review: blocked.value?.review, text: blocked.value?.text?.slice(0, 200), outcome: blocked.value?.outcome });
  verdict("review-line-names-the-blocking-errors", Boolean(blocked.value?.text.includes(copy.reviewBlocked)), { text: blocked.value?.text?.slice(0, 200) });
  const finalize = blocked.value?.controls.find((c) => c.control === "finalize");
  verdict("finalize-disabled-while-blocked", finalize?.disabled === true, { finalize });
  await openHistory();
  const failing = (await waitUntil(readHistory, (rows) => rows.some((row) => row.key === dragRow!.key && row.text.includes(copy.targetMissing)), 15000)).value.find((row) => row.key === dragRow!.key);
  verdict("failing-row-reads-error-target-missing", Boolean(failing?.text.includes(copy.targetMissing) && copy.severityError.test(failing.text)), { row: failing?.text?.slice(0, 200), expected: copy.targetMissing });
  await shot("blocked");
  emit({ kind: "inventory", tag: "blocked", rows: await historyInventory(), band: blocked.value });
  const rounds: Record<string, unknown>[] = [];
  for (let round = 0; round < 4 && (await band())?.review === "blocked"; round++) {
    const next: Record<string, unknown> = await pressAuthored("framework.history.timeTravel.nextProblem");
    if (!next.present) {
      await openHistory();
      const problem = (await readHistory()).find((row) => row.kind === "mutation" && copy.severityError.test(row.text));
      next.fallback = problem ? `edit on failing row ${problem.key} via ${await pressRowAction(problem.id, copy.edit)}` : "no failing row";
    }
    const generationBefore = (await band())?.generation;
    const reopened = await waitUntil(band, (b) => b?.stage === "editing", 15000);
    await sleep(800);
    const generationEditing = (await band())?.generation;
    const heading = (await editor())?.heading ?? (await band())?.target;
    const pulled: Record<string, unknown> = await pressAuthored("framework.history.editor.withdraw");
    const marked = await waitUntil(readHistory, (rows) => rows.some((row) => row.key === dragRow!.key && /Withdrawn|Zurückgezogen/.test(row.text)), 8000);
    const dragText = marked.value.find((row) => row.key === dragRow!.key)?.text?.slice(0, 160);
    await pressBand("accept");
    const settled = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
    rounds.push({ round, next, reopened: reopened.ok, generationBefore, generationEditing, heading, pulled, dragMarkedWithdrawn: marked.ok, dragText, review: settled.value?.review, accepted: settled.value?.text?.match(/Accepted changes: \d+|Übernommene Änderungen: \d+/)?.[0] });
  }
  verdict("next-problem-control-reachable", rounds.length > 0 && (rounds[0].next as { present?: boolean } | undefined)?.present === true, { first: rounds[0]?.next, authored: "framework.history.timeTravel.nextProblem(.row)" });
  const ready = await band();
  verdict("withdrawing-the-failing-drag-makes-the-review-ready", ready?.review === "ready", { rounds, review: ready?.review, text: ready?.text?.slice(0, 160) });
  await shot("resolved");
  await pressBand("exit");
  const gone = await waitUntil(band, (b) => b === null, 20000);
  verdict("exit-closes-the-session", gone.ok, { band: gone.value?.stage ?? null });
  const after = await waitUntil(positions, (p) => movedIds(head, p).length === 0, 15000);
  verdict("exit-leaves-head-positions-unchanged", after.ok, { drift: movedIds(head, after.value).slice(0, 8) });
  const seqOf = (row: string) => Number(row.match(/framework\.history\.entry\.(\d+)=/)?.[1] ?? -1);
  const newestPre = Math.max(...rowsPre.map(seqOf));
  const rowsPost = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  const addedRows = rowsPost.filter((row) => seqOf(row) > newestPre);
  const relabelled = rowsPost.filter((row) => seqOf(row) <= newestPre && rowsPre.some((pre) => seqOf(pre) === seqOf(row)) && !rowsPre.includes(row));
  verdict("exit-leaves-no-new-rows", addedRows.length === 0 && relabelled.length === 0, { newestPre, added: addedRows.map((row) => row.slice(0, 120)), relabelled: relabelled.map((row) => row.slice(0, 120)) });
  await shot("exited");
  await closePanels();
};

/** 🧾️ Step 9 — the locale's console: uncaught page errors and hard guest faults fail; errors, warnings and `[DEBUG] `
 * lines are digested with their most frequent texts. */
const step9 = async (_ctx: Ctx) => {
  const mine = consoleRows.filter((row) => row.locale === currentLocale);
  const errors = mine.filter((row) => row.type === "error" && !BENIGN_RE.test(row.text));
  const warnings = mine.filter((row) => row.type === "warning" && !BENIGN_RE.test(row.text));
  const debug = mine.filter((row) => row.text.startsWith("[DEBUG] "));
  const http = mine.filter((row) => row.type === "http");
  const top = (rows: ConsoleRow[]) => [...rows.reduce((map, row) => map.set(row.text.slice(0, 160), (map.get(row.text.slice(0, 160)) ?? 0) + 1), new Map<string, number>())].sort((x, y) => y[1] - x[1]).slice(0, 12);
  const refusals = [...new Set(mine.map((row) => /refused a local batch \S+ ((?:local|sync)\.[\w.-]+)/.exec(row.text)?.[1]).filter((code): code is string => Boolean(code)))];
  const shownCodes = await noticeCodes();
  if (!refusals.length) note("no-command-rejection-in-this-run", {});
  else if (renderer === "wgpu") verdict("rejection-notices-carry-their-code", false, { refusals, prerequisite: "the wgpu transient notice is painted only: its message and `ShellTransientNotice.code` are not projected into the ARIA mirror (only `shell.notice.close` is a hit), so no DOM carries React's `data-notice-code` — project it as a polite status node (key `shell.notice`, description = code)" });
  else verdict("rejection-notices-carry-their-code", refusals.every((code) => shownCodes.includes(code)), { refusals, shownCodes });
  const uncaught = pageErrors.filter((row) => row.locale === currentLocale);
  verdict("no-uncaught-page-errors", uncaught.length === 0, { count: uncaught.length, first: uncaught.slice(0, 3) });
  const hard = hardFaults.filter((row) => row.locale === currentLocale);
  verdict("no-hard-guest-faults", hard.length === 0, { count: hard.length, first: hard.slice(0, 3) });
  note("console-digest", { lines: mine.length, errors: errors.length, warnings: warnings.length, debug: debug.length, http: top(http), topErrors: top(errors), topWarnings: top(warnings), topDebug: top(debug) });
};

const STEPS: Record<number, (ctx: Ctx) => Promise<void>> = { 1: step1, 2: step2, 3: step3, 4: step4, 5: step5, 6: step6, 7: step7, 8: step8, 9: step9 };
//#endregion 🔖️Steps

//#region 🔖️Main
/** 🌍️ One locale pass: a fresh context whose `navigator.language` selects the UI locale, one tab, steps 1–9. */
const runLocale = async (locale: Locale) => {
  currentLocale = locale;
  currentStep = 0;
  context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: locale === "de" ? "de-DE" : "en-US", acceptDownloads: false });
  page = await context.newPage();
  if (renderer === "wgpu") await page.addInitScript(() => {
    try {
      globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
    } catch {
      return;
    }
  });
  listen(page);
  navigations = 0;
  expectedReloads = 0;
  page.on("framenavigated", (frame) => {
    if (frame !== page.mainFrame()) return;
    navigations += 1;
    if (navigations > 1) log(`main frame navigated (#${navigations}) → ${frame.url().slice(0, 120)}`);
  });
  mod = (await page.evaluate(() => navigator.platform).catch(() => "MacIntel")).includes("Mac") ? "Meta" : "Control";
  log(`navigating to :${port} renderer=${renderer} locale=${locale} mod=${mod}`);
  await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle2d`, { waitUntil: "domcontentloaded", timeout: 90000 }).catch((error) => log(`goto failed ${String(error).split("\n")[0]}`));
  const booted = await waitForBoot("boot");
  currentStep = 1;
  if (!verdict("boot", booted, {})) {
    dumpJson("boot-failure", { inventory: await historyInventory(), console: consoleRows.filter((row) => row.locale === locale).slice(-80) });
    await shot("boot-failure");
    await context.close().catch(() => {});
    return;
  }
  await installBandTrace();
  await installNoticeTrace();
  if (explore && renderer === "wgpu") {
    const structure = await evalSafe(async () => (await (window as unknown as { semioWgpuIntrospection?: { dumpStructure: () => Promise<string> } }).semioWgpuIntrospection?.dumpStructure()) ?? "", "");
    const before = await mirror();
    await openHistory();
    const history = await mirror();
    await attachFolder(join(OUT, `folder-explore-${stamp}-${locale}`));
    dumpJson("explore", { windowIds: (() => { try { return (JSON.parse(structure || "{}") as { windowIds?: string[] }).windowIds; } catch { return null; } })(), board: await wgpuBoard(), boardContract: WGPU_BOARD_CONTRACT, chrome: await chromeHits(), mirrorBefore: before.map((node) => `${node.window}|${node.key}|${node.role}|${node.label.slice(0, 60)}`), mirrorHistory: history.map((node) => `${node.window}|${node.key}|${node.role}|${node.label.slice(0, 60)}`), mirrorAfterSync: (await mirror()).map((node) => `${node.window}|${node.key}|${node.role}|${node.label.slice(0, 60)}`), band: await band(), presence: await presenceText() });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  if (explore) {
    await openHistory();
    const syncIds = () => evalSafe(() => Array.from(document.querySelectorAll("[id]")).filter((el) => /sync/i.test(el.id)).map((el) => `${el.tagName.toLowerCase()}#${el.id}[${el.getAttribute("data-slot") ?? el.getAttribute("role") ?? ""}] vis=${(el as HTMLElement).offsetParent !== null} ${(el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 60) ?? ""}`), [] as string[]);
    const syncBefore = await syncIds();
    await page.locator('[id="s-sync-status"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(1000);
    const syncOpen = await syncIds();
    const picked = await pickSyncFolder();
    log(`sync folder pick: ${picked}`);
    await sleep(1000);
    const syncFolder = await syncIds();
    const popovers = await evalSafe(() => Array.from(document.querySelectorAll("[data-radix-popper-content-wrapper], [role=dialog]")).map((el) => (el as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 200)), [] as string[]);
    await shot("explore-sync");
    await page.keyboard.press("Escape").catch(() => {});
    dumpJson("explore", { syncBefore, syncOpen, syncFolder, popovers, inventory: await historyInventory(), rows: await readHistory(), band: await band(), vitals: await vitals(), tabs: await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((el) => `${el.id}:${(el as HTMLElement).innerText.trim()}`), [] as string[]) });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  const ctx: Ctx = {};
  const boardBlocked = renderer === "wgpu" && (await wgpuBoard()) === null;
  if (boardBlocked) verdict("wgpu-board-introspection-present", false, { prerequisite: WGPU_BOARD_CONTRACT, reading: "the wgpu Board2d surface projects no accessibility nodes and no dump carries its positions, camera or selection, so no pointer gesture can be aimed at a node; steps 2–8 are blocked until the export exists" });
  for (const step of [1, 2, 3, 4, 5, 6, 7, 8, 9]) {
    if (only && !only.has(step)) continue;
    if (boardBlocked && step >= 2 && step <= 8) {
      currentStep = step;
      verdict("blocked-by-missing-board-introspection", false, { prerequisite: "semioWgpuIntrospection.dumpBoard2d" });
      continue;
    }
    if (step === 9 && (!only || only.has(5))) {
      currentStep = 5;
      try {
        await reloadCheck(ctx);
      } catch (error) {
        verdict("reload-check-threw", false, { error: String(error).slice(0, 400) });
      }
    }
    currentStep = step;
    const navigationsBefore = navigations;
    const reloadsBefore = expectedReloads;
    log(`step ${step} begins`);
    try {
      await STEPS[step](ctx);
    } catch (error) {
      verdict("step-threw", false, { error: String(error).slice(0, 400) });
    }
    if (navigations - navigationsBefore > expectedReloads - reloadsBefore) note("page-reloaded-under-the-step", { navigations: navigations - navigationsBefore, reading: "the dev serve reloaded the page (Vite reconnect / supervisor recycle); verdicts after that point measure a fresh document" });
    const results = verdicts.filter((row) => row.locale === locale && row.step === step);
    if (results.some((row) => !row.ok)) dumpJson("failure", { inventory: await historyInventory(), band: await band(), editor: await editor(), dialog: await finalizeDialog(), vitals: await vitals(), console: consoleRows.filter((row) => row.locale === locale && row.step === step).slice(-60) });
    await shot("end");
  }
  if (only?.has(5) && !only.has(9)) {
    currentStep = 5;
    await reloadCheck(ctx);
  }
  emit({ kind: "context", ctx });
  await context.close().catch(() => {});
};

for (const locale of locales) await runLocale(locale);
await browser.close().catch(() => {});

const stepLines: string[] = [];
for (const locale of locales) {
  for (const step of [1, 2, 3, 4, 5, 6, 7, 8, 9]) {
    const rows = verdicts.filter((row) => row.locale === locale && row.step === step);
    if (!rows.length) continue;
    const failed = rows.filter((row) => !row.ok);
    stepLines.push(`STEP ${locale}/${step} ${failed.length ? "FAIL" : "PASS"} (${rows.length - failed.length}/${rows.length})${failed.length ? ` — ${failed.map((row) => row.name).join(", ")}` : ""}`);
  }
}
const passed = verdicts.filter((row) => row.ok).length;
const summary = `time-travel probe renderer=${renderer} PASS=${passed} FAIL=${verdicts.length - passed} uncaught=${pageErrors.length} hard=${hardFaults.length} locales=${locales.join(",")} only=${only ? [...only].join(",") : "all"}`;
console.log(summary);
for (const line of stepLines) console.log(line);
emit({ kind: "summary", summary, steps: stepLines });
writeFileSync(
  join(OUT, `${base}.md`),
  [
    `# ${base} (renderer ${renderer}, port ${port}, locales ${locales.join(",")}, chords ${[...chordLocales].join(",")}, only ${only ? [...only].join(",") : "all"})`,
    "",
    summary,
    "",
    "## Steps",
    ...stepLines,
    "",
    "## Verdicts",
    ...verdicts.map((row) => `- ${row.ok ? "PASS" : "FAIL"} ${row.locale}/${row.step} ${row.name} ${JSON.stringify(row.detail).slice(0, 700)}`),
    "",
    "## Notes",
    ...(notes.length ? notes.map((row) => `- ${row}`) : ["(none)"]),
    "",
    "## Uncaught page errors",
    ...(pageErrors.length ? pageErrors.map((row) => `- ${row.locale}/${row.step} ${row.text.slice(0, 600)}`) : ["(none)"]),
    "",
    "## Hard faults",
    ...(hardFaults.length ? hardFaults.slice(0, 40).map((row) => `- ${row.locale}/${row.step} ${row.text}`) : ["(none)"]),
    "",
    "## Timeline",
    ...timeline,
    "",
    "## Console (errors and warnings, last 200)",
    ...consoleRows.filter((row) => row.type === "error" || row.type === "warning" || row.type === "pageerror" || row.type === "http").slice(-200).map((row) => `${row.t}s ${row.locale}/${row.step} ${row.source} ${row.type}: ${row.text.slice(0, 400)}`),
  ].join("\n"),
);
process.exit(verdicts.length - passed === 0 && pageErrors.length === 0 ? 0 : 1);
//#endregion 🔖️Main
