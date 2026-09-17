/** 🔬️ Headless runtime battery for the puzzle 🖐️5d React serve on 127.0.0.1:6014 — boots the shell, resolves
 * the PAIR of panes 5d lays out (a `SurfaceKind::Board2d` board window `puzzle5d-2d` and a
 * `SurfaceKind::World3d` world window `puzzle5d-3d`), drives one browser step per lane of the 3d/2d parity
 * matrix (`📓️E6-battery-matrix.md` §4) and writes findings + screenshots + a machine-readable verdict
 * stream into `🗑️generated/`. Ticket 26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D, slice 5F.
 *
 * Run: `bun 🔍️browser-probe-5d.ts [--battery] [--only=step,step] [--port=<n>] [--explore]
 * [--reload-between-groups] [--settle=<seconds>] [--tail=<n>] [--board-surface=<id>] [--world-surface=<id>]`.
 *
 * `--explore` boots, dumps the chrome inventory (window instances, panel tabs, tool rail, utility rails,
 * every published `data-*` vital, every visible control id) and exits — the first thing to run against a
 * serve whose ids are not yet known. `--battery` runs every registered step ordered by blast radius,
 * `read` (nothing mutates) → `mutate` (reversible edits) → `replace` (the document itself is swapped);
 * `--only=a,b` runs exactly those. `--reload-between-groups` reboots the page between groups so a group
 * never inherits the previous group's document, exactly like the puzzle 3d battery
 * (`…/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`). Every group closes with a `guest-alive-<group>`
 * verdict read off `data-plugin-recovery` plus BOTH panes' vitals, because a 5d pane pair shares one guest
 * actor and a corpse in either pane is a death for the whole step.
 *
 * Outputs: `probe5d-<stamp>.md` (prose timeline), `probe5d-<stamp>.ndjson` (one JSON record per verdict plus
 * a final `battery PASS=n FAIL=n FAULTS=n HARD=n first-hard-fault-at=<s> guest-death-faults=n` summary
 * record) and `probe5d-<stamp>-<step>.png` screenshots.
 *
 * 🧭️ The verbs and DOM ids this battery drives are being built by sibling slices 5A1–5G. Every lane names
 * the selector it reached for in its own verdict detail (`hook`), so a red reads as "that hook is not there
 * yet, and here is its name" rather than as an unattributed failure. */
import { chromium, type Locator } from "playwright";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

// 📁️ `new URL(".", import.meta.url)` rather than Bun's `import.meta.dir` so the file also type-checks
// under a plain `tsc --noEmit` without `bun-types` installed; both resolve to this ticket folder.
const TICKET = decodeURIComponent(new URL(".", import.meta.url).pathname);
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const battery = process.argv.includes("--battery");
const explore = process.argv.includes("--explore");
const onlyArg = process.argv.find((a) => a.startsWith("--only="))?.slice(7);
const only = onlyArg ? new Set(onlyArg.split(",").map((name) => name.trim()).filter(Boolean)) : null;
const reloadBetweenGroups = process.argv.includes("--reload-between-groups");
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6014";
const plugin = process.argv.find((a) => a.startsWith("--plugin="))?.slice(9) ?? "puzzle5d";
const settleSeconds = Number(process.argv.find((a) => a.startsWith("--settle="))?.slice(9) ?? "3") || 3;
/** ⏱️ Boot polls of 3 s each (default 90 = 4.5 min — a cold 5d boot stages ~20 wasm plugins). `--boot-polls=1`
 * is the self-check a maintainer runs against a dead port to prove the file still executes end to end. */
const bootPolls = Number(process.argv.find((a) => a.startsWith("--boot-polls="))?.slice(13) ?? "90") || 90;
const boardSurfaceArg = process.argv.find((a) => a.startsWith("--board-surface="))?.slice(16) ?? null;
const worldSurfaceArg = process.argv.find((a) => a.startsWith("--world-surface="))?.slice(16) ?? null;
const ndjsonPath = join(OUT, `probe5d-${stamp}.ndjson`);
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
const CONSOLE_TAIL_LINES = Number(process.argv.find((a) => a.startsWith("--tail="))?.slice(7) ?? "160") || 160;
const consoleBuf: string[] = [];
let consoleSeq = 0;
const consoleCursor = () => consoleSeq;
const consoleSince = (mark: number) => consoleBuf.slice(Math.max(0, mark - (consoleSeq - consoleBuf.length)));
/** 🧾️ `puzzle5d-` matches the app's OWN `Fault::from("puzzle5d-…")` codes (`puzzle5d-board-event-decode`,
 * `puzzle5d-envelope.page-too-large`, …) and must keep doing so — but the dev host's boot banner prints one
 * `[stale] … run: bun nx run @semio-tech/framework-os-dev:activate-puzzle5d-react-dev` line per staged
 * module, every one of which carries the same substring. The recipe name is excluded by a lookahead and the
 * banner as a whole by {@link BENIGN_CONSOLE_RE}; a fault regex that counts the dev-activation advisory
 * reports a green run as FAULTS=58 (`📓️E7-2d-battery-failures-root-cause.md` §5). */
const FAULT_RE =
  /intake-budget-exhausted|fixed-capacity|section-root-mismatch|native-owner-required|terminal-fault|unreachable|shard .* (lost|terminated)|did not publish|missing field|malformed|admission failed|worker fault|\[semio-plugin panic\]|panicked at|Credits \{|NodeCapacity|SemioFaultError|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected|unknown Puzzle 5D action|DuplicateSiblingKey|undeclared lane|puzzle5d-(?!react-dev)/i;
/** 🩹️ Console lines that say nothing about the feature under test: the staged-plugin advisory, the
 * transform-freshness census and the document-sources dump are dev-server bookkeeping printed every boot. */
const BENIGN_CONSOLE_RE = /contributions document sources|staged plugin module|\[stale\]|activate-puzzle5d-react-dev|activate-puzzle3d-react-dev|activate-puzzle2d-react-dev|transform freshness/i;
const HARD_FAULT_RE =
  /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected|unknown Puzzle 5D action|DuplicateSiblingKey/i;
const GUEST_DEATH_RE = /reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected|\[semio-plugin panic\]/i;
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

// 🖥️ `--use-angle=metal` — headless chromium otherwise falls back to SwiftShader and the world pane's WebGL
// frame budget stops being a measurement of anything (project memory: "Headless Chromium Uses SwiftShader").
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
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
/** ⏳️ Polls `read` until `settled` holds or `timeoutMs` passes; returns the last value and the wait. Every
 * mutation lane reports its `waitedMs` so a red distinguishes "never happened" from "too slow". */
const waitUntil = async <T,>(read: () => Promise<T>, settled: (value: T) => boolean, timeoutMs = 30000, everyMs = 500) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < timeoutMs) {
    await page.waitForTimeout(everyMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: settled(value) };
};
/** ⏳️ The standard mutation budget: 30 s, the wall the 3d battery settled on for a guest round trip that
 * has to re-plan, re-publish and re-page a scene surface. */
const MUTATION_MS = 30000;
/** 🌙️ `capsule-dream` must be matched on BOTH words: the other 5d example is "Nakagin Capsule Tower",
 * so a bare `/capsule/i` picks the wrong document and the lane silently measures Nakagin. */
const CAPSULE_DREAM = /capsule\s*[-_]?\s*dream/i;

//#region 🔖️Panes
/** 🪟️ 5d lays out exactly two panes and the probe must NOT hardcode their DOM ids: the window *instance*
 * id the shell mints (`puzzle3d-main-perspective` for 3d, `2d-overview` for 2d) is a layout-seed product,
 * not the `WINDOW_KIND_ID` the guest declares. Both panes are therefore resolved by CAPABILITY — the board
 * pane is the `[data-surface-id]` element that publishes `data-board-nodes` (`🖥️Board2dHost/🟦️.tsx`), the
 * world pane the one that publishes `data-instances-json` (`🌐️World3dHost/🟦️.tsx`) — and `--board-surface=`
 * / `--world-surface=` override the result when a serve grows a third surface. */
type SurfaceRow = { surface: string; window: string; kind: "board" | "world" | "other"; attrs: Record<string, string> };
const SURFACE_ATTRS = [
  "data-window-instance-id",
  "data-board-nodes", "data-board-edges", "data-board-handles", "data-board-positions-json", "data-board-selection-json",
  "data-board-camera-json", "data-board-hovered-id", "data-board-active-utility", "data-board-fixture-parsed",
  "data-board-suggestion-menu-json", "data-board-target-regions-json",
  "data-selection-json", "data-guest-selection-json", "data-instances-json", "data-meshes-json", "data-vortices-json",
  "data-target-volumes-json", "data-engagement-preview-json", "data-camera-json", "data-viewport-camera-json",
  "data-suggestion-menu-json", "data-interaction-json", "data-status-json", "data-sun-json",
  "data-gumball-hits", "data-vortex-hits", "data-hover-paint-id",
] as const;

const readSurfaces = () =>
  evalSafe(
    (names: readonly string[]) =>
      Array.from(document.querySelectorAll("[data-surface-id]")).map((el) => {
        const attrs: Record<string, string> = {};
        for (const name of names) {
          const value = el.getAttribute(name);
          if (value !== null) attrs[name] = value;
        }
        const owner = el.closest('[data-slot="window"]');
        const kind = el.hasAttribute("data-board-nodes") ? "board" : el.hasAttribute("data-instances-json") ? "world" : "other";
        return { surface: el.getAttribute("data-surface-id") ?? "?", window: owner?.id ?? "", kind, attrs } as SurfaceRow;
      }),
    [] as SurfaceRow[],
    SURFACE_ATTRS as unknown as string[],
  );

let boardSurface = boardSurfaceArg;
let worldSurface = worldSurfaceArg;
let boardWindow = "";
let worldWindow = "";
/** 🪟️ The `WINDOW_KIND_ID`s the guest authors its per-window control ids from
 * (`…/🖐️5d/…/🪟️windows/{◻️2d,🧊️3d}/🦀️.rs:19,28`) — the engagement input is `puzzle5d-engagement-<kind>`. */
const BOARD_KIND = "puzzle5d-2d";
const WORLD_KIND = "puzzle5d-3d";

const resolvePanes = async () => {
  const rows = await readSurfaces();
  const board = rows.find((row) => (boardSurfaceArg ? row.surface === boardSurfaceArg : row.kind === "board"));
  const world = rows.find((row) => (worldSurfaceArg ? row.surface === worldSurfaceArg : row.kind === "world"));
  if (board) {
    boardSurface = board.surface;
    boardWindow = board.window;
  }
  if (world) {
    worldSurface = world.surface;
    worldWindow = world.window;
  }
  return { rows, board, world };
};

const surfaceRow = async (surface: string | null) => (surface ? (await readSurfaces()).find((row) => row.surface === surface) ?? null : null);
const attr = (row: SurfaceRow | null, name: string) => row?.attrs[name] ?? "";
const jsonOf = <T,>(raw: string, fallback: T): T => {
  try {
    return raw ? (JSON.parse(raw) as T) : fallback;
  } catch {
    return fallback;
  }
};

/** 🧾️ Board vitals, read straight off `Board2dHost`'s `data-board-*` block without a guest round trip. */
const board = async () => {
  const row = await surfaceRow(boardSurface);
  return {
    present: Boolean(row),
    surface: boardSurface ?? "",
    nodes: Number(attr(row, "data-board-nodes") || "-1"),
    edges: Number(attr(row, "data-board-edges") || "-1"),
    handles: Number(attr(row, "data-board-handles") || "-1"),
    selection: attr(row, "data-board-selection-json"),
    camera: attr(row, "data-board-camera-json"),
    hovered: attr(row, "data-board-hovered-id"),
    utility: attr(row, "data-board-active-utility"),
    parsed: attr(row, "data-board-fixture-parsed"),
    suggestions: attr(row, "data-board-suggestion-menu-json"),
    regions: attr(row, "data-board-target-regions-json"),
    positions: attr(row, "data-board-positions-json"),
  };
};
/** 🧾️ World vitals. `World3dHost` publishes no single bundled block the way `Board2dHost` does, so the
 * census is assembled from the several JSON attributes it does publish (`📓️E6` §4). */
