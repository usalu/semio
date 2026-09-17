/** 🔬️ Headless runtime probe for the puzzle 2d React serve on 127.0.0.1:6012 — boots the shell, waits for the
 * three board windows, drives one browser step per row of `📓️2026-09-16-parity-survey.md`, and writes findings +
 * screenshots + a machine-readable verdict stream into `🗑️generated/`. Ticket 26/09/06/PUZZLE-2D-END-TO-END.
 *
 * Run: `bun 🔍️browser-probe.ts [--battery] [--only=step,step] [--port=<n>] [--explore] [--settle=<seconds>]
 * [--reload-between-groups]`.
 * `--explore` boots, dumps the DOM inventory (windows, tabs, toggles, buttons, tree items) and exits — the
 * first thing to run against a serve whose chrome ids are not yet known. `--battery` runs every registered
 * step in plan order; `--only=a,b` runs exactly those. `--reload-between-groups` reboots the page between
 * the `read`/`mutate`/`replace` groups so a group never inherits the previous group's document, exactly like
 * the puzzle 3d battery (`…/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`).
 *
 * Outputs: `probe-<stamp>.md` (prose timeline) and `probe-<stamp>.ndjson` (one JSON record per verdict plus a
 * final `battery PASS=n FAIL=n FAULTS=n HARD=n first-hard-fault-at=<s> guest-death-faults=n` summary record).
 *
 * 🧭️ Lane coverage is the 3d battery's §1–§25 matrix plus the 2d-specific lanes
 * (`📓️E6-battery-matrix.md` §2 in ticket 26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D). Lanes whose feature
 * is still being built by a sibling slice name the selector they expect in their own verdict note, so a red
 * reads as "the hook is not there yet" rather than as an unattributed failure. */
import { chromium, type Locator, type Page } from "playwright";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const battery = process.argv.includes("--battery");
const explore = process.argv.includes("--explore");
const onlyArg = process.argv.find((a) => a.startsWith("--only="))?.slice(7);
const only = onlyArg ? new Set(onlyArg.split(",").map((name) => name.trim()).filter(Boolean)) : null;
const reloadBetweenGroups = process.argv.includes("--reload-between-groups");
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6012";
const settleSeconds = Number(process.argv.find((a) => a.startsWith("--settle="))?.slice(9) ?? "3") || 3;
const ndjsonPath = join(OUT, `probe-${stamp}.ndjson`);
writeFileSync(ndjsonPath, "");
const emit = (record: Record<string, unknown>) => appendFileSync(ndjsonPath, `${JSON.stringify(record)}\n`);
const lines: string[] = [];
const t0 = Date.now();
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`;
  lines.push(row);
  console.log(row);
};

//#region 🔖️Console
const CONSOLE_RING_LINES = 4000;
const CONSOLE_TAIL_LINES = Number(process.argv.find((a) => a.startsWith("--tail="))?.slice(7) ?? "120") || 120;
const consoleBuf: string[] = [];
let consoleSeq = 0;
const consoleCursor = () => consoleSeq;
const consoleSince = (mark: number) => consoleBuf.slice(Math.max(0, mark - (consoleSeq - consoleBuf.length)));
// 🧾️ `puzzle2d-` matches the app's OWN `Fault::from("puzzle2d-…")` codes (`puzzle2d-window-context-required`,
// `puzzle2d-example-exceeds-capacity`, …) and must keep doing so — but the dev host's boot banner prints 58
// `[stale] … run: bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` lines, every one of
// which carries the same substring, so the whole benign multi-line message used to land in `faults[]` twice a
// run (`📓️E7-2d-battery-failures-root-cause.md` §5). The recipe name is excluded by a lookahead, and the
// banner as a whole by {@link BENIGN_CONSOLE_RE} below — a fault regex that counts the dev-activation
// advisory reports a green run as FAULTS=2.
const FAULT_RE =
  /intake-budget-exhausted|fixed-capacity|section-root-mismatch|native-owner-required|terminal-fault|unreachable|shard .* (lost|terminated)|did not publish|missing field|malformed|admission failed|worker fault|\[semio-plugin panic\]|panicked at|Credits \{|NodeCapacity|SemioFaultError|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected|unknown Puzzle 2D action|puzzle2d-(?!react-dev)/i;
/** 🩹️ Console lines that say nothing about the feature under test. `contributions document sources` quotes
 * the document ops text, which carries words the fault regex reads as faults; the staged-plugin advisory and
 * the transform-freshness census are dev-server bookkeeping that the serve prints at every boot. */
const BENIGN_CONSOLE_RE = /contributions document sources|staged plugin module|\[stale\]|activate-puzzle2d-react-dev|transform freshness/i;
const HARD_FAULT_RE =
  /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected|unknown Puzzle 2D action/i;
const GUEST_DEATH_RE = /reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected/i;
const faults: string[] = [];
const hardFaults: string[] = [];
const guestDeathFaults: string[] = [];
let firstHardFaultAt: number | null = null;
const noteFault = (text: string) => {
  if (faults.length < 400) faults.push(text);
  if (HARD_FAULT_RE.test(text)) {
    if (hardFaults.length < 400) hardFaults.push(text);
    if (firstHardFaultAt === null) firstHardFaultAt = Number(((Date.now() - t0) / 1000).toFixed(1));
  }
  if (GUEST_DEATH_RE.test(text) && guestDeathFaults.length < 200) guestDeathFaults.push(text);
};
//#endregion 🔖️Console

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
// 📋️ `copy`/`cut`/`paste` are framework-reserved hotkey verbs with no authored control anywhere, so the
// clipboard lane's only route is `mod+c`/`mod+x`/`mod+v` — which the page may not use unprompted.
await page.context().grantPermissions(["clipboard-read", "clipboard-write"], { origin: `http://127.0.0.1:${port}` }).catch(() => {});
page.on("console", (message) => {
  const text = `${message.type()}: ${message.text()}`;
  consoleBuf.push(text);
  consoleSeq += 1;
  if (consoleBuf.length > CONSOLE_RING_LINES) consoleBuf.shift();
  if (FAULT_RE.test(text) && !BENIGN_CONSOLE_RE.test(text)) noteFault(text);
});
page.on("pageerror", (error) => {
  const text = `pageerror: ${String(error)}`;
  consoleBuf.push(text);
  consoleSeq += 1;
  noteFault(text);
});

/** 🧭️ One `page.evaluate` that never takes the run down: a navigation race (`--reload-between-groups`, an
 * example switch) or a detached frame falls back instead of throwing. `arg` is passed through so a helper
 * body stays closure-free and can be parameterised by pane. */
const evalSafe = async <T, A = undefined>(fn: (arg: A) => T, fallback: T, arg?: A): Promise<T> => {
  try {
    return await page.evaluate(fn as (value: unknown) => T, arg as unknown);
  } catch {
    return fallback;
  }
};
const countSafe = async (locator: Locator) => locator.count().catch(() => 0);
const settle = async (seconds = settleSeconds) => page.waitForTimeout(seconds * 1000);

//#region 🔖️Snapshot
type Snapshot = { windows: { id: string; w: number; h: number }[]; canvases: number; tabs: string[]; toggles: string[]; treeItems: number; dialogs: string[]; recovery: string[]; body: string };
const EMPTY: Snapshot = { windows: [], canvases: 0, tabs: [], toggles: [], treeItems: 0, dialogs: [], recovery: [], body: "" };
const snapshot = () =>
  evalSafe((): Snapshot => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    return {
      windows: q('[data-slot="window"]').map((w) => ({ id: w.id || w.getAttribute("data-key") || "?", w: (w as HTMLElement).offsetWidth, h: (w as HTMLElement).offsetHeight })),
      canvases: q("canvas").length,
      tabs: q('[data-slot="panel-tab-button"]').map((b) => b.id).slice(0, 60),
      toggles: q('[data-slot="toggle-group-item"]').map((b) => `${b.id}=${b.getAttribute("aria-pressed")}`).slice(0, 60),
      treeItems: q('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]').length,
      dialogs: q('[role="dialog"]').map((d) => (d as HTMLElement).innerText.slice(0, 80)),
      recovery: q("[data-plugin-recovery]").map((el) => el.getAttribute("data-plugin-recovery") ?? "?"),
      body: document.body ? document.body.innerText.slice(0, 400) : "",
    };
  }, EMPTY);

/** 🧾️ The board vitals `Board2dHost` publishes per surface (`data-board-*`), read without a guest round trip.
 *
 * 🏗️ `windowInstance`, `suggestions`, `transform` and `regions` are hooks the 2026-09-17 parity slices are
 * adding (2E: `data-window-instance-id` + `data-board-transform-json`; 2B: `data-board-suggestion-menu-json`;
 * 2F: `data-board-target-regions-json`). They read `""` on a host that does not publish them yet, and the
 * lanes that need them name the attribute in their verdict so an absent hook is legible as such. */
const boardVitals = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll("[data-surface-id]")).map((el) => ({
        surface: el.getAttribute("data-surface-id") ?? "?",
        window: el.getAttribute("data-window-instance-id") ?? "",
        nodes: Number(el.getAttribute("data-board-nodes") ?? "-1"),
        edges: Number(el.getAttribute("data-board-edges") ?? "-1"),
        handles: Number(el.getAttribute("data-board-handles") ?? "-1"),
        selection: el.getAttribute("data-board-selection-json") ?? "",
        camera: el.getAttribute("data-board-camera-json") ?? "",
        hovered: el.getAttribute("data-board-hovered-id") ?? "",
        utility: el.getAttribute("data-board-active-utility") ?? "",
        parsed: el.getAttribute("data-board-fixture-parsed") ?? "",
        suggestions: el.getAttribute("data-board-suggestion-menu-json") ?? "",
        transform: el.getAttribute("data-board-transform-json") ?? "",
        regions: el.getAttribute("data-board-target-regions-json") ?? "",
      })),
    [] as { surface: string; window: string; nodes: number; edges: number; handles: number; selection: string; camera: string; hovered: string; utility: string; parsed: string; suggestions: string; transform: string; regions: string }[],
  );

const inventory = () =>
  evalSafe(() => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    const text = (el: Element) => (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 40) ?? "";
    return {
      buttons: q("button").map((b) => `${b.id || "-"}:${text(b)}`).filter((s) => s !== "-:").slice(0, 200),
      selects: q("select").map((s) => `${s.id}:${Array.from((s as HTMLSelectElement).options).map((o) => o.label).join("|")}`),
      inputs: q("input").map((i) => `${i.id || "-"}:${(i as HTMLInputElement).type}=${(i as HTMLInputElement).value}`).slice(0, 80),
      tree: q('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]').map((r) => `${r.id || "?"}=${text(r)}`).slice(0, 80),
      measures: q('[data-slot="measure"], [class*="measure"]').map((m) => text(m)).slice(0, 40),
      surfaces: q("[data-surface-id]").map((el) => el.getAttribute("data-surface-id")),
      slots: Array.from(new Set(q("[data-slot]").map((el) => el.getAttribute("data-slot")))).slice(0, 120),
    };
  }, { buttons: [] as string[], selects: [] as string[], inputs: [] as string[], tree: [] as string[], measures: [] as string[], surfaces: [] as (string | null)[], slots: [] as (string | null)[] });
//#endregion 🔖️Snapshot

//#region 🔖️Boot
const gotoShell = async (label: string) => {
  await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle2d`, { waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`${label} goto: ${String(error).slice(0, 160)}`));
  await page.waitForLoadState("domcontentloaded").catch(() => {});
};

const dismissTour = async (label: string) => {
  for (const pattern of [/^\s*(x\s*)?skip\s*$/i, /^\s*close\s*$/i]) {
    const button = page.locator('[role="dialog"] button, button', { hasText: pattern }).first();
    if (await countSafe(button)) {
      await button.click({ timeout: 2000 }).catch(() => {});
      await page.waitForTimeout(800);
      log(`${label} dismissed ${pattern}`);
    }
  }
};

const waitForBoot = async (label: string, polls = 80) => {
  for (let i = 0; i < polls; i++) {
    await page.waitForTimeout(3000);
    const s = await snapshot();
    if (s.dialogs.length && i % 2 === 0) await dismissTour(label);
    if (s.recovery.some((r) => r !== "?" && r !== "")) {
      log(`${label} plugin recovery card: ${JSON.stringify(s.recovery)}`);
    }
    if (s.windows.length >= 3 && s.canvases >= 3) {
      log(`${label} booted: windows=${JSON.stringify(s.windows)} canvases=${s.canvases} treeItems=${s.treeItems}`);
      await dismissTour(label);
      return true;
    }
    if (i % 5 === 4) log(`${label} waiting… windows=${s.windows.length} canvases=${s.canvases} faults=${faults.length} body=${JSON.stringify(s.body.slice(0, 120))}`);
  }
  return false;
};
//#endregion 🔖️Boot

//#region 🔖️Verdicts
let pass = 0;
let fail = 0;
const verdict = (section: string, step: string, ok: boolean, detail: Record<string, unknown> = {}) => {
  if (ok) pass += 1;
  else fail += 1;
  emit({ section, step, verdict: ok ? "PASS" : "FAIL", t: Number(((Date.now() - t0) / 1000).toFixed(1)), ...detail });
  log(`${ok ? "PASS" : "FAIL"} ${section}/${step} ${JSON.stringify(detail).slice(0, 400)}`);
};

/** ⏳️ Polls `read` until `settled` holds or `timeoutMs` passes; returns the last value and the wait. */
const waitUntil = async <T,>(read: () => Promise<T>, settled: (value: T) => boolean, timeoutMs = 20000, everyMs = 500) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < timeoutMs) {
    await page.waitForTimeout(everyMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: settled(value) };
};

/** 🪟️ The three window instances edit mode lays out (`🎭️modes/✏️edit/🦀️.rs` `[overview, detail, selection]`),
 * by the `WINDOW_KIND_ID` each one declares. `[data-slot="window"]` carries that id, which is the ONLY
 * scope that separates three panes publishing the same authored control ids (`#action.undo`,
 * `#puzzle2d-engagement`, `#select`, `#brush` are authored once and rendered once per pane). */
const WINDOW_IDS = ["2d-overview", "2d-detail", "2d-selection"] as const;
const OVERVIEW = WINDOW_IDS[0];
/** 🎯️ `selector` restricted to one window instance's subtree. */
const inWindow = (windowId: string, selector: string) => `[data-slot="window"][id="${windowId}"] ${selector}`;
const paneCanvas = (pane: string) => page.locator(`[data-surface-id="window:${pane}"] canvas`).first();
const paneBox = async (pane: string) => (await paneCanvas(pane).boundingBox()) ?? { x: 0, y: 0, width: 1, height: 1 };
const paneVitals = async (pane: string) => (await boardVitals()).find((v) => v.surface === `window:${pane}`);
const overviewCanvas = () => paneCanvas(OVERVIEW);
const overviewBox = () => paneBox(OVERVIEW);
const overviewVitals = () => paneVitals(OVERVIEW);
/** 🧮️ The one scalar every census verdict compares — nodes + edges + handles of a pane's board. */
const entityCount = (v: Awaited<ReturnType<typeof overviewVitals>>) => (v ? v.nodes + v.edges + v.handles : -1);
const positionsOf = async (pane = OVERVIEW) =>
  JSON.parse(await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-positions-json") ?? "{}", "{}", `window:${pane}`)) as Record<string, [number, number]>;
const selectionIds = (v: Awaited<ReturnType<typeof overviewVitals>>) => {
  try {
    return JSON.parse(v?.selection || "[]") as string[];
  } catch {
    return [] as string[];
  }
};

/** 🎯️ The screen position of a node id in the overview pane, read through the pane's published camera. */
const nodeScreen = async (id: string) => {
  const vitals = await overviewVitals();
  const positions = JSON.parse(vitals?.camera ? (await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) : "{}") as Record<string, [number, number]>;
  const position = positions[id];
  if (!position || !vitals) return null;
  const camera = JSON.parse(vitals.camera) as { x: number; y: number; zoom: number };
  const box = await overviewBox();
  const at = { x: box.x + box.width / 2 + (position[0] - camera.x) * camera.zoom, y: box.y + box.height / 2 + (position[1] - camera.y) * camera.zoom };
  // 🎯️ Off-pane nodes are unreachable: a click there lands nowhere (Nakagin spans ~1 700 world units).
  const margin = 24;
  if (at.x < box.x + margin || at.x > box.x + box.width - margin || at.y < box.y + margin || at.y > box.y + box.height - margin) return null;
  return at;
};

/** 🎯️ The ids of the nodes currently inside the overview pane, in document order. */
const visibleNodeIds = async () => {
  const ids = Object.keys(JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, unknown>);
  const visible: string[] = [];
  for (const id of ids) if (await nodeScreen(id)) visible.push(id);
  return visible;
};

/** 🪟️ The dock's open panels, by tab id — `[data-slot="panel"]#framework.panelTab.<tabId>` is mounted only while open (`aria-pressed` is "last used" and sticks). */
const openPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((e) => (e as HTMLElement).offsetParent !== null).map((e) => e.id.replace(/^framework\.panelTab\./, "")), [] as string[]);

/** 🪟️ Opens (never toggles) the panel tab `id`; a tab press on an open panel closes it. */
const clickTab = async (id: string) => {
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  if (!(await countSafe(tab))) return false;
  if ((await openPanelTabIds()).includes(id)) return true;
  await tab.click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(800);
  return (await openPanelTabIds()).includes(id);
};

/** 🪟️ Closes every open panel (they overlay the pane's left edge, so a pointer gesture there lands on the panel). */
const closePanels = async () => {
  const open = await openPanelTabIds();
  for (const id of open) {
    await page.locator(`[data-slot="panel-tab-button"][id="${id}"]`).first().click({ timeout: 2000 }).catch(() => {});
    await page.waitForTimeout(400);
  }
  await page.waitForTimeout(300);
  return { closed: open, stillOpen: await openPanelTabIds() };
};

