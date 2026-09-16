/** 🔬️ Headless runtime probe for the puzzle 2d React serve on 127.0.0.1:6012 — boots the shell, waits for the
 * three board windows, drives one browser step per row of `📓️2026-09-16-parity-survey.md`, and writes findings +
 * screenshots + a machine-readable verdict stream into `🗑️generated/`. Ticket 26/09/06/PUZZLE-2D-END-TO-END.
 *
 * Run: `bun 🔍️browser-probe.ts [--battery] [--only=step,step] [--port=<n>] [--explore] [--settle=<seconds>]`.
 * `--explore` boots, dumps the DOM inventory (windows, tabs, toggles, buttons, tree items) and exits — the
 * first thing to run against a serve whose chrome ids are not yet known. `--battery` runs every registered
 * step in plan order; `--only=a,b` runs exactly those.
 *
 * Outputs: `probe-<stamp>.md` (prose timeline) and `probe-<stamp>.ndjson` (one JSON record per verdict plus a
 * final `battery PASS=n FAIL=n FAULTS=n first-hard-fault-at=<s>` summary record). */
import { chromium, type Locator, type Page } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const battery = process.argv.includes("--battery");
const explore = process.argv.includes("--explore");
const onlyArg = process.argv.find((a) => a.startsWith("--only="))?.slice(7);
const only = onlyArg ? new Set(onlyArg.split(",").map((name) => name.trim()).filter(Boolean)) : null;
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
const consoleBuf: string[] = [];
let consoleSeq = 0;
const consoleCursor = () => consoleSeq;
const consoleSince = (mark: number) => consoleBuf.slice(Math.max(0, mark - (consoleSeq - consoleBuf.length)));
const FAULT_RE =
  /intake-budget-exhausted|fixed-capacity|section-root-mismatch|native-owner-required|terminal-fault|unreachable|shard .* (lost|terminated)|did not publish|missing field|malformed|admission failed|worker fault|\[semio-plugin panic\]|panicked at|Credits \{|NodeCapacity|SemioFaultError|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected|unknown Puzzle 2D action|puzzle2d-/i;
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
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (message) => {
  const text = `${message.type()}: ${message.text()}`;
  consoleBuf.push(text);
  consoleSeq += 1;
  if (consoleBuf.length > CONSOLE_RING_LINES) consoleBuf.shift();
  if (FAULT_RE.test(text)) noteFault(text);
});
page.on("pageerror", (error) => {
  const text = `pageerror: ${String(error)}`;
  consoleBuf.push(text);
  consoleSeq += 1;
  noteFault(text);
});

const evalSafe = async <T,>(fn: () => T, fallback: T): Promise<T> => {
  try {
    return await page.evaluate(fn);
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
      treeItems: q('[data-slot="tree-item"], [role="treeitem"]').length,
      dialogs: q('[role="dialog"]').map((d) => (d as HTMLElement).innerText.slice(0, 80)),
      recovery: q("[data-plugin-recovery]").map((el) => el.getAttribute("data-plugin-recovery") ?? "?"),
      body: document.body ? document.body.innerText.slice(0, 400) : "",
    };
  }, EMPTY);

/** 🧾️ The board vitals `Board2dHost` publishes per surface (`data-board-*`), read without a guest round trip. */
const boardVitals = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll("[data-surface-id]")).map((el) => ({
        surface: el.getAttribute("data-surface-id") ?? "?",
        nodes: Number(el.getAttribute("data-board-nodes") ?? "-1"),
        edges: Number(el.getAttribute("data-board-edges") ?? "-1"),
        selection: el.getAttribute("data-board-selection-json") ?? "",
        camera: el.getAttribute("data-board-camera-json") ?? "",
        hovered: el.getAttribute("data-board-hovered-id") ?? "",
        utility: el.getAttribute("data-board-active-utility") ?? "",
      })),
    [] as { surface: string; nodes: number; edges: number; selection: string; camera: string; hovered: string; utility: string }[],
  );