const world = async () => {
  const row = await surfaceRow(worldSurface);
  const instances = jsonOf(attr(row, "data-instances-json"), [] as { id?: string; position?: number[]; rotation?: number[]; scale?: number[]; selected?: boolean }[]);
  return {
    present: Boolean(row),
    surface: worldSurface ?? "",
    instances,
    instanceCount: instances.length,
    instanceIds: instances.map((instance) => instance.id ?? "?"),
    instancesRaw: attr(row, "data-instances-json"),
    meshes: attr(row, "data-meshes-json"),
    meshCount: jsonOf(attr(row, "data-meshes-json"), [] as unknown[]).length,
    selection: attr(row, "data-selection-json"),
    guestSelection: attr(row, "data-guest-selection-json"),
    vortices: attr(row, "data-vortices-json"),
    volumes: attr(row, "data-target-volumes-json"),
    camera: attr(row, "data-camera-json"),
    viewportCamera: attr(row, "data-viewport-camera-json"),
    suggestionMenu: attr(row, "data-suggestion-menu-json"),
    interaction: attr(row, "data-interaction-json"),
    status: attr(row, "data-status-json"),
    sun: attr(row, "data-sun-json"),
    preview: attr(row, "data-engagement-preview-json"),
    gumballHits: attr(row, "data-gumball-hits"),
    vortexHits: attr(row, "data-vortex-hits"),
    hoverPaint: attr(row, "data-hover-paint-id"),
  };
};

const idsIn = (raw: string): string[] => {
  const parsed = jsonOf<unknown>(raw, []);
  if (Array.isArray(parsed)) return parsed.map((entry) => (typeof entry === "string" ? entry : ((entry as { id?: string })?.id ?? "?")));
  const record = parsed as { ids?: string[]; selectedIds?: string[]; selection?: { ids?: string[] } };
  return record?.selectedIds ?? record?.ids ?? record?.selection?.ids ?? [];
};
const boardSelection = async () => idsIn((await board()).selection);
/** 🕹️ The world pane's selection has TWO channels — the host's optimistic echo (`data-selection-json`) and
 * the guest-confirmed one (`data-guest-selection-json`). A cross-pane law must hold on the CONFIRMED one. */
const worldSelection = async () => {
  const w = await world();
  const confirmed = idsIn(w.guestSelection);
  return { confirmed, local: idsIn(w.selection), selectedInstances: w.instances.filter((instance) => instance.selected).map((instance) => instance.id ?? "?") };
};

const paneCanvas = (surface: string | null) => page.locator(`[data-surface-id="${surface ?? "__none__"}"] canvas`).first();
const paneBox = async (surface: string | null) => (await paneCanvas(surface).boundingBox()) ?? { x: 0, y: 0, width: 1, height: 1 };
const boardBox = () => paneBox(boardSurface);
const worldBox = () => paneBox(worldSurface);
const boardCentre = async () => {
  const box = await boardBox();
  return { x: box.x + box.width / 2, y: box.y + box.height / 2 };
};
const worldCentre = async () => {
  const box = await worldBox();
  return { x: box.x + box.width / 2, y: box.y + box.height / 2 };
};

/** 🎯️ The screen position of a board node, projected through the board pane's published camera. Nodes that
 * land outside the pane are unreachable: a click there hits the shell, not the board. */
const boardNodeScreen = async (id: string) => {
  const vitals = await board();
  const positions = jsonOf(vitals.positions, {} as Record<string, [number, number]>);
  const camera = jsonOf(vitals.camera, null as null | { x: number; y: number; zoom: number });
  const position = positions[id];
  if (!position || !camera) return null;
  const box = await boardBox();
  const at = { x: box.x + box.width / 2 + (position[0] - camera.x) * camera.zoom, y: box.y + box.height / 2 + (position[1] - camera.y) * camera.zoom };
  const margin = 28;
  if (at.x < box.x + margin || at.x > box.x + box.width - margin || at.y < box.y + margin || at.y > box.y + box.height - margin) return null;
  return at;
};
const visibleBoardNodeIds = async () => {
  const positions = jsonOf((await board()).positions, {} as Record<string, [number, number]>);
  const visible: string[] = [];
  for (const id of Object.keys(positions)) if (await boardNodeScreen(id)) visible.push(id);
  return visible;
};
/** 🧮️ The one scalar every census verdict compares: parts + fasteners + grips on the board pane. */
const boardCensus = async () => {
  const v = await board();
  return v.nodes + v.edges + v.handles;
};
//#endregion 🔖️Panes

//#region 🔖️Snapshot
type Snapshot = { windows: { id: string; w: number; h: number }[]; canvases: number; tabs: string[]; toggles: string[]; treeItems: number; dialogs: string[]; recovery: string[]; body: string };
const EMPTY: Snapshot = { windows: [], canvases: 0, tabs: [], toggles: [], treeItems: 0, dialogs: [], recovery: [], body: "" };
const snapshot = () =>
  evalSafe((): Snapshot => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    return {
      windows: q('[data-slot="window"]').map((w) => ({ id: w.id || w.getAttribute("data-key") || "?", w: (w as HTMLElement).offsetWidth, h: (w as HTMLElement).offsetHeight })),
      canvases: q("canvas").length,
      tabs: q('[data-slot="panel-tab-button"]').map((b) => b.id).slice(0, 80),
      toggles: q('[data-slot="toggle-group-item"]').map((b) => `${b.id}=${b.getAttribute("aria-pressed")}`).slice(0, 80),
      treeItems: q('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]').length,
      dialogs: q('[role="dialog"]').map((d) => (d as HTMLElement).innerText.slice(0, 120)),
      recovery: q("[data-plugin-recovery]").map((el) => el.getAttribute("data-plugin-recovery") ?? "?"),
      body: document.body ? document.body.innerText.slice(0, 400) : "",
    };
  }, EMPTY);

const inventory = () =>
  evalSafe(() => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    const text = (el: Element) => (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 48) ?? "";
    const visible = (el: Element) => (el as HTMLElement).offsetParent !== null;
    return {
      windows: q('[data-slot="window"]').map((w) => `${w.id}[active=${w.getAttribute("data-active")}]`),
      tabs: q('[data-slot="panel-tab-button"]').map((b) => `${b.id}=${text(b)}`),
      toggles: q('[data-slot="toggle-group-item"]').map((b) => `${b.id}=${b.getAttribute("aria-pressed")}:${text(b)}`),
      buttons: q("button").filter(visible).map((b) => `${b.id || "-"}:${text(b)}`).filter((s) => s !== "-:").slice(0, 260),
      selects: q("select").map((s) => `${s.id}:${Array.from((s as HTMLSelectElement).options).map((o) => o.label).join("|")}`),
      inputs: q("input").map((i) => `${i.id || "-"}:${(i as HTMLInputElement).type}=${(i as HTMLInputElement).value}`).slice(0, 120),
      tree: q('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]').map((r) => `${r.id || "?"}=${text(r)}`).slice(0, 120),
      surfaces: q("[data-surface-id]").map((el) => el.getAttribute("data-surface-id")),
      slots: Array.from(new Set(q("[data-slot]").map((el) => el.getAttribute("data-slot")))).slice(0, 160),
    };
  }, { windows: [] as string[], tabs: [] as string[], toggles: [] as string[], buttons: [] as string[], selects: [] as string[], inputs: [] as string[], tree: [] as string[], surfaces: [] as (string | null)[], slots: [] as (string | null)[] });
//#endregion 🔖️Snapshot

//#region 🔖️Boot
const gotoShell = async (label: string) => {
  await page.goto(`http://127.0.0.1:${port}/?plugin=${plugin}`, { waitUntil: "domcontentloaded", timeout: 90000 }).catch((error) => log(`${label} goto: ${String(error).slice(0, 200)}`));
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

/** 🚀️ Booted means BOTH panes are mounted and publishing: two window instances, two canvases, one surface
 * that answers `data-board-nodes` and one that answers `data-instances-json`. A single-pane boot is a
 * layout/`windowKinds` fault and must not be mistaken for a slow one. */
const waitForBoot = async (label: string, polls = bootPolls) => {
  for (let i = 0; i < polls; i++) {
    await page.waitForTimeout(3000);
    const s = await snapshot();
    if (s.dialogs.length && i % 2 === 0) await dismissTour(label);
    if (s.recovery.some((r) => r !== "?" && r !== "")) log(`${label} plugin recovery card: ${JSON.stringify(s.recovery)}`);
    const { board: b, world: w } = await resolvePanes();
    if (b && w && s.canvases >= 2) {
      log(`${label} booted: windows=${JSON.stringify(s.windows)} canvases=${s.canvases} board=${boardSurface}@${boardWindow} world=${worldSurface}@${worldWindow}`);
      await dismissTour(label);
      return true;
    }
    if (i % 5 === 4) log(`${label} waiting… windows=${s.windows.length} canvases=${s.canvases} board=${b?.surface ?? "-"} world=${w?.surface ?? "-"} faults=${faults.length} body=${JSON.stringify(s.body.slice(0, 140))}`);
  }
  return false;
};
//#endregion 🔖️Boot

//#region 🔖️Chrome
let pass = 0;
let fail = 0;
const verdicts: string[] = [];
let currentGroup: "read" | "mutate" | "replace" | "boot" = "boot";
const verdict = (section: string, step: string, ok: boolean, detail: Record<string, unknown> = {}) => {
  if (ok) pass += 1;
  else fail += 1;
  const row = `${ok ? "PASS" : "FAIL"} ${section}/${step} ${JSON.stringify(detail).slice(0, 500)}`;
  verdicts.push(row);
  emit({ section, step, group: currentGroup, verdict: ok ? "PASS" : "FAIL", t: Number(((Date.now() - t0) / 1000).toFixed(1)), ...detail });
  log(row);
};

/** 🪟️ The dock's open panels, by tab id — `[data-slot="panel"]#framework.panelTab.<tabId>` is mounted only
 * while open (`aria-pressed` is "last used" and sticks, so it is never the truth). */
const openPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((e) => (e as HTMLElement).offsetParent !== null).map((e) => e.id.replace(/^framework\.panelTab\./, "")), [] as string[]);
/** 🪟️ Opens (never toggles) the panel tab `id`; a tab press on an open panel closes it. */
const clickTab = async (id: string) => {
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  if (!(await countSafe(tab))) return false;
  if ((await openPanelTabIds()).includes(id)) return true;
  await tab.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(900);
  return (await openPanelTabIds()).includes(id);
};
/** 🪟️ Closes every open panel — they overlay the pane's LEFT edge, so any pointer gesture that starts
 * there lands on the panel and never reaches the canvas. */
const closePanels = async () => {
  const open = await openPanelTabIds();
  for (const id of open) {
    await page.locator(`[data-slot="panel-tab-button"][id="${id}"]`).first().click({ timeout: 2500 }).catch(() => {});
    await page.waitForTimeout(400);
  }
  await page.waitForTimeout(300);
  return { closed: open, stillOpen: await openPanelTabIds() };
};
const panelText = (tabId: string) =>
  evalSafe((id) => (document.querySelector(`[data-slot="panel"][id="framework.panelTab.${id}"]`) as HTMLElement | null)?.innerText.replace(/\s+/g, " ").slice(0, 1200) ?? "", "", tabId);
const panelRows = (match: string) =>
  evalSafe(
    (needle) =>
      Array.from(document.querySelectorAll('[data-slot="tree-item"], [data-slot="tree-item-row"], [role="treeitem"]'))
        .filter((r) => (needle ? r.id.includes(needle) : true))
        .map((r) => `${r.id || "?"}=${(r as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 70)}`)
        .slice(0, 120),
    [] as string[],
    match,
  );

/** 🎛️ A window instance id camelised the way the framework mints its per-window control ids
 * (`2d-overview` → `framework.window.2dOverview.engagement.toggle`). */
const camelWindow = (windowId: string) => windowId.replace(/-([a-z0-9])/g, (_m, c: string) => c.toUpperCase());
/** 🎛️ Whether ONE window instance's Actions pane is open. Both 5d panes mount at once and the framework
 * authors the same control ids into each, so every read and every toggle is scoped to one window subtree —
 * the same scope the framework's own key router uses (`shouldRouteKeysToWindowSearch`). */
const actionsOpen = (windowId: string) =>
  evalSafe((scope) => Array.from(document.querySelectorAll(`${scope} [id^="action."], ${scope} [id^="puzzle5d-engagement-"]`)).some((el) => (el as HTMLElement).offsetParent !== null), false, `[data-slot="window"][id="${windowId}"]`);