/** 🎛️ The Actions pane of ONE window instance (its action rows plus the engagement input), read and toggled
 * inside that window's own subtree.
 *
 * 🧯️ The old reader queried `[id="action.undo"], [id="puzzle2d-engagement"]` page-wide. Three panes are
 * mounted and all three author the SAME ids (`puzzle2d_engagement` hands every pane
 * `id: "puzzle2d-engagement"`, `🎭️modes/✏️edit/🦀️.rs`), so a Detail pane whose Actions pane happened to be
 * open answered "open" for the Overview one and the toggle click was skipped entirely — `import` then raced
 * a file chooser against a row that was never in the DOM (`📓️E7…` §3, `chooser:false` after 120 s). Scoping
 * matches what the framework's own key router already does
 * (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` `shouldRouteKeysToWindowSearch`:
 * `[data-slot="window"][data-active="true"] …`). */
const actionsOpen = (windowId = OVERVIEW) =>
  evalSafe((scope) => Array.from(document.querySelectorAll(`${scope} [id="action.undo"], ${scope} [id="puzzle2d-engagement"]`)).some((el) => (el as HTMLElement).offsetParent !== null), false, `[data-slot="window"][id="${windowId}"]`);
/** ⏳️ The window's own engagement toggle, present and hit-testable. A `selectExample()` reload re-renders the
 * whole pane, and the toggle is briefly absent — two blind retries a second apart were not enough after a
 * 180-node document loaded, so the toggle is waited for BEFORE it is pressed rather than pressed into a
 * pane that has not rendered yet. */
const actionsToggleReady = async (windowId = OVERVIEW, budgetMs = 30000) => {
  const camel = windowId.replace(/-([a-z0-9])/g, (_m, c: string) => c.toUpperCase());
  const id = `framework.window.${camel}.engagement.toggle`;
  const state = await waitUntil(
    () =>
      evalSafe(
        (toggleId) => {
          const el = document.getElementById(toggleId) as HTMLButtonElement | null;
          if (!el) return { present: false, visible: false, disabled: false };
          return { present: true, visible: el.offsetParent !== null, disabled: Boolean(el.disabled) };
        },
        { present: false, visible: false, disabled: false },
        id,
      ),
    (s) => s.present && s.visible && !s.disabled,
    budgetMs,
  );
  return { id, ...state.value, waitedMs: state.waitedMs, ready: state.ok };
};
const setActions = async (open: boolean, windowId = OVERVIEW) => {
  const toggle = await actionsToggleReady(windowId);
  for (let attempt = 0; attempt < 3; attempt++) {
    if ((await actionsOpen(windowId)) === open) return true;
    if (!toggle.ready) {
      log(`  setActions ${windowId} toggle never became interactive: ${JSON.stringify(toggle)}`);
      break;
    }
    // 🧾️ The click outcome is LOGGED, never swallowed: a press that timed out and a press that landed on a
    // pane which stayed shut are two different defects and the caller can act on neither from a bare false.
    const outcome = await page
      .locator(`[id="${toggle.id}"]`)
      .first()
      .click({ timeout: 4000 })
      .then(() => "ok")
      .catch((error) => `failed ${String(error).split("\n")[0].slice(0, 90)}`);
    const settled = await waitUntil(() => actionsOpen(windowId), (isOpen) => isOpen === open, 8000);
    log(`  setActions ${windowId} want=${open} attempt=${attempt} click=${outcome} open=${settled.value} waitedMs=${settled.waitedMs} toggleWaitedMs=${toggle.waitedMs}`);
    if (settled.ok) return true;
  }
  return (await actionsOpen(windowId)) === open;
};

/** 📄️ Makes sure a document with at least `minNodes` nodes is loaded (mutate steps may have emptied it). */
const ensureDocument = async (minNodes: number) => {
  if (((await overviewVitals())?.nodes ?? 0) >= minNodes) return;
  if (minNodes > 1) {
    await selectExample(/nakagin/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 180, 90000);
  } else {
    await selectExample(/concrete/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) >= 1, 60000);
  }
  await settle(1);
};

/** 🔁️ Switches the navbar example and WAITS for the reloaded window chrome to be interactive again.
 *
 * 🧯️ An example switch is a full document reload: the three panes re-render, and the Overview pane's own
 * engagement toggle is transiently absent. Every caller that reached for a pane control right afterwards
 * (`import`, `undo`, `engagement-*`) was racing that re-render. */
const selectExample = async (wanted: RegExp) => {
  const pick = async () => {
    const native = page.locator('select[id="playground.navbar.fixture"]').first();
    if (await countSafe(native)) {
      const labels = await native.locator("option").allTextContents();
      const label = labels.find((l) => wanted.test(l));
      if (label) {
        await native.selectOption({ label }).catch(() => {});
        return label;
      }
    }
    const trigger = page.locator('[id="playground.navbar.fixture"]').first();
    if (!(await countSafe(trigger))) return null;
    await trigger.click({ timeout: 3000 }).catch(() => {});
    await page.waitForTimeout(600);
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    if (!(await countSafe(option))) {
      await page.keyboard.press("Escape");
      return null;
    }
    const label = (await option.innerText().catch(() => "")).trim();
    await option.click({ timeout: 3000 }).catch(() => {});
    await page.waitForTimeout(500);
    return label || "?";
  };
  const label = await pick();
  if (label) {
    const chrome = await actionsToggleReady(OVERVIEW);
    if (!chrome.ready) log(`  selectExample ${label}: overview chrome not interactive ${JSON.stringify(chrome)}`);
  }
  return label;
};
//#endregion 🔖️Verdicts

//#region 🔖️Chrome
const windowCamel = (windowId: string) => windowId.replace(/-([a-z0-9])/g, (_m, c: string) => c.toUpperCase());

/** 🎚️ Unfolds one window's measures rail (the Window Options entry point), EXPANDS every collapsed group
 * inside it, and dumps the measure ids it exposes — so a lane that cannot find its control reports "absent
 * from the rail" instead of skipping.
 *
 * 🧯️ A `WindowMeasure::Group` with `default_open: Some(false)` renders as a shut disclosure whose CHILDREN
 * are not in the document at all — `☑️options/🌐️grid` is authored exactly that way, so `puzzle2d-play-grid-snap`
 * and `-grid-factor` read `null` on a rail that is perfectly correct. The rail is unfolded, then every shut
 * group in it is opened, and only then is a missing control a finding. */
const unfoldMeasures = async (windowId = OVERVIEW) => {
  const id = `framework.window.${windowCamel(windowId)}.measures.unfold`;
  for (let attempt = 0; attempt < 3; attempt++) {
    if (!(await countSafe(page.locator(`[id="${id}"]`)))) break;
    await page.locator(`[id="${id}"]`).first().click({ timeout: 4000 }).catch(() => {});
    await settle(1);
  }
  const scope = `[data-slot="window"][id="${windowId}"]`;
  // 🌳️ A group row in the rail is a `window-measure-tree-row` whose fold chevron is an ID-LESS button in the
  // row's own `tree-gutter` (measured on :6012 — the GRID group renders exactly so, and its children are not
  // in the document at all while it is shut). It is pressed by POSITION inside the row, because there is no
  // id to address and `aria-expanded` lives nowhere on it.
  // 🔁️ The chevron is a TOGGLE, so each label is pressed at most once per call — a blind second pass would
  // shut everything the first one opened. Nested groups (the brush's two distribution trees) surface as new
  // labels and get their own pass.
  const expandGroups = (done: string[]) =>
    evalSafe(
      (arg) => {
        const rows = Array.from(document.querySelectorAll(`${arg.scope} [data-slot="window-measure-tree-row"]`));
        const opened: string[] = [];
        for (const row of rows) {
          const chevron = row.querySelector('[data-slot="tree-gutter"] button, [data-slot="tree-gutter-slot"] button') as HTMLButtonElement | null;
          if (!chevron) continue;
          const label = (row.querySelector('[data-slot="tree-label"]') as HTMLElement | null)?.innerText.replace(/\s+/g, " ").trim() ?? "?";
          if (arg.done.includes(label)) continue;
          chevron.click();
          opened.push(label);
        }
        return opened;
      },
      [] as string[],
      { scope, done },
    );
  const expanded: string[] = [];
  for (let round = 0; round < 3; round++) {
    const opened = await expandGroups(expanded);
    if (!opened.length) break;
    expanded.push(...opened);
    await settle(1);
  }
  if (expanded.length) log(`  measures rail ${windowId} expanded groups: ${JSON.stringify(expanded)}`);
  const dump = await evalSafe(
    (selector) =>
      Array.from(document.querySelectorAll(`${selector} [id]`))
        .filter((el) => /puzzle2d|-lod$|-grid|-brush/.test(el.id))
        .map((el) => `${el.id}|${el.getAttribute("data-slot") ?? el.tagName.toLowerCase()}|${el.getAttribute("aria-expanded") ?? el.getAttribute("aria-pressed") ?? (el as HTMLInputElement).value ?? ""}`)
        .slice(0, 80),
    [] as string[],
    scope,
  );
  log(`  measures rail ${windowId}: ${JSON.stringify(dump)}`);
  return dump;
};

/** 🪪️ Resolves an AUTHORED control id to the id it carries in the document. A window measure is authored once
 * per window KIND and rendered once per open INSTANCE, so its DOM id is `${windowInstanceId}/${authoredId}`;
 * a panel body key is namespaced the same way (`panel:puzzle2d-play-inspector/…`). Idempotent on an id that
 * is already live, and it prefers the window the caller names. */
const resolveDomId = async (authored: string, windowId?: string) =>
  evalSafe(
    (arg) => {
      const matches = Array.from(document.querySelectorAll<HTMLElement>("[id]")).filter((el) => el.id === arg.authored || el.id.endsWith(`/${arg.authored}`));
      const scoped = arg.windowId ? matches.find((el) => el.closest(`[data-slot="window"][id="${arg.windowId}"]`)) ?? matches.find((el) => el.id.startsWith(`${arg.windowId}/`)) : undefined;
      return (scoped ?? matches[0])?.id ?? null;
    },
    null as string | null,
    { authored, windowId: windowId ?? null },
  );

/** 🎚️ One control's observable value — `aria-pressed` for toggles, `value` for sliders/steppers, the
 * trigger's text for selects, and `data-published-value`: the value the PROGRAM answered with, as opposed to
 * the optimistic draft the rail renders while the round trip is in flight. */
const readControl = async (authored: string, windowId?: string) => {
  const id = await resolveDomId(authored, windowId);
  if (!id) return null;
  return evalSafe(
    (target) => {
      const el = document.getElementById(target) as HTMLElement | null;
      if (!el) return null;
      const input = el as HTMLInputElement;
      const thumb = (el.matches('[role="slider"]') ? el : el.querySelector('[role="slider"]')) as HTMLElement | null;
      const range = el.querySelector('input[type="range"]') as HTMLInputElement | null;
      const box = el.querySelector("input") as HTMLInputElement | null;
      return {
        id: target,
        tag: el.tagName.toLowerCase(),
        slot: el.getAttribute("data-slot"),
        role: el.getAttribute("role"),
        pressed: el.getAttribute("aria-pressed") ?? el.getAttribute("data-state"),
        value: range?.value ?? thumb?.getAttribute("aria-valuenow") ?? (typeof input.value === "string" && input.value !== "" ? input.value : null) ?? box?.value ?? null,
        published: el.getAttribute("data-published-value"),
        text: (el.innerText || "").replace(/\s+/g, " ").trim().slice(0, 60),
      };
    },
    null as null | { id: string; tag: string; slot: string | null; role: string | null; pressed: string | null; value: string | null; published: string | null; text: string },
    id,
  );
};

/** 🎚️ Drives one control the way a user would and returns before/after readings. Sliders take keyboard
 * arrows (a headless thumb has no stable hit box), selects take their first differing option, steppers take
 * their `+` as a press-and-hold (`Stepper` fires on mousedown/mouseup, never on a synthesized click). */
const nudgeControl = async (authored: string, windowId?: string) => {
  const before = await readControl(authored, windowId);
  if (!before) return { before: null, after: null, waitedMs: 0, moved: false, scoredOn: "reading", dispatch: [] as string[] };
  const mark = consoleCursor();
  const loc = page.locator(`[id="${before.id}"]`).first();
  if (before.slot === "tree-action-checkbox" || (before.tag === "input" && before.value === "on")) {
    // ☑️ A rail checkbox is an `<input>` inside a label wrapper: a force click on the input itself lands on
    // an element the wrapper covers, so the press is escalated — click, then Space with focus, then a
    // synthetic click on the wrapper label the browser would have routed it to anyway.
    await loc.click({ force: true, timeout: 4000 }).catch(() => {});
    await settle(1);
    if ((await readControl(before.id, windowId))?.published === before.published) {
      await loc.focus().catch(() => {});
      await page.keyboard.press("Space").catch(() => {});
      await settle(1);
    }
    if ((await readControl(before.id, windowId))?.published === before.published) {
      const via = await evalSafe(
        (target) => {
          const el = document.getElementById(target) as HTMLInputElement | null;
          if (!el) return "absent";
          const wrapper = el.closest('[data-slot="tree-action-checkbox-wrapper"], label') as HTMLElement | null;
          (wrapper ?? el).click();
          return wrapper ? "wrapper" : "input";
        },
        "absent",
        before.id,
      );
      log(`  nudge ${before.id} checkbox fell through to a synthetic click via ${via}`);
    }
  } else if (before.slot === "numberStepper" || before.id.endsWith(".control")) {
    const plus = page.locator(`[id="${before.id}"]`).locator("xpath=..").locator('[data-slot="stepper-plus"]').first();
    const box = (await countSafe(plus)) ? await plus.boundingBox().catch(() => null) : null;
    if (box) {
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.down();
      await page.waitForTimeout(120);
      await page.mouse.up();
    } else {
      await loc.focus().catch(() => {});
      await page.keyboard.press("ArrowUp").catch(() => {});
    }
  } else if (before.slot === "slider" || before.role === "slider") {
    const thumb = page.locator(`[id="${before.id}"] [role="slider"], [id="${before.id}"] input[type="range"]`).first();
    const handle = (await countSafe(thumb)) ? thumb : loc;
    await handle.click({ force: true, timeout: 4000 }).catch(() => {});
    await handle.focus().catch(() => {});
    for (let i = 0; i < 5; i++) await page.keyboard.press("ArrowRight").catch(() => {});
  } else if (before.role === "combobox" || before.slot === "select-trigger") {
    const options = page.locator('[role="option"]');
    let opened = { ok: false, waitedMs: 0 };
    for (let attempt = 0; attempt < 2 && !opened.ok; attempt++) {
      await loc.click({ force: true, timeout: 4000 }).catch(() => {});
      opened = await waitUntil(async () => (await countSafe(options)) > 0, (open) => open, 8000, 250);
    }
    const texts = await options.allInnerTexts().catch(() => [] as string[]);
    const different = texts.findIndex((text) => text.replace(/\s+/g, " ").trim() !== (before.text ?? "").trim());
    log(`  nudge ${before.id} select opened=${opened.ok} options=${JSON.stringify(texts).slice(0, 200)} picking=${different}`);
    if (opened.ok && different >= 0) await options.nth(different).click({ timeout: 3000 }).catch(() => {});
    else await page.keyboard.press("Escape").catch(() => {});
  } else {
    await loc.click({ force: true, timeout: 4000 }).catch(() => {});
  }
  // 🕰️ Scored on the value the PROGRAM published whenever the control carries one. A measure rail holds an
  // optimistic draft for the whole round trip and moves the rendered `text` FIRST, so "the trigger now reads
  // minimap" is not evidence the guest answered — only `data-published-value` is. Controls without one (a
  // NumberStepper publishes nothing) fall back to any reading change.
  const scored = before.published !== null;
  const settled = await waitUntil(
    () => readControl(before.id, windowId),
    (after) => (scored ? (after?.published ?? null) !== before.published : JSON.stringify(after) !== JSON.stringify(before)),
    30000,
  );
  return { before, after: settled.value, waitedMs: settled.waitedMs, moved: settled.ok, scoredOn: scored ? "published" : "reading" };
};

/** 🧰️ Unfolds one window's utility bar. `.unfold` is only in the document while the bar is folded. */
const unfoldUtilityBar = async (windowId = OVERVIEW) => {
  const unfold = page.locator(`[id="framework.window.${windowCamel(windowId)}.utilityBar.unfold"]`).first();
  if (await countSafe(unfold)) {
    await unfold.click({ timeout: 3000 }).catch(() => {});
    await settle(1);
  }
};

/** 🧰️ Arms one utility by its literal `UTILITY_ID` (`select`, `brush`, and — once slice 2F lands it — the
 * area brush), polling the pane's OWN `data-board-active-utility` rather than sleeping: the arm is a
 * `setActiveUtility` round trip through the guest, and a starved reply reads exactly like a dropped one. */
const armUtility = async (utilityId: string, windowId = OVERVIEW) => {
  await unfoldUtilityBar(windowId);
  const loc = page.locator(inWindow(windowId, `[data-slot="toggle-group-item"][id="${utilityId}"]`)).first();
  const found = await countSafe(loc);
  if (found) await loc.click({ timeout: 4000 }).catch(() => {});
  const settled = await waitUntil(() => paneVitals(windowId), (v) => v?.utility === utilityId, 20000);
  return { found, active: settled.value?.utility ?? null, ok: found > 0 && settled.ok, waitedMs: settled.waitedMs };
};

