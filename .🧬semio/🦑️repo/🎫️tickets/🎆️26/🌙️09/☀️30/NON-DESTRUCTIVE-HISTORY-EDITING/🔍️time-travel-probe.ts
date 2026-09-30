/** 🔍️ Time-travel end-to-end probe for the puzzle 2d React serve on `127.0.0.1:<port>` (ticket 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING, work package W3-E2E). One headless Chromium, one tab at a time, one fresh browser
 * context per UI locale (`en`, then `de`, chosen through `navigator.language`), nine numbered steps per locale:
 *
 * 1. boot: `data-board-fixture-parsed` first, then a board with nodes and the locale of the history panel;
 * 2. select two nodes, drag them by (+80,+40) world units → exactly one new document row labelled from its
 *    `drag-selection` mutation, `data-board-positions-json` moved by the offset; then one downstream drag of a third
 *    node, so "downstream is not applied" is observable;
 * 3. Edit the drag mutation (`historyEditBegin`) → the React band (`role=status`) in `editing`, the preview = state
 *    before the drag + the draft (design §4), the downstream drag not applied, the editor's dx/dy steppers (snap step)
 *    and the targets reference list;
 * 4. dx → 120 through the stepper (keyboard: fill, Enter, ArrowUp, ArrowDown) → preview; Accept → replay → review
 *    `ready` → head at +120 with the downstream drag re-applied;
 * 5. Finalize → dialog (destructive Overwrite, New alternative + name) → Overwrite → band gone, an
 *    "History edited — overwrite" row, positions survive a reload;
 * 6. Undo (chord, then button) → +80; Redo → +120; the undone/redone rows;
 * 7. a second session on dy → New alternative with a name → the alternative row and head, then an alternative switcher;
 * 8. fatal path: duplicate a node, drag the clone, withdraw the clone's `create-node` → review `blocked`, the drag row
 *    reads Error "Target missing", Finalize disabled; Next problem → Withdraw the drag → `ready`; Exit → zero trace;
 * 9. console: uncaught page errors fail the run; errors, warnings, hard faults and `[DEBUG] ` lines are digested.
 *
 * Run: `bun 🔍️time-travel-probe.ts --port=6012 [--only=1,2,3] [--locales=en,de] [--chords=de] [--explore]`.
 * `--chords=<locales>` drives Accept/Exit through the band chords (`alt+enter`, `alt+shift+backspace`) in those
 * locales and through the band buttons elsewhere. `--explore` boots each locale, opens the history panel, dumps the DOM
 * inventory and exits. Outputs: `🗑️generated/e2e/probe-<stamp>.md` (step lines, verdicts, notes, timeline, console
 * digest), `probe-<stamp>.ndjson` (one record per verdict/note/dump) and `probe-<stamp>-<locale>-s<step>-<tag>.png|json`.
 * @see ./📋️design.md
 * @see ./📓️w2-a-report.md
 * @see ./📓️w2-b-report.md
 * @see ../../☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts */
import { chromium, type BrowserContext, type ConsoleMessage, type Page } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🔖️Arguments
type Locale = "en" | "de";
type Positions = Record<string, [number, number]>;
type Point = { x: number; y: number };

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated", "e2e");
mkdirSync(OUT, { recursive: true });
const argument = (name: string) => process.argv.find((entry) => entry.startsWith(`--${name}=`))?.slice(name.length + 3);
const port = argument("port") ?? "6012";
const only = argument("only") ? new Set(argument("only")!.split(",").map((entry) => Number(entry.trim())).filter(Number.isFinite)) : null;
const locales = (argument("locales") ?? "en,de").split(",").map((entry) => entry.trim()).filter((entry): entry is Locale => entry === "en" || entry === "de");
const chordLocales = new Set((argument("chords") ?? "de").split(",").map((entry) => entry.trim()));
const explore = process.argv.includes("--explore");
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const base = `probe-${stamp}`;
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
    reviewBlocked: "Errors in later mutations block finalizing",
    dialogTitle: "Finish editing history",
    overwrite: "Overwrite",
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
    reviewBlocked: "Fehler in späteren Mutationen verhindern den Abschluss",
    dialogTitle: "Verlaufsbearbeitung abschließen",
    overwrite: "Überschreiben",
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