const actionsToggleReady = async (windowId: string, budgetMs = MUTATION_MS) => {
  const id = `framework.window.${camelWindow(windowId)}.engagement.toggle`;
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
const setActions = async (open: boolean, windowId: string) => {
  const toggle = await actionsToggleReady(windowId);
  for (let attempt = 0; attempt < 3; attempt++) {
    if ((await actionsOpen(windowId)) === open) return true;
    if (!toggle.ready) {
      log(`  setActions ${windowId} toggle never became interactive: ${JSON.stringify(toggle)}`);
      break;
    }
    const outcome = await page.locator(`[id="${toggle.id}"]`).first().click({ timeout: 5000 }).then(() => "ok").catch((error) => `failed ${String(error).split("\n")[0].slice(0, 90)}`);
    const settled = await waitUntil(() => actionsOpen(windowId), (isOpen) => isOpen === open, 10000);
    log(`  setActions ${windowId} want=${open} attempt=${attempt} click=${outcome} open=${settled.value} waitedMs=${settled.waitedMs}`);
    if (settled.ok) return true;
  }
  return (await actionsOpen(windowId)) === open;
};
/** 🎛️ Clicks one action row inside ONE window's Actions pane, opening the pane first if it is shut. */
const runAction = async (windowId: string, actionId: string) => {
  await setActions(true, windowId);
  const row = page.locator(`[data-slot="window"][id="${windowId}"] [id="action.${actionId}"], [id="action.${actionId}"]`).first();
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 4000 }).catch(() => {});
  return { hook: `action.${actionId}`, present: present > 0 };
};
/** 🖐️ The engagement command line of ONE pane (`puzzle5d-engagement-<WINDOW_KIND_ID>`,
 * `…/🎭️modes/✏️edit/🦀️.rs:57`). Typed whole and submitted with Enter — a per-character echo race swallows
 * a label typed key by key (project memory: "Shell Action Line Echo Race"). */
const engage = async (windowId: string, kindId: string, text: string) => {
  await setActions(true, windowId);
  const input = page.locator(`[id="puzzle5d-engagement-${kindId}"]`).first();
  const present = await countSafe(input);
  if (!present) return { hook: `puzzle5d-engagement-${kindId}`, present: false, text };
  await input.fill(text).catch(() => {});
  await input.press("Enter").catch(() => {});
  return { hook: `puzzle5d-engagement-${kindId}`, present: true, text };
};
/** 🪛️ The utility rail of ONE pane; the rail folds itself when the pane is narrow. */
const unfoldUtilities = async (windowId: string) => {
  const unfold = page.locator(`[id="framework.window.${camelWindow(windowId)}.utilityBar.unfold"]`).first();
  if (await countSafe(unfold)) {
    await unfold.click({ timeout: 3000 }).catch(() => {});
    await settle(1);
  }
};
/** 🪛️ Arms one utility in one pane and waits for the pane to echo it back. The utility is per-window —
 * reading a flat `active_utility_id` names the ACTIVE window's utility, not this pane's (project memory:
 * "Active Utility Is Per-Window In React Host"). */
const armUtility = async (windowId: string, name: RegExp, echo: () => Promise<string>) => {
  await unfoldUtilities(windowId);
  const toggle = page.locator(`[data-slot="window"][id="${windowId}"] [data-slot="toggle-group-item"]`).filter({ hasText: name }).first();
  const present = await countSafe(toggle);
  if (present) await toggle.click({ timeout: 4000 }).catch(() => {});
  const settled = await waitUntil(echo, (value) => name.test(value), 15000);
  return { present: present > 0, utility: settled.value, ok: present > 0 && settled.ok, waitedMs: settled.waitedMs };
};
const boardUtility = async (name: RegExp) => armUtility(boardWindow, name, async () => (await board()).utility);
const worldUtility = async (name: RegExp) =>
  armUtility(worldWindow, name, async () => {
    const interaction = jsonOf((await world()).interaction, {} as { activeUtility?: string });
    return interaction.activeUtility ?? "";
  });

/** 🗂️ Right-clicks a screen point and returns the menu rows it opened. */
const contextMenuAt = async (x: number, y: number) => {
  await page.mouse.click(x, y, { button: "right" });
  await settle(1.5);
  const rows = await evalSafe(
    () => Array.from(document.querySelectorAll('[role="menuitem"], [role="menu"] [data-slot="tree-item"]')).map((m) => (m as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)).filter(Boolean),
    [] as string[],
  );
  const menus = await evalSafe(() => document.querySelectorAll('[role="menu"], [data-slot="context-menu"]').length, 0);
  return { rows, menus };
};
const clickMenuRow = async (match: RegExp) => {
  const row = page.locator('[role="menuitem"]').filter({ hasText: match }).first();
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 4000 }).catch(() => {});
  else await page.keyboard.press("Escape").catch(() => {});
  return present > 0;
};

/** 🔁️ Switches the navbar example and waits for the reloaded pane chrome to be interactive again — an
 * example switch is a full document reload and every pane control is transiently absent. A refusal notice
 * is returned rather than thrown: `capsule-dream` may legitimately be refused. */
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
      return null;
    }
    const trigger = page.locator('[id="playground.navbar.fixture"]').first();
    if (!(await countSafe(trigger))) return null;
    await trigger.click({ timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(700);
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    if (!(await countSafe(option))) {
      await page.keyboard.press("Escape");
      return null;
    }
    const label = (await option.innerText().catch(() => "")).trim();
    await option.click({ timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(600);
    return label || "?";
  };
  const mark = consoleCursor();
  const label = await pick();
  if (label) {
    const chrome = await actionsToggleReady(boardWindow, 20000);
    if (!chrome.ready) log(`  selectExample ${label}: board chrome not interactive ${JSON.stringify(chrome)}`);
  }
  const notices = consoleSince(mark).filter((l) => /refus|notice|not supported|unavailable/i.test(l) && !BENIGN_CONSOLE_RE.test(l)).slice(0, 6);
  return { label, notices };
};
const exampleOptions = async () => {
  const native = page.locator('select[id="playground.navbar.fixture"]').first();
  if (await countSafe(native)) return native.locator("option").allTextContents();
  const trigger = page.locator('[id="playground.navbar.fixture"]').first();
  if (!(await countSafe(trigger))) return [] as string[];
  await trigger.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(700);
  const options = await evalSafe(() => Array.from(document.querySelectorAll('[role="option"]')).map((o) => (o as HTMLElement).innerText.replace(/\s+/g, " ").trim()), [] as string[]);
  await page.keyboard.press("Escape").catch(() => {});
  return options;
};
/** 📄️ Makes sure a document with at least `minParts` parts is loaded — a mutate lane may have emptied it. */
const ensureDocument = async (minParts = 1) => {
  if ((await board()).nodes >= minParts) return true;
  await selectExample(minParts > 1 ? /nakagin/i : /concrete/i);
  const loaded = await waitUntil(board, (v) => v.nodes >= minParts, 90000);
  return loaded.ok;
};
/** 🎯️ Picks the first reachable board node and confirms the pane echoed the selection. */
const pickBoardNode = async () => {
  const first = (await visibleBoardNodeIds())[0];
  const at = first ? await boardNodeScreen(first) : null;
  if (!at || !first) return { id: null as string | null, at: null as null | { x: number; y: number }, ok: false };
  await page.mouse.click(at.x, at.y);
  const picked = await waitUntil(boardSelection, (ids) => ids.includes(first), 15000);
  return { id: first, at, ok: picked.ok, waitedMs: picked.waitedMs };
};
/** 🎯️ Picks something in the world pane by clicking its centre; returns the confirmed selection. */
const pickWorldInstance = async () => {
  const centre = await worldCentre();
  await page.mouse.click(centre.x, centre.y);
  const picked = await waitUntil(async () => (await worldSelection()).confirmed, (ids) => ids.length > 0, 15000);
  return { at: centre, ids: picked.value, ok: picked.ok, waitedMs: picked.waitedMs };
};
/** 🌍️ Orbits the world pane by dragging with the left button clear of any object. */
const orbitWorld = async (dx = 220, dy = 90) => {
  const box = await worldBox();
  const from = { x: box.x + box.width * 0.78, y: box.y + box.height * 0.22 };
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move(from.x - dx, from.y + dy, { steps: 14 });
  await page.mouse.up();
  await settle(1);
};
//#endregion 🔖️Chrome

//#region 🔖️Steps
type Step = { name: string; section: string; group: "read" | "mutate" | "replace"; run: () => Promise<void> };
const steps: Step[] = [];
const add = (name: string, section: string, group: Step["group"], run: () => Promise<void>) => steps.push({ name, section, group, run });

// ── §1 panes ────────────────────────────────────────────────────────────────────────────────────────
add("panes", "§1-panes", "read", async () => {
  const s = await snapshot();
  const b = await board();
  const w = await world();
  verdict("§1-panes", "two-windows", s.windows.length >= 2 && s.canvases >= 2, { windows: s.windows, canvases: s.canvases, boardWindow, worldWindow });
  verdict("§1-panes", "board-vitals-published", b.present && b.nodes >= 0 && b.parsed === "true", { hook: "data-board-nodes|data-board-fixture-parsed", surface: b.surface, nodes: b.nodes, edges: b.edges, handles: b.handles, parsed: b.parsed });
  verdict("§1-panes", "world-vitals-published", w.present && w.instancesRaw !== "" && w.meshes !== "", { hook: "data-instances-json|data-meshes-json", surface: w.surface, instances: w.instanceCount, meshes: w.meshCount });
});

// ── §5 examples ─────────────────────────────────────────────────────────────────────────────────────
add("example-inventory", "§5-examples", "read", async () => {
  const options = await exampleOptions();
  verdict("§5-examples", "picker-lists-three", options.length >= 3 && /concrete/i.test(options.join(" ")) && /nakagin/i.test(options.join(" ")) && CAPSULE_DREAM.test(options.join(" ")), {
    hook: "playground.navbar.fixture",
    options,
  });
});

add("example-concrete-forest", "§5-examples", "replace", async () => {
  const picked = await selectExample(/concrete/i);
  const r = await waitUntil(board, (v) => v.nodes >= 1 && v.parsed === "true", 90000);
  const w = await world();
  verdict("§5-examples", "concrete-forest-loads", Boolean(picked.label) && r.ok, { label: picked.label, nodes: r.value.nodes, edges: r.value.edges, instances: w.instanceCount, waitedMs: r.waitedMs });
});

add("example-nakagin", "§5-examples", "replace", async () => {
  const picked = await selectExample(/nakagin/i);
  const r = await waitUntil(board, (v) => v.nodes > 1 && v.parsed === "true", 120000);
  const w = await waitUntil(world, (v) => v.instanceCount > 0, 30000);
  verdict("§5-examples", "nakagin-loads", Boolean(picked.label) && r.ok, { label: picked.label, nodes: r.value.nodes, edges: r.value.edges, instances: w.value.instanceCount, waitedMs: r.waitedMs });
  const s = await snapshot();
  verdict("§5-examples", "nakagin-no-recovery-card", !s.recovery.some((x) => x && x !== "?"), { recovery: s.recovery });
});

/** 🌙️ `capsule-dream` is 5d-only and may legitimately be REFUSED with a notice (an example larger than the
 * envelope page budget). A refusal without a fault is a PASS: what must never happen is a silent swallow. */
add("example-capsule-dream", "§5-examples", "replace", async () => {
  const hardBefore = hardFaults.length;
  const picked = await selectExample(CAPSULE_DREAM);
  const r = await waitUntil(board, (v) => v.parsed === "true", 90000);
  const refused = picked.notices.length > 0;
  verdict("§5-examples", "capsule-dream-loads-or-refuses", Boolean(picked.label) && (r.ok || refused) && hardFaults.length === hardBefore, {
    label: picked.label,
    refused,
    notices: picked.notices,
    nodes: r.value.nodes,
    newHardFaults: hardFaults.length - hardBefore,
    waitedMs: r.waitedMs,
  });
});

add("example-switch-back", "§5-examples", "replace", async () => {
  const picked = await selectExample(/concrete/i);
  const r = await waitUntil(board, (v) => v.nodes >= 1 && v.parsed === "true", 90000);
  verdict("§5-examples", "switch-returns-to-concrete", Boolean(picked.label) && r.ok, { label: picked.label, nodes: r.value.nodes, waitedMs: r.waitedMs });
});

// ── §16-19 panels ───────────────────────────────────────────────────────────────────────────────────
add("panels", "§17-19-panels", "read", async () => {
  await ensureDocument(1);
  for (const [tab, label] of [
    ["framework.panel.artifact", "outliner"],
    ["framework.panel.catalogue", "catalogue"],
    ["framework.panel.inspection", "inspector"],
    ["framework.panel.history", "history"],
    ["framework.panel.toolRun", "tool-runs"],
  ] as const) {
    const opened = await clickTab(tab);
    const text = await panelText(tab);
    const s = await snapshot();
    verdict("§17-19-panels", `${label}-opens`, opened && text.trim().length > 0, { hook: tab, opened, treeItems: s.treeItems, head: text.slice(0, 160) });
    if (opened) await clickTab(tab);
  }
  log(`  closed panels: ${JSON.stringify(await closePanels())}`);
});

/** 🛍️ The catalogue must be non-empty. 5d infers its kind rows from the document when the manifest declares
 * none (slice 5C's kind-row inference fallback) — an empty catalogue means that fallback did not fire. */
add("catalogue-rows", "§18-catalogue", "read", async () => {
  await ensureDocument(1);
  const opened = await clickTab("framework.panel.catalogue");
  await settle(1.5);
  const rows = await panelRows("puzzle5d-play-kinds");
  verdict("§18-catalogue", "kind-rows-non-empty", opened && rows.length > 0, { hook: "puzzle5d-play-kinds.{parts,grips,fasteners,ropes}", rows: rows.slice(0, 12), count: rows.length });
  await closePanels();
});

add("inspector-per-entity", "§16-inspection", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickBoardNode();
  const opened = await clickTab("framework.panel.inspection");
  await settle(2);
  const rows = await panelRows("puzzle5d-play-inspector");
  const named = picked.id ? rows.some((row) => row.includes(picked.id!)) : false;
  verdict("§16-inspection", "inspector-shows-selected-part", opened && picked.ok && named, { hook: "puzzle5d-play-inspector.*", id: picked.id, rows: rows.slice(0, 10) });
  verdict("§16-inspection", "inspector-not-document-summary", rows.length > 0 && !rows.some((row) => row.includes("puzzle5d-play-inspector.empty")), { rows: rows.slice(0, 6) });
  await closePanels();
});

add("settings-panel", "§19-settings", "read", async () => {
  const opened = await clickTab("framework.settings");
  await settle(1.5);
  const child = page.locator('[data-slot="panel-tab-button"][id="puzzle5d.panel.settings"]').first();
  const childPresent = await countSafe(child);
  if (childPresent) await child.click({ timeout: 4000 }).catch(() => {});
  await settle(1.5);
  const inputs = await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"] input')).map((e) => `${e.id}=${(e as HTMLInputElement).value}`), [] as string[]);
  verdict("§19-settings", "settings-panel-present", opened && childPresent > 0, { hook: "puzzle5d.panel.settings", inputs: inputs.slice(0, 10) });
  const stepper = page.locator('[data-slot="panel"] input').first();
  const present = (await countSafe(stepper)) > 0;
  const before = present ? await stepper.inputValue().catch(() => "") : "";
  const plus = page.locator('[data-slot="stepper-plus"]').first();
  if (await countSafe(plus)) await plus.click({ timeout: 4000 }).catch(() => {});
  const r = await waitUntil(async () => (present ? await stepper.inputValue().catch(() => "") : ""), (value) => value !== "" && value !== before, 15000);
  verdict("§19-settings", "settings-stepper-changes-value", present && r.ok, { hook: "[data-slot=stepper-plus]", before, after: r.value, waitedMs: r.waitedMs });
  await closePanels();
});

// ── §2 camera, per pane ─────────────────────────────────────────────────────────────────────────────
add("camera-board-wheel", "§2-camera", "read", async () => {
  await closePanels();
  const before = (await board()).camera;
  const worldBefore = (await world()).camera;
  const centre = await boardCentre();
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.wheel(0, -320);
  const r = await waitUntil(board, (v) => Boolean(v.camera) && v.camera !== before, 20000);
  verdict("§2-camera", "board-wheel-zoom-persists", r.ok, { hook: "data-board-camera-json", before, after: r.value.camera, waitedMs: r.waitedMs });
  const worldAfter = (await world()).camera;
  verdict("§2-camera", "board-zoom-leaves-world-camera", worldAfter === worldBefore, { hook: "data-camera-json", worldBefore, worldAfter });
});

add("camera-world-orbit", "§2-camera", "read", async () => {
  await closePanels();
  const before = (await world()).camera;
  const boardBefore = (await board()).camera;
  await orbitWorld();
  const r = await waitUntil(world, (v) => Boolean(v.camera) && v.camera !== before, 20000);
  verdict("§2-camera", "world-orbit-persists-camera", r.ok, { hook: "data-camera-json (setCamera3d)", before: String(before).slice(0, 120), after: String(r.value.camera).slice(0, 120), waitedMs: r.waitedMs });
  const boardAfter = (await board()).camera;
  verdict("§2-camera", "world-orbit-leaves-board-camera", boardAfter === boardBefore, { hook: "data-board-camera-json", boardBefore, boardAfter });
});

// ── §6 selection, cross-pane ────────────────────────────────────────────────────────────────────────
add("select-board-to-world", "§6-selection", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickBoardNode();
  const mirrored = await waitUntil(async () => (await worldSelection()).confirmed, (ids) => (picked.id ? ids.includes(picked.id) : false), MUTATION_MS);
  verdict("§6-selection", "board-click-selects", picked.ok, { hook: "data-board-selection-json", id: picked.id, at: picked.at, waitedMs: picked.waitedMs });
  verdict("§6-selection", "board-selection-highlights-world", picked.ok && mirrored.ok, { hook: "data-guest-selection-json", id: picked.id, world: mirrored.value.slice(0, 8), waitedMs: mirrored.waitedMs });
});