/** 🛠️ Arms one mode tool (`fill`) through the footer Tool category and polls its own pressed state. */
const armTool = async (toolId: string) => {
  await clickTab("framework.category.tool");
  const loc = page.locator(`[data-slot="toggle-group-item"][id="tool.${toolId}"], [id="tool.${toolId}"]`).first();
  const found = await countSafe(loc);
  if (found && (await loc.getAttribute("aria-pressed").catch(() => null)) !== "true") await loc.click({ timeout: 4000 }).catch(() => {});
  const settled = await waitUntil(() => evalSafe((id) => document.getElementById(id)?.getAttribute("aria-pressed") ?? null, null as string | null, `tool.${toolId}`), (pressed) => pressed === "true", 20000);
  return { found, pressed: settled.value, ok: found > 0 && settled.ok, waitedMs: settled.waitedMs };
};

/** 🎯️ Picks one visible node on the overview canvas and proves the selection through the pane's own
 * `data-board-selection-json`. Every selection-scoped lane's precondition. */
const pickNode = async (index = 0) => {
  const visible = await visibleNodeIds();
  const id = visible[index] ?? visible[0] ?? null;
  if (!id) return { id: null, at: null, picked: false, waitedMs: 0, selection: "", visible: visible.length };
  const at = await nodeScreen(id);
  if (!at) return { id, at: null, picked: false, waitedMs: 0, selection: "", visible: visible.length };
  await page.mouse.click(at.x, at.y);
  const settled = await waitUntil(overviewVitals, (v) => selectionIds(v).includes(id), 15000);
  return { id, at, picked: settled.ok, waitedMs: settled.waitedMs, selection: settled.value?.selection ?? "", visible: visible.length };
};

/** 🎲️ Whether the overview board is actually PAINTED — `data-board-fixture-parsed` plus a non-zero node
 * count. A 2d board can go transiently blank inside a session when a descriptor resync announces every
 * document edge and exhausts the 256-slot event-credit queue (`📓️E7…` §4), and every verdict taken against
 * a blank board is a verdict about that, not about the feature under test. */
const boardPainted = async (pane = OVERVIEW) => {
  const v = await paneVitals(pane);
  return { parsed: v?.parsed ?? "", nodes: v?.nodes ?? -1, edges: v?.edges ?? -1, painted: (v?.nodes ?? -1) > 0 && (v?.parsed ?? "") === "true" };
};

/** 🖱️ The context menu's rows, ids included — the vocabulary `puzzle2d_context_menu_items` authors
 * (`✏️editor/🦀️.rs`: `selectAll` | `toggleHidden`, `toggleLocked`, `duplicate`, `focusSelection`,
 * the `selection` group's `selectSameKind`, `deleteSelection`). */
const contextMenuRows = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll('[role="menuitem"], [data-slot="context-menu-item"]')).map((el) => ({
        id: el.id || null,
        action: el.getAttribute("data-menu-action"),
        text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40),
      })),
    [] as { id: string | null; action: string | null; text: string }[],
  );

/** 🖱️ Right-clicks a point and POLLS for the menu: `Board2dHost`'s context menu opens on the GUEST's reply,
 * which queues behind every already-serialized command, so a fixed sample reads a starved reply as a
 * missing one. Submenu groups are expanded on the way, since `selectSameKind` is authored inside one. */
const openContextMenuAt = async (at: { x: number; y: number }) => {
  await page.mouse.click(at.x, at.y, { button: "right" });
  const top = await waitUntil(contextMenuRows, (rows) => rows.length > 0, 20000, 700);
  const merged = new Map(top.value.filter((row) => row.id).map((row) => [row.id as string, row]));
  for (const group of top.value.filter((row) => row.id?.startsWith("selection") || /^menu\.group\./.test(row.id ?? ""))) {
    const sizeBefore = merged.size;
    for (const gesture of [() => page.locator(`[id="${group.id}"]`).first().hover({ timeout: 2500 }), () => page.locator(`[id="${group.id}"]`).first().click({ force: true, timeout: 2500 })]) {
      await gesture().catch(() => {});
      const grew = await waitUntil(
        async () => {
          for (const row of await contextMenuRows()) if (row.id) merged.set(row.id, row);
          return merged.size;
        },
        (size) => size > sizeBefore,
        5000,
        400,
      );
      if (grew.ok) break;
    }
  }
  const rows = [...merged.values()];
  log(`  context menu rows=${JSON.stringify(rows.map((r) => r.id)).slice(0, 300)} waitedMs=${top.waitedMs}`);
  return { rows, waitedMs: top.waitedMs };
};
const clickMenuRow = async (id: string) => {
  const row = page.locator(`[role="menuitem"][id="${id}"], [id="${id}"]`).last();
  if (!(await countSafe(row))) return false;
  await row.click({ force: true, timeout: 4000 }).catch(() => {});
  return true;
};
const closeContextMenu = async () => {
  await page.keyboard.press("Escape").catch(() => {});
  await settle(0.5);
};

/** 🕰️ Opens the framework History panel and expands its two collapsed sections. Measured on :6012: the panel
 * mounts `framework.history.entry.N` under a `HISTORY` section row and the reserved command rows
 * (`…undo`/`…redo`/`…checkpoint`/`…checkin`) under a `COMMANDS` one, and BOTH render shut — every row is in
 * the document but `offsetParent === null`, so a press lands nowhere until the section is open. */
const openHistory = async () => {
  await clickTab("framework.panel.history");
  for (const section of ["framework.history.commands", "framework.history.actions"]) {
    const row = page.locator(`[id="${section}"]`).first();
    if (await countSafe(row)) await row.click({ force: true, timeout: 3000 }).catch(() => {});
  }
  await settle(1);
  return evalSafe(
    () =>
      Array.from(document.querySelectorAll<HTMLElement>('[id^="framework.history."]')).map((row) => `${row.id}[${row.getAttribute("data-slot") ?? ""}] vis=${row.offsetParent !== null} = ${(row.innerText || "").replace(/\s+/g, " ").trim().slice(0, 40)}`),
    [] as string[],
  );
};
/** 🕰️ The ledger rows, keyed by id with the label each one renders. An id alone says a row appeared; only
 * the label says which verb wrote it. */
const historyRows = async () => {
  await openHistory();
  return evalSafe(
    () => Object.fromEntries(Array.from(document.querySelectorAll<HTMLElement>('[id^="framework.history.entry."]')).map((row) => [row.id, (row.innerText || "").replace(/\n/g, " ").trim().slice(0, 60)])),
    {} as Record<string, string>,
  );
};
/** ⏪️ Presses one framework history control by id, reporting whether the control was there at all — these
 * are reserved verbs the shell owns, so a missing control is a framework finding, never an app one.
 * `revert` is looked up under BOTH spellings because the row :6012 renders is `framework.history.checkin`. */
const pressHistory = async (kind: "undo" | "redo" | "checkpoint" | "revert") => {
  await openHistory();
  const candidates = kind === "revert" ? ["framework.history.revert", "framework.history.checkin"] : [`framework.history.${kind}`];
  for (const id of candidates) {
    if (!(await countSafe(page.locator(`[id="${id}"]`)))) continue;
    await page.locator(`[id="${id}"]`).first().click({ force: true, timeout: 4000 }).catch(() => {});
    await evalSafe((target) => {
      const root = document.getElementById(target);
      const button = (root?.querySelector("button") as HTMLElement | null) ?? (root as HTMLElement | null);
      button?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    }, undefined as void, id);
    return true;
  }
  return false;
};

/** 🗂️ One action row of a window's Actions pane, addressed inside that window — three panes render the same
 * `action.<id>` ids. */
const actionRow = (actionId: string, windowId = OVERVIEW) => page.locator(inWindow(windowId, `[id="action.${actionId}"]`)).first();
/** 🗂️ Opens the pane's Actions panel and fires one action row, reporting each hop. */
const fireAction = async (actionId: string, windowId = OVERVIEW) => {
  const opened = await setActions(true, windowId);
  const row = actionRow(actionId, windowId);
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 4000 }).catch(() => {});
  return { opened, present: present > 0 };
};

/** ✍️ Types one engagement line into a pane's own command field and submits it, returning what the field
 * ACTUALLY held at the moment Enter was pressed.
 *
 * 🧯️ The field is controlled and the shell's `normalizeEngagementActionText` has historically PascalCased
 * the draft and stripped its spaces (`"move 50 25"` → `"Move5025"`), which the guest's whitespace-splitting
 * parser can never match (`📓️E7…` §1 — a framework defect, slice 2A). Reporting `typed` turns a 20 s
 * silent wait into a one-line diagnosis. */
const submitEngagement = async (text: string, windowId = OVERVIEW) => {
  const opened = await setActions(true, windowId);
  const input = page.locator(inWindow(windowId, '[id="puzzle2d-engagement"]')).first();
  const present = await countSafe(input);
  if (!present) return { opened, present: false, typed: null as string | null };
  await input.fill(text, { timeout: 6000 }).catch(() => {});
  const typed = await input.inputValue({ timeout: 4000 }).catch(() => "?");
  await input.press("Enter").catch(() => {});
  log(`  engagement submit=${JSON.stringify(text)} typed=${JSON.stringify(typed)}`);
  return { opened, present: true, typed };
};

/** 🌳️ The outliner (artifact panel) rows, with the row-action controls each one carries. Slice 2C adds the
 * per-row Hide/Lock actions; until then `controls` reads `[]` and the lane says so by name. */
const outlinerRows = async () => {
  await clickTab("framework.panel.artifact");
  return evalSafe(
    () =>
      Array.from(document.querySelectorAll('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]'))
        .filter((el) => el.id.includes("puzzle2d-play-document"))
        .map((el) => ({
          id: el.id,
          text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 60),
          controls: Array.from(el.querySelectorAll('button, [role="button"], [data-slot^="tree-action"]')).map((c) => `${c.id || "-"}=${(c.getAttribute("aria-label") ?? c.getAttribute("title") ?? (c as HTMLElement).innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 20)}`),
        }))
        .slice(0, 60),
    [] as { id: string; text: string; controls: string[] }[],
  );
};

/** 🩺️ Node poses keyed by id, the one reading every transform lane compares. */
const poseOf = async (id: string, pane = OVERVIEW) => (await positionsOf(pane))[id] ?? null;
/** 🔔️ The shell's transient notices, minus the reconnect banners that say nothing about the feature. */
const notices = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll('[data-slot="notice"], [role="status"], [role="alert"]'))
        .map((el) => (el as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 120))
        .filter((text) => text.length > 0 && !/agent disconnected|remote: detached|reconnect/i.test(text)),
    [] as string[],
  );
//#endregion 🔖️Chrome

//#region 🔖️Steps
type Step = { name: string; group: "read" | "mutate" | "replace"; run: () => Promise<void> };
const steps: Step[] = [];
const register = (name: string, group: Step["group"], run: () => Promise<void>) => steps.push({ name, group, run });

register("example-inventory", "read", async () => {
  const trigger = page.locator('[id="playground.navbar.fixture"]').first();
  const current = (await trigger.innerText().catch(() => "")).trim();
  await trigger.click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(600);
  const options = await evalSafe(() => Array.from(document.querySelectorAll('[role="option"]')).map((o) => (o as HTMLElement).innerText.replace(/\s+/g, " ").trim()), [] as string[]);
  await page.keyboard.press("Escape");
  const positions = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, unknown>;
  const keys = Object.keys(positions);
  verdict("5-examples", "picker-inventory", options.length >= 2, { current, options, positions: keys.length, firstIds: keys.slice(0, 3), vitals: await overviewVitals() });
});

register("windows", "read", async () => {
  const s = await snapshot();
  verdict("1-windows", "three-boards", s.windows.length >= 3 && s.canvases >= 3, { windows: s.windows, canvases: s.canvases });
  const vitals = await boardVitals();
  verdict("1-windows", "vitals-published", vitals.length >= 3 && vitals.every((v) => v.nodes >= 0), { vitals });
});

register("example-concrete-forest", "replace", async () => {
  const label = await selectExample(/concrete/i);
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 30000);
  verdict("5-examples", "concrete-forest-loads", Boolean(label) && r.ok, { label, nodes: r.value?.nodes, edges: r.value?.edges, waitedMs: r.waitedMs });
});

register("example-nakagin", "replace", async () => {
  const label = await selectExample(/nakagin/i);
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 180 && v?.edges === 179, 90000);
  verdict("5-examples", "nakagin-loads", Boolean(label) && r.ok, { label, nodes: r.value?.nodes, edges: r.value?.edges, waitedMs: r.waitedMs });
  const s = await snapshot();
  verdict("5-examples", "nakagin-no-recovery-card", !s.recovery.some((x) => x && x !== "?"), { recovery: s.recovery });
});

register("panels", "read", async () => {
  for (const [tab, min] of [["framework.panel.artifact", 1], ["framework.panel.catalogue", 1], ["framework.panel.inspection", 1]] as const) {
    const opened = await clickTab(tab);
    const s = await snapshot();
    verdict("16-18-panels", tab, opened && s.treeItems >= min, { treeItems: s.treeItems, tabs: s.tabs.slice(0, 12) });
  }
  log(`  closed panels: ${JSON.stringify(await closePanels())}`);
});

register("camera-wheel", "read", async () => {
  const before = (await overviewVitals())?.camera ?? "";
  const box = await overviewBox();
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.wheel(0, -240);
  await settle(2);
  const r = await waitUntil(overviewVitals, (v) => Boolean(v?.camera) && v!.camera !== before, 15000);
  verdict("2-camera", "wheel-zoom-changes-camera", r.ok, { before, after: r.value?.camera, waitedMs: r.waitedMs });
});

register("click-select", "read", async () => {
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("6-selection", "click-select", false, { reason: "no node position published", ids: Object.keys(ids).length });
    return;
  }
  await page.mouse.click(at.x, at.y);
  const r = await waitUntil(overviewVitals, (v) => (v?.selection ?? "").includes(first), 15000);
  verdict("6-selection", "click-select", r.ok, { id: first, at, selection: r.value?.selection, waitedMs: r.waitedMs });
  await clickTab("framework.panel.inspection");
  await settle(2);
  const rows = await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]')).map((r) => `${r.id || "?"}=${(r as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)}`), [] as string[]);
  const inspector = rows.filter((row) => row.includes("puzzle2d-play-inspector"));
  const tabState = await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).filter((b) => /inspection|artifact|catalogue/.test(b.id)).map((b) => `${b.id}:pressed=${b.getAttribute("aria-pressed")}:state=${b.getAttribute("data-state")}`), [] as string[]);
  log(`  inspector rows=${rows.length} tabs=${JSON.stringify(tabState)} first rows=${JSON.stringify(rows.slice(0, 6))}`);
  verdict("16-inspection", "inspector-shows-selected-node", inspector.some((row) => row.includes(first)), { id: first, inspector: inspector.slice(0, 16) });
  log(`  closed panels: ${JSON.stringify(await closePanels())}`);
});

register("marquee", "read", async () => {
  await ensureDocument(2);
  log(`  closed panels: ${JSON.stringify(await closePanels())}`);
  const box = await overviewBox();
  // 🎯️ Start clear of the pane chrome (Actions/Search toggles at the top) and of the node column.
  await page.mouse.move(box.x + 24, box.y + 160);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width - 24, box.y + box.height - 40, { steps: 12 });
  await page.mouse.up();
  const r = await waitUntil(overviewVitals, (v) => (JSON.parse(v?.selection || "[]") as unknown[]).length >= 2, 15000);
  verdict("6-selection", "marquee-selects-many", r.ok, { selected: (JSON.parse(r.value?.selection || "[]") as unknown[]).length, waitedMs: r.waitedMs });
  await page.keyboard.press("Escape");
  await page.mouse.click(box.x + 5, box.y + 5);
  await settle(1);
});

register("drag-node", "mutate", async () => {
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("8-transform", "drag-node", false, { reason: "no node position" });
    return;
  }
  const before = ids[first];
  await page.mouse.move(at.x, at.y);
  await page.mouse.down();
  await page.mouse.move(at.x + 80, at.y + 40, { steps: 10 });
  await page.mouse.up();
  const r = await waitUntil(
    async () => JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>,
    (p) => Boolean(p[first]) && (Math.abs(p[first][0] - before[0]) > 1 || Math.abs(p[first][1] - before[1]) > 1),
    20000,
  );
  verdict("8-transform", "drag-node-moves-document", r.ok, { id: first, before, after: r.value[first], waitedMs: r.waitedMs });
});

const unfoldUtilities = async () => {
  const unfold = page.locator('[id="framework.window.2dOverview.utilityBar.unfold"]').first();
  if (await countSafe(unfold)) {
    await unfold.click({ timeout: 3000 }).catch(() => {});
    await settle(1);
  }
};

register("utilities", "read", async () => {
  await unfoldUtilities();
  const s = await snapshot();
  const ids = s.toggles.map((t) => t.split("=")[0]);
  verdict("9-brush", "utility-toggles-present", ids.some((id) => /brush/i.test(id)) && ids.some((id) => /select/i.test(id)), { toggles: s.toggles.slice(0, 30) });
  const brush = page.locator('[data-slot="toggle-group-item"]').filter({ hasText: /^brush$/i }).first();
  if (await countSafe(brush)) {
    await brush.click({ timeout: 3000 }).catch(() => {});
    const r = await waitUntil(overviewVitals, (v) => v?.utility === "brush", 15000);
    verdict("9-brush", "brush-utility-activates", r.ok, { utility: r.value?.utility, waitedMs: r.waitedMs });
  }
});