/** 🧭️ One `page.evaluate` that never takes the run down (a reload race answers `fallback`). */
const evalSafe = async <T, A = undefined>(fn: (arg: A) => T, fallback: T, arg?: A): Promise<T> => {
  try {
    return await page.evaluate(fn as (value: unknown) => T, arg as unknown);
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

//#region 🔖️Board
type Vitals = { surface: string; nodes: number; edges: number; handles: number; parsed: string; selection: string; camera: string; transform: string; utility: string };

/** 🩺️ The overview pane's `data-board-*` vitals, read without a guest round trip. */
const vitals = () =>
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

/** 📍️ Every node position the overview pane publishes (`data-board-positions-json`). */
const positions = async () => JSON.parse(await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-positions-json") ?? "{}", "{}", `window:${OVERVIEW}`)) as Positions;
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
const paneBox = async () => (await page.locator(`[data-surface-id="window:${OVERVIEW}"] canvas`).first().boundingBox().catch(() => null)) ?? { x: 0, y: 0, width: 1, height: 1 };

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

/** 🪟️ The dock panels that are open, by tab id. */
const openPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((el) => (el as HTMLElement).offsetParent !== null).map((el) => el.id.replace(/^framework\.panelTab\./, "")), [] as string[]);

/** 🪟️ Opens (never toggles) the panel tab `id`. */
const openTab = async (id: string) => {
  if ((await openPanelTabIds()).includes(id)) return true;
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  if (!(await tab.count().catch(() => 0))) return false;
  await tab.click({ timeout: 4000 }).catch(() => {});
  return (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
};

/** 🪟️ Closes every open dock panel (they overlay the board, and a pointer gesture there lands on the panel). */
const closePanels = async () => {
  for (const id of await openPanelTabIds()) {
    await page.locator(`[data-slot="panel-tab-button"][id="${id}"]`).first().click({ timeout: 3000 }).catch(() => {});
    await sleep(300);
  }
  return openPanelTabIds();
};

/** 🙈️ Dismisses a boot tour dialog by its Skip/Close button (en or de), scoped to dialogs only. */
const dismissTour = async () => {
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
    const shape = await evalSafe(() => ({ windows: document.querySelectorAll('[data-slot="window"]').length, canvases: document.querySelectorAll("canvas").length, dialogs: document.querySelectorAll('[role="dialog"]').length, body: document.body?.innerText.slice(0, 160) ?? "" }), { windows: 0, canvases: 0, dialogs: 0, body: "" });
    if (shape.dialogs) await dismissTour();
    if (shape.windows >= 3 && shape.canvases >= 3) {
      log(`${label} booted after ${((index + 1) * 3).toFixed(0)} s: ${JSON.stringify(shape).slice(0, 200)}`);
      await dismissTour();
      return true;
    }
    if (index % 5 === 4) log(`${label} waiting… ${JSON.stringify(shape).slice(0, 200)}`);
  }
  return false;
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

/** 🕰️ Every materialised history row (`framework.history.entry.<seq>` and `framework.history.mutation.<id>`) in DOM
 * order; a mutation row belongs to the entry row above it. */
const readHistory = () =>
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

/** 📜️ Scrolls the history panel's scroll container to its start or end (the Commands section is windowed). */
const scrollHistory = (where: "start" | "end") =>
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
  const body = await waitUntil(() => resolveDomId("framework.history.actions"), (id) => id !== null, 15000);
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

/** 🌳️ Expands one entry row through its disclosure button and waits for its mutation children. */
const expandEntry = async (entryId: string) => {
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
  const id = await resolveDomId(authored);
  if (!id) return { present: false, disabled: null as boolean | null };
  const disabled = await evalSafe((target) => {
    const el = document.getElementById(target);
    const button = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
    return button ? button.disabled : null;
  }, null as boolean | null, id);
  const target = page.locator(`[id="${id}"]`).first();
  const tag = await target.evaluate((el) => el.tagName.toLowerCase()).catch(() => "");
  await (tag === "button" ? target : target.locator("button").first()).click({ force: true, timeout: 4000 }).catch(() => target.click({ force: true, timeout: 4000 }).catch(() => {}));
  return { present: true, disabled };
};

/** 🧾️ The history-panel DOM inventory for diagnosis: every id under `framework.history`, with slot, role, state. */
const historyInventory = () =>
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

/** 📣️ The React time-travel band (`[data-semio-time-travel]`), or null when no session is live. */
const band = () =>
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
const bandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: { t: number; key: string }[] }).__probeBandTrace ?? []).slice(), [] as { t: number; key: string }[]);
const clearBandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: unknown[] }).__probeBandTrace = []).length, 0);