add("select-world-to-board", "§6-selection", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickWorldInstance();
  const first = picked.ids[0];
  const mirrored = await waitUntil(boardSelection, (ids) => (first ? ids.includes(first) : false), MUTATION_MS);
  verdict("§6-selection", "world-click-selects", picked.ok, { hook: "data-guest-selection-json", ids: picked.ids.slice(0, 6), waitedMs: picked.waitedMs });
  verdict("§6-selection", "world-selection-highlights-board", picked.ok && mirrored.ok, { hook: "data-board-selection-json", id: first, board: mirrored.value.slice(0, 8), waitedMs: mirrored.waitedMs });
});

/** 🫴️ Hover is READABLE on both hosts and asserted by NEITHER existing battery (`📓️E6` §4.4) — the 5d
 * battery closes that gap: hovering in one pane must name the same entity in the other. */
add("hover-pairing", "§7-hover", "read", async () => {
  await ensureDocument(1);
  await closePanels();
  const first = (await visibleBoardNodeIds())[0];
  const at = first ? await boardNodeScreen(first) : null;
  if (!at || !first) {
    verdict("§7-hover", "board-hover-publishes", false, { hook: "data-board-hovered-id", reason: "no reachable board node" });
    return;
  }
  await page.mouse.move(at.x, at.y, { steps: 6 });
  await page.mouse.move(at.x + 1, at.y + 1, { steps: 2 });
  const hovered = await waitUntil(board, (v) => v.hovered === first, 20000);
  verdict("§7-hover", "board-hover-publishes", hovered.ok, { hook: "data-board-hovered-id", id: first, hovered: hovered.value.hovered, waitedMs: hovered.waitedMs });
  const paired = await waitUntil(
    async () => {
      const w = await world();
      const interaction = jsonOf(w.interaction, {} as { hoverTarget?: { id?: string } | null; hoveredVortexFullId?: string | null });
      return `${interaction.hoverTarget?.id ?? ""}|${interaction.hoveredVortexFullId ?? ""}|${w.hoverPaint}`;
    },
    (value) => value.includes(first),
    20000,
  );
  verdict("§7-hover", "hover-pairs-across-panes", paired.ok, { hook: "data-interaction-json.hoverTarget|data-hover-paint-id", id: first, world: paired.value, waitedMs: paired.waitedMs });
});

add("marquee", "§6-selection", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const box = await boardBox();
  await page.mouse.move(box.x + 30, box.y + 170);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width - 30, box.y + box.height - 40, { steps: 14 });
  await page.mouse.up();
  const r = await waitUntil(boardSelection, (ids) => ids.length >= 2, 20000);
  verdict("§6-selection", "marquee-selects-many", r.ok, { hook: "data-board-selection-json", selected: r.value.length, waitedMs: r.waitedMs });
  await page.keyboard.press("Escape").catch(() => {});
});

add("select-same-kind", "§6-selection", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const picked = await pickBoardNode();
  if (!picked.at) {
    verdict("§6-selection", "select-same-kind", false, { hook: "context menu › same kind", reason: "no reachable board node" });
    return;
  }
  const menu = await contextMenuAt(picked.at.x, picked.at.y);
  const clicked = await clickMenuRow(/same kind/i);
  const r = await waitUntil(boardSelection, (ids) => ids.length > 1, MUTATION_MS);
  verdict("§6-selection", "select-same-kind", clicked && r.ok, { hook: "selectSameKindSelection", rows: menu.rows.slice(0, 12), selected: r.value.length, waitedMs: r.waitedMs });
  await page.keyboard.press("Escape").catch(() => {});
});

/** 🗂️ The row VOCABULARY must follow the selection kind — a part, a grip and a fastener do not offer the
 * same verbs, and a menu ported verbatim from 3d would carry ids that are dead in 5d's own registry
 * (`📓️E3` §6: `focusSelection` vs `zoomToSelection` have opposite polarity between the two artifacts). */
add("context-menu-rows", "§15-context-menu", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickBoardNode();
  const boardMenu = picked.at ? await contextMenuAt(picked.at.x, picked.at.y) : { rows: [] as string[], menus: 0 };
  await page.keyboard.press("Escape").catch(() => {});
  const joined = boardMenu.rows.join(" | ");
  verdict("§15-context-menu", "board-part-menu-opens", boardMenu.menus > 0 && boardMenu.rows.length > 0, { hook: '[role="menu"]', rows: boardMenu.rows.slice(0, 16) });
  verdict("§15-context-menu", "board-part-menu-vocabulary", /delete/i.test(joined) && /duplicate/i.test(joined) && /same kind/i.test(joined) && /(hide|show)/i.test(joined) && /(lock|unlock)/i.test(joined) && /(zoom|focus)/i.test(joined), { rows: boardMenu.rows.slice(0, 16) });
  const centre = await worldCentre();
  await page.mouse.click(centre.x, centre.y);
  await settle(1);
  const worldMenu = await contextMenuAt(centre.x, centre.y);
  await page.keyboard.press("Escape").catch(() => {});
  verdict("§15-context-menu", "world-menu-opens", worldMenu.menus > 0 && worldMenu.rows.length > 0, { hook: '[role="menu"] (world pane)', rows: worldMenu.rows.slice(0, 16) });
});

// ── §8 transform ────────────────────────────────────────────────────────────────────────────────────
/** 🚚️ A drag on the BOARD is a flat-pose move only: the 2d pane owns x/y, never the 3d pose. */
add("drag-part-board", "§8-transform", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const first = (await visibleBoardNodeIds())[0];
  const at = first ? await boardNodeScreen(first) : null;
  if (!at || !first) {
    verdict("§8-transform", "board-drag-moves-flat-pose", false, { hook: "data-board-positions-json", reason: "no reachable board node" });
    return;
  }
  const before = jsonOf((await board()).positions, {} as Record<string, [number, number]>)[first];
  await page.mouse.move(at.x, at.y);
  await page.mouse.down();
  await page.mouse.move(at.x + 90, at.y + 50, { steps: 12 });
  await page.mouse.up();
  const r = await waitUntil(
    async () => jsonOf((await board()).positions, {} as Record<string, [number, number]>),
    (p) => Boolean(p[first]) && (Math.abs(p[first][0] - before[0]) > 1 || Math.abs(p[first][1] - before[1]) > 1),
    MUTATION_MS,
  );
  verdict("§8-transform", "board-drag-moves-flat-pose", r.ok, { hook: "applyBoardEvents → data-board-positions-json", id: first, before, after: r.value[first], waitedMs: r.waitedMs });
});

/** 🔄️ The gumball lives in the world pane. Translate then rotate, and after each the 3d pose must change
 * AND the board's flat pose must stay coherent with it — a 5d document has one truth, projected twice. */