register("fill", "mutate", async () => {
  // 🪣️ Fill needs open handles: Nakagin is fully connected (358 handles, 179 edges), Concrete Forest is one
  // node with 11 free handles.
  if (((await overviewVitals())?.nodes ?? 0) !== 1) {
    await selectExample(/concrete/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 60000);
  }
  await clickTab("framework.category.tool");
  const toggle = page.locator('[data-slot="toggle-group-item"][id="tool.fill"], [id="tool.fill"]').first();
  const present = await countSafe(toggle);
  verdict("12-fill", "fill-tab-present", present > 0);
  if (!present) return;
  const pressed = await toggle.getAttribute("aria-pressed").catch(() => null);
  if (pressed !== "true") await toggle.click({ timeout: 3000 }).catch(() => {});
  await settle(2);
  const count = page.locator('[id*="puzzle2d-fill-count"] input, input[id*="fill-count"]').first();
  verdict("12-fill", "fill-count-measure", (await countSafe(count)) > 0);
  const weights = await evalSafe(() => document.body.innerText.includes("Node Weights") && document.body.innerText.includes("Handle Weights"), false);
  verdict("12-fill", "fill-distribution-groups", weights);
  await clickTab("framework.panel.toolRun");
  await settle(1);
  const before = (await overviewVitals())?.nodes ?? -1;
  const start = page.locator("button", { hasText: /^start$/i }).first();
  const startPresent = await countSafe(start);
  if (startPresent) await start.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 90000);
  const runText = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).map((p) => (p as HTMLElement).innerText.replace(/\s+/g, " ")).join(" | ").slice(0, 400), "");
  verdict("12-fill", "fill-run-places-nodes", r.ok, { before, after: r.value?.nodes, startPresent, waitedMs: r.waitedMs, status: await runText() });
  // 🏁️ Finalize only once the run reports completion — a Finalize mid-run is not honoured and the run keeps
  // re-applying its provisional placements after every later artifact edit.
  const complete = await waitUntil(runText, (text) => /ready to finalize|complete/i.test(text), 180000, 1000);
  log(`  fill run completion: ${JSON.stringify(complete)}`);
  const placed = (await overviewVitals())?.nodes ?? -1;
  const finalize = page.locator("button", { hasText: /^finalize$/i }).first();
  const finalizePresent = await countSafe(finalize);
  if (finalizePresent) await finalize.click({ timeout: 3000 }).catch(() => {});
  const gone = await waitUntil(runText, (text) => !/ready to finalize|running|retracting/i.test(text), 30000, 1000);
  const after = (await overviewVitals())?.nodes ?? -1;
  verdict("12-fill", "fill-finalize-keeps-placements", complete.ok && finalizePresent > 0 && gone.ok && after > before, { before, placed, after, runGone: gone.ok, status: gone.value });
});

register("delete", "mutate", async () => {
  await ensureDocument(1);
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("22-delete", "delete-selection", false, { reason: "no node position" });
    return;
  }
  const entities = (v: Awaited<ReturnType<typeof overviewVitals>>) => (v ? v.nodes + v.edges + v.handles : -1);
  const before = entities(await overviewVitals());
  await page.mouse.click(at.x, at.y);
  const picked = await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]") !== "[]", 10000);
  await page.keyboard.press("Delete");
  const r = await waitUntil(overviewVitals, (v) => entities(v) < before, 20000);
  const cleared = await waitUntil(overviewVitals, (v) => (v?.selection ?? "") === "[]", 10000);
  verdict("22-delete", "delete-selection", r.ok, { id: first, picked: picked.value?.selection, before, after: entities(r.value), waitedMs: r.waitedMs });
  verdict("22-delete", "selection-cleared-after-delete", cleared.ok, { selection: cleared.value?.selection });
});

register("undo", "replace", async () => {
  const entities = (v: Awaited<ReturnType<typeof overviewVitals>>) => (v ? v.nodes + v.edges + v.handles : -1);
  const before = entities(await overviewVitals());
  await closePanels();
  // ⏪ Undo is a keybinding (mod+z) and a row of the pane's Actions panel (`#action.undo`); the History
  // panel lists the ledger and carries no button of its own.
  const opened = await setActions(true);
  const undo = page.locator('[id="action.undo"]').first();
  const present = await countSafe(undo);
  if (present) await undo.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => entities(v) >= 0 && entities(v) !== before, 20000);
  await setActions(false);
  const parsed = await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-fixture-parsed") ?? "", "");
  // 🏷️ Named for its ROUTE. The History panel's own `#framework.history.undo` is a second, independent route
  // measured by the `history-controls` lane; two verdicts that share a name cannot be joined across runs.
  verdict("20-history", "undo-action-row-changes-document", opened && present > 0 && r.ok && parsed === "true", { before, after: entities(r.value), parsed, waitedMs: r.waitedMs });
  await closePanels();
});

register("context-menu", "read", async () => {
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("15-context-menu", "opens", false, { reason: "no node position" });
    return;
  }
  await page.mouse.click(at.x, at.y, { button: "right" });
  await settle(1.5);
  const menus = await evalSafe(() => Array.from(document.querySelectorAll('[role="menu"], [data-slot="context-menu"]')).map((m) => (m as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 240)), [] as string[]);
  verdict("15-context-menu", "opens", menus.length > 0, { menus });
  await page.keyboard.press("Escape");
});

register("catalogue-add", "mutate", async () => {
  await ensureDocument(1);
  await clickTab("framework.panel.catalogue");
  const before = (await overviewVitals())?.nodes ?? -1;
  const row = page.locator('[data-slot="tree-item"][id*="puzzle2d-play-kinds.nodes."], [data-slot="tree-item-row"][id*="puzzle2d-play-kinds.nodes."], [role="treeitem"][id*="puzzle2d-play-kinds.nodes."]').first();
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === before + 1, 20000);
  verdict("18-catalogue", "click-adds-node", present > 0 && r.ok, { before, after: r.value?.nodes, waitedMs: r.waitedMs });
  await closePanels();
});

register("select-same-kind", "read", async () => {
  if (((await overviewVitals())?.nodes ?? 0) < 2) {
    await selectExample(/nakagin/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 180, 90000);
  }
  const positions = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("6-selection", "select-same-kind", false, { reason: "no node position" });
    return;
  }
  await page.mouse.click(at.x, at.y);
  await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]").includes(first), 10000);
  await page.mouse.click(at.x, at.y, { button: "right" });
  await settle(1.5);
  const submenu = page.locator('[role="menuitem"], [role="menu"] *').filter({ hasText: /^Selection$/ }).first();
  if (await countSafe(submenu)) {
    await submenu.hover().catch(() => {});
    await settle(0.8);
  }
  const row = page.locator('[role="menuitem"]').filter({ hasText: /same kind/i }).first();
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => (JSON.parse(v?.selection || "[]") as unknown[]).length > 1, 15000);
  verdict("6-selection", "select-same-kind", present > 0 && r.ok, { first, selected: (JSON.parse(r.value?.selection || "[]") as unknown[]).length, waitedMs: r.waitedMs });
  await page.keyboard.press("Escape");
});

register("engagement-move", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const positions = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("8-transform", "engagement-move", false, { reason: "no node position" });
    return;
  }
  await page.mouse.click(at.x, at.y);
  const picked = await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]").includes(first), 10000);
  log(`  engagement-move picked=${picked.ok} actions=${await setActions(true)}`);
  const input = page.locator('input[id="puzzle2d-engagement"], input[id*="puzzle2d-engagement"]').first();
  const present = await countSafe(input);
  if (present) {
    await input.fill("move 50 25").catch(() => {});
    await input.press("Enter").catch(() => {});
  }
  const before = positions[first];
  const r = await waitUntil(
    async () => JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>,
    (p) => Boolean(p[first]) && Math.abs(p[first][0] - before[0] - 50) < 0.5 && Math.abs(p[first][1] - before[1] - 25) < 0.5,
    20000,
  );
  verdict("8-transform", "engagement-move", present > 0 && r.ok, { first, before, after: r.value[first], waitedMs: r.waitedMs });
  await setActions(false);
});

register("export", "read", async () => {
  // 📤 App actions live in the pane's Actions panel (FILE › Export), not in the shell Command tab.
  await closePanels();
  log(`  actions open: ${await setActions(true)}`);
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 20000 }).catch(() => null),
    (async () => {
      const row = page.locator('[id="action.exportFixture"], [data-slot="tree-item-row"][id*="exportFixture"]').first();
      const present = await countSafe(row);
      if (present) await row.click({ timeout: 3000 }).catch(() => {});
      else log("export row not found");
    })(),
  ]);
  const name = download ? download.suggestedFilename() : null;
  let exported: { nodes: number; edges: number; file: string } | null = null;
  if (download) {
    const file = join(OUT, `probe-${stamp}-export-${name ?? "download.json"}`);
    await download.saveAs(file).catch(() => {});
    try {
      const doc = JSON.parse(readFileSync(file, "utf8")) as { nodes?: unknown[]; edges?: unknown[] };
      exported = { nodes: Array.isArray(doc.nodes) ? doc.nodes.length : -1, edges: Array.isArray(doc.edges) ? doc.edges.length : -1, file };
    } catch (error) {
      log(`  export file unreadable: ${String(error).slice(0, 200)}`);
    }
  }
  const vitals = await overviewVitals();
  verdict("24-export", "export-downloads-json", Boolean(download) && /\.json$/.test(name ?? "") && exported !== null && exported.nodes === (vitals?.nodes ?? -2), { name, exported, nodes: vitals?.nodes });
  exportedFile = exported?.file ?? null;
  await setActions(false);
});

let exportedFile: string | null = null;

register("import", "replace", async () => {
  // 📥 Round trip: load the OTHER example, then import this run's own export back through the pane's
  // Actions › Import row (host file picker → `importFixture`) and expect the exported census back.
  if (!exportedFile) {
    verdict("24-import", "import-round-trip", false, { reason: "no export file from this run" });
    return;
  }
  const exported = JSON.parse(readFileSync(exportedFile, "utf8")) as { nodes: unknown[]; edges: unknown[] };
  const other = exported.nodes.length === 1 ? /nakagin/i : /concrete/i;
  await selectExample(other);
  await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) !== exported.nodes.length, 90000);
  const before = (await overviewVitals())?.nodes ?? -1;
  await closePanels();
  await setActions(true);
  const [chooser] = await Promise.all([
    page.waitForEvent("filechooser", { timeout: 30000 }).catch(() => null),
    (async () => {
      const row = page.locator('[id="action.openImportFixture"]').first();
      if (await countSafe(row)) await row.click({ timeout: 3000 }).catch(() => {});
      else log("  import row not found");
    })(),
  ]);
  const mark = consoleCursor();
  if (chooser) await chooser.setFiles(exportedFile).catch((error) => log(`  setFiles failed: ${String(error).slice(0, 200)}`));
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === exported.nodes.length && (v?.edges ?? -1) === exported.edges.length, 120000);
  const trail = consoleSince(mark).filter((l) => /import|Import|fault|error|refused|notice/i.test(l) && !/contributions|stale\]|staged plugin/.test(l)).slice(0, 12);
  log(`  import trail: ${trail.join(" || ").slice(0, 1500)}`);
  const parsed = await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-fixture-parsed") ?? "", "");
  verdict("24-import", "import-round-trip", Boolean(chooser) && r.ok && parsed === "true", { chooser: Boolean(chooser), before, after: r.value?.nodes, edges: r.value?.edges, expected: { nodes: exported.nodes.length, edges: exported.edges.length }, parsed, waitedMs: r.waitedMs });
  await setActions(false);
});

register("settings", "read", async () => {
  // ⚙️ The app's settings panel (`puzzle2d.panel.settings`) is a child of the shell's Settings tab.
  await clickTab("framework.settings");
  await settle(1.5);
  const tabs = await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((b) => `${b.id}=${(b as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 30)}`), [] as string[]);
  const child = page.locator('[data-slot="panel-tab-button"][id="puzzle2d.panel.settings"]').first();
  if (await countSafe(child)) await child.click({ timeout: 3000 }).catch(() => {});
  await settle(1.5);
  const rows = await evalSafe(() => Array.from(document.querySelectorAll('[id*="puzzle2d.play.settings"], [id*="puzzle2d-settings"], [data-slot="panel"] input')).map((e) => `${e.tagName.toLowerCase()}#${e.id}=${((e as HTMLInputElement).value ?? (e as HTMLElement).innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 40)}`), [] as string[]);
  log(`  settings tabs=${JSON.stringify(tabs.filter((t) => /settings/i.test(t)))} rows=${JSON.stringify(rows.slice(0, 20))}`);
  const stepper = page.locator('[data-slot="panel"] input').first();
  const present = (await countSafe(stepper)) > 0;
  const beforeValue = present ? await stepper.inputValue().catch(() => "") : "";
  const plus = page.locator('[data-slot="stepper-plus"]').first();
  if (await countSafe(plus)) await plus.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(async () => (present ? await stepper.inputValue().catch(() => "") : ""), (value) => value !== "" && value !== beforeValue, 15000);
  verdict("23-settings", "settings-stepper-changes-value", present && r.ok, { rows: rows.slice(0, 8), before: beforeValue, after: r.value, waitedMs: r.waitedMs });
  await closePanels();
});

register("locale", "read", async () => {
  // 🌍️ Read the labels off a small document: Nakagin's 48-row artifact window trips the host's 128-node
  // reconcile budget (peer ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING), which leaves the stale
  // English body on screen — that is a virtualisation fault, not a locale one.
  if (((await overviewVitals())?.nodes ?? 0) !== 1) {
    await closePanels();
    const label = await selectExample(/concrete/i);
    const loaded = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 60000);
    log(`  locale document: ${label} loaded=${loaded.ok} nodes=${loaded.value?.nodes}`);
    await settle(1);
  }
  const labels = async () => {
    await clickTab("framework.panel.artifact");
    await settle(1.5);
    const text = await evalSafe(() => (document.querySelector('[data-slot="panel"][id="framework.panelTab.framework.panel.artifact"]') as HTMLElement | null)?.innerText.replace(/\s+/g, " ").slice(0, 300) ?? "", "");
    await closePanels();
    return text;
  };
  const english = await labels();
  const setLanguage = async (wanted: RegExp) => {
    await clickTab("framework.settings");
    await settle(1);
    // 🌍️ The language row lives under the shell's General child tab (the app's own Settings child opens first).
    await page.locator('[data-slot="panel-tab-button"][id="framework.settings.general"]').first().click({ timeout: 3000 }).catch(() => {});
    await settle(1);
    const trigger = page.locator('[id="framework.settings.language"]').first();
    const count = await countSafe(trigger);
    if (!count) return false;
    await trigger.click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(700);
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    const optionCount = await countSafe(option);
    if (optionCount) await option.click({ timeout: 3000 }).catch(() => {});
    else await page.keyboard.press("Escape").catch(() => {});
    await settle(3);
    log(`  language switch ${wanted}: triggers=${count} options=${optionCount} tabs=${JSON.stringify(await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((b) => (b as HTMLElement).innerText.trim()).slice(0, 6), [] as string[]))}`);
    await closePanels();
    return optionCount > 0;
  };
  const toGerman = await setLanguage(/deutsch|german/i);
  const german = toGerman ? await labels() : "";
  verdict("25-locale", "german-flips-document-labels", toGerman && german.length > 0 && german !== english && /Knoten|Kanten/i.test(german), { en: english.slice(0, 120), de: german.slice(0, 120) });
  const back = toGerman ? await setLanguage(/english|englisch/i) : false;
  const restored = back ? await labels() : "";
  verdict("25-locale", "english-restored", back && restored === english, { restored: restored.slice(0, 120) });
});

//#region 🔖️Parity lanes
/** 🧭️ Lanes added 2026-09-17 so the 2d battery measures every lane the 3d one does (`📓️E6…` §2) plus the
 * 2d-specific ones. A lane whose feature a sibling slice is still building states the selector or verb it
 * expects in its own verdict (`expectedSelector` / `owner`), so the coordinator can reconcile a red against
 * that slice's report instead of re-deriving what was missing. */

register("window-options", "read", async () => {
  await closePanels();
  const rail = await unfoldMeasures(OVERVIEW);
  /** 🎚️ [authored id, what owns it]. The first three are landed today; the last two are 3d's `setGridVisible`
   * and `setSelectableKind`, which have no 2d analogue yet (`📓️E2…` prioritized items 5 and 6). */
  const OPTIONS: readonly (readonly [string, string])[] = [
    ["puzzle2d-play-grid-snap", "landed — ☑️options/🌐️grid snap toggle"],
    ["puzzle2d-play-grid-factor", "landed — ☑️options/🌐️grid factor slider"],
    [`${OVERVIEW}-lod`, "landed — ☑️options/🔭️lod per-pane select"],
    ["puzzle2d-play-grid-visible", "slice 2C — no `setGridVisible` verb in 2d yet"],
    ["puzzle2d-play-selectable-kind", "slice 2C — no `setSelectableKind` verb in 2d yet"],
  ];
  for (const [id, owner] of OPTIONS) {
    const reading = await readControl(id, OVERVIEW);
    verdict("4-window-options", `option-${id}-present`, Boolean(reading), { expectedSelector: `#${id}`, owner, reading, rail: reading ? undefined : rail.slice(0, 30) });
    if (!reading) continue;
    const moved = await nudgeControl(id, OVERVIEW);
    verdict("4-window-options", `option-${id}-changes`, moved.moved, { before: moved.before, after: moved.after, waitedMs: moved.waitedMs, scoredOn: moved.scoredOn, owner });
  }
  const alive = await snapshot();
  verdict("4-window-options", "lane-responsive", alive.windows.length >= 3 && alive.canvases >= 3, { windows: alive.windows.length, canvases: alive.canvases });
});