const inventory = () =>
  evalSafe(() => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    const text = (el: Element) => (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 40) ?? "";
    return {
      buttons: q("button").map((b) => `${b.id || "-"}:${text(b)}`).filter((s) => s !== "-:").slice(0, 200),
      selects: q("select").map((s) => `${s.id}:${Array.from((s as HTMLSelectElement).options).map((o) => o.label).join("|")}`),
      inputs: q("input").map((i) => `${i.id || "-"}:${(i as HTMLInputElement).type}=${(i as HTMLInputElement).value}`).slice(0, 80),
      tree: q('[data-slot="tree-item"], [role="treeitem"]').map((r) => `${r.id || "?"}=${text(r)}`).slice(0, 80),
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

const overviewCanvas = () => page.locator('[data-surface-id$=".2d-overview"] canvas').first();
const overviewBox = async () => (await overviewCanvas().boundingBox()) ?? { x: 0, y: 0, width: 1, height: 1 };
const overviewVitals = async () => (await boardVitals()).find((v) => v.surface.endsWith(".2d-overview"));

/** 🎯️ The screen position of a node id in the overview pane, read through the pane's published camera. */
const nodeScreen = async (id: string) => {
  const vitals = await overviewVitals();
  const positions = JSON.parse(vitals?.camera ? (await evalSafe(() => document.querySelector('[data-surface-id$=".2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) : "{}") as Record<string, [number, number]>;
  const position = positions[id];
  if (!position || !vitals) return null;
  const camera = JSON.parse(vitals.camera) as { x: number; y: number; zoom: number };
  const box = await overviewBox();
  return { x: box.x + box.width / 2 + (position[0] - camera.x) * camera.zoom, y: box.y + box.height / 2 + (position[1] - camera.y) * camera.zoom };
};

const clickTab = async (id: string) => {
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  if (!(await countSafe(tab))) return false;
  await tab.click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(800);
  return true;
};

const selectExample = async (wanted: RegExp) => {
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
//#endregion 🔖️Verdicts

//#region 🔖️Steps
type Step = { name: string; group: "read" | "mutate" | "replace"; run: () => Promise<void> };
const steps: Step[] = [];
const register = (name: string, group: Step["group"], run: () => Promise<void>) => steps.push({ name, group, run });

register("windows", "read", async () => {
  const s = await snapshot();
  verdict("1-windows", "three-boards", s.windows.length >= 3 && s.canvases >= 3, { windows: s.windows, canvases: s.canvases });
  const vitals = await boardVitals();
  verdict("1-windows", "vitals-published", vitals.length >= 3 && vitals.every((v) => v.nodes >= 0), { vitals });
});

register("example-concrete-forest", "replace", async () => {
  const label = await selectExample(/concrete/i);
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > 0, 30000);
  verdict("5-examples", "concrete-forest-loads", Boolean(label) && r.ok, { label, nodes: r.value?.nodes, edges: r.value?.edges, waitedMs: r.waitedMs });
});

register("example-nakagin", "replace", async () => {
  const label = await selectExample(/nakagin/i);
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) >= 100, 90000);
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
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id$=".2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = Object.keys(ids)[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("6-selection", "click-select", false, { reason: "no node position published", ids: Object.keys(ids).length });
    return;
  }
  await page.mouse.click(at.x, at.y);
  const r = await waitUntil(overviewVitals, (v) => (v?.selection ?? "").includes(first), 15000);
  verdict("6-selection", "click-select", r.ok, { id: first, at, selection: r.value?.selection, waitedMs: r.waitedMs });
  const inspection = await (async () => {
    await clickTab("framework.panel.inspection");
    return snapshot();
  })();
  verdict("16-inspection", "inspector-shows-selected-node", inspection.body.includes(first) || (await evalSafe(() => document.body.innerText, "")).includes(first), { id: first });
});

register("marquee", "read", async () => {
  const box = await overviewBox();
  await page.mouse.move(box.x + 20, box.y + 20);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width - 20, box.y + box.height - 20, { steps: 12 });
  await page.mouse.up();
  const r = await waitUntil(overviewVitals, (v) => (JSON.parse(v?.selection || "[]") as unknown[]).length >= 2, 15000);
  verdict("6-selection", "marquee-selects-many", r.ok, { selected: (JSON.parse(r.value?.selection || "[]") as unknown[]).length, waitedMs: r.waitedMs });
  await page.keyboard.press("Escape");
  await page.mouse.click(box.x + 5, box.y + 5);
  await settle(1);
});

register("drag-node", "mutate", async () => {
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id$=".2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = Object.keys(ids)[0];
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
    async () => JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id$=".2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>,
    (p) => Boolean(p[first]) && (Math.abs(p[first][0] - before[0]) > 1 || Math.abs(p[first][1] - before[1]) > 1),
    20000,
  );
  verdict("8-transform", "drag-node-moves-document", r.ok, { id: first, before, after: r.value[first], waitedMs: r.waitedMs });
});

register("utilities", "read", async () => {
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
  const toggle = page.locator('[data-slot="toggle-group-item"][id="tool.fill"], [id="tool.fill"]').first();
  const present = await countSafe(toggle);
  verdict("12-fill", "fill-tab-present", present > 0);
  if (!present) return;
  await toggle.click({ timeout: 3000 }).catch(() => {});
  await settle(2);
  const count = page.locator('[id*="puzzle2d-fill-count"] input, input[id*="fill-count"]').first();
  verdict("12-fill", "fill-count-measure", (await countSafe(count)) > 0);
  const before = (await overviewVitals())?.nodes ?? -1;
  const start = page.locator("button", { hasText: /^(start|run|apply|finalize)$/i }).first();
  const startPresent = await countSafe(start);
  if (startPresent) await start.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) > before, 60000);
  verdict("12-fill", "fill-run-places-nodes", r.ok, { before, after: r.value?.nodes, startPresent, waitedMs: r.waitedMs });
});