add("gumball-translate", "§8-transform", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickWorldInstance();
  const armed = await worldUtility(/move|translate|transform/i);
  const poseBefore = (await world()).instancesRaw;
  const boardBefore = (await board()).positions;
  const centre = await worldCentre();
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.down();
  await page.mouse.move(centre.x + 120, centre.y, { steps: 16 });
  await page.mouse.up();
  const r = await waitUntil(world, (v) => v.instancesRaw.length > 0 && v.instancesRaw !== poseBefore, MUTATION_MS);
  verdict("§8-transform", "gumball-translate-changes-pose", picked.ok && r.ok, { hook: "translateSelection → data-instances-json", armed: armed.ok, utility: armed.utility, selected: picked.ids.slice(0, 4), before: poseBefore.length, after: r.value.instancesRaw.length, waitedMs: r.waitedMs });
  const flat = await waitUntil(board, (v) => v.positions !== boardBefore, MUTATION_MS);
  verdict("§8-transform", "gumball-translate-updates-board", flat.ok, { hook: "data-board-positions-json", waitedMs: flat.waitedMs });
});

add("gumball-rotate", "§8-transform", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickWorldInstance();
  const armed = await worldUtility(/rotate/i);
  const poseBefore = (await world()).instancesRaw;
  const box = await worldBox();
  const from = { x: box.x + box.width / 2 + 70, y: box.y + box.height / 2 };
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move(from.x, from.y + 110, { steps: 16 });
  await page.mouse.up();
  const r = await waitUntil(world, (v) => v.instancesRaw.length > 0 && v.instancesRaw !== poseBefore, MUTATION_MS);
  verdict("§8-transform", "gumball-rotate-changes-pose", picked.ok && r.ok, { hook: "rotateSelection → data-instances-json", armed: armed.ok, utility: armed.utility, waitedMs: r.waitedMs });
});

// ── §9 brush ────────────────────────────────────────────────────────────────────────────────────────
/** 🖌️ The brush places from a GRIP: arm the utility, hover a published grip until the pane previews a
 * candidate, cycle the candidates with Tab / Shift+Tab, then click to place. */
add("brush-place-from-grip", "§9-brush", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const armed = await boardUtility(/brush/i);
  const before = await boardCensus();
  const grips = jsonOf((await world()).vortices, [] as { fullId?: string; sx?: number; sy?: number }[]);
  const box = await worldBox();
  const aim = grips.find((grip) => typeof grip.sx === "number" && typeof grip.sy === "number" && grip.sx! > 8 && grip.sy! > 8 && grip.sx! < box.width - 8 && grip.sy! < box.height - 8);
  const point = aim ? { x: box.x + aim.sx!, y: box.y + aim.sy! } : await worldCentre();
  for (let i = 0; i < 24; i++) await page.mouse.move(point.x + (i % 3) - 1, point.y + (i % 2), { steps: 1 });
  await settle(1.5);
  const previewed = await waitUntil(world, (v) => v.preview.length > 2 || v.hoverPaint !== "", 20000);
  verdict("§9-brush", "brush-previews-candidate", armed.ok && previewed.ok, { hook: "data-engagement-preview-json|data-vortices-json", armed: armed.utility, grips: grips.length, preview: previewed.value.preview.slice(0, 120), waitedMs: previewed.waitedMs });
  const firstPreview = (await world()).preview;
  await page.keyboard.press("Tab").catch(() => {});
  const cycled = await waitUntil(world, (v) => v.preview !== firstPreview, 15000);
  verdict("§9-brush", "tab-cycles-candidate", cycled.ok, { hook: "cycleBrushCandidate", before: firstPreview.slice(0, 80), after: cycled.value.preview.slice(0, 80), waitedMs: cycled.waitedMs });
  await page.keyboard.press("Shift+Tab").catch(() => {});
  const back = await waitUntil(world, (v) => v.preview !== cycled.value.preview, 15000);
  verdict("§9-brush", "shift-tab-cycles-back", back.ok, { hook: "cycleBrushCandidate(-1)", waitedMs: back.waitedMs });
  await page.mouse.click(point.x, point.y);
  const placed = await waitUntil(boardCensus, (census) => census > before, MUTATION_MS);
  verdict("§9-brush", "brush-click-places-part", placed.ok, { hook: "addBrushPart", before, after: placed.value, waitedMs: placed.waitedMs });
});

/** 🫧️ The grip suggestions popup (3d's alt+right-click gesture). */
add("grip-suggestions", "§13-suggestions", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  await boardUtility(/brush/i);
  const centre = await worldCentre();
  await page.mouse.move(centre.x, centre.y, { steps: 6 });
  await settle(1);
  await page.keyboard.down("Alt").catch(() => {});
  await page.mouse.click(centre.x, centre.y, { button: "right" });
  await settle(2);
  await page.keyboard.up("Alt").catch(() => {});
  const menuJson = (await world()).suggestionMenu;
  const rows = await evalSafe(() => Array.from(document.querySelectorAll('[role="menuitem"]')).map((m) => (m as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)), [] as string[]);
  verdict("§13-suggestions", "suggestions-popup-opens", menuJson.length > 2 || rows.length > 0, { hook: "data-suggestion-menu-json|targetBrushSuggestions", menu: menuJson.slice(0, 200), rows: rows.slice(0, 10) });
  await page.keyboard.press("Escape").catch(() => {});
});

// ── §10 target volumes ──────────────────────────────────────────────────────────────────────────────
add("volume-brush", "§10-volumes", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const armed = await worldUtility(/volume/i);
  const before = jsonOf((await world()).volumes, [] as unknown[]).length;
  const box = await worldBox();
  await page.mouse.move(box.x + box.width * 0.35, box.y + box.height * 0.6);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width * 0.65, box.y + box.height * 0.35, { steps: 18 });
  await page.mouse.up();
  const r = await waitUntil(async () => jsonOf((await world()).volumes, [] as unknown[]).length, (count) => count > before, MUTATION_MS);
  verdict("§10-volumes", "volume-brush-paints-volume", armed.ok && r.ok, { hook: "data-target-volumes-json|addTargetVolume", armed: armed.utility, before, after: r.value, waitedMs: r.waitedMs });
});

/** 🧊️ A painted target volume CONSTRAINS the fill: every instance the run places must sit inside the union
 * of the published volumes. The verdict names the escapees rather than a bare boolean, because "fill
 * ignored the volume" and "the volume was published with the wrong extent" are different defects. */
add("fill-inside-volume", "§10-volumes", "mutate", async () => {
  const volumes = jsonOf((await world()).volumes, [] as { min?: number[]; max?: number[]; origin?: number[]; size?: number[] }[]);
  if (!volumes.length) {
    verdict("§10-volumes", "fill-stays-inside-volume", false, { hook: "data-target-volumes-json", reason: "no target volume published — run volume-brush first" });
    return;
  }
  const boxes = volumes.map((volume) => {
    const min = volume.min ?? volume.origin ?? [0, 0, 0];
    const max = volume.max ?? (volume.origin && volume.size ? volume.origin.map((value, index) => value + (volume.size![index] ?? 0)) : [0, 0, 0]);
    return { min, max };
  });
  const before = new Set((await world()).instanceIds);
  await clickTab("framework.category.tool");
  const toggle = page.locator('[id="tool.fill"]').first();
  if ((await countSafe(toggle)) && (await toggle.getAttribute("aria-pressed").catch(() => null)) !== "true") await toggle.click({ timeout: 4000 }).catch(() => {});
  await clickTab("framework.panel.toolRun");
  const start = page.locator("button", { hasText: /^start$/i }).first();
  if (await countSafe(start)) await start.click({ timeout: 4000 }).catch(() => {});
  const grew = await waitUntil(world, (v) => v.instanceIds.some((id) => !before.has(id)), 120000);
  const tolerance = 0.5;
  const escapees = grew.value.instances
    .filter((instance) => instance.id && !before.has(instance.id) && Array.isArray(instance.position))
    .filter((instance) => !boxes.some((box) => instance.position!.every((value, index) => value >= (box.min[index] ?? -Infinity) - tolerance && value <= (box.max[index] ?? Infinity) + tolerance)))
    .map((instance) => `${instance.id}@${JSON.stringify(instance.position)}`);
  verdict("§10-volumes", "fill-stays-inside-volume", grew.ok && escapees.length === 0, { hook: "fill ∩ data-target-volumes-json", volumes: boxes.slice(0, 4), placed: grew.value.instanceIds.filter((id) => !before.has(id)).length, escapees: escapees.slice(0, 8), waitedMs: grew.waitedMs });
  const abort = page.locator("button", { hasText: /^abort$/i }).first();
  if (await countSafe(abort)) await abort.click({ timeout: 4000 }).catch(() => {});
  await closePanels();
});

// ── §12 fill ────────────────────────────────────────────────────────────────────────────────────────
/** 🪣️ Fill is a first-class Tool in 5d (`🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, `TOOL_ID = "fill"`), so it
 * has the full ToolRun chrome: an activate toggle, a count measure, part/grip weight groups, and
 * start/pause/step/abort/finalize. The provisional placements must show in BOTH panes while the run is
 * live, and the whole run must undo in ONE history step (a per-placement ledger row would kill the store
 * after 64 edits). */
add("fill-tool-chrome", "§12-fill", "read", async () => {
  await ensureDocument(1);
  await clickTab("framework.category.tool");
  const toggle = page.locator('[data-slot="toggle-group-item"][id="tool.fill"], [id="tool.fill"]').first();
  const present = await countSafe(toggle);
  verdict("§12-fill", "fill-tool-tab-present", present > 0, { hook: "tool.fill (setActiveTool)" });
  if (!present) return;
  if ((await toggle.getAttribute("aria-pressed").catch(() => null)) !== "true") await toggle.click({ timeout: 4000 }).catch(() => {});
  await settle(2);
  const count = page.locator('[id*="puzzle5d-fill-count"] input, input[id*="fill-count"]').first();
  verdict("§12-fill", "fill-count-measure", (await countSafe(count)) > 0, { hook: "puzzle5d-fill-count (setFillCount)" });
  const body = await evalSafe(() => document.body.innerText, "");
  verdict("§12-fill", "fill-weight-groups", /part weights|teilgewichte/i.test(body) && /grip weights|griffgewichte/i.test(body), { hook: "setObjectKindWeight|setVortexKindWeight" });
});

add("fill-run", "§12-fill", "mutate", async () => {
  await ensureDocument(1);
  await clickTab("framework.category.tool");
  const toggle = page.locator('[id="tool.fill"]').first();
  if ((await countSafe(toggle)) && (await toggle.getAttribute("aria-pressed").catch(() => null)) !== "true") await toggle.click({ timeout: 4000 }).catch(() => {});
  await clickTab("framework.panel.toolRun");
  await settle(1);
  const runText = () => panelText("framework.panel.toolRun");
  const boardBeforeCount = (await board()).nodes;
  const worldBeforeCount = (await world()).instanceCount;
  const start = page.locator("button", { hasText: /^start$/i }).first();
  const startPresent = await countSafe(start);
  if (startPresent) await start.click({ timeout: 4000 }).catch(() => {});
  const grew = await waitUntil(board, (v) => v.nodes > boardBeforeCount, 120000);
  verdict("§12-fill", "fill-start-places-provisional-board", startPresent > 0 && grew.ok, { hook: "start (s.puzzle.puzzle5d.fill.run)", before: boardBeforeCount, after: grew.value.nodes, waitedMs: grew.waitedMs });
  const grewWorld = await waitUntil(world, (v) => v.instanceCount > worldBeforeCount, MUTATION_MS);
  verdict("§12-fill", "fill-provisional-visible-in-world", grewWorld.ok, { hook: "data-instances-json", before: worldBeforeCount, after: grewWorld.value.instanceCount, waitedMs: grewWorld.waitedMs });
  for (const [label, match] of [["pause", /^pause$/i], ["step", /^step$/i], ["resume", /^resume$/i]] as const) {
    const button = page.locator("button", { hasText: match }).first();
    const has = await countSafe(button);
    if (has) await button.click({ timeout: 4000 }).catch(() => {});
    await settle(1);
    verdict("§12-fill", `fill-${label}-control`, has > 0, { hook: `ToolRun ${label}`, status: (await runText()).slice(0, 140) });
  }
  // 🎚️ Raising and lowering the target count MID-RUN must re-plan, not restart: the run keeps its identity
  // and the census follows the new target.
  const count = page.locator('[id*="puzzle5d-fill-count"] input, input[id*="fill-count"]').first();
  const countPresent = await countSafe(count);
  const countBefore = countPresent ? await count.inputValue().catch(() => "") : "";
  if (countPresent) {
    await count.fill(String(Number(countBefore || "0") + 6)).catch(() => {});
    await count.press("Enter").catch(() => {});
  }
  const raised = await waitUntil(async () => (countPresent ? await count.inputValue().catch(() => "") : ""), (value) => value !== countBefore && value !== "", MUTATION_MS);
  verdict("§12-fill", "fill-raise-mid-run", countPresent > 0 && raised.ok, { hook: "setFillCount", before: countBefore, after: raised.value, waitedMs: raised.waitedMs });
  if (countPresent) {
    await count.fill(countBefore || "1").catch(() => {});
    await count.press("Enter").catch(() => {});
  }
  const lowered = await waitUntil(async () => (countPresent ? await count.inputValue().catch(() => "") : ""), (value) => value === (countBefore || "1"), MUTATION_MS);
  verdict("§12-fill", "fill-lower-mid-run", countPresent > 0 && lowered.ok, { hook: "setFillCount", after: lowered.value, waitedMs: lowered.waitedMs });
  const complete = await waitUntil(runText, (text) => /ready to finalize|complete|abgeschlossen/i.test(text), 240000, 1000);
  const placed = (await board()).nodes;
  const finalize = page.locator("button", { hasText: /^finalize$/i }).first();
  const finalizePresent = await countSafe(finalize);
  if (finalizePresent) await finalize.click({ timeout: 4000 }).catch(() => {});
  const gone = await waitUntil(runText, (text) => !/ready to finalize|running|retracting/i.test(text), MUTATION_MS, 1000);
  const after = (await board()).nodes;
  verdict("§12-fill", "fill-finalize-keeps-placements", complete.ok && finalizePresent > 0 && gone.ok && after > boardBeforeCount, { hook: "finalize", before: boardBeforeCount, placed, after, waitedMs: gone.waitedMs });
  const undone = await (async () => {
    const before = await boardCensus();
    await runAction(boardWindow, "undo");
    const r = await waitUntil(boardCensus, (census) => census < before, MUTATION_MS);
    await setActions(false, boardWindow);
    return { before, after: r.value, ok: r.ok, waitedMs: r.waitedMs };
  })();
  verdict("§12-fill", "fill-undoes-in-one-step", undone.ok && undone.after <= boardBeforeCount + (await board()).edges, { hook: "action.undo", ...undone });
  await closePanels();
});