/** 🔘️ Presses one band control by its button, or by its chord in a chord locale (`accept`, `discard`, `exit`). */
const pressBand = async (control: string) => {
  const chords: Record<string, string> = { accept: "Alt+Enter", discard: "Alt+Backspace", exit: "Alt+Shift+Backspace" };
  if (chordLocales.has(currentLocale) && chords[control]) {
    await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
    await page.keyboard.press(chords[control]).catch(() => {});
    return `chord ${chords[control]}`;
  }
  await page.locator(`[data-semio-time-travel-control="${control}"]`).first().click({ timeout: 4000 }).catch((error) => log(`band ${control} click failed ${String(error).split("\n")[0]}`));
  return "button";
};

/** 🪟️ The finalize dialog (`finalizeHistoryEdit`): title, the destructive Overwrite choice, the name field, submit. */
const finalizeDialog = () =>
  evalSafe(
    () => {
      const dialog = Array.from(document.querySelectorAll('[role="dialog"]')).find((el) => el.querySelector('[id="ui.dialog.submit"]')) as HTMLElement | undefined;
      if (!dialog) return null;
      const button = (id: string) => {
        const el = dialog.querySelector(`[id="${id}"]`);
        const control = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
        return el ? { text: (el as HTMLElement).innerText.trim(), destructive: el.getAttribute("data-destructive") ?? control?.getAttribute("data-destructive") ?? null, tone: el.getAttribute("data-tone") ?? control?.getAttribute("data-tone") ?? null, disabled: control?.disabled ?? null } : null;
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
    null as null | { title: string; text: string; overwrite: { text: string; destructive: string | null; tone: string | null; disabled: boolean | null } | null; submit: { text: string; destructive: string | null; tone: string | null; disabled: boolean | null } | null; cancel: { text: string } | null; name: { id: string; value: string } | null },
  );

type Editor = { heading: string | null; status: string | null; dx: StepperRead | null; dy: StepperRead | null; targets: { id: string; chips: string[]; useSelection: boolean; text: string } | null; inputs: string[] };
type StepperRead = { id: string; tag: string; stepper: boolean; value: string | null; step: string | null; min: string | null; max: string | null; plus: boolean; minus: boolean; role: string | null };

/** ✏️ The draft editor section: its heading, the Rust band status line, the dx/dy controls and the targets list. */
const editor = () =>
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

/** ⌨️ Types `value` into the editor's number control at `pointer` (one `fill` = one input event = one draft), then
 * Enter to commit and blur; answers the value the field held before Enter. */
const typeEditorNumber = async (pointer: string, value: number) => {
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
};

/** 🧭️ Opens the drag's entry row and presses Edit on its `drag-selection` mutation; answers the band once `editing`. */
const beginDragEdit = async (ctx: Ctx) => {
  await openHistory();
  const rows = await allHistoryRows();
  const entry = rows.find((row) => row.id === ctx.dragEntry) ?? rows.find((row) => row.kind === "entry" && row.key === ctx.dragEntry?.split(".").pop());
  if (!entry) return { entry: null, via: "absent", band: null as Band | null, rows };
  const expanded = await expandEntry(entry.id);
  const mutation = expanded.mutations.find((row) => row.key === ctx.dragMutation) ?? expanded.mutations[0];
  if (!mutation) return { entry, via: "no-mutation-row", band: null as Band | null, rows };
  const via = await pressRowAction(mutation.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  return { entry, mutation, via, band: settled.value, waitedMs: settled.waitedMs, rows };
};

/** 🥾️ Step 1 — the board is parsed and populated, and the history panel speaks the locale under test. */
const step1 = async (_ctx: Ctx) => {
  const parsed = await waitUntil(vitals, (v) => v?.parsed === "true", 60000);
  verdict("fixture-parsed", parsed.ok, { parsed: parsed.value?.parsed, waitedMs: parsed.waitedMs });
  const populated = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
  const p = await positions();
  verdict("board-has-nodes", (populated.value?.nodes ?? 0) > 0 && Object.keys(p).length > 0, { nodes: populated.value?.nodes, edges: populated.value?.edges, positions: Object.keys(p).length, camera: populated.value?.camera });
  const opened = await openHistory();
  const commands = await resolveDomId("framework.history.commands");
  const commandsText = commands ? await byId(commands).innerText().catch(() => "") : "";
  verdict("history-panel-speaks-the-locale", opened && commandsText.includes(COPY[currentLocale].commands), { opened, commands, commandsText: commandsText.slice(0, 80), expected: COPY[currentLocale].commands, lang: await evalSafe(() => navigator.language, "") });
  emit({ kind: "inventory", tag: "boot-history", rows: await historyInventory() });
  await shot("boot");
  await closePanels();
};

/** ✋️ Step 2 — select two nodes, drag them by (+80,+40) world units, one new row; then a downstream drag of a third node. */
const step2 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const rowsBefore = documentEntries(await allHistoryRows());
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
  const grew = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => !rowsBefore.some((before) => before.id === row.id)), (rows) => rows.length >= 1, 30000, 1000);
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
  const cRows = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((entry) => !rowsBefore.some((before) => before.id === entry.id) && entry.id !== ctx.dragEntry), (rows) => rows.length >= 1, 30000, 1000);
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
  const indicator = await evalSafe(() => Array.from(document.querySelectorAll("[data-semio-time-travel-indicator]")).map((el) => `${el.getAttribute("data-semio-time-travel-indicator")}:${el.getAttribute("aria-label")}`), [] as string[]);
  verdict("windows-wear-the-time-travel-indicator", indicator.length > 0, { indicator: indicator.slice(0, 4) });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], ctx.dx!, ctx.dy!) && placed(p, ctx.c!, ctx.p0![ctx.c!], 0, 0), 15000);
  verdict("preview-is-state-before-target-plus-draft", placed(preview.value, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(preview.value, ctx.b, ctx.p0[ctx.b], ctx.dx!, ctx.dy!), {
    a: preview.value[ctx.a],
    aBefore: ctx.p0[ctx.a],
    draft: [ctx.dx, ctx.dy],
    design: "§4: the preview is the document as of the edited mutation with the DRAFT applied (Begin drafts the original input), so the dragged nodes read pre-drag + draft",
  });
  verdict("downstream-not-applied-while-editing", placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { c: preview.value[ctx.c], cBeforeItsDrag: ctx.p0[ctx.c], cHead: ctx.pc[ctx.c], waitedMs: preview.waitedMs });
  const rows = await readHistory();
  const cRow = rows.find((row) => row.kind === "mutation" && row.key === ctx.cMutation);
  verdict("downstream-row-reads-not-applied", Boolean(cRow?.text.includes(COPY[currentLocale].pending)), { cRow: cRow?.text?.slice(0, 160), expected: COPY[currentLocale].pending });
  const e = await waitUntil(editor, (value) => Boolean(value?.dx && value?.dy && value?.targets), 15000);
  const ed = e.value;
  verdict("editor-shows-dx-dy-steppers-with-grid-snap-step", Boolean(ed?.dx?.stepper && ed?.dy?.stepper && ed.dx.step === "1" && ed.dy.step === "1" && ed.dx.plus && ed.dx.minus), { dx: ed?.dx, dy: ed?.dy, note: "snapSource {config: gridFactor} resolves to the grid factor (default 1) as the stepper step" });
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
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.b!, ctx.p0![ctx.b!], 120, ctx.dy!), 20000);
  verdict("dx-120-updates-the-preview", preview.ok && placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { typed, a: preview.value[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dy!], c: preview.value[ctx.c], waitedMs: preview.waitedMs });
  await keyEditorNumber("dx", "ArrowUp");
  const up = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 121, ctx.dy!), 15000);
  await keyEditorNumber("dx", "ArrowDown");
  const down = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!), 15000);
  verdict("arrow-keys-step-dx-by-the-snap-step", up.ok && down.ok, { up: up.value[ctx.a], down: down.value[ctx.a], dxField: (await editor())?.dx?.value });
  const accepted = (await editor())?.dx?.value;
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
  verdict("dialog-offers-destructive-overwrite", Boolean(d?.overwrite && d.overwrite.text.includes(copy.overwrite) && d.overwrite.destructive === "true"), { overwrite: d?.overwrite });
  verdict("dialog-offers-new-alternative-with-a-name-field", Boolean(d?.submit && d.submit.text.includes(copy.newAlternative) && d.name && d.name.value === copy.defaultName), { submit: d?.submit, name: d?.name });
  verdict("band-reads-choosing-while-the-dialog-is-open", (await band())?.stage === "choosing", { band: (await band())?.stage });
  await shot("dialog");
  await page.locator('[id="ui.dialog.choice.overwrite"]').first().click({ timeout: 4000 }).catch((error) => log(`overwrite click failed ${String(error).split("\n")[0]}`));
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict("overwrite-closes-the-session", gone.ok && (await finalizeDialog()) === null, { waitedMs: gone.waitedMs, band: gone.value?.stage ?? null });
  const expected = copy.row("edit", null, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("overwrite-row-appears", rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  ctx.afterOverwrite = await positions();
  verdict("head-keeps-plus-120-after-overwrite", placed(ctx.afterOverwrite, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.c, ctx.pc![ctx.c], 0, 0), { a: ctx.afterOverwrite[ctx.a], c: ctx.afterOverwrite[ctx.c] });
  await shot("overwritten");
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebooted = await waitForBoot("reload", 60);
  await installBandTrace();
  const reloaded = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!), 60000, 1000);
  const drift = movedIds(ctx.afterOverwrite, reloaded.value, 1e-6);
  verdict("positions-persist-after-reload", rebooted && reloaded.ok && drift.length === 0, { rebooted, a: reloaded.value[ctx.a], expected: ctx.afterOverwrite[ctx.a], drift: drift.slice(0, 8), driftCount: drift.length, waitedMs: reloaded.waitedMs });
  await shot("reloaded");
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
    await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
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
  verdict("overwrite-row-survives-the-reload", labels.some((label) => label.startsWith(copy.row("edit", null, 1))), { labels: labels.slice(-8) });
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
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  verdict("second-session-opens", begun.band?.stage === "editing", { via: begun.via, stage: begun.band?.stage });
  const ed = (await waitUntil(editor, (value) => Boolean(value?.dx && value?.dy), 15000)).value;
  verdict("editor-reads-the-effective-input", Boolean(ed?.dx && near(Number(ed.dx.value), 120, 0.005)), { dx: ed?.dx?.value, dy: ed?.dy?.value, note: "after the redo the drag's effective input is the overwritten dx=120" });
  ctx.dyEdited = Math.round(ctx.dy! + 30);
  const typed = await typeEditorNumber("dy", ctx.dyEdited);
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dyEdited!), 20000);
  verdict("dy-edit-updates-the-preview", preview.ok, { typed, a: preview.value[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
  await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("second-review-ready", review.value?.review === "ready", { review: review.value?.review, text: review.value?.text?.slice(0, 160) });
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  ctx.alternative = `probe ${currentLocale} ${stamp.slice(11, 19)}`;
  const name = page.locator('[role="dialog"] input[id="name"], [role="dialog"] input[type="text"]').first();
  await name.fill(ctx.alternative, { timeout: 4000 }).catch(() => {});
  const nameValue = await name.inputValue({ timeout: 2000 }).catch(() => null);
  verdict("name-field-takes-the-alternative-name", dialog.ok && nameValue === ctx.alternative, { nameValue, dialog: dialog.value?.submit });
  await page.locator('[id="ui.dialog.submit"]').first().click({ timeout: 4000 }).catch((error) => log(`submit click failed ${String(error).split("\n")[0]}`));
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict("new-alternative-closes-the-session", gone.ok, { band: gone.value?.stage ?? null, fault: gone.value?.fault ?? null });
  const expected = COPY[currentLocale].row("edit", ctx.alternative, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("alternative-row-appears", rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dyEdited!), 15000);
  verdict("head-is-on-the-edited-alternative", head.ok, { a: head.value[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
  await shot("alternative");
  const switchers = await evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id], [aria-label], [title]"))
        .filter((el) => /alternative|switchAlternative/i.test(`${el.id} ${el.getAttribute("aria-label") ?? ""} ${el.getAttribute("title") ?? ""}`) && !/createAlternative|ui\.dialog/.test(el.id))
        .map((el) => `${el.tagName.toLowerCase()}#${el.id}[${el.getAttribute("role") ?? el.getAttribute("data-slot") ?? ""}] ${(el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 60) ?? ""}`)
        .slice(0, 20),
    [] as string[],
  );
  const select = page.locator('select, [role="combobox"]').filter({ hasText: ctx.alternative }).first();
  if (await select.count().catch(() => 0)) {
    await select.click({ timeout: 3000 }).catch(() => {});
    const original = page.locator('[role="option"]').filter({ hasNotText: ctx.alternative }).first();
    await original.click({ timeout: 3000 }).catch(() => {});
    const back = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!), 20000);
    verdict("switching-to-the-original-alternative-shows-original-positions", back.ok, { a: back.value[ctx.a], switchers });
  } else {
    verdict("alternative-switcher-present", false, {
      switchers,
      finding: "no React control switches alternatives: the Rust history body renders no alternatives list, and `switchAlternative` declares no `alternativeId` argument, so neither the palette nor a dialog can dispatch it (only the tutorial driver does, `🏛️ShellHost/🟦️.tsx` applyTutorialSliceToShell)",
    });
  }
  await closePanels();
};

/** 💥️ Step 8 — duplicate a node, drag the clone, withdraw the clone's `create-node` → blocked review with an Error row,
 * Finalize disabled; Next problem → withdraw the drag → ready; Exit → zero trace. */
const step8 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(1, [[24 + 70, 24 + 70]], exclude, 0);
  const d = picked[0];
  if (!d) {
    verdict("clickable-node-for-the-fatal-path", false, {});
    return;
  }
  const rowsBefore = documentEntries(await allHistoryRows());
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(d.at.x, d.at.y);
  const selected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === d.id, 15000);
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  const grew = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  verdict("duplicate-adds-one-clone", selected.ok && grew.ok && Boolean(clone), { d: d.id, clone, nodes: [nodesBefore, grew.value?.nodes] });
  if (!clone) return;
  const reselected = await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  const grab = { x: cloneAt.x + 4, y: cloneAt.y + 4 };
  await dragBy(grab, 70 * camera.zoom, 70 * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const cloneOffset = offsetOf(beforeDrag, dragged.value, clone);
  verdict("clone-drag-moves-only-the-clone", dragged.ok && movedIds(beforeDrag, dragged.value).every((id) => id === clone), { reselected: reselected.ok, cloneOffset, moved: movedIds(beforeDrag, dragged.value).slice(0, 4) });
  const head = dragged.value;
  const added = (await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => !rowsBefore.some((before) => before.id === row.id)), (rows) => rows.length >= 2, 30000, 1000)).value;
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
  const preview = await positions();
  verdict("editing-the-create-node-previews-the-clone-before-its-drag", editing.ok && Boolean(cloneOffset && placed(preview, clone, beforeDrag[clone], 0, 0)), { via, clone: preview[clone], beforeDrag: beforeDrag[clone] });
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  const withdrawn = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("withdraw-previews-the-document-without-the-clone", withdraw.present && withdrawn.ok, { withdraw, hasClone: Boolean(withdrawn.value[clone]) });
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
    const next = await pressAuthored("framework.history.timeTravel.nextProblem");
    const reopened = await waitUntil(band, (b) => b?.stage === "editing", 15000);
    const heading = (await editor())?.heading;
    const pulled = await pressAuthored("framework.history.editor.withdraw");
    await sleep(500);
    await pressBand("accept");
    const settled = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
    rounds.push({ round, next, reopened: reopened.ok, heading, pulled, review: settled.value?.review });
  }
  const ready = await band();
  verdict("withdrawing-the-failing-drag-makes-the-review-ready", ready?.review === "ready", { rounds, review: ready?.review, text: ready?.text?.slice(0, 160) });
  await shot("resolved");
  await pressBand("exit");
  const gone = await waitUntil(band, (b) => b === null, 20000);
  verdict("exit-closes-the-session", gone.ok, { band: gone.value?.stage ?? null });
  const after = await waitUntil(positions, (p) => movedIds(head, p).length === 0, 15000);
  verdict("exit-leaves-head-positions-unchanged", after.ok, { drift: movedIds(head, after.value).slice(0, 8) });
  const rowsPost = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  verdict("exit-leaves-no-new-rows", JSON.stringify(rowsPost) === JSON.stringify(rowsPre), { added: rowsPost.filter((row) => !rowsPre.includes(row)), removed: rowsPre.filter((row) => !rowsPost.includes(row)) });
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
  const top = (rows: ConsoleRow[]) => [...rows.reduce((map, row) => map.set(row.text.slice(0, 160), (map.get(row.text.slice(0, 160)) ?? 0) + 1), new Map<string, number>())].sort((x, y) => y[1] - x[1]).slice(0, 12);
  const uncaught = pageErrors.filter((row) => row.locale === currentLocale);
  verdict("no-uncaught-page-errors", uncaught.length === 0, { count: uncaught.length, first: uncaught.slice(0, 3) });
  const hard = hardFaults.filter((row) => row.locale === currentLocale);
  verdict("no-hard-guest-faults", hard.length === 0, { count: hard.length, first: hard.slice(0, 3) });
  note("console-digest", { lines: mine.length, errors: errors.length, warnings: warnings.length, debug: debug.length, topErrors: top(errors), topWarnings: top(warnings), topDebug: top(debug) });
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
  listen(page);
  mod = (await page.evaluate(() => navigator.platform).catch(() => "MacIntel")).includes("Mac") ? "Meta" : "Control";
  log(`navigating to :${port} locale=${locale} mod=${mod}`);
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
  if (explore) {
    await openHistory();
    dumpJson("explore", { inventory: await historyInventory(), rows: await readHistory(), band: await band(), vitals: await vitals(), tabs: await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((el) => `${el.id}:${(el as HTMLElement).innerText.trim()}`), [] as string[]) });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  const ctx: Ctx = {};
  for (const step of [1, 2, 3, 4, 5, 6, 7, 8, 9]) {
    if (only && !only.has(step)) continue;
    currentStep = step;
    log(`step ${step} begins`);
    try {
      await STEPS[step](ctx);
    } catch (error) {
      verdict("step-threw", false, { error: String(error).slice(0, 400) });
    }
    const results = verdicts.filter((row) => row.locale === locale && row.step === step);
    if (results.some((row) => !row.ok)) dumpJson("failure", { inventory: await historyInventory(), band: await band(), editor: await editor(), dialog: await finalizeDialog(), vitals: await vitals(), console: consoleRows.filter((row) => row.locale === locale && row.step === step).slice(-60) });
    await shot("end");
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
const summary = `time-travel probe PASS=${passed} FAIL=${verdicts.length - passed} uncaught=${pageErrors.length} hard=${hardFaults.length} locales=${locales.join(",")} only=${only ? [...only].join(",") : "all"}`;
console.log(summary);
for (const line of stepLines) console.log(line);
emit({ kind: "summary", summary, steps: stepLines });
writeFileSync(
  join(OUT, `${base}.md`),
  [
    `# ${base} (port ${port}, locales ${locales.join(",")}, chords ${[...chordLocales].join(",")}, only ${only ? [...only].join(",") : "all"})`,
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
    ...consoleRows.filter((row) => row.type === "error" || row.type === "warning" || row.type === "pageerror").slice(-200).map((row) => `${row.t}s ${row.locale}/${row.step} ${row.source} ${row.type}: ${row.text.slice(0, 400)}`),
  ].join("\n"),
);
process.exit(verdicts.length - passed === 0 && pageErrors.length === 0 ? 0 : 1);
//#endregion 🔖️Main