register("delete", "mutate", async () => {
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id$=".2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = Object.keys(ids)[0];
  const at = first ? await nodeScreen(first) : null;
  if (!at) {
    verdict("22-delete", "delete-selection", false, { reason: "no node position" });
    return;
  }
  const before = (await overviewVitals())?.nodes ?? -1;
  await page.mouse.click(at.x, at.y);
  await settle(1);
  await page.keyboard.press("Delete");
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === before - 1, 20000);
  verdict("22-delete", "delete-selection", r.ok, { id: first, before, after: r.value?.nodes, waitedMs: r.waitedMs });
});

register("undo", "replace", async () => {
  const before = (await overviewVitals())?.nodes ?? -1;
  const opened = await clickTab("framework.panel.history");
  const undo = page.locator('[id="framework.history.undo"], button', { hasText: /^undo$/i }).first();
  const present = await countSafe(undo);
  if (present) await undo.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) !== before, 20000);
  verdict("20-history", "undo-changes-document", opened && present > 0 && r.ok, { before, after: r.value?.nodes, waitedMs: r.waitedMs });
});

register("context-menu", "read", async () => {
  const ids = JSON.parse(await evalSafe(() => document.querySelector('[data-surface-id$=".2d-overview"]')?.getAttribute("data-board-positions-json") ?? "{}", "{}")) as Record<string, [number, number]>;
  const first = Object.keys(ids)[0];
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
  await clickTab("framework.panel.catalogue");
  const before = (await overviewVitals())?.nodes ?? -1;
  const row = page.locator('[data-slot="tree-item"][id^="puzzle2d-play-kinds.nodes."], [role="treeitem"][id^="puzzle2d-play-kinds.nodes."]').first();
  const present = await countSafe(row);
  if (present) await row.click({ timeout: 3000 }).catch(() => {});
  const r = await waitUntil(overviewVitals, (v) => (v?.nodes ?? -1) === before + 1, 20000);
  verdict("18-catalogue", "click-adds-node", present > 0 && r.ok, { before, after: r.value?.nodes, waitedMs: r.waitedMs });
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

if (booted && !explore) {
  const plan = only ? steps.filter((s) => only.has(s.name)) : battery ? [...steps].sort((a, b) => ["read", "mutate", "replace"].indexOf(a.group) - ["read", "mutate", "replace"].indexOf(b.group)) : steps.filter((s) => s.name === "windows" || s.name === "guest-alive");
  // 🧱️ Examples load first so read steps have a document to read.
  const ordered = [...plan.filter((s) => s.name.startsWith("example-")), ...plan.filter((s) => !s.name.startsWith("example-"))];
  for (const step of ordered) {
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
  }
}

const summary = `battery PASS=${pass} FAIL=${fail} FAULTS=${faults.length} HARD=${hardFaults.length} first-hard-fault-at=${firstHardFaultAt ?? "none"} guest-death-faults=${guestDeathFaults.length}`;
log(summary);
emit({ summary, pass, fail, faults: faults.length, hard: hardFaults.length, firstHardFaultAt, guestDeath: guestDeathFaults.length });
lines.push("", "## faults", ...faults.slice(0, 80), "", "## console (last 120)", ...consoleBuf.slice(-120));
writeFileSync(join(OUT, `probe-${stamp}.md`), lines.join("\n"));
await browser.close();
//#endregion 🔖️Main