add("fill-abort", "§12-fill", "mutate", async () => {
  await ensureDocument(1);
  await clickTab("framework.category.tool");
  const toggle = page.locator('[id="tool.fill"]').first();
  if ((await countSafe(toggle)) && (await toggle.getAttribute("aria-pressed").catch(() => null)) !== "true") await toggle.click({ timeout: 4000 }).catch(() => {});
  await clickTab("framework.panel.toolRun");
  const start = page.locator("button", { hasText: /^start$/i }).first();
  if (await countSafe(start)) await start.click({ timeout: 4000 }).catch(() => {});
  await settle(3);
  await page.keyboard.press("Escape").catch(() => {});
  const runText = () => panelText("framework.panel.toolRun");
  const aborted = await waitUntil(runText, (text) => !/running/i.test(text), MUTATION_MS, 1000);
  verdict("§12-fill", "escape-aborts-run", aborted.ok, { hook: "engagementAbort", status: aborted.value.slice(0, 160), waitedMs: aborted.waitedMs });
  await closePanels();
});

// ── §11 relocate / fasteners ────────────────────────────────────────────────────────────────────────
add("world-relocate", "§11-relocate", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickWorldInstance();
  const armed = await worldUtility(/relocate/i);
  const before = (await world()).instancesRaw;
  const centre = await worldCentre();
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.down();
  await page.mouse.move(centre.x + 140, centre.y + 60, { steps: 16 });
  await page.mouse.up();
  const r = await waitUntil(world, (v) => v.instancesRaw.length > 0 && v.instancesRaw !== before, MUTATION_MS);
  verdict("§11-relocate", "world-relocate-moves-part", picked.ok && armed.ok && r.ok, { hook: "worldRelocate", armed: armed.utility, waitedMs: r.waitedMs });
});

add("fastener-crud", "§11-fasteners", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const before = (await board()).edges;
  const ids = await visibleBoardNodeIds();
  const a = ids[0] ? await boardNodeScreen(ids[0]) : null;
  const b = ids[1] ? await boardNodeScreen(ids[1]) : null;
  if (!a || !b) {
    verdict("§11-fasteners", "fastener-create", false, { hook: "createFastener", reason: "need two reachable board parts" });
    return;
  }
  await page.mouse.click(a.x, a.y);
  // 🖱️ `page.mouse.click` takes no `modifiers` — the chord is held on the keyboard around the press.
  await page.keyboard.down("Shift").catch(() => {});
  await page.mouse.click(b.x, b.y);
  await page.keyboard.up("Shift").catch(() => {});
  const engaged = await engage(boardWindow, BOARD_KIND, "connect");
  const created = await waitUntil(board, (v) => v.edges > before, MUTATION_MS);
  verdict("§11-fasteners", "fastener-create", engaged.present && created.ok, { hook: "createFastener / engagement `connect`", before, after: created.value.edges, waitedMs: created.waitedMs });
  const menu = await contextMenuAt(b.x, b.y);
  const retarget = await clickMenuRow(/retarget/i);
  verdict("§11-fasteners", "fastener-retarget-row", retarget, { hook: "retargetFastener", rows: menu.rows.slice(0, 12) });
  await page.keyboard.press("Escape").catch(() => {});
  const afterCreate = (await board()).edges;
  await page.mouse.click(b.x, b.y);
  await page.keyboard.press("Delete").catch(() => {});
  const deleted = await waitUntil(board, (v) => v.edges < afterCreate, MUTATION_MS);
  verdict("§11-fasteners", "fastener-delete", deleted.ok, { hook: "deleteFastener", before: afterCreate, after: deleted.value.edges, waitedMs: deleted.waitedMs });
  await setActions(false, boardWindow);
});

add("proximity-connect", "§11-fasteners", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const before = (await board()).edges;
  const engaged = await engage(boardWindow, BOARD_KIND, "connect");
  const r = await waitUntil(board, (v) => v.edges !== before, MUTATION_MS);
  verdict("§11-fasteners", "proximity-connect", engaged.present && r.ok, { hook: "proximityConnect", before, after: r.value.edges, waitedMs: r.waitedMs });
  await setActions(false, boardWindow);
});

// ── §16 inspector write-back ────────────────────────────────────────────────────────────────────────
add("inspector-patch", "§16-inspection", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const picked = await pickBoardNode();
  await clickTab("framework.panel.inspection");
  await settle(2);
  const field = page.locator('[data-slot="panel"] input').first();
  const present = await countSafe(field);
  const before = present ? await field.inputValue().catch(() => "") : "";
  if (present) {
    await field.fill(`${before}1`).catch(() => {});
    await field.press("Enter").catch(() => {});
  }
  const r = await waitUntil(async () => (present ? await field.inputValue().catch(() => "") : ""), (value) => value !== before && value !== "", MUTATION_MS);
  verdict("§16-inspection", "inspector-patch-writes-back", picked.ok && present > 0 && r.ok, { hook: "patchPart|patchGrip|patchFastener", id: picked.id, before, after: r.value, waitedMs: r.waitedMs });
  await closePanels();
});

// ── §17 outliner ────────────────────────────────────────────────────────────────────────────────────
add("outliner-hide-lock", "§17-outliner", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const opened = await clickTab("framework.panel.artifact");
  await settle(1.5);
  const rows = await panelRows("puzzle5d-play-document");
  const hide = page.locator('[data-slot="panel"] button', { hasText: /^(hide|show|ausblenden|einblenden)$/i }).first();
  const hidePresent = await countSafe(hide);
  if (hidePresent) await hide.click({ timeout: 4000 }).catch(() => {});
  await settle(1.5);
  const afterHide = await panelRows("puzzle5d-play-document");
  verdict("§17-outliner", "outliner-hide-toggles", opened && hidePresent > 0 && JSON.stringify(afterHide) !== JSON.stringify(rows), { hook: "setSelectionFlag(hidden)", rows: rows.slice(0, 6), after: afterHide.slice(0, 6) });
  const lock = page.locator('[data-slot="panel"] button', { hasText: /^(lock|unlock|sperren|entsperren)$/i }).first();
  const lockPresent = await countSafe(lock);
  if (lockPresent) await lock.click({ timeout: 4000 }).catch(() => {});
  await settle(1.5);
  const afterLock = await panelRows("puzzle5d-play-document");
  verdict("§17-outliner", "outliner-lock-toggles", lockPresent > 0 && JSON.stringify(afterLock) !== JSON.stringify(afterHide), { hook: "setSelectionFlag(locked)", after: afterLock.slice(0, 6) });
  // 🔁️ Restore: a battery that leaves rows hidden/locked poisons every later lane.
  if (lockPresent) await page.locator('[data-slot="panel"] button', { hasText: /^(lock|unlock|sperren|entsperren)$/i }).first().click({ timeout: 4000 }).catch(() => {});
  if (hidePresent) await page.locator('[data-slot="panel"] button', { hasText: /^(hide|show|ausblenden|einblenden)$/i }).first().click({ timeout: 4000 }).catch(() => {});
  await settle(1.5);
  const restored = await panelRows("puzzle5d-play-document");
  verdict("§17-outliner", "outliner-show-restores", JSON.stringify(restored) === JSON.stringify(rows), { before: rows.slice(0, 6), restored: restored.slice(0, 6) });
  await closePanels();
});

// ── §18 catalogue add ───────────────────────────────────────────────────────────────────────────────
add("catalogue-click-add", "§18-catalogue", "mutate", async () => {
  await ensureDocument(1);
  await clickTab("framework.panel.catalogue");
  await settle(1.5);
  const before = (await board()).nodes;
  const row = page.locator('[id*="puzzle5d-play-kinds.parts."]').first();
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 4000 }).catch(() => {});
  const r = await waitUntil(board, (v) => v.nodes === before + 1, MUTATION_MS);
  verdict("§18-catalogue", "catalogue-click-adds-part", present > 0 && r.ok, { hook: "puzzle5d-play-kinds.parts.* → addPartKind", before, after: r.value.nodes, waitedMs: r.waitedMs });
  await closePanels();
});

add("catalogue-drag-drop", "§18-catalogue", "mutate", async () => {
  await ensureDocument(1);
  await clickTab("framework.panel.catalogue");
  await settle(1.5);
  const row = page.locator('[id*="puzzle5d-play-kinds.parts."]').first();
  if (!(await countSafe(row))) {
    verdict("§18-catalogue", "catalogue-drag-into-board", false, { hook: "puzzle5d-play-kinds.parts.*", reason: "no catalogue row" });
    return;
  }
  const rowBox = (await row.boundingBox()) ?? { x: 0, y: 0, width: 1, height: 1 };
  for (const [label, target, census] of [
    ["board", boardBox, async () => (await board()).nodes],
    ["world", worldBox, async () => (await world()).instanceCount],
  ] as const) {
    const before = await census();
    const box = await target();
    await page.mouse.move(rowBox.x + rowBox.width / 2, rowBox.y + rowBox.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width * 0.6, box.y + box.height * 0.5, { steps: 18 });
    await page.mouse.up();
    const r = await waitUntil(census, (count) => count > before, MUTATION_MS);
    verdict("§18-catalogue", `catalogue-drag-into-${label}`, r.ok, { hook: label === "board" ? "pushPuzzle5dFixtureDropPreview" : "WorldCatalogueDropPreviewStore", before, after: r.value, waitedMs: r.waitedMs });
  }
  await closePanels();
});

// ── §23 add-part dialog ─────────────────────────────────────────────────────────────────────────────
add("add-part-dialog", "§23-add-dialog", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const before = (await board()).nodes;
  const opened = await runAction(boardWindow, "addNode");
  await settle(2);
  const dialog = await evalSafe(() => Array.from(document.querySelectorAll('[role="dialog"]')).map((d) => (d as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 200)), [] as string[]);
  const options = await evalSafe(() => Array.from(document.querySelectorAll('[role="dialog"] option, [role="dialog"] [role="option"]')).map((o) => (o as HTMLElement).innerText.trim()).filter(Boolean), [] as string[]);
  verdict("§23-add-dialog", "add-part-dialog-opens", opened.present && dialog.length > 0, { hook: opened.hook, dialog, options: options.slice(0, 12) });
  verdict("§23-add-dialog", "add-part-dialog-lists-live-kinds", options.length > 1, { hook: "addNode kind options", options: options.slice(0, 12) });
  const confirm = page.locator('[role="dialog"] button', { hasText: /^(add|ok|create|hinzufügen)$/i }).first();
  if (await countSafe(confirm)) await confirm.click({ timeout: 4000 }).catch(() => {});
  const r = await waitUntil(board, (v) => v.nodes > before, MUTATION_MS);
  verdict("§23-add-dialog", "add-part-dialog-adds", r.ok, { before, after: r.value.nodes, waitedMs: r.waitedMs });
  await page.keyboard.press("Escape").catch(() => {});
  await setActions(false, boardWindow);
});