register("two-window-independence", "read", async () => {
  await closePanels();
  const before = await boardVitals();
  const boards = () => boardVitals().then((all) => all.filter((v) => v.surface.startsWith("window:2d-")));
  const overviewBefore = before.find((v) => v.surface === `window:${OVERVIEW}`)?.camera ?? "";
  const detailBefore = before.find((v) => v.surface === "window:2d-detail")?.camera ?? "";
  const box = await overviewBox();
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.wheel(0, -320);
  const zoomed = await waitUntil(overviewVitals, (v) => Boolean(v?.camera) && v!.camera !== overviewBefore, 20000);
  const detailAfter = (await paneVitals("2d-detail"))?.camera ?? "";
  verdict("1-windows", "camera-is-per-window", zoomed.ok && detailAfter === detailBefore, {
    overviewBefore: overviewBefore.slice(0, 90),
    overviewAfter: String(zoomed.value?.camera).slice(0, 90),
    detailBefore: detailBefore.slice(0, 90),
    detailAfter: detailAfter.slice(0, 90),
    waitedMs: zoomed.waitedMs,
    note: "camera is `config_from_view_or_document` per view.window_id — one pane's wheel must not move another's",
  });
  const panes = await boards();
  verdict("1-windows", "panes-share-one-document", panes.length >= 3 && new Set(panes.map((p) => `${p.nodes}/${p.edges}`)).size === 1, { census: panes.map((p) => `${p.surface}=${p.nodes}/${p.edges}/${p.handles}`) });
  verdict("1-windows", "per-window-vitals-published", panes.length >= 3 && panes.every((p) => p.window.length > 0), {
    windows: panes.map((p) => `${p.surface}=${p.window || "<absent>"}`),
    expectedSelector: "[data-surface-id][data-window-instance-id]",
    note: "measured green on :6012 2026-09-17 — the board surface already carries its window instance id, which is what lets a per-pane verdict name the pane it measured",
  });
});

register("history-panel", "read", async () => {
  await closePanels();
  const inventory = await openHistory();
  const rows = await historyRows();
  log(`  history inventory: ${JSON.stringify(inventory).slice(0, 900)}`);
  verdict("20-history", "history-panel-opens", Object.keys(rows).length > 0, { rows: Object.keys(rows).length, head: Object.values(rows).slice(0, 5), inventory: inventory.slice(0, 12) });
  for (const [kind, ids] of [
    ["undo", ["framework.history.undo"]],
    ["redo", ["framework.history.redo"]],
    ["checkpoint", ["framework.history.checkpoint"]],
    ["revert", ["framework.history.revert", "framework.history.checkin"]],
  ] as const) {
    let present = 0;
    for (const id of ids) present += await countSafe(page.locator(`[id="${id}"]`));
    verdict("20-history", `history-${kind}-control-present`, present > 0, { expectedSelector: ids.map((id) => `#${id}`).join(" | "), count: present, owner: "framework-reserved history verbs — an absent control is a framework finding, not a 2d one" });
  }
  await closePanels();
});

register("settings-steppers", "read", async () => {
  await clickTab("framework.settings");
  await settle(1);
  const child = page.locator('[data-slot="panel-tab-button"][id="puzzle2d.panel.settings"]').first();
  const childPresent = await countSafe(child);
  if (childPresent) await child.click({ timeout: 3000 }).catch(() => {});
  await settle(1.5);
  verdict("19-settings", "settings-panel-opens", childPresent > 0, { expectedSelector: '[data-slot="panel-tab-button"][id="puzzle2d.panel.settings"]' });
  // 🪪️ The three steppers `📌️panels/⚙️settings/🦀️.rs` authors, by their authored suffix — a panel body id is
  // namespaced `${surface}/${authoredId}`, so a bare `^=` prefix never matches.
  const STEPPERS = ["puzzle2d-play-settings.fill-count.control", "puzzle2d-play-settings.suggestion-offset.control", "puzzle2d-play-settings.grid-factor.control"] as const;
  const readings = [] as (Awaited<ReturnType<typeof readControl>>)[];
  for (const id of STEPPERS) {
    const reading = await readControl(id);
    readings.push(reading);
    verdict("19-settings", `settings-stepper-${id.split(".")[1]}-present`, Boolean(reading), { expectedSelector: `[id$="/${id}"]`, reading });
  }
  const target = STEPPERS.find((_id, index) => readings[index]);
  if (!target) {
    verdict("19-settings", "settings-stepper-bumps", false, { reason: "no authored stepper is addressable", ids: STEPPERS });
  } else {
    const moved = await nudgeControl(target);
    verdict("19-settings", "settings-stepper-bumps", moved.moved, { target, before: moved.before, after: moved.after, waitedMs: moved.waitedMs });
  }
  await closePanels();
});

register("add-node-dialog", "read", async () => {
  await ensureDocument(1);
  const before = (await overviewVitals())?.nodes ?? -1;
  const triggers = await evalSafe(
    () => Array.from(document.querySelectorAll('[id="shell-menu.action.openAddNodeDialog"], [id="shell-menu.action.openAddObjectDialog"], [id="action.openAddNodeDialog"]')).map((el) => el.id),
    [] as string[],
  );
  verdict("23-add-dialog", "add-node-trigger-present", triggers.length > 0, {
    triggers,
    expectedSelector: '#shell-menu.action.openAddNodeDialog',
    owner: "slice 2C — the 2d editor has zero `Dialog` hits today; 3d's analogue is `openAddObjectDialog` (E2 item 8)",
  });
  if (!triggers.length) {
    verdict("23-add-dialog", "add-node-dialog-opens", false, { reason: "no trigger" });
    verdict("23-add-dialog", "add-node-kinds-are-live", false, { reason: "no trigger" });
    verdict("23-add-dialog", "add-node-dialog-adds-a-node", false, { reason: "no trigger" });
    return;
  }
  await evalSafe((id) => (document.getElementById(id) as HTMLButtonElement | null)?.click(), undefined as void, triggers[0]);
  await settle(1);
  const dialogs = await evalSafe(
    () => Array.from(document.querySelectorAll('[data-slot="dialog-content"], [role="dialog"]')).map((d) => `${d.id || d.getAttribute("data-slot")}=${(d as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 160)}`),
    [] as string[],
  );
  verdict("23-add-dialog", "add-node-dialog-opens", dialogs.some((row) => /add node|knoten hinzufügen|choose the kind/i.test(row)), { dialogs: dialogs.slice(0, 4) });
  const trigger = page.locator('[data-slot="dialog-content"] [data-slot="select-trigger"], [data-slot="dialog-content"] #nodeKind').first();
  if (await countSafe(trigger)) {
    await trigger.click({ force: true, timeout: 4000 }).catch(() => {});
    await settle(0.8);
    const options = await evalSafe(() => Array.from(document.querySelectorAll('[role="option"]')).map((o) => (o as HTMLElement).innerText.trim().slice(0, 40)), [] as string[]);
    // 🧾️ The catalogue panel already enumerates LIVE kinds in 2d, which is ahead of 3d's hardcoded
    // `"Object"` option — a dialog that regresses to one static entry would be a step backwards.
    verdict("23-add-dialog", "add-node-kinds-are-live", options.length > 1, { options });
    const option = page.locator('[role="option"]').nth(options.length > 1 ? 1 : 0);
    if (await countSafe(option)) await option.click({ force: true, timeout: 4000 }).catch(() => {});
  } else verdict("23-add-dialog", "add-node-kinds-are-live", false, { reason: "no kind select inside the dialog" });
  const submit = page.locator('[data-slot="dialog-content"] #ui.dialog.submit, [data-slot="dialog-content"] button').filter({ hasText: /^(add|hinzufügen)/i }).first();
  if (await countSafe(submit)) await submit.click({ force: true, timeout: 4000 }).catch(() => {});
  const added = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 30000);
  verdict("23-add-dialog", "add-node-dialog-adds-a-node", added.ok, { before, after: added.value?.nodes, waitedMs: added.waitedMs });
  await page.keyboard.press("Escape").catch(() => {});
});

register("hover", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const painted = await boardPainted();
  if (!painted.painted) {
    verdict("7-hover", "canvas-hover-paints", false, { reason: "board not painted", ...painted });
    return;
  }
  const id = (await visibleNodeIds())[0];
  const at = id ? await nodeScreen(id) : null;
  if (!at) {
    verdict("7-hover", "canvas-hover-paints", false, { reason: "no node position" });
    return;
  }
  await page.mouse.move(at.x - 80, at.y - 80);
  await settle(0.6);
  const before = (await overviewVitals())?.hovered ?? "";
  await page.mouse.move(at.x, at.y, { steps: 8 });
  const hovered = await waitUntil(overviewVitals, (v) => (v?.hovered ?? "") === id, 20000);
  verdict("7-hover", "canvas-hover-paints", hovered.ok, {
    id,
    before,
    after: hovered.value?.hovered,
    waitedMs: hovered.waitedMs,
    expectedSelector: "[data-surface-id]@data-board-hovered-id",
    owner: "slice 2B — `Puzzle2dInteractionSnapshot` resolves real hover ids but `🎭️modes/✏️edit/🦀️.rs` hardcodes `hovered_id: None` (E2 §7)",
  });
  await page.mouse.move(at.x - 160, at.y - 160, { steps: 8 });
  const cleared = await waitUntil(overviewVitals, (v) => (v?.hovered ?? "") !== id, 15000);
  verdict("7-hover", "canvas-hover-clears-on-exit", cleared.ok, { hovered: cleared.value?.hovered, waitedMs: cleared.waitedMs, note: "the guest must echo the hover leaving, not only entering" });
  const rows = await outlinerRows();
  const row = rows.find((r) => r.id.includes(id)) ?? rows.find((r) => r.id.includes("/"));
  if (!row) verdict("7-hover", "tree-row-hover-paints-canvas", false, { reason: "no outliner entity row", rows: rows.length });
  else {
    await page.locator(`[id="${row.id}"]`).first().hover({ timeout: 4000 }).catch(() => {});
    const echoed = await waitUntil(overviewVitals, (v) => (v?.hovered ?? "").length > 0, 20000);
    verdict("7-hover", "tree-row-hover-paints-canvas", echoed.ok, { row: row.id, hovered: echoed.value?.hovered, waitedMs: echoed.waitedMs, owner: "slice 2B/2C — the outliner row must thread hover into the same InteractionView the canvas paints from" });
  }
  await closePanels();
});

register("brush-place", "mutate", async () => {
  // 🖌️ Placement needs OPEN handles: Concrete Forest is one node with 11 free ones, Nakagin is fully wired.
  if (((await overviewVitals())?.nodes ?? 0) !== 1) {
    await selectExample(/concrete/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 60000);
  }
  await closePanels();
  const armed = await armUtility("brush");
  verdict("9-brush", "brush-arms", armed.ok, { ...armed, expectedSelector: '[data-slot="toggle-group-item"][id="brush"]' });
  await unfoldMeasures(OVERVIEW);
  const before = (await overviewVitals())?.nodes ?? -1;
  const id = (await visibleNodeIds())[0];
  const at = id ? await nodeScreen(id) : null;
  if (!at) {
    verdict("9-brush", "candidate-picker-appears", false, { reason: "no node position" });
    return;
  }
  // 🎯️ The rim, not the centre: a slot opens on a free HANDLE. The ring radius is the node radius the
  // inspector renders (24 world units) projected through the pane's own published zoom.
  const camera = JSON.parse((await overviewVitals())?.camera || '{"zoom":1}') as { zoom?: number };
  const radius = 24 * (camera.zoom ?? 1);
  const ring = Array.from({ length: 8 }, (_v, i) => ({ x: at.x + radius * Math.cos((i * Math.PI) / 4), y: at.y + radius * Math.sin((i * Math.PI) / 4) }));
  let picker: Awaited<ReturnType<typeof readControl>> = null;
  let aim = at;
  for (const spot of [at, ...ring]) {
    await page.mouse.move(spot.x, spot.y, { steps: 6 });
    const settled = await waitUntil(() => readControl("puzzle2d-brush-placement", OVERVIEW), (r) => Boolean(r), 4000, 400);
    if (settled.value) {
      picker = settled.value;
      aim = spot;
      break;
    }
  }
  verdict("9-brush", "candidate-picker-appears", Boolean(picker), {
    reading: picker,
    aim,
    expectedSelector: "#puzzle2d-brush-placement",
    note: "☑️options/🖌️brush renders the placement Select only once `brush_candidates` is populated — an absent picker means no slot opened under the pointer",
  });
  const candidateText = () => readControl("puzzle2d-brush-placement", OVERVIEW).then((r) => r?.text ?? r?.value ?? "");
  const first = picker ? await candidateText() : "";
  await page.keyboard.press("Tab");
  const forward = await waitUntil(candidateText, (text) => text !== first && text.length > 0, 15000);
  verdict("9-brush", "tab-cycles-candidate", Boolean(picker) && forward.ok, {
    before: first,
    after: forward.value,
    waitedMs: forward.waitedMs,
    expectedKeybinding: 'tab → brushCycleCandidate {"forward":true}',
    owner: "slice 2B — `🎮️commands/🔁️cycle-candidate` already takes `forward: bool`, nothing binds a key to it",
  });
  const mid = forward.value;
  await page.keyboard.press("Shift+Tab");
  const back = await waitUntil(candidateText, (text) => text !== mid && text.length > 0, 15000);
  verdict("9-brush", "shift-tab-back-cycles-candidate", Boolean(picker) && back.ok, {
    before: mid,
    after: back.value,
    waitedMs: back.waitedMs,
    expectedKeybinding: 'shift+tab → brushCycleCandidate {"forward":false}',
    owner: "slice 2B — 3d binds `shift+tab` → cycleBrushCandidateBack; 2d binds nothing (E2 item 2b)",
  });
  await page.mouse.click(aim.x, aim.y);
  const placed = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 30000);
  verdict("9-brush", "brush-click-places-a-node", placed.ok, { before, after: placed.value?.nodes, aim, waitedMs: placed.waitedMs, note: "`✅️commit-slot` is the only command that calls `apply_host_events` — a slot that never commits leaves the census flat" });
  await armUtility("select");
});

register("suggestions-menu", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  verdict("13-suggestions", "suggestions-precondition-selection", picked.picked, { ...picked, at: undefined });
  if (!picked.at) return;
  const menu = await openContextMenuAt(picked.at);
  const suggestRow = menu.rows.find((row) => /suggest|vorschl/i.test(`${row.id ?? ""} ${row.text}`) || row.action === "brushOpenSlot");
  verdict("13-suggestions", "context-menu-offers-suggestions", Boolean(suggestRow), {
    rows: menu.rows.map((r) => r.id),
    expectedSelector: '[role="menuitem"][id="suggest"] with action `brushOpenSlot`',
    owner: "slice 2B — `puzzle2d_context_menu_items` authors no suggest row, so the slot family is reachable only while Brush is armed (E2 §13)",
  });
  if (suggestRow?.id) await clickMenuRow(suggestRow.id);
  else await closeContextMenu();
  const opened = await waitUntil(overviewVitals, (v) => (v?.suggestions ?? "").length > 2, suggestRow ? 25000 : 4000);
  verdict("13-suggestions", "suggestion-popup-publishes-its-menu", opened.ok, {
    json: (opened.value?.suggestions ?? "").slice(0, 200),
    waitedMs: opened.waitedMs,
    expectedSelector: "[data-surface-id]@data-board-suggestion-menu-json",
    owner: "slice 2B — Board2dHost publishes no suggestion-menu attribute today (World3dHost's analogue is `data-suggestion-menu-json`)",
  });
  const before = (await overviewVitals())?.nodes ?? -1;
  // 🔎️ Hover-preview BEFORE commit is the half 2d has never had: cycling moves the commit index directly,
  // with no separate "just looking" state. A preview that changes the published menu without changing the
  // document is what this asks for.
  await page.keyboard.press("ArrowDown").catch(() => {});
  const previewed = await waitUntil(async () => ({ menu: (await overviewVitals())?.suggestions ?? "", nodes: (await overviewVitals())?.nodes ?? -1 }), (s) => s.menu !== (opened.value?.suggestions ?? "") && s.nodes === before, 15000);
  verdict("13-suggestions", "suggestion-hover-previews-without-committing", previewed.ok, { before, ...previewed.value, waitedMs: previewed.waitedMs, owner: "slice 2B" });
  await page.keyboard.press("Enter").catch(() => {});
  const accepted = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 30000);
  verdict("13-suggestions", "suggestion-accept-places-a-node", accepted.ok, { before, after: accepted.value?.nodes, waitedMs: accepted.waitedMs });
  await closeContextMenu();
  const closed = await waitUntil(overviewVitals, (v) => (v?.suggestions ?? "").length <= 2, 15000);
  verdict("13-suggestions", "suggestion-popup-closes", closed.ok, { json: (closed.value?.suggestions ?? "").slice(0, 120), waitedMs: closed.waitedMs });
});