// ── §22 delete / duplicate / focus ──────────────────────────────────────────────────────────────────
add("delete-duplicate-focus", "§22-keys", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const picked = await pickBoardNode();
  const before = await boardCensus();
  await page.keyboard.press("Delete").catch(() => {});
  const deleted = await waitUntil(boardCensus, (census) => census < before, MUTATION_MS);
  verdict("§22-keys", "delete-selection", picked.ok && deleted.ok, { hook: "deleteSelection", before, after: deleted.value, waitedMs: deleted.waitedMs });
  const picked2 = await pickBoardNode();
  const beforeDuplicate = await boardCensus();
  await page.keyboard.press("Control+d").catch(() => {});
  const duplicated = await waitUntil(boardCensus, (census) => census > beforeDuplicate, MUTATION_MS);
  verdict("§22-keys", "duplicate-selection", picked2.ok && duplicated.ok, { hook: "duplicateSelection", before: beforeDuplicate, after: duplicated.value, waitedMs: duplicated.waitedMs });
  const cameraBefore = (await world()).camera;
  await orbitWorld(260, 120);
  await pickWorldInstance();
  await page.keyboard.press("f").catch(() => {});
  const focused = await waitUntil(world, (v) => v.camera !== cameraBefore && v.camera !== "", MUTATION_MS);
  verdict("§22-keys", "focus-selection-moves-camera", focused.ok, { hook: "focusSelection|zoomToSelection", waitedMs: focused.waitedMs });
});

add("clipboard", "§21-clipboard", "mutate", async () => {
  await ensureDocument(2);
  await closePanels();
  const picked = await pickBoardNode();
  const before = await boardCensus();
  await page.keyboard.press("Control+c").catch(() => {});
  await settle(1);
  await page.keyboard.press("Control+v").catch(() => {});
  const pasted = await waitUntil(boardCensus, (census) => census > before, MUTATION_MS);
  verdict("§21-clipboard", "copy-paste-adds", picked.ok && pasted.ok, { hook: "Puzzle5dClipboardJob", before, after: pasted.value, waitedMs: pasted.waitedMs });
  const picked2 = await pickBoardNode();
  const beforeCut = await boardCensus();
  await page.keyboard.press("Control+x").catch(() => {});
  const cut = await waitUntil(boardCensus, (census) => census < beforeCut, MUTATION_MS);
  verdict("§21-clipboard", "cut-removes", picked2.ok && cut.ok, { hook: "Puzzle5dClipboardJob", before: beforeCut, after: cut.value, waitedMs: cut.waitedMs });
  await page.keyboard.press("Control+v").catch(() => {});
  const back = await waitUntil(boardCensus, (census) => census >= beforeCut, MUTATION_MS);
  verdict("§21-clipboard", "paste-restores-cut", back.ok, { before: beforeCut, after: back.value, waitedMs: back.waitedMs });
});

// ── §20 history ─────────────────────────────────────────────────────────────────────────────────────
add("history", "§20-history", "replace", async () => {
  await ensureDocument(1);
  await closePanels();
  const opened = await clickTab("framework.panel.history");
  const text = await panelText("framework.panel.history");
  verdict("§20-history", "history-panel-lists-ledger", opened && text.trim().length > 0, { hook: "framework.panel.history", head: text.slice(0, 200) });
  await closePanels();
  const before = await boardCensus();
  const undo = await runAction(boardWindow, "undo");
  const undone = await waitUntil(boardCensus, (census) => census !== before, MUTATION_MS);
  verdict("§20-history", "undo-changes-document", undo.present && undone.ok, { hook: undo.hook, before, after: undone.value, waitedMs: undone.waitedMs });
  const redo = await runAction(boardWindow, "redo");
  const redone = await waitUntil(boardCensus, (census) => census === before, MUTATION_MS);
  verdict("§20-history", "redo-restores-document", redo.present && redone.ok, { hook: redo.hook, before, after: redone.value, waitedMs: redone.waitedMs });
  const checkpoint = await runAction(boardWindow, "checkpoint");
  verdict("§20-history", "checkpoint-available", checkpoint.present, { hook: checkpoint.hook });
  await setActions(false, boardWindow);
  const parsed = (await board()).parsed;
  verdict("§20-history", "board-still-parses-after-history", parsed === "true", { hook: "data-board-fixture-parsed", parsed });
});

// ── §24 export / import ─────────────────────────────────────────────────────────────────────────────
let exportedFile: string | null = null;
add("export", "§24-export", "read", async () => {
  await ensureDocument(1);
  await closePanels();
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 30000 }).catch(() => null),
    runAction(boardWindow, "exportFixture"),
  ]);
  const name = download ? download.suggestedFilename() : null;
  let exported: { parts: number; fasteners: number; file: string } | null = null;
  if (download) {
    const file = join(OUT, `probe5d-${stamp}-export-${name ?? "download.json"}`);
    await download.saveAs(file).catch(() => {});
    try {
      const doc = JSON.parse(readFileSync(file, "utf8")) as { parts?: unknown[]; fasteners?: unknown[]; nodes?: unknown[]; edges?: unknown[] };
      exported = { parts: (doc.parts ?? doc.nodes ?? []).length, fasteners: (doc.fasteners ?? doc.edges ?? []).length, file };
    } catch (error) {
      log(`  export file unreadable: ${String(error).slice(0, 200)}`);
    }
  }
  const vitals = await board();
  verdict("§24-export", "export-downloads-json", Boolean(download) && /\.json$/.test(name ?? "") && exported !== null, { hook: "action.exportFixture", name, exported, nodes: vitals.nodes });
  verdict("§24-export", "export-names-the-example", /concrete|nakagin|capsule/i.test(name ?? ""), { hook: "export filename", name });
  exportedFile = exported?.file ?? null;
  await setActions(false, boardWindow);
});

add("import", "§24-import", "replace", async () => {
  if (!exportedFile) {
    verdict("§24-import", "import-round-trip", false, { hook: "action.openImportFixture", reason: "no export file from this run" });
    return;
  }
  const exported = JSON.parse(readFileSync(exportedFile, "utf8")) as { parts?: unknown[]; fasteners?: unknown[]; nodes?: unknown[]; edges?: unknown[] };
  const parts = (exported.parts ?? exported.nodes ?? []).length;
  const fasteners = (exported.fasteners ?? exported.edges ?? []).length;
  await selectExample(/nakagin/i);
  await waitUntil(board, (v) => v.nodes !== parts, 120000);
  const before = (await board()).nodes;
  await closePanels();
  const [chooser] = await Promise.all([
    page.waitForEvent("filechooser", { timeout: 40000 }).catch(() => null),
    runAction(boardWindow, "openImportFixture"),
  ]);
  const mark = consoleCursor();
  if (chooser) await chooser.setFiles(exportedFile).catch((error) => log(`  setFiles failed: ${String(error).slice(0, 200)}`));
  const r = await waitUntil(board, (v) => v.nodes === parts && v.edges === fasteners, 180000);
  const trail = consoleSince(mark).filter((l) => /import|fault|error|refused|notice/i.test(l) && !BENIGN_CONSOLE_RE.test(l)).slice(0, 12);
  log(`  import trail: ${trail.join(" || ").slice(0, 1500)}`);
  verdict("§24-import", "import-round-trip", Boolean(chooser) && r.ok && r.value.parsed === "true", { hook: "action.openImportFixture", chooser: Boolean(chooser), before, after: r.value.nodes, edges: r.value.edges, expected: { parts, fasteners }, parsed: r.value.parsed, waitedMs: r.waitedMs });
  await setActions(false, boardWindow);
});

// ── §14 engagement grammar ──────────────────────────────────────────────────────────────────────────
/** 🗣️ The engagement line is the app's own grammar, typed WHOLE (a per-character echo race swallows a
 * label typed key by key). Each verb is measured on the census or pose it is supposed to move. */
add("engagement-grammar", "§14-engagement", "mutate", async () => {
  await ensureDocument(1);
  await closePanels();
  const placeholder = await evalSafe((id) => (document.getElementById(id) as HTMLInputElement | null)?.placeholder ?? "", "", `puzzle5d-engagement-${BOARD_KIND}`);
  verdict("§14-engagement", "engagement-advertises-verbs", placeholder.length > 0, { hook: `puzzle5d-engagement-${BOARD_KIND}`, placeholder: placeholder.slice(0, 200) });
  const beforeFill = (await board()).nodes;
  const filled = await engage(boardWindow, BOARD_KIND, "fill 12");
  const grew = await waitUntil(board, (v) => v.nodes > beforeFill, 120000);
  verdict("§14-engagement", "engagement-fill-12", filled.present && grew.ok, { hook: filled.hook, before: beforeFill, after: grew.value.nodes, waitedMs: grew.waitedMs });
  await page.keyboard.press("Escape").catch(() => {});
  const aborted = await waitUntil(() => panelText("framework.panel.toolRun"), (text) => !/running/i.test(text), MUTATION_MS, 1000);
  verdict("§14-engagement", "escape-aborts-engagement-run", aborted.ok, { hook: "engagementAbort", status: aborted.value.slice(0, 140) });
  const picked = await pickBoardNode();
  const poseBefore = jsonOf((await board()).positions, {} as Record<string, [number, number]>)[picked.id ?? ""];
  const moved = await engage(boardWindow, BOARD_KIND, "move 50 25");
  const movedOk = await waitUntil(
    async () => jsonOf((await board()).positions, {} as Record<string, [number, number]>),
    (p) => Boolean(picked.id && p[picked.id] && poseBefore) && Math.abs(p[picked.id!][0] - poseBefore[0] - 50) < 0.5 && Math.abs(p[picked.id!][1] - poseBefore[1] - 25) < 0.5,
    MUTATION_MS,
  );
  verdict("§14-engagement", "engagement-move", moved.present && movedOk.ok, { hook: moved.hook, id: picked.id, before: poseBefore, after: picked.id ? movedOk.value[picked.id] : null, waitedMs: movedOk.waitedMs });
  const worldBefore = (await world()).instancesRaw;
  const rotated = await engage(worldWindow, WORLD_KIND, "rotate 15");
  const rotatedOk = await waitUntil(world, (v) => v.instancesRaw !== worldBefore && v.instancesRaw.length > 0, MUTATION_MS);
  verdict("§14-engagement", "engagement-rotate", rotated.present && rotatedOk.ok, { hook: rotated.hook, waitedMs: rotatedOk.waitedMs });
  const scaleBefore = (await world()).instancesRaw;
  const scaled = await engage(worldWindow, WORLD_KIND, "scale 2");
  const scaledOk = await waitUntil(world, (v) => v.instancesRaw !== scaleBefore && v.instancesRaw.length > 0, MUTATION_MS);
  verdict("§14-engagement", "engagement-scale", scaled.present && scaledOk.ok, { hook: scaled.hook, waitedMs: scaledOk.waitedMs });
  const repeatBefore = (await world()).instancesRaw;
  await setActions(true, worldWindow);
  const repeat = page.locator(`[data-slot="window"][id="${worldWindow}"] [id*="repeatLast"], [id*="engagementRepeatLast"]`).first();
  const repeatPresent = await countSafe(repeat);
  if (repeatPresent) await repeat.click({ timeout: 4000 }).catch(() => {});
  else await page.keyboard.press("Enter").catch(() => {});
  const repeated = await waitUntil(world, (v) => v.instancesRaw !== repeatBefore && v.instancesRaw.length > 0, MUTATION_MS);
  verdict("§14-engagement", "engagement-repeat-last", repeated.ok, { hook: "engagementRepeatLast", control: repeatPresent > 0, waitedMs: repeated.waitedMs });
  await setActions(false, worldWindow);
  await setActions(false, boardWindow);
});

// ── §4 window options, per pane ─────────────────────────────────────────────────────────────────────
/** ☑️ Each pane owns its own options. The board pane has grid + LOD + selectable kinds; the world pane adds
 * sun, projection and grip show/direction. Every option is a `WindowConfig` publish, so the proof is that
 * the pane's own published config changes and the OTHER pane's does not. */
add("window-options-board", "§4-options", "read", async () => {
  await closePanels();
  await setActions(true, boardWindow);
  const controls = await evalSafe(
    (scope) => Array.from(document.querySelectorAll(`${scope} [id*="puzzle5d-play-board"], ${scope} [id*="grid"], ${scope} [id*="lod"], ${scope} [id*="select"]`)).filter((e) => (e as HTMLElement).offsetParent !== null).map((e) => `${e.id}=${e.getAttribute("aria-pressed") ?? (e as HTMLInputElement).value ?? ""}`),
    [] as string[],
    `[data-slot="window"][id="${boardWindow}"]`,
  );
  verdict("§4-options", "board-options-present", controls.some((c) => /grid/i.test(c)) && controls.some((c) => /lod/i.test(c)) && controls.some((c) => /select/i.test(c)), { hook: "puzzle5d-play-board-grid|-lod|-select", controls: controls.slice(0, 20) });
  const before = (await board()).camera;
  const grid = page.locator(`[data-slot="window"][id="${boardWindow}"] [id*="puzzle5d-play-board-grid"]`).first();
  const gridPresent = await countSafe(grid);
  if (gridPresent) await grid.click({ timeout: 4000 }).catch(() => {});
  await settle(2);
  verdict("§4-options", "board-grid-toggle-accepts", gridPresent > 0, { hook: "puzzle5d-play-board-grid (setGridSnapEnabled)", cameraBefore: before });
  await setActions(false, boardWindow);
});

add("window-options-world", "§4-options", "read", async () => {
  await closePanels();
  await setActions(true, worldWindow);
  const controls = await evalSafe(
    (scope) => Array.from(document.querySelectorAll(`${scope} [id*="puzzle5d-play-world"], ${scope} [id*="sun"], ${scope} [id*="projection"], ${scope} [id*="grip"]`)).filter((e) => (e as HTMLElement).offsetParent !== null).map((e) => `${e.id}=${e.getAttribute("aria-pressed") ?? (e as HTMLInputElement).value ?? ""}`),
    [] as string[],
    `[data-slot="window"][id="${worldWindow}"]`,
  );
  verdict("§4-options", "world-options-present", controls.some((c) => /grid/i.test(c)) && controls.some((c) => /lod/i.test(c)) && controls.some((c) => /grip-show/i.test(c)) && controls.some((c) => /grip-direction/i.test(c)), { hook: "puzzle5d-play-world-{grid,lod,grip-show,grip-direction,select}", controls: controls.slice(0, 24) });
  const sunBefore = (await world()).sun;
  const sun = page.locator(`[data-slot="window"][id="${worldWindow}"] [id*="sun"]`).first();
  const sunPresent = await countSafe(sun);
  if (sunPresent) await sun.click({ timeout: 4000 }).catch(() => {});
  const sunChanged = await waitUntil(world, (v) => v.sun !== sunBefore, MUTATION_MS);
  verdict("§4-options", "world-sun-toggle-publishes", sunPresent > 0 && sunChanged.ok, { hook: "toggleSun → data-sun-json", before: sunBefore.slice(0, 80), after: sunChanged.value.sun.slice(0, 80), waitedMs: sunChanged.waitedMs });
  const cameraBefore = (await world()).camera;
  const projection = page.locator(`[data-slot="window"][id="${worldWindow}"] [id*="projection"]`).first();
  const projectionPresent = await countSafe(projection);
  if (projectionPresent) await projection.click({ timeout: 4000 }).catch(() => {});
  const projectionChanged = await waitUntil(world, (v) => v.camera !== cameraBefore, MUTATION_MS);
  verdict("§4-options", "world-projection-switches", projectionPresent > 0 && projectionChanged.ok, { hook: "projection option → data-camera-json", waitedMs: projectionChanged.waitedMs });
  const gripsBefore = (await world()).vortices;
  const grip = page.locator(`[data-slot="window"][id="${worldWindow}"] [id*="puzzle5d-play-world-grip-show"]`).first();
  const gripPresent = await countSafe(grip);
  if (gripPresent) await grip.click({ timeout: 4000 }).catch(() => {});
  const gripsChanged = await waitUntil(world, (v) => v.vortices !== gripsBefore, MUTATION_MS);
  verdict("§4-options", "world-grip-show-toggles", gripPresent > 0 && gripsChanged.ok, { hook: "puzzle5d-play-world-grip-show → data-vortices-json", waitedMs: gripsChanged.waitedMs });
  await setActions(false, worldWindow);
});

// ── §25 locale ──────────────────────────────────────────────────────────────────────────────────────
add("locale", "§25-locale", "read", async () => {
  await ensureDocument(1);
  const labels = async () => {
    await clickTab("framework.panel.artifact");
    await settle(1.5);
    const text = await panelText("framework.panel.artifact");
    await closePanels();
    return text;
  };
  const english = await labels();
  const setLanguage = async (wanted: RegExp) => {
    await clickTab("framework.settings");
    await settle(1);
    await page.locator('[data-slot="panel-tab-button"][id="framework.settings.general"]').first().click({ timeout: 4000 }).catch(() => {});
    await settle(1);
    const trigger = page.locator('[id="framework.settings.language"]').first();
    if (!(await countSafe(trigger))) return false;
    await trigger.click({ force: true, timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(800);
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    const optionCount = await countSafe(option);
    if (optionCount) await option.click({ timeout: 4000 }).catch(() => {});
    else await page.keyboard.press("Escape").catch(() => {});
    await settle(3);
    await closePanels();
    return optionCount > 0;
  };
  const toGerman = await setLanguage(/deutsch|german/i);
  const german = toGerman ? await labels() : "";
  verdict("§25-locale", "german-flips-labels", toGerman && german.length > 0 && german !== english, { hook: "framework.settings.language", en: english.slice(0, 140), de: german.slice(0, 140) });
  const back = toGerman ? await setLanguage(/english|englisch/i) : false;
  const restored = back ? await labels() : "";
  verdict("§25-locale", "english-restored", back && restored === english, { restored: restored.slice(0, 140) });
});

// ── §0 guest ────────────────────────────────────────────────────────────────────────────────────────
add("guest-alive", "§0-vitals", "read", async () => {
  const s = await snapshot();
  const b = await board();
  const w = await world();
  verdict("§0-vitals", "guest-alive", !s.recovery.some((x) => x && x !== "?") && guestDeathFaults.length === 0 && b.present && w.present, {
    recovery: s.recovery,
    guestDeath: guestDeathFaults.slice(0, 3),
    board: { nodes: b.nodes, parsed: b.parsed },
    world: { instances: w.instanceCount },
  });
});
//#endregion 🔖️Steps

//#region 🔖️Main
log(`navigating to :${port}/?plugin=${plugin}`);
await gotoShell("boot");
const booted = await waitForBoot("boot");
await page.screenshot({ path: join(OUT, `probe5d-${stamp}-boot.png`) }).catch(() => {});
emit({ section: "§0-boot", step: "booted", verdict: booted ? "PASS" : "FAIL", booted, board: boardSurface, world: worldSurface, faults: faults.length });
if (booted) pass += 1;
else fail += 1;

if (explore || !booted) {
  const inv = await inventory();
  const rows = await readSurfaces();
  log(`explore windows: ${JSON.stringify(inv.windows)}`);
  log(`explore panel tabs: ${JSON.stringify(inv.tabs)}`);
  log(`explore toggles (tool + utility rails): ${JSON.stringify(inv.toggles)}`);
  log(`explore surfaces: ${JSON.stringify(rows.map((row) => ({ surface: row.surface, window: row.window, kind: row.kind, attrs: Object.keys(row.attrs) })), null, 1)}`);
  log(`explore vitals board=${JSON.stringify(await board()).slice(0, 1200)}`);
  log(`explore vitals world=${JSON.stringify(await world()).slice(0, 1600)}`);
  log(`explore inventory: ${JSON.stringify(inv, null, 1).slice(0, 20000)}`);
  await page.screenshot({ path: join(OUT, `probe5d-${stamp}-explore.png`) }).catch(() => {});
}

const GROUP_ORDER = ["read", "mutate", "replace"] as const;
if (booted && !explore) {
  const plan = only ? steps.filter((s) => only.has(s.name)) : battery ? steps : steps.filter((s) => s.name === "panes" || s.name === "guest-alive");
  log(`plan: ${JSON.stringify(GROUP_ORDER.map((group) => ({ group, steps: plan.filter((s) => s.group === group).map((s) => s.name) })))}`);
  let ranGroup = false;
  for (const group of GROUP_ORDER) {
    // 🧱️ Examples load first inside each group so the group's other steps have a document to read.
    const entries = [...plan.filter((s) => s.group === group && s.name.startsWith("example-")), ...plan.filter((s) => s.group === group && !s.name.startsWith("example-"))];
    if (!entries.length) continue;
    if (ranGroup && reloadBetweenGroups) {
      log(`reloading page before group ${group}`);
      await gotoShell(`reboot:${group}`);
      const rebooted = await waitForBoot(`reboot:${group}`, Math.min(bootPolls, 45));
      currentGroup = group;
      verdict("§0-boot", `reboot-${group}`, rebooted, { group, board: boardSurface, world: worldSurface });
    }
    ranGroup = true;
    for (const step of entries) {
      currentGroup = group;
      const mark = consoleCursor();
      const faultsBefore = hardFaults.length;
      log(`step ${step.name} (${group}, ${step.section})`);
      try {
        await step.run();
      } catch (error) {
        verdict(step.section, step.name, false, { error: String(error).slice(0, 400) });
      }
      await settle(1);
      const fresh = hardFaults.slice(faultsBefore);
      if (fresh.length) log(`  hard faults during ${step.name}: ${fresh.slice(0, 3).join(" || ").slice(0, 700)}`);
      const tail = consoleSince(mark).filter((l) => /error|fault|panic|warn/i.test(l) && !BENIGN_CONSOLE_RE.test(l)).slice(-4);
      if (tail.length) log(`  console tail: ${tail.join(" || ").slice(0, 700)}`);
      await page.screenshot({ path: join(OUT, `probe5d-${stamp}-${step.name}.png`) }).catch(() => {});
    }
    // 🫀️ One guest actor drives BOTH panes, so a corpse in either is a death for the whole group.
    currentGroup = "boot";
    const s = await snapshot();
    const b = await board();
    const w = await world();
    verdict("§0-vitals", `guest-alive-${group}`, s.recovery.length === 0 && guestDeathFaults.length === 0 && s.canvases >= 2 && b.present && w.present, {
      recovery: s.recovery,
      canvases: s.canvases,
      board: { nodes: b.nodes, parsed: b.parsed },
      world: { instances: w.instanceCount },
      guestDeathFaults: guestDeathFaults.length,
      firstHardFaultAt,
    });
  }
  currentGroup = "boot";
  verdict("§0-vitals", "battery-hard-faults", hardFaults.length === 0, { hard: hardFaults.length, first: (hardFaults[0] ?? "none").slice(0, 200) });
}

const summary = `battery PASS=${pass} FAIL=${fail} FAULTS=${faults.length} HARD=${hardFaults.length} first-hard-fault-at=${firstHardFaultAt ?? "none"} guest-death-faults=${guestDeathFaults.length}`;
log(summary);
emit({ summary, pass, fail, faults: faults.length, hard: hardFaults.length, firstHardFaultAt, guestDeath: guestDeathFaults.length, board: boardSurface, world: worldSurface });
writeFileSync(
  join(OUT, `probe5d-${stamp}.md`),
  `# probe5d ${stamp} (battery=${battery} only=${onlyArg ?? "-"} reloadBetweenGroups=${reloadBetweenGroups} port=${port})\n\n` +
    `board=${boardSurface ?? "-"}@${boardWindow || "-"} world=${worldSurface ?? "-"}@${worldWindow || "-"}\n\n` +
    `## verdicts\n${verdicts.join("\n") || "(none)"}\n\n## timeline\n${lines.join("\n")}\n\n` +
    `## faults (raw ${faults.length}, hard ${hardFaults.length}, guest-death ${guestDeathFaults.length}, first-hard-fault-at ${firstHardFaultAt ?? "none"})\n` +
    `### guest death\n${guestDeathFaults.slice(0, 20).join("\n") || "(none)"}\n\n### hard\n${hardFaults.slice(0, 60).join("\n") || "(none)"}\n\n### raw\n${faults.slice(0, 80).join("\n") || "(none)"}\n\n` +
    `## console (last ${CONSOLE_TAIL_LINES})\n\`\`\`\n${consoleBuf.slice(-CONSOLE_TAIL_LINES).join("\n")}\n\`\`\`\n`,
);
log(`done booted=${booted} → 🗑️generated/probe5d-${stamp}.md + probe5d-${stamp}.ndjson`);
await browser.close();
process.exit(0);
//#endregion 🔖️Main