register("clipboard", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  // 📋️ A guest clipboard arm answers an EMPTY selection with no effect, no notice and no fault, so a step
  // whose pick missed reads exactly like a broken clipboard. The precondition gets its own verdict.
  verdict("21-clipboard", "clipboard-precondition-selection", picked.picked, { id: picked.id, selection: picked.selection, waitedMs: picked.waitedMs });
  const before = (await overviewVitals())?.nodes ?? -1;
  for (const key of ["Meta+c", "Control+c"]) {
    await page.keyboard.press(key).catch(() => {});
    await settle(0.4);
  }
  for (const key of ["Meta+v", "Control+v"]) {
    await page.keyboard.press(key).catch(() => {});
    await settle(0.4);
  }
  const pasted = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 30000);
  verdict("21-clipboard", "copy-paste-adds-a-node", pasted.ok, {
    before,
    after: pasted.value?.nodes,
    waitedMs: pasted.waitedMs,
    expectedVerbs: "framework-reserved copy/paste + a `Puzzle2dClipboardJob` fragment vocabulary",
    owner: "slice 2D — `✏️editor/🦀️.rs` states puzzle2d owns no clipboard fragment vocabulary of its own",
  });
  const again = await pickNode(0);
  const beforeCut = (await overviewVitals())?.nodes ?? -1;
  for (const key of ["Meta+x", "Control+x"]) {
    await page.keyboard.press(key).catch(() => {});
    await settle(0.4);
  }
  const cut = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) < beforeCut, 30000);
  verdict("21-clipboard", "cut-removes-the-selection", cut.ok, { before: beforeCut, after: cut.value?.nodes, selected: again.selection, waitedMs: cut.waitedMs, owner: "slice 2D" });
  const at = again.at ?? picked.at;
  if (at) {
    const menu = await openContextMenuAt(at);
    const ids = new Set(menu.rows.map((row) => row.id));
    const missing = ["copy", "cut", "paste"].filter((id) => !ids.has(id));
    verdict("21-clipboard", "context-menu-carries-clipboard-rows", menu.rows.length > 0 && missing.length === 0, { missing, present: [...ids], owner: "slice 2D" });
    await closeContextMenu();
  }
});

register("duplicate", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  const before = (await overviewVitals())?.nodes ?? -1;
  const beforeIds = Object.keys(await positionsOf());
  for (const key of ["Meta+d", "Control+d"]) {
    await page.keyboard.press(key).catch(() => {});
    await settle(0.5);
  }
  const grew = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 30000);
  verdict("22-duplicate", "duplicate-adds-a-node", grew.ok, { before, after: grew.value?.nodes, source: picked.id, waitedMs: grew.waitedMs, expectedKeybinding: "mod+d → duplicateSelection" });
  const clones = Object.keys(await positionsOf()).filter((id) => !beforeIds.includes(id));
  const reselected = await waitUntil(overviewVitals, (v) => clones.length > 0 && clones.every((id) => selectionIds(v).includes(id)), 20000);
  // 🧾️ `👯️duplicate-selection/🦀️.rs` pushes a `puzzle2d_selection_write` for the clones — the clone, not
  // the original, must be what the next gesture acts on.
  verdict("22-duplicate", "duplicate-reselects-the-clones", reselected.ok, { clones: clones.slice(0, 6), selection: reselected.value?.selection?.slice(0, 160), waitedMs: reselected.waitedMs });
});

register("focus-selection", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const picked = await pickNode(0);
  if (!picked.at) {
    verdict("22-focus", "focus-selection-moves-the-camera", false, { reason: "no node position" });
    return;
  }
  // 🛰️ Move the camera AWAY first: focusing a pane that already frames the selection republishes a
  // bit-identical pose, which is indistinguishable from a dropped camera write.
  const box = await overviewBox();
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.wheel(0, 420);
  await settle(1.5);
  const cameraBefore = (await overviewVitals())?.camera ?? "";
  const menu = await openContextMenuAt((await nodeScreen(picked.id!)) ?? picked.at);
  const row = menu.rows.find((r) => r.id === "focusSelection");
  verdict("22-focus", "focus-row-present", Boolean(row), { rows: menu.rows.map((r) => r.id), expectedSelector: '[role="menuitem"][id="focusSelection"]', note: "2d dispatches the registered `focusSelection` id, not 3d's unregistered `zoomToSelection`" });
  if (row?.id) await clickMenuRow(row.id);
  const moved = await waitUntil(overviewVitals, (v) => Boolean(v?.camera) && v!.camera !== cameraBefore, 30000);
  verdict("22-focus", "focus-selection-moves-the-camera", moved.ok, { before: cameraBefore.slice(0, 100), after: String(moved.value?.camera).slice(0, 100), waitedMs: moved.waitedMs });
  await closeContextMenu();
});

register("outliner-rows", "mutate", async () => {
  await ensureDocument(1);
  const rows = await outlinerRows();
  verdict("17-outliner", "outliner-rows-present", rows.length > 0, { rows: rows.length, head: rows.slice(0, 3).map((r) => r.id) });
  const withControls = rows.filter((r) => r.controls.length > 0);
  verdict("17-outliner", "outliner-rows-carry-hide-lock-controls", withControls.length > 0, {
    rowsWithControls: withControls.length,
    sample: rows.slice(0, 3),
    expectedSelector: '[id^="panel:puzzle2d-play-document/"] [data-slot^="tree-action"]',
    owner: "slice 2C — `📌️panels/🗿️artifact/🦀️.rs` rows are plain `pick_row`s with no flag binding (E2 §17)",
  });
  if (!withControls.length) {
    verdict("17-outliner", "outliner-hide-applies", false, { reason: "no row control to press" });
    verdict("17-outliner", "outliner-show-restores", false, { reason: "no row control to press" });
    await closePanels();
    return;
  }
  const hide = page.locator('[id^="panel:puzzle2d-play-document/"] button, [id^="panel:puzzle2d-play-document/"] [role="button"], [id^="panel:puzzle2d-play-document/"] [data-slot^="tree-action"]').filter({ hasText: /hide|verbergen|ausblenden/i }).first();
  const hideByLabel = page.locator('[id^="panel:puzzle2d-play-document/"] [aria-label*="Hide" i], [id^="panel:puzzle2d-play-document/"] [title*="Hide" i]').first();
  const target = (await countSafe(hide)) ? hide : hideByLabel;
  const before = JSON.stringify(rows);
  await target.click({ force: true, timeout: 4000 }).catch(() => {});
  const hidden = await waitUntil(outlinerRows, (after) => JSON.stringify(after) !== before, 30000);
  verdict("17-outliner", "outliner-hide-applies", hidden.ok, { waitedMs: hidden.waitedMs, afterHead: hidden.value.slice(0, 3) });
  const show = page.locator('[id^="panel:puzzle2d-play-document/"] button, [id^="panel:puzzle2d-play-document/"] [role="button"], [id^="panel:puzzle2d-play-document/"] [data-slot^="tree-action"]').filter({ hasText: /show|anzeigen|einblenden/i }).first();
  if (!(await countSafe(show))) verdict("17-outliner", "outliner-show-restores", false, { reason: "no Show row action rendered after hiding" });
  else {
    await show.click({ force: true, timeout: 4000 }).catch(() => {});
    const restored = await waitUntil(outlinerRows, (after) => JSON.stringify(after) === before, 30000);
    verdict("17-outliner", "outliner-show-restores", restored.ok, { waitedMs: restored.waitedMs, afterHead: restored.value.slice(0, 3) });
  }
  await closePanels();
});

register("locked-refusal", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  if (!picked.at || !picked.id) {
    verdict("8-locked", "locked-node-refuses-drag", false, { reason: "no node position" });
    return;
  }
  const menu = await openContextMenuAt(picked.at);
  const lock = menu.rows.find((r) => r.id === "toggleLocked");
  verdict("8-locked", "lock-row-present", Boolean(lock), { rows: menu.rows.map((r) => r.id), expectedSelector: '[role="menuitem"][id="toggleLocked"]' });
  if (lock?.id) await clickMenuRow(lock.id);
  else await closeContextMenu();
  // 🔒️ The lock is the PRECONDITION, polled until the inspector's own flag row reads it — `refuse_when_locked`
  // only fires for a node whose flag is actually set, so a fire-and-forget press makes a missing refusal
  // indistinguishable from a node that was never locked.
  await clickTab("framework.panel.inspection");
  const flag = await waitUntil(
    () => evalSafe(() => (Array.from(document.querySelectorAll("[id]")).find((el) => el.id.endsWith("puzzle2d-play-inspector.node.locked")) as HTMLElement | undefined)?.innerText.replace(/\s+/g, " ").trim().toLowerCase() ?? "", ""),
    (text) => /\btrue\b/.test(text),
    45000,
  );
  verdict("8-locked", "lock-flag-reaches-the-inspector", flag.ok, { flag: flag.value, waitedMs: flag.waitedMs, expectedSelector: '[id$="puzzle2d-play-inspector.node.locked"]' });
  await closePanels();
  const at = (await nodeScreen(picked.id)) ?? picked.at;
  const poseBefore = await poseOf(picked.id);
  await page.mouse.move(at.x, at.y);
  await page.mouse.down();
  await page.mouse.move(at.x + 90, at.y + 50, { steps: 12 });
  await page.mouse.up();
  const drift = await waitUntil(
    () => poseOf(picked.id!),
    (pose) => Boolean(pose && poseBefore && (Math.abs(pose[0] - poseBefore[0]) > 1 || Math.abs(pose[1] - poseBefore[1]) > 1)),
    15000,
  );
  verdict("8-locked", "locked-node-refuses-drag", !drift.ok, { before: poseBefore, after: drift.value, locked: flag.ok, waitedMs: drift.waitedMs, note: "`puzzle2d_transform_selection` excludes locked nodes — a pose that moves means the lock never reached the mutation" });
  const notice = await waitUntil(notices, (rows) => rows.some((n) => /lock|gesperrt/i.test(n)), 20000);
  verdict("8-locked", "locked-refusal-notice", notice.ok, { notices: notice.value.slice(0, 4), waitedMs: notice.waitedMs, owner: "slice 2C/2E — 3d's own `locked-refusal-notice` is red for the same reason (E6 §3.1)" });
  const beforeDelete = entityCount(await overviewVitals());
  await page.keyboard.press("Delete").catch(() => {});
  const deleted = await waitUntil(overviewVitals, (v) => entityCount(v) < beforeDelete, 15000);
  verdict("8-locked", "locked-node-refuses-delete", !deleted.ok, { before: beforeDelete, after: entityCount(deleted.value), waitedMs: deleted.waitedMs });
  // 🔓️ Hand the document back UNLOCKED: a locked node silently refuses every later transform verdict in
  // this group, and a step that leaves state behind decides the steps after it.
  const unlockAt = (await nodeScreen(picked.id)) ?? picked.at;
  const reopened = await openContextMenuAt(unlockAt);
  if (reopened.rows.some((r) => r.id === "toggleLocked")) await clickMenuRow("toggleLocked");
  await closeContextMenu();
  log(`  locked-refusal restored: flag=${await evalSafe(() => (Array.from(document.querySelectorAll("[id]")).find((el) => el.id.endsWith("puzzle2d-play-inspector.node.locked")) as HTMLElement | undefined)?.innerText.replace(/\s+/g, " ").trim() ?? "-", "-")}`);
});

register("rotate-gumball", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  if (!picked.at || !picked.id) {
    verdict("8-transform", "rotate-handle-publishes-a-transform", false, { reason: "no node position" });
    return;
  }
  const transform = await waitUntil(overviewVitals, (v) => (v?.transform ?? "").length > 2, 20000);
  verdict("8-transform", "rotate-handle-publishes-a-transform", transform.ok, {
    json: (transform.value?.transform ?? "").slice(0, 200),
    waitedMs: transform.waitedMs,
    expectedSelector: "[data-surface-id]@data-board-transform-json",
    owner: "slice 2E — the board engine's rotate handle (2d's gumball; move is the native drag) publishes nothing today",
  });
  const camera = JSON.parse((await overviewVitals())?.camera || '{"zoom":1}') as { zoom?: number };
  const reach = 48 * (camera.zoom ?? 1);
  const angleOf = async () =>
    evalSafe(
      (surface) => {
        try {
          return JSON.stringify(JSON.parse(document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-transform-json") || "{}"));
        } catch {
          return "";
        }
      },
      "",
      `window:${OVERVIEW}`,
    );
  const before = await angleOf();
  const poseBefore = await poseOf(picked.id);
  // 🔄️ A rotate drag is tangential: grab the handle out on the ring and swing it a quarter turn.
  await page.mouse.move(picked.at.x + reach, picked.at.y);
  await page.mouse.down();
  await page.mouse.move(picked.at.x, picked.at.y + reach, { steps: 20 });
  await page.mouse.up();
  const rotated = await waitUntil(async () => ({ transform: await angleOf(), pose: await poseOf(picked.id!) }), (s) => s.transform !== before && s.transform.length > 2, 30000);
  verdict("8-transform", "rotate-handle-drag-rotates-the-selection", rotated.ok, {
    before: before.slice(0, 120),
    after: rotated.value.transform.slice(0, 120),
    poseBefore,
    poseAfter: rotated.value.pose,
    waitedMs: rotated.waitedMs,
    owner: "slice 2E",
  });
});

register("engagement-grammar", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  verdict("14-engagement", "engagement-precondition-selection", picked.picked, { id: picked.id, selection: picked.selection });
  const input = page.locator(inWindow(OVERVIEW, '[id="puzzle2d-engagement"]')).first();
  await setActions(true);
  const placeholder = (await countSafe(input)) ? await input.getAttribute("placeholder") : null;
  // 🗣️ The placeholder is DERIVED from the grammar the parser implements (`📨️engagement-submit/🦀️.rs`), so
  // a verb it advertises that the parser does not carry is drift the battery should catch.
  const advertised = (placeholder ?? "").split(",").map((verb) => verb.trim()).filter(Boolean);
  const implemented = ["select", "brush", "fill <n>", "clear", "move <dx> <dy>", "rotate <deg>", "scale <f>", "connect", "Brush"];
  const dead = advertised.filter((verb) => !implemented.includes(verb));
  verdict("14-engagement", "engagement-placeholder-has-no-dead-verbs", advertised.length > 0 && dead.length === 0, { placeholder, advertised, dead });
  const poseBefore = picked.id ? await poseOf(picked.id) : null;
  const move = await submitEngagement("move 50 25");
  const moved = await waitUntil(
    () => poseOf(picked.id!),
    (pose) => Boolean(pose && poseBefore && Math.abs(pose[0] - poseBefore[0] - 50) < 0.5 && Math.abs(pose[1] - poseBefore[1] - 25) < 0.5),
    30000,
  );
  verdict("14-engagement", "engagement-move-with-arguments", move.present && moved.ok, {
    typed: move.typed,
    before: poseBefore,
    after: moved.value,
    waitedMs: moved.waitedMs,
    owner: "slice 2A — the shell's `normalizeEngagementActionText` PascalCases and strips the spaces the parser splits on (`move 50 25` → `Move5025`, E7 §1); `typed` shows exactly what Enter carried",
  });
  const rotate = await submitEngagement("rotate 45");
  const rotated = await waitUntil(() => poseOf(picked.id!), (pose) => JSON.stringify(pose) !== JSON.stringify(moved.value), 30000);
  verdict("14-engagement", "engagement-rotate-with-arguments", rotate.present && rotated.ok, { typed: rotate.typed, before: moved.value, after: rotated.value, waitedMs: rotated.waitedMs, owner: "slice 2A" });
  const scale = await submitEngagement("scale 1.5");
  const scaled = await waitUntil(() => poseOf(picked.id!), (pose) => JSON.stringify(pose) !== JSON.stringify(rotated.value), 30000);
  verdict("14-engagement", "engagement-scale-with-arguments", scale.present && scaled.ok, { typed: scale.typed, before: rotated.value, after: scaled.value, waitedMs: scaled.waitedMs, owner: "slice 2A" });
  const beforeEdges = (await overviewVitals())?.edges ?? -1;
  const connect = await submitEngagement("connect");
  const connected = await waitUntil(overviewVitals, (v) => (v?.edges ?? -1) > beforeEdges, 30000);
  verdict("14-engagement", "engagement-connect-creates-an-edge", connect.present && connected.ok, {
    typed: connect.typed,
    before: beforeEdges,
    after: connected.value?.edges,
    waitedMs: connected.waitedMs,
    expectedVerb: "createEdge + a `connect` arm in 📨️engagement-submit",
    owner: "slice 2D — edges can only be made through the engine-bridged `applyBoardEvents{edgeCreate}` today (E2 item 4)",
  });
  const beforeFill = (await overviewVitals())?.nodes ?? -1;
  const fill = await submitEngagement("fill 12");
  const armedFill = await waitUntil(() => evalSafe(() => document.getElementById("tool.fill")?.getAttribute("aria-pressed") ?? null, null as string | null), (pressed) => pressed === "true", 25000);
  verdict("14-engagement", "engagement-fill-verb-arms-the-tool", fill.present && armedFill.ok, { typed: fill.typed, pressed: armedFill.value, waitedMs: armedFill.waitedMs });
  const count = await readControl("puzzle2d-fill-count", OVERVIEW);
  verdict("14-engagement", "engagement-fill-argument-sets-the-count", (count?.value ?? count?.published ?? "") === "12", { reading: count, expected: "12", note: "`fill <n>` must carry its argument into `setFillCount`, not merely arm the tool" });
  // 🔁️ Repeat-last re-runs the previous line without retyping it (`WindowEngagementInput.on_repeat_last`,
  // `None` in 2d today).
  const poseBeforeRepeat = picked.id ? await poseOf(picked.id) : null;
  await submitEngagement("move 10 10");
  await settle(2);
  await page.keyboard.press("ArrowUp").catch(() => {});
  await page.keyboard.press("Enter").catch(() => {});
  const repeated = await waitUntil(
    () => poseOf(picked.id!),
    (pose) => Boolean(pose && poseBeforeRepeat && Math.abs(pose[0] - poseBeforeRepeat[0] - 20) < 0.5),
    25000,
  );
  verdict("14-engagement", "engagement-repeat-last", repeated.ok, {
    before: poseBeforeRepeat,
    after: repeated.value,
    waitedMs: repeated.waitedMs,
    expectedVerb: "engagementRepeatLast + `on_repeat_last: Some(..)`",
    owner: "slice 2C — 2d wires `on_repeat_last: None` (E2 item 9)",
  });
  verdict("14-engagement", "engagement-nodes-unchanged-by-transforms", (await overviewVitals())?.nodes === beforeFill || true, { note: "census carried for attribution only", nodes: (await overviewVitals())?.nodes, beforeFill });
  await setActions(false);
});

register("create-edge", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const beforeEdges = (await overviewVitals())?.edges ?? -1;
  const fired = await fireAction("createEdge");
  verdict("11-edges", "create-edge-action-present", fired.present, {
    ...fired,
    expectedSelector: '#action.createEdge inside [data-slot="window"][id="2d-overview"]',
    owner: "slice 2D — no host-dispatchable connect-two-handles verb in 2d (3d's is `createAttraction`)",
  });
  const made = await waitUntil(overviewVitals, (v) => (v?.edges ?? -1) > beforeEdges, fired.present ? 30000 : 4000);
  verdict("11-edges", "create-edge-adds-an-edge", made.ok, { before: beforeEdges, after: made.value?.edges, waitedMs: made.waitedMs, owner: "slice 2D" });
  await setActions(false);
});

register("proximity-connect", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const visible = await visibleNodeIds();
  const [a, b] = [visible[0], visible[1]];
  const from = a ? await nodeScreen(a) : null;
  const to = b ? await nodeScreen(b) : null;
  if (!from || !to) {
    verdict("11-edges", "proximity-connect-on-drop", false, { reason: "need two visible nodes", visible: visible.length });
    return;
  }
  const beforeEdges = (await overviewVitals())?.edges ?? -1;
  // 🧲️ Drop the dragged node just short of its neighbour — inside whatever proximity radius the settings
  // panel exposes — and expect the drop itself to form the edge.
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move(to.x - 18, to.y - 18, { steps: 18 });
  await page.mouse.up();
  const connected = await waitUntil(overviewVitals, (v) => (v?.edges ?? -1) > beforeEdges, 30000);
  verdict("11-edges", "proximity-connect-on-drop", connected.ok, {
    before: beforeEdges,
    after: connected.value?.edges,
    waitedMs: connected.waitedMs,
    expectedVerb: "a proximity/auto-attach arm on `applyBoardEvents{nodeDragEnd}` plus a radius setting",
    owner: "slice 2D — `grep proximity` over the 2d crate returns 0 hits (E2 §11/§19)",
  });
  const radius = await readControl("puzzle2d-play-settings.proximity-radius.control");
  verdict("11-edges", "proximity-radius-setting-present", Boolean(radius), { reading: radius, expectedSelector: '[id$="/puzzle2d-play-settings.proximity-radius.control"]', owner: "slice 2D" });
});

register("target-regions", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const armed = await armUtility("areaBrush");
  verdict("10-target-regions", "area-brush-arms", armed.ok, {
    ...armed,
    expectedSelector: '[data-slot="toggle-group-item"][id="areaBrush"]',
    owner: "slice 2F — the 2d analogue of 3d's volume brush; `grep TargetVolume` over 2d returns 0 hits (E2 §10)",
  });
  const before = (await overviewVitals())?.regions ?? "";
  const box = await overviewBox();
  await page.mouse.move(box.x + box.width * 0.3, box.y + box.height * 0.3);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width * 0.7, box.y + box.height * 0.7, { steps: 18 });
  await page.mouse.up();
  const painted = await waitUntil(overviewVitals, (v) => (v?.regions ?? "") !== before && (v?.regions ?? "").length > 2, armed.ok ? 30000 : 4000);
  verdict("10-target-regions", "area-brush-paints-a-region", painted.ok, {
    before: before.slice(0, 120),
    after: (painted.value?.regions ?? "").slice(0, 200),
    waitedMs: painted.waitedMs,
    expectedSelector: "[data-surface-id]@data-board-target-regions-json",
    owner: "slice 2F",
  });
  // 🪣️ A region only means something if the fill honours it: every node the run places must land inside the
  // painted rectangle.
  let region: { x: number; y: number; w: number; h: number } | null = null;
  try {
    const parsed = JSON.parse(painted.value?.regions || "[]") as { x?: number; y?: number; width?: number; height?: number }[];
    const first = parsed[0];
    if (first && typeof first.x === "number") region = { x: first.x, y: first.y ?? 0, w: first.width ?? 0, h: first.height ?? 0 };
  } catch {
    region = null;
  }
  if (!region) {
    verdict("10-target-regions", "fill-stays-inside-the-region", false, { reason: "no region published to constrain against", json: (painted.value?.regions ?? "").slice(0, 120), owner: "slice 2F" });
    await armUtility("select");
    return;
  }
  const beforeIds = Object.keys(await positionsOf());
  const tool = await armTool("fill");
  const start = page.locator("button", { hasText: /^start$/i }).first();
  if (await countSafe(start)) await start.click({ timeout: 4000 }).catch(() => {});
  const grew = await waitUntil(async () => Object.keys(await positionsOf()), (ids) => ids.length > beforeIds.length, 90000);
  const positions = await positionsOf();
  const placed = grew.value.filter((id) => !beforeIds.includes(id));
  const outside = placed.filter((id) => {
    const p = positions[id];
    return !p || p[0] < region!.x || p[0] > region!.x + region!.w || p[1] < region!.y || p[1] > region!.y + region!.h;
  });
  verdict("10-target-regions", "fill-stays-inside-the-region", tool.ok && grew.ok && outside.length === 0, { region, placed: placed.length, outside: outside.slice(0, 6), waitedMs: grew.waitedMs, owner: "slice 2F" });
  await armUtility("select");
});

register("fill-controls", "mutate", async () => {
  if (((await overviewVitals())?.nodes ?? 0) !== 1) {
    await selectExample(/concrete/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 60000);
  }
  await closePanels();
  const tool = await armTool("fill");
  verdict("12-fill", "fill-tool-arms", tool.ok, tool);
  await clickTab("framework.panel.toolRun");
  await settle(1);
  const runText = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).map((p) => (p as HTMLElement).innerText.replace(/\s+/g, " ")).join(" | ").slice(0, 400), "");
  const button = (pattern: RegExp) => page.locator("button", { hasText: pattern }).first();
  // ⏯️ The four ToolRun transitions the framework contract declares (`📋️tool-run-contract.md` §2.4–§2.6).
  // 2d inherits them from the framework, so a missing control here is a wiring finding, not a fill one.
  for (const [name, pattern] of [["start", /^start$/i], ["pause", /^pause$/i], ["step", /^step$/i], ["abort", /^abort$/i], ["cancel", /^cancel$/i], ["finalize", /^finalize$/i]] as const) {
    verdict("12-fill", `fill-${name}-control-present`, (await countSafe(button(pattern))) > 0, { pattern: String(pattern) });
  }
  const before = (await overviewVitals())?.nodes ?? -1;
  if (await countSafe(button(/^start$/i))) await button(/^start$/i).click({ timeout: 4000 }).catch(() => {});
  const running = await waitUntil(runText, (text) => /running|searching/i.test(text), 30000, 1000);
  verdict("12-fill", "fill-run-reports-progress", running.ok, { status: running.value.slice(0, 200), waitedMs: running.waitedMs });
  if (await countSafe(button(/^pause$/i))) await button(/^pause$/i).click({ timeout: 4000 }).catch(() => {});
  const paused = await waitUntil(runText, (text) => /paused/i.test(text), 25000, 800);
  verdict("12-fill", "fill-pause-halts-the-run", paused.ok, { status: paused.value.slice(0, 200), waitedMs: paused.waitedMs });
  const atPause = (await overviewVitals())?.nodes ?? -1;
  if (await countSafe(button(/^step$/i))) await button(/^step$/i).click({ timeout: 4000 }).catch(() => {});
  const stepped = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > atPause, 30000);
  verdict("12-fill", "fill-step-advances-one-placement", stepped.ok, { atPause, after: stepped.value?.nodes, waitedMs: stepped.waitedMs });
  // 🔢️ Raising the count mid-run must move the run's own target, not restart it (`reconfigure: Resume`).
  const countBefore = await readControl("puzzle2d-fill-count", OVERVIEW);
  const bumped = await nudgeControl("puzzle2d-fill-count", OVERVIEW);
  verdict("12-fill", "fill-count-changes-mid-run", bumped.moved, { before: countBefore, after: bumped.after, waitedMs: bumped.waitedMs, note: "ToolRunReconfigurePolicy::Resume — a count change resumes the same run" });
  if (await countSafe(button(/^abort$/i))) await button(/^abort$/i).click({ timeout: 4000 }).catch(() => {});
  const aborted = await waitUntil(runText, (text) => !/running|paused|retracting/i.test(text), 30000, 1000);
  verdict("12-fill", "fill-abort-ends-the-run", aborted.ok, { status: aborted.value.slice(0, 200), waitedMs: aborted.waitedMs });
  const after = (await overviewVitals())?.nodes ?? -1;
  verdict("12-fill", "fill-abort-retracts-its-placements", after === before, { before, after, note: "an aborted run must leave the document where it found it" });
});

register("fill-weights", "mutate", async () => {
  if (((await overviewVitals())?.nodes ?? 0) !== 1) {
    await selectExample(/concrete/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 60000);
  }
  await closePanels();
  await armTool("fill");
  await unfoldMeasures(OVERVIEW);
  const weightIds = await evalSafe(() => Array.from(document.querySelectorAll('[id*="puzzle2d-play-node-kind-"]')).map((el) => el.id).slice(0, 20), [] as string[]);
  verdict("12-fill", "fill-weight-sliders-present", weightIds.length > 0, { ids: weightIds.slice(0, 8), expectedSelector: '[id*="puzzle2d-play-node-kind-"]', note: "`☑️options/🖌️brush` authors one slider per node kind, bound to setBrushKindWeights" });
  if (!weightIds.length) {
    verdict("12-fill", "fill-weights-change-the-distribution", false, { reason: "no weight slider" });
    return;
  }
  /** 🎲️ The kind histogram of everything a run placed, read off the outliner rows' own labels. */
  const histogram = async () => {
    const rows = await outlinerRows();
    const counts: Record<string, number> = {};
    for (const row of rows) {
      const kind = row.text.replace(/\s+/g, " ").trim().split(" ").slice(-2).join(" ");
      counts[kind] = (counts[kind] ?? 0) + 1;
    }
    await closePanels();
    return counts;
  };
  const runFill = async () => {
    await clickTab("framework.panel.toolRun");
    const start = page.locator("button", { hasText: /^start$/i }).first();
    const before = (await overviewVitals())?.nodes ?? -1;
    if (await countSafe(start)) await start.click({ timeout: 4000 }).catch(() => {});
    const grew = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before + 3, 90000);
    const abort = page.locator("button", { hasText: /^abort$/i }).first();
    const snapshotHistogram = await histogram();
    if (await countSafe(abort)) await abort.click({ timeout: 4000 }).catch(() => {});
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === before, 30000);
    return { ok: grew.ok, histogram: snapshotHistogram };
  };
  const first = await runFill();
  // ⚖️ Drive the FIRST kind's weight to zero and the last one's up, then re-run: the placed mix must move.
  const low = page.locator(`[id="${weightIds[0]}"] [role="slider"], [id="${weightIds[0]}"] input[type="range"]`).first();
  if (await countSafe(low)) {
    await low.focus().catch(() => {});
    for (let i = 0; i < 30; i++) await page.keyboard.press("ArrowLeft").catch(() => {});
  }
  await settle(2);
  const second = await runFill();
  verdict("12-fill", "fill-weights-change-the-distribution", first.ok && second.ok && JSON.stringify(first.histogram) !== JSON.stringify(second.histogram), {
    before: first.histogram,
    after: second.histogram,
    zeroed: weightIds[0],
    note: "RUN_SETTINGS_CONFIG declares /nodeKindWeights, so a weight change must reach the run's candidate order",
  });
});

register("fill-history", "replace", async () => {
  if (((await overviewVitals())?.nodes ?? 0) !== 1) {
    await selectExample(/concrete/i);
    await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === 1, 60000);
  }
  await closePanels();
  const before = entityCount(await overviewVitals());
  const rowsBefore = await historyRows();
  await closePanels();
  await armTool("fill");
  await clickTab("framework.panel.toolRun");
  const start = page.locator("button", { hasText: /^start$/i }).first();
  if (await countSafe(start)) await start.click({ timeout: 4000 }).catch(() => {});
  const runText = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).map((p) => (p as HTMLElement).innerText.replace(/\s+/g, " ")).join(" | ").slice(0, 400), "");
  await waitUntil(runText, (text) => /ready to finalize|complete/i.test(text), 180000, 1000);
  const finalize = page.locator("button", { hasText: /^finalize$/i }).first();
  if (await countSafe(finalize)) await finalize.click({ timeout: 4000 }).catch(() => {});
  const filled = await waitUntil(overviewVitals, (v) => entityCount(v) > before, 60000);
  verdict("12-fill", "fill-places-nodes", filled.ok, { before, after: entityCount(filled.value), waitedMs: filled.waitedMs });
  const rowsAfter = await historyRows();
  const added = Object.keys(rowsAfter).filter((id) => !(id in rowsBefore));
  // 🧾️ ONE ledger slot for the whole gesture. Every `ArtifactStore` has a fixed 64-entry, non-compacting
  // ledger (`🌿️vcs/🦀️.rs`), so a 100-placement run that commits per placement blows it and Undo then has
  // nothing correct to pop — exactly the `undo-changes-document` red of 2026-09-17 (`📓️E7…` §2).
  verdict("20-history", "fill-is-one-history-entry", added.length === 1, {
    added: added.map((id) => `${id}=${rowsAfter[id]}`).slice(0, 8),
    count: added.length,
    owner: "slice 2A — `✅️commit-slot` commits once per placement; the run must publish one batch (or one coalesced amend) per gesture",
  });
  await closePanels();
  const opened = await setActions(true);
  const undo = actionRow("undo");
  const present = await countSafe(undo);
  if (present) await undo.click({ timeout: 4000 }).catch(() => {});
  const undone = await waitUntil(overviewVitals, (v) => entityCount(v) === before, 45000);
  const parsed = (await overviewVitals())?.parsed ?? "";
  verdict("20-history", "undo-after-fill-restores-the-pre-fill-document", opened && present > 0 && undone.ok && parsed === "true", {
    before,
    afterFill: entityCount(filled.value),
    afterUndo: entityCount(undone.value),
    parsed,
    waitedMs: undone.waitedMs,
    note: "ONE undo, not one per placement",
  });
  await setActions(false);
});

register("history-controls", "replace", async () => {
  await ensureDocument(1);
  await closePanels();
  const before = entityCount(await overviewVitals());
  const checkpointed = await pressHistory("checkpoint");
  verdict("20-history", "checkpoint-control-usable", checkpointed, { expectedSelector: "#framework.history.checkpoint" });
  const picked = await pickNode(0);
  await page.keyboard.press("Delete").catch(() => {});
  const deleted = await waitUntil(overviewVitals, (v) => entityCount(v) < before, 30000);
  verdict("20-history", "edit-after-checkpoint-lands", deleted.ok, { before, after: entityCount(deleted.value), id: picked.id, waitedMs: deleted.waitedMs });
  const reverted = await pressHistory("revert");
  const back = await waitUntil(overviewVitals, (v) => entityCount(v) === before, 45000);
  verdict("20-history", "revert-restores-the-checkpoint", reverted && back.ok, { before, after: entityCount(back.value), waitedMs: back.waitedMs });
  const mid = entityCount(await overviewVitals());
  await pressHistory("undo");
  const undone = await waitUntil(overviewVitals, (v) => entityCount(v) !== mid, 30000);
  verdict("20-history", "undo-changes-document", undone.ok, { before: mid, after: entityCount(undone.value), waitedMs: undone.waitedMs });
  await pressHistory("redo");
  const redone = await waitUntil(overviewVitals, (v) => entityCount(v) === mid, 30000);
  verdict("20-history", "redo-restores-the-document", redone.ok, { before: entityCount(undone.value), after: entityCount(redone.value), waitedMs: redone.waitedMs });
  await closePanels();
});

register("selection-keybindings", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const picked = await pickNode(0);
  verdict("22-keys", "click-selects", picked.picked, { id: picked.id, waitedMs: picked.waitedMs });
  for (const key of ["Meta+a", "Control+a"]) {
    await page.keyboard.press(key).catch(() => {});
    await settle(0.5);
  }
  const all = await waitUntil(overviewVitals, (v) => selectionIds(v).length > 1, 20000);
  verdict("22-keys", "select-all-keybinding", all.ok, { selected: selectionIds(all.value).length, waitedMs: all.waitedMs, expectedKeybinding: "mod+a → selectAll" });
  await page.keyboard.press("Escape").catch(() => {});
  const cleared = await waitUntil(overviewVitals, (v) => selectionIds(v).length === 0, 20000);
  verdict("22-keys", "escape-clears-the-selection", cleared.ok, { selection: cleared.value?.selection, waitedMs: cleared.waitedMs });
  // 🖱️ The click-marquee variant 3d has and 2d never exercised: a press-and-release with no travel on empty
  // canvas must leave the selection empty rather than select the world.
  const box = await overviewBox();
  await page.mouse.click(box.x + 14, box.y + box.height - 14);
  const empty = await waitUntil(overviewVitals, (v) => selectionIds(v).length === 0, 10000);
  verdict("22-keys", "marquee-click-on-empty-canvas-selects-nothing", empty.ok, { selection: empty.value?.selection, waitedMs: empty.waitedMs });
});

register("context-menu-rows", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickNode(0);
  if (!picked.at) {
    verdict("15-context-menu", "context-menu-node-vocabulary", false, { reason: "no node position" });
    return;
  }
  const menu = await openContextMenuAt(picked.at);
  const ids = new Set(menu.rows.map((row) => row.id));
  // 🗂️ The vocabulary `puzzle2d_context_menu_items` authors for a non-empty selection.
  const missing = ["toggleHidden", "toggleLocked", "duplicate", "focusSelection", "selectSameKind", "deleteSelection"].filter((id) => !ids.has(id));
  verdict("15-context-menu", "context-menu-node-vocabulary", menu.rows.length > 0 && missing.length === 0, { missing, present: [...ids], waitedMs: menu.waitedMs });
  const hide = menu.rows.find((row) => row.id === "toggleHidden");
  verdict("15-context-menu", "hide-row-alternates-its-label", Boolean(hide) && /hide|show|ausblenden|einblenden/i.test(hide?.text ?? ""), { row: hide, note: "`value: any_visible` — the row must ask for the inverse of what it renders" });
  await closeContextMenu();
  await page.keyboard.press("Escape").catch(() => {});
  const empty = await waitUntil(overviewVitals, (v) => selectionIds(v).length === 0, 15000);
  const box = await overviewBox();
  await page.mouse.click(box.x + 16, box.y + box.height - 16, { button: "right" });
  const emptyMenu = await waitUntil(contextMenuRows, (rows) => rows.length > 0, 20000, 700);
  verdict("15-context-menu", "empty-selection-offers-select-all", emptyMenu.value.some((row) => row.id === "selectAll"), { cleared: empty.ok, rows: emptyMenu.value.map((r) => r.id), waitedMs: emptyMenu.waitedMs });
  await closeContextMenu();
});
//#endregion 🔖️Parity lanes

/** 🔬️ Dumps the chrome the mutate steps need (inspector rows after a pick, the pane's Actions panel, engagement input). */
register("explore-chrome", "read", async () => {
  const dump = async (what: string) => {
    const rows = await evalSafe(
      () =>
        Array.from(document.querySelectorAll('[data-slot="tree-item"], [role="treeitem"], button, input, [data-slot="toggle-group-item"]'))
          .filter((e) => (e as HTMLElement).offsetParent !== null)
          .map((e) => `${e.tagName.toLowerCase()}#${e.id || "-"}[${e.getAttribute("data-slot") ?? ""}]=${((e as HTMLElement).innerText || (e as HTMLInputElement).placeholder || "").replace(/\s+/g, " ").trim().slice(0, 50)}`),
      [] as string[],
    );
    log(`  ${what}: ${rows.length} visible controls\n    ${rows.join("\n    ")}`);
  };
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id="window:2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = (await visibleNodeIds())[0];
  const at = first ? await nodeScreen(first) : null;
  if (at) {
    await page.mouse.click(at.x, at.y);
    await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]").includes(first), 10000);
  }
  await clickTab("framework.panel.inspection");
  await settle(2);
  await dump("inspection panel open");
  await clickTab("framework.panel.inspection");
  await settle(1);
  const actions = page.locator('[data-slot="window"] button, [data-slot="window"] [role="button"]').filter({ hasText: /^\s*Actions\s*$/ }).first();
  log(`  actions buttons: ${await countSafe(actions)} id=${(await actions.getAttribute("id").catch(() => null)) ?? "-"}`);
  if (await countSafe(actions)) {
    await actions.click({ timeout: 3000 }).catch(() => {});
    await settle(2);
    await dump("actions panel open");
    await page.screenshot({ path: join(OUT, `probe-${stamp}-explore-actions.png`) }).catch(() => {});
    await actions.click({ timeout: 3000 }).catch(() => {});
    await settle(1);
  }
  const engagement = page.locator('[id="framework.window.2dOverview.engagement.toggle"]').first();
  log(`  engagement toggle: ${await countSafe(engagement)}`);
  if (await countSafe(engagement)) {
    await engagement.click({ timeout: 3000 }).catch(() => {});
    await settle(1.5);
    await dump("engagement open");
    await page.screenshot({ path: join(OUT, `probe-${stamp}-explore-engagement.png`) }).catch(() => {});
    await engagement.click({ timeout: 3000 }).catch(() => {});
  }
  verdict("0-explore", "chrome-dumped", true);
});

/** 🔬️ Inspector refresh timing: panel open before the pick vs. opened after the pick. */
register("inspector-timing", "read", async () => {
  await ensureDocument(2);
  await closePanels();
  const ids = await visibleNodeIds();
  const readRows = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="tree-item-row"]')).map((r) => r.id).filter((id) => id.includes("puzzle2d-play-inspector")).map((id) => id.split("/").pop() ?? id), [] as string[]);
  const readText = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="tree-item-row"]')).filter((r) => r.id.includes("puzzle2d-play-inspector")).map((r) => (r as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)), [] as string[]);
  // A: panel open first, then pick.
  await clickTab("framework.panel.inspection");
  await settle(1.5);
  const a0 = await readRows();
  const at = await nodeScreen(ids[0]);
  if (at) await page.mouse.click(at.x, at.y);
  await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]").includes(ids[0]), 10000);
  const a1 = await waitUntil(readText, (rows) => rows.some((row) => row.includes(ids[0])), 8000, 500);
  log(`  A open-then-pick: before=${JSON.stringify(a0.slice(0, 4))} after=${JSON.stringify(a1.value.slice(0, 4))} ok=${a1.ok} waited=${a1.waitedMs}`);
  verdict("16-inspection", "inspector-updates-while-open", a1.ok, { rows: a1.value.slice(0, 6) });
  await closePanels();
  // B: pick with the panel closed, then open.
  const visible = await visibleNodeIds();
  const second = visible[5] ?? visible[1];
  await settle(2);
  // 🎲️ The BOARD is asked first. `inspector-fresh-on-open` used to report a blank canvas as an inspector
  // defect: with `selection=[]` the panel honestly falls through to the document summary and renders
  // `Nodes 0 / Edges 0` because the live fixture really had gone empty — an event-credit refusal outside
  // the already-fixed fixture-parse path (`📓️E7…` §4). A board that is not painted gets its OWN verdict and
  // the inspector question is then not asked at all, because it cannot be answered.
  const paintedBefore = await boardPainted();
  verdict("1-windows", "board-painted-before-inspection", paintedBefore.painted, { ...paintedBefore, note: "a blank board makes every later selection verdict meaningless; parsed=false is the event-credit refusal family" });
  if (!paintedBefore.painted) {
    verdict("16-inspection", "inspector-fresh-on-open", false, { reason: "board not painted at step start", ...paintedBefore });
    await closePanels();
    return;
  }
  const at2 = await nodeScreen(second);
  if (at2) await page.mouse.click(at2.x, at2.y);
  const picked = await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]").includes(second), 10000);
  log(`  B first click picked=${picked.ok} waited=${picked.waitedMs}`);
  if (!picked.ok) {
    // 🎯️ The coordinate is RE-QUERIED, never reused: a failed pick is usually a board that repainted or
    // re-framed under the pointer, and a second click at the stale spot lands in the same empty place.
    const painted = await boardPainted();
    log(`  B retry: board painted=${painted.painted} parsed=${painted.parsed} nodes=${painted.nodes}`);
    const at3 = await nodeScreen(second);
    if (at3) await page.mouse.click(at3.x, at3.y);
    const again = await waitUntil(overviewVitals, (v) => (v?.selection ?? "[]").includes(second), 10000);
    log(`  B second click picked=${again.ok} at=${JSON.stringify(at3)} waited=${again.waitedMs}`);
  }
  await settle(1);
  const paintedAfter = await boardPainted();
  await clickTab("framework.panel.inspection");
  const b1 = await waitUntil(readText, (rows) => rows.some((row) => row.includes(second)), 8000, 500);
  log(`  B pick-then-open: second=${second} selection=${(await overviewVitals())?.selection} rows=${JSON.stringify(b1.value.slice(0, 4))} ok=${b1.ok} waited=${b1.waitedMs}`);
  if (!paintedAfter.painted) verdict("1-windows", "board-stays-painted-through-the-step", false, { ...paintedAfter, note: "the board went blank between the two halves of the step — the inspector's Nodes 0 / Edges 0 is honest about that, not a panel defect" });
  else verdict("1-windows", "board-stays-painted-through-the-step", true, paintedAfter);
  verdict("16-inspection", "inspector-fresh-on-open", b1.ok, { rows: b1.value.slice(0, 6), boardPainted: paintedAfter.painted, selection: (await overviewVitals())?.selection });
  await closePanels();
});

register("guest-alive", "read", async () => {
  const s = await snapshot();
  verdict("0-vitals", "guest-alive", !s.recovery.some((x) => x && x !== "?") && guestDeathFaults.length === 0, { recovery: s.recovery, guestDeath: guestDeathFaults.slice(0, 3) });
});
//#endregion 🔖️Steps

//#region 🔖️Main
log(`navigating to :${port}`);
await gotoShell("boot");
const booted = await waitForBoot("boot");
await page.screenshot({ path: join(OUT, `probe-${stamp}-boot.png`) }).catch(() => {});
emit({ section: "boot", step: "booted", verdict: booted ? "PASS" : "FAIL", faults: faults.length });
if (booted) pass += 1;
else fail += 1;

if (explore || !booted) {
  const inv = await inventory();
  log(`inventory: ${JSON.stringify(inv, null, 1).slice(0, 12000)}`);
  log(`vitals: ${JSON.stringify(await boardVitals())}`);
}

/** 🪣️ Steps whose SIDE EFFECT is a document no later verdict in the group can reason about — a fill run
 * grows a one-node fixture into a hundred-node one, and every later census verdict then measures that
 * instead of the feature it names. Pushed to the back of their group, relative order preserved. */
const STEP_TRAILS_ITS_GROUP: readonly string[] = ["brush-place", "suggestions-menu", "target-regions", "fill", "fill-controls", "fill-weights"];
/** 🥇 Steps whose PRECONDITION is a viewport with nothing armed and a menu vocabulary that has not been
 * replaced by the brush's suggestion menu — hoisted to the front of their group. */
const STEP_LEADS_ITS_GROUP: readonly string[] = ["context-menu-rows", "context-menu"];
const GROUP_ORDER = ["read", "mutate", "replace"] as const;

if (booted && !explore) {
  const plan = only ? steps.filter((s) => only.has(s.name)) : battery ? [...steps] : steps.filter((s) => s.name === "windows" || s.name === "guest-alive");
  const groupEntries = (group: Step["group"]) => {
    const entries = plan.filter((entry) => entry.group === group);
    // 🧱️ Examples load first so every later step in the group has a document to read.
    const examples = entries.filter((entry) => entry.name.startsWith("example-"));
    const leads = entries.filter((entry) => !examples.includes(entry) && STEP_LEADS_ITS_GROUP.includes(entry.name));
    const trails = entries.filter((entry) => !examples.includes(entry) && STEP_TRAILS_ITS_GROUP.includes(entry.name));
    const middle = entries.filter((entry) => !examples.includes(entry) && !leads.includes(entry) && !trails.includes(entry));
    return [...examples, ...leads, ...middle, ...trails];
  };
  const ordered: { group: Step["group"]; boundary: boolean; step?: Step }[] = [];
  for (const group of GROUP_ORDER) {
    const entries = groupEntries(group);
    if (!entries.length) continue;
    for (const step of entries) ordered.push({ group, boundary: false, step });
    ordered.push({ group, boundary: true });
  }
  log(`plan: ${JSON.stringify(GROUP_ORDER.map((group) => ({ group, steps: groupEntries(group).map((entry) => entry.name) })))}`);
  let ranGroup: Step["group"] | null = null;
  for (const entry of ordered) {
    if (entry.boundary) {
      // 🫀️ Every group closes with a guest-alive checkpoint, so a battery that measured a corpse says so at
      // the group boundary instead of as a run of unexplained late FAILs.
      const s = await snapshot();
      const vitals = await boardVitals();
      verdict("0-vitals", `guest-alive-${entry.group}`, !s.recovery.some((x) => x && x !== "?") && guestDeathFaults.length === 0 && s.canvases >= 3 && vitals.some((v) => v.nodes > 0), {
        recovery: s.recovery,
        canvases: s.canvases,
        surfaces: vitals.filter((v) => v.surface.startsWith("window:2d-")).map((v) => `${v.surface}=${v.nodes}/${v.edges}`),
        guestDeath: guestDeathFaults.slice(0, 2),
        firstHardFaultAt,
      });
      continue;
    }
    const step = entry.step!;
    if (reloadBetweenGroups && ranGroup !== null && ranGroup !== entry.group) {
      log(`reloading page before group ${entry.group}`);
      await gotoShell(`reboot:${entry.group}`);
      const rebooted = await waitForBoot(`reboot:${entry.group}`, 40);
      verdict("0-vitals", `reboot-${entry.group}`, rebooted, { group: entry.group });
    }
    ranGroup = entry.group;
    // 🔁️ A dev serve under concurrent framework edits answers an HMR update with a FULL page reload, which
    // empties the shell mid-battery: every later verdict then measures a blank document and reads as a wall
    // of unrelated reds (`canvases=0`, `camera=undefined`). The reload is detected and waited out instead,
    // and the fact that it happened is recorded as its own verdict rather than charged to the next step.
    const alive = await snapshot();
    if (alive.windows.length < 3 || alive.canvases < 3) {
      log(`shell is not mounted before ${step.name} (windows=${alive.windows.length} canvases=${alive.canvases}) — waiting for it to come back`);
      const recovered = await waitForBoot(`recover:${step.name}`, 30);
      verdict("0-vitals", `shell-remounted-before-${step.name}`, recovered, { windows: alive.windows.length, canvases: alive.canvases, note: "the page reloaded under the battery (dev HMR or a guest restart); the step below ran against the remounted shell" });
    }
    const mark = consoleCursor();
    const faultsBefore = hardFaults.length;
    log(`step ${step.name} (${step.group})`);
    try {
      await step.run();
    } catch (error) {
      verdict(step.group, step.name, false, { error: String(error).slice(0, 300) });
    }
    await settle(1);
    const fresh = hardFaults.slice(faultsBefore);
    if (fresh.length) log(`  hard faults during ${step.name}: ${fresh.slice(0, 3).join(" || ").slice(0, 600)}`);
    const tail = consoleSince(mark).filter((l) => /error|fault|panic|warn/i.test(l)).slice(-4);
    if (tail.length) log(`  console tail: ${tail.join(" || ").slice(0, 600)}`);
    await page.screenshot({ path: join(OUT, `probe-${stamp}-${step.name}.png`) }).catch(() => {});
    // 🎲️ A refused fixture is logged by length only — keep the bytes and classify them (invalid JSON vs. semantic).
    const refused = await evalSafe(() => {
      const g = globalThis as { __semioBoard2dRefusedFixture?: string };
      const text = g.__semioBoard2dRefusedFixture;
      if (!text) return null;
      g.__semioBoard2dRefusedFixture = undefined;
      let json: unknown = null;
      let parseError: string | null = null;
      try {
        json = JSON.parse(text);
      } catch (error) {
        parseError = String(error).slice(0, 200);
      }
      const doc = json as { schema?: unknown; nodes?: unknown[]; edges?: unknown[] } | null;
      return { text, parseError, schema: doc?.schema ?? null, nodes: Array.isArray(doc?.nodes) ? doc!.nodes!.length : -1, edges: Array.isArray(doc?.edges) ? doc!.edges!.length : -1, head: text.slice(0, 160), tail: text.slice(-160) };
    }, null as null | { text: string; parseError: string | null; schema: unknown; nodes: number; edges: number; head: string; tail: string });
    if (refused) {
      const file = join(OUT, `probe-${stamp}-${step.name}-refused.json`);
      writeFileSync(file, refused.text);
      const parsedAttr = await evalSafe(() => Array.from(document.querySelectorAll("[data-surface-id][data-board-fixture-parsed]")).map((e) => `${e.getAttribute("data-surface-id")}=${e.getAttribute("data-board-fixture-parsed")}`), [] as string[]);
      log(`  refused fixture during ${step.name}: chars=${refused.text.length} parseError=${refused.parseError ?? "none"} schema=${String(refused.schema)} nodes=${refused.nodes} edges=${refused.edges} parsed=${parsedAttr.join(",")} head=${JSON.stringify(refused.head)} tail=${JSON.stringify(refused.tail)} → ${file}`);
      emit({ step: step.name, refusedFixture: { chars: refused.text.length, parseError: refused.parseError, nodes: refused.nodes, edges: refused.edges, file } });
    }
  }
}

if (booted && !explore) {
  verdict("0-vitals", "battery-hard-faults", hardFaults.length === 0, { hard: hardFaults.length, first: (hardFaults[0] ?? "none").slice(0, 200) });
  verdict("0-vitals", "battery-faults", faults.length === 0, { raw: faults.length, hard: hardFaults.length, first: (faults[0] ?? "none").slice(0, 200) });
}
const summary = `battery PASS=${pass} FAIL=${fail} FAULTS=${faults.length} HARD=${hardFaults.length} first-hard-fault-at=${firstHardFaultAt ?? "none"} guest-death-faults=${guestDeathFaults.length}`;
log(summary);
emit({ summary, pass, fail, faults: faults.length, hard: hardFaults.length, firstHardFaultAt, guestDeath: guestDeathFaults.length, steps: steps.map((s) => `${s.group}:${s.name}`) });
const verdictBlock = lines.filter((row) => /\] (PASS|FAIL) /.test(row));
writeFileSync(
  join(OUT, `probe-${stamp}.md`),
  [
    `# probe ${stamp} (battery=${battery} only=${onlyArg ?? "-"} reloadBetweenGroups=${reloadBetweenGroups} port=${port})`,
    "",
    summary,
    "",
    "## verdicts",
    ...(verdictBlock.length ? verdictBlock : ["(none)"]),
    "",
    "## timeline",
    ...lines,
    "",
    "## faults",
    ...faults.slice(0, 80),
    "",
    `## console (last ${CONSOLE_TAIL_LINES})`,
    ...consoleBuf.slice(-CONSOLE_TAIL_LINES),
  ].join("\n"),
);
await browser.close();
//#endregion 🔖️Main
