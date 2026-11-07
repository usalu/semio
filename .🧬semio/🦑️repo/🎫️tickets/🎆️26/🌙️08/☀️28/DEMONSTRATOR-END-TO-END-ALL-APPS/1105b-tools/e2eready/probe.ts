/** 🔍️ Per-pane browser verdicts for the mit-bestand demonstrator: boot, content, one real action, console. Reads `expect.json` and the acceptance spec's case files; starts no server.
 * Usage: `bun probe.ts <base-url> [--panes a,b] [--languages en,de] [--no-landing] [--out dir]` or `bun probe.ts --check`. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { chromium, type Browser, type BrowserContext, type Locator, type Page } from "playwright";

type Stage = "PASS" | "FAIL" | "SKIP";
type Surface = "world3d" | "nodeGraph" | "table" | "tree" | "tiledMap" | "placeholder";
type Words = Readonly<Record<string, string>>;
interface Language { readonly language: string; readonly browserLocale: string; readonly introNext: string }
interface WindowSpec { readonly kind: string; readonly instances?: readonly string[]; readonly surface: Surface; readonly content: boolean; readonly placeholder?: Words }
interface ActionSpec { readonly type: string; readonly cases: string; readonly follow?: readonly string[]; readonly [key: string]: unknown }
interface PaneSpec { readonly id: string; readonly windows: readonly WindowSpec[]; readonly action: ActionSpec }
interface Expect {
  readonly schema: string;
  readonly sources: { readonly brand: string; readonly catalog: string; readonly spec: string; readonly cases: Readonly<Record<string, string>> };
  readonly languages: readonly Language[];
  readonly unsupportedBrowserLocale: string;
  readonly timeoutsMs: { readonly shellReady: number; readonly content: number; readonly action: number };
  readonly landing: { readonly titleKey: string; readonly taglineKey: string; readonly pickerChoose: string };
  readonly panes: readonly PaneSpec[];
}
interface Row { readonly unit: string; boot: Stage; content: Stage; action: Stage; console: Stage; ms: number; readonly detail: string[] }
type Stages = "boot" | "content" | "action";
type Cases = Readonly<Record<string, any>>;

const here = import.meta.dirname;
const findRoot = (from: string): string => (existsSync(join(from, "AGENTS.md")) ? from : dirname(from) === from ? from : findRoot(dirname(from)));
const root = process.env.SEMIO_ROOT ?? findRoot(here);
const readJson = <T>(relative: string): T => JSON.parse(readFileSync(join(root, relative), "utf8")) as T;
const expectation = JSON.parse(readFileSync(join(here, "expect.json"), "utf8")) as Expect;
const timeouts = expectation.timeoutsMs;

export const brandPaneIds = (): string[] => {
  const block = /export const DEMONSTRATOR_PANES[^=]*=\s*\[(.*?)\n\];/s.exec(readFileSync(join(root, expectation.sources.brand), "utf8"));
  if (!block?.[1]) throw new Error("brand file declares no DEMONSTRATOR_PANES literal");
  return [...block[1].matchAll(/\bid:\s*"([^"]+)"/g)].map((m) => m[1] ?? "");
};
const loadCases = (): Cases => Object.fromEntries(Object.entries(expectation.sources.cases).map(([name, path]) => [name, readJson<unknown>(path)]));
const catalog = (): Record<string, Words> => readJson<{ texts: Record<string, Words> }>(expectation.sources.catalog).texts;
const dig = (cases: Cases, path: string): Cases => path.split(".").reduce<any>((value, key) => value?.[key], cases);
const word = (words: Words, language: string): string => words[language] ?? "";
const flat = (text: string | null | undefined): string => (text ?? "").replace(/\s+/g, " ").trim();
const firstLine = (error: unknown): string => String(error instanceof Error ? error.message : error).split("\n")[0]?.slice(0, 240) ?? "";
const sleep = (ms: number): Promise<void> => new Promise((done) => setTimeout(done, ms));

export const elementIdSegment = (raw: string): string => {
  let out = "";
  let up = false;
  for (const ch of raw) {
    if ("-_ .".includes(ch)) { up = true; continue; }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (out === "") out += ch.toLowerCase();
    else if (up) { out += ch.toUpperCase(); up = false; } else out += ch;
  }
  return out;
};
export const windowId = (kind: string): string => `framework.window.${elementIdSegment(kind)}`;
export const paneSelector = (pane: string, id: string): string => `[data-shell-id="${pane}"] [id="${id}"], [data-shell-id="${pane}"] [data-element-alias~="${id}"]`;

export async function until<T>(read: () => Promise<T>, ok: (value: T) => boolean, ms: number, label: string): Promise<T> {
  const end = Date.now() + ms;
  let last: T | undefined;
  for (;;) {
    last = await read().catch(() => undefined);
    if (last !== undefined && ok(last)) return last;
    if (Date.now() > end) throw new Error(`${label}: timed out after ${ms} ms, last=${JSON.stringify(last)?.slice(0, 160)}`);
    await sleep(400);
  }
}

const arrayLength = (raw: string | null): number => {
  try { const value: unknown = JSON.parse(raw ?? "[]"); return Array.isArray(value) ? value.length : 0; } catch { return 0; }
};
const attribute = (loc: Locator, name: string): Promise<string | null> => loc.first().getAttribute(name, { timeout: 1500 });
const text = async (loc: Locator): Promise<string> => flat(await loc.first().textContent({ timeout: 1500 }));

export const worldCount = async (box: Locator): Promise<number> => {
  const host = box.locator(".semio-world-3d-host").first();
  if ((await host.count()) === 0) return 0;
  return arrayLength(await attribute(host, "data-meshes-json")) + arrayLength(await attribute(host, "data-instances-json"));
};
export const graphWidgetCount = async (box: Locator): Promise<number> => {
  const host = box.locator(".semio-node-graph-host").first();
  if ((await host.count()) === 0) return 0;
  try { return (JSON.parse((await attribute(host, "data-host-snapshot-json")) ?? "{}") as { widgets?: unknown[] }).widgets?.length ?? 0; } catch { return 0; }
};
export const tableRowCount = (box: Locator): Promise<number> => box.locator(".semio-table-host [data-row-id]").count();
export const treeRowCount = (box: Locator): Promise<number> => box.locator('[role="treeitem"]').count();
export const projection = async (box: Locator): Promise<string> => {
  const host = box.locator(".semio-world-3d-host").first();
  if ((await host.count()) === 0) return "";
  return JSON.stringify({
    meshes: JSON.parse((await attribute(host, "data-meshes-json")) ?? "[]"),
    instances: (JSON.parse((await attribute(host, "data-instances-json")) ?? "[]") as Record<string, unknown>[]).map(({ selected, hovered, highlighted, ...rest }) => rest),
  });
};
export const instanceIds = async (box: Locator): Promise<string[]> => {
  const host = box.locator(".semio-world-3d-host").first();
  if ((await host.count()) === 0) return [];
  const list = JSON.parse((await attribute(host, "data-instances-json")) ?? "[]") as { id: string }[];
  return [...new Set(list.map((item) => item.id))].sort();
};
export const latestToolRunGroup = (page: Page, pane: string): Promise<string> =>
  page.locator(`[data-shell-id="${pane}"] [id], [data-shell-id="${pane}"] [data-element-alias]`).evaluateAll((els) => {
    const runs = els.flatMap((el) => [el.id, ...(el.getAttribute("data-element-alias") ?? "").split(/\s+/)]).flatMap((id) => /^framework\.toolRun\.(\d+)(?:\.|$)/.exec(id)?.[1] ?? []);
    const latest = runs.reduce((hi, run) => (BigInt(run) > BigInt(hi) ? run : hi), "0");
    return runs.length ? `framework.toolRun.${latest}` : "";
  });
export const shellOutcome = (page: Page, pane: string): Promise<string> =>
  page.evaluate((id) => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
    if (!el) return "absent";
    if (el.dataset.shellReady !== undefined) return "ready";
    if (el.dataset.shellError !== undefined) return `error:${el.dataset.shellError.slice(0, 120)}`;
    if (el.dataset.shellNotFound !== undefined) return "not-found";
    return "booting";
  }, pane);

async function distinctColors(page: Page, canvas: Locator): Promise<number> {
  const shot = (await canvas.screenshot()).toString("base64");
  return page.evaluate(async (base64) => {
    const bytes = Uint8Array.from(atob(base64), (ch) => ch.charCodeAt(0));
    const bitmap = await createImageBitmap(new Blob([bytes], { type: "image/png" }));
    const probe = document.createElement("canvas");
    probe.width = bitmap.width;
    probe.height = bitmap.height;
    const ctx = probe.getContext("2d");
    if (!ctx) return 0;
    ctx.drawImage(bitmap, 0, 0);
    const { data } = ctx.getImageData(Math.floor(bitmap.width * 0.2), Math.floor(bitmap.height * 0.2), Math.max(1, Math.floor(bitmap.width * 0.6)), Math.max(1, Math.floor(bitmap.height * 0.6)));
    const seen = new Set<number>();
    for (let i = 0; i < data.length; i += 4) seen.add(((data[i] ?? 0) << 24) | ((data[i + 1] ?? 0) << 16) | ((data[i + 2] ?? 0) << 8) | (data[i + 3] ?? 0));
    return seen.size;
  }, shot);
}

const viewport = (): { width: number; height: number } => {
  const [width, height] = (process.env.E2E_VIEWPORT ?? "1600x1000").split("x").map(Number);
  return { width: width || 1600, height: height || 1000 };
};

interface Session { readonly context: BrowserContext; readonly page: Page; readonly errors: string[] }
async function openSession(browser: Browser, locale: string): Promise<Session> {
  const context = await browser.newContext({ locale, viewport: viewport() });
  const page = await context.newPage();
  page.setDefaultTimeout(20_000);
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(`pageerror ${error.message.slice(0, 240)}`));
  page.on("console", (message) => {
    const body = message.text();
    if (message.type() === "error" && !/^Failed to load resource: the server responded with a status of 404(?: \(Not Found\))?$/i.test(body)) errors.push(`console ${body.slice(0, 240)}`);
  });
  return { context, page, errors };
}

async function dismissIntroduction(page: Page, scope: string): Promise<void> {
  const skip = page.locator(scope);
  if (!(await skip.first().isVisible({ timeout: 3_000 }).catch(() => false))) return;
  const clicked = await skip.first().click({ timeout: 20_000 }).then(() => true).catch(() => false);
  if (!clicked) await page.keyboard.press("Escape");
  await until(() => skip.count(), (n) => n === 0, 15_000, "introduction overlay dismissed");
}

interface Ctx { readonly page: Page; readonly pane: string; readonly language: string; readonly action: ActionSpec; readonly cases: Cases; readonly win: (kind: string) => Locator; readonly at: (id: string) => Locator }
const mk = (page: Page, pane: string, language: string, action: ActionSpec, all: Cases): Ctx => ({
  page, pane, language, action, cases: dig(all, action.cases),
  win: (kind) => page.locator(paneSelector(pane, windowId(kind))),
  at: (id) => page.locator(paneSelector(pane, id)),
});
const param = <T>(ctx: Ctx, key: string): T => ctx.action[key] as T;

const openCompactPanel = async (ctx: Ctx): Promise<void> => {
  const toggle = ctx.page.locator(`[data-shell-id="${ctx.pane}"] [id="ui.mobilePanel.toggle"]:visible, [id="ui.mobilePanel.toggle"]:visible`).first();
  if (!(await toggle.count())) return;
  if ((await toggle.getAttribute("aria-pressed")) === "true" || (await toggle.getAttribute("data-state")) === "on") return;
  await toggle.click();
  await ctx.page.waitForTimeout(700);
};
const tabButton = async (ctx: Ctx, id: string): Promise<Locator> => {
  const button = ctx.page.locator(`[data-shell-id="${ctx.pane}"] [data-tab-id="${id}"]:visible`).first();
  if (!(await button.count())) await openCompactPanel(ctx);
  return button;
};
const docTab = async (ctx: Ctx, tab: string): Promise<void> => {
  const button = await tabButton(ctx, `framework.panel.${tab}`);
  if ((await button.getAttribute("aria-pressed")) !== "true") await button.click();
};
const paneTab = async (ctx: Ctx, category: string, tab: string): Promise<void> => {
  const shell = ctx.page.locator(`[data-shell-id="${ctx.pane}"]`);
  let button = shell.locator(`[data-tab-id="${tab}"]:visible`).first();
  if (!(await button.count())) {
    await (await tabButton(ctx, category)).click();
    button = shell.locator(`[data-tab-id="${tab}"]:visible`).first();
  }
  if ((await button.getAttribute("aria-pressed")) !== "true") await button.click();
};
const history = async (ctx: Ctx, box: Locator, direction: "undo" | "redo"): Promise<void> => {
  await box.locator("canvas").first().click({ position: { x: 8, y: 8 } });
  await ctx.page.keyboard.press(direction === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
};
const settled = async (box: Locator): Promise<void> => {
  await until(() => worldCount(box), (n) => n > 0, timeouts.content, "world scene content");
  await until(() => box.locator('[data-slot="world-compute-status"][aria-busy="true"]').count(), (n) => n === 0, timeouts.action, "compute idle");
  const phase = await box.locator('[data-slot="world-compute-status"]').first().getAttribute("data-compute-phase", { timeout: 1500 }).catch(() => null);
  if (phase && /cancel|fault|fail|error/i.test(phase)) throw new Error(`compute phase ${phase}`);
};
const tokens = (words: Words, language: string): RegExp => new RegExp(`^${word(words, language).replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`);

const actions: Readonly<Record<string, (ctx: Ctx) => Promise<void>>> = {
  async graphValueEdit(ctx) {
    const k = ctx.cases;
    const graph = ctx.win(param(ctx, "graph"));
    const preview = ctx.win(param(ctx, "preview"));
    const height = async (): Promise<number | undefined> => {
      const host = graph.locator(".semio-node-graph-host").first();
      return (JSON.parse((await attribute(host, "data-host-snapshot-json")) ?? "{}") as { widgets?: { id: string; value?: number }[] }).widgets?.find((w) => w.id === k.heightId)?.value;
    };
    await until(height, (v) => v === k.initial, timeouts.content, "initial height widget");
    await settled(preview);
    const before = await projection(preview);
    await docTab(ctx, "artifact");
    await ctx.at(k.heightId).locator('[data-slot="tree-label"]').first().click();
    await docTab(ctx, "inspection");
    const input = ctx.at(param(ctx, "inputId")).first();
    await until(() => input.inputValue(), (v) => v === String(k.initial), timeouts.action, "inspector shows initial value");
    await input.fill(String(k.changed));
    await input.press("Tab");
    await until(height, (v) => v === k.changed, timeouts.action, "height widget after edit");
    await until(() => projection(preview), (v) => v !== before, timeouts.action, "preview geometry after edit");
    await settled(preview);
    const after = await projection(preview);
    for (const [direction, value, geometry] of [["undo", k.initial, before], ["redo", k.changed, after]] as const) {
      await history(ctx, graph, direction);
      await until(height, (v) => v === value, timeouts.action, `height after ${direction}`);
      await until(() => projection(preview), (v) => v === geometry, timeouts.action, `preview after ${direction}`);
    }
  },

  async catalogueAdd(ctx) {
    const k = ctx.cases;
    const gains = param<string[]>(ctx, "gains").map((id) => ctx.win(id));
    const same = param<string[]>(ctx, "unchanged").map((id) => ctx.win(id));
    const lead = gains[0];
    if (!lead) throw new Error("catalogueAdd needs a gaining window");
    for (const box of [...gains, ...same]) await settled(box);
    const base = await instanceIds(lead);
    if (k.seedId !== undefined && !base.includes(k.seedId)) throw new Error(`seed instance ${k.seedId} missing`);
    const initial = await Promise.all(gains.map(projection));
    const unchanged = await Promise.all(same.map(projection));
    await lead.locator("canvas").first().click({ position: { x: 8, y: 8 } });
    await docTab(ctx, "catalogue");
    await ctx.at(String(k[param<string>(ctx, "targetKey")])).locator('[data-slot="tree-label"]').first().click();
    const grown = await until(() => instanceIds(lead), (ids) => ids.filter((id) => !base.includes(id)).length === 1, timeouts.action, "one new instance");
    const added = grown.find((id) => !base.includes(id)) ?? "";
    for (const box of gains) await until(() => instanceIds(box), (ids) => ids.includes(added), timeouts.action, "new instance in every gaining window");
    await docTab(ctx, "artifact");
    const row = ctx.at(added);
    const rowText = String(param<string | undefined>(ctx, "rowText") ?? k[param<string>(ctx, "rowTextKey")]);
    await until(() => text(row), (t) => new RegExp(rowText).test(t), timeouts.action, "document row for the new instance");
    if (k.seedId !== undefined) {
      if (!/^puzzle3d\.object\.[0-9a-f]+$/.test(added)) throw new Error(`new instance id ${added}`);
      for (const box of gains) {
        const host = box.locator(".semio-world-3d-host").first();
        const meshes = (await attribute(host, "data-meshes-json")) ?? "";
        if (!new RegExp(String(k.meshUrl).replaceAll(".", "\\.")).test(meshes)) throw new Error(`mesh ${k.meshUrl} missing from the window`);
        const list = JSON.parse((await attribute(host, "data-instances-json")) ?? "[]") as { id: string; meshId: string }[];
        if (list.find((item) => item.id === added)?.meshId !== list.find((item) => item.id === k.seedId)?.meshId) throw new Error("new instance mesh differs from the seed mesh");
        const vortices = JSON.parse((await attribute(host, "data-vortices-json")) ?? "[]") as { objectId: string }[];
        if (vortices.filter((vortex) => vortex.objectId === added).length !== k.vortexCount) throw new Error(`vortex count differs from ${k.vortexCount}`);
      }
    }
    const changed = await Promise.all(gains.map(projection));
    await Promise.all(same.map(async (box, i) => { if ((await projection(box)) !== unchanged[i]) throw new Error("a window that must not change did change"); }));
    for (const [direction, present] of [["undo", false], ["redo", true]] as const) {
      await history(ctx, lead, direction);
      await until(() => instanceIds(lead), (ids) => ids.includes(added) === present, timeouts.action, `instance ${present ? "back" : "gone"} after ${direction}`);
      await until(() => row.count(), (n) => (n > 0) === present, timeouts.action, `document row after ${direction}`);
      if (k.seedId !== undefined) for (const [index, box] of gains.entries()) await until(() => projection(box), (v) => v === (present ? changed : initial)[index], timeouts.action, `window projection after ${direction}`);
    }
  },

  async rowToggle(ctx) {
    const k = ctx.cases;
    const box = ctx.win(param(ctx, "window"));
    const field = (id: string): Locator => ctx.at(`${param<string>(ctx, "fieldPrefix")}${id}`);
    const button = tokens(param<Words>(ctx, "buttonName"), ctx.language);
    const row = ctx.at(k.stepId).first();
    const inspect = async (enabled: string): Promise<void> => {
      await docTab(ctx, "inspection");
      await until(() => text(field("enabled")), (t) => t.includes(enabled), timeouts.action, `step enabled=${enabled}`);
    };
    await settled(box);
    await docTab(ctx, "artifact");
    await until(() => row.count(), (n) => n > 0, timeouts.content, "step row present");
    await row.locator('[data-slot="tree-label"]').first().click();
    await inspect("true");
    const before = await projection(box);
    await docTab(ctx, "artifact");
    await row.getByRole("button", { name: button }).click();
    await inspect("false");
    await until(() => projection(box), (v) => v !== before, timeouts.action, "workpiece geometry after toggle");
    await settled(box);
    const after = await projection(box);
    for (const [direction, enabled, geometry] of [["undo", "true", before], ["redo", "false", after]] as const) {
      await history(ctx, box, direction);
      await docTab(ctx, "artifact");
      await row.locator('[data-slot="tree-label"]').first().click();
      await inspect(enabled);
      await until(() => projection(box), (v) => v === geometry, timeouts.action, `workpiece after ${direction}`);
    }
  },

  async featureAdd(ctx) {
    const k = ctx.cases;
    const map = ctx.win(param(ctx, "window"));
    const summary = (kind: string): Locator => ctx.at(`${param<string>(ctx, "summaryPrefix")}${kind}-count`).locator('[data-slot="tree-description"]');
    const counts = (): Promise<string> => Promise.all([text(summary("positions")), text(summary("routes")), text(summary("regions"))]).then((all) => all.join("|"));
    await docTab(ctx, "inspection");
    const start = await until(counts, (c) => /^\d+\|/.test(c), timeouts.content, "feature summary rows");
    const positions = Number(start.split("|")[0]);
    const tail = start.split("|").slice(1).join("|");
    await ctx.at(param(ctx, "engagementToggleId")).first().click();
    const pane = map.locator('[data-slot="window-action-pane"]');
    await pane.locator('[id="action.addFeature"] [data-slot="tree-label"]').first().click();
    await pane.locator('[id="action.addFeature.arg.label"] input').fill(k.label);
    await until(() => pane.locator('[id="action.addFeature.arg.lon"] input').inputValue(), (v) => v === String(k.longitude), timeouts.action, "longitude default");
    await ctx.at(param(ctx, "executeId")).first().click();
    const want = (n: number): string => `${n}|${tail}`;
    await until(counts, (c) => c === want(positions + k.delta), timeouts.action, "position count after add");
    for (const [direction, n] of [["undo", positions], ["redo", positions + k.delta]] as const) {
      await history(ctx, map, direction);
      await until(counts, (c) => c === want(n), timeouts.action, `position count after ${direction}`);
    }
  },

  async toolRun(ctx) {
    const k = ctx.cases;
    const sim = ctx.win(param(ctx, "window"));
    const idle = word(param<Words>(ctx, "idle"), ctx.language);
    await until(() => text(sim), (t) => t.includes(idle), timeouts.content, "idle simulation text");
    await sim.getByRole("tree").click();
    await paneTab(ctx, k.toolCategoryId, k.toolTabId);
    await sim.getByRole("tree").click();
    await ctx.page.keyboard.press("ControlOrMeta+Enter");
    const group = await until(() => latestToolRunGroup(ctx.page, ctx.pane), (g) => g !== "", timeouts.action, "tool run group");
    const status = ctx.at(`${group}.status`);
    await until(() => ctx.at(group).getByRole("progressbar").first().isVisible(), Boolean, timeouts.action, "run progressbar visible");
    await until(() => attribute(status, "aria-live"), (v) => /^(polite|off|assertive)$/.test(v ?? ""), timeouts.action, "status aria-live");
    await until(() => text(status), (t) => new RegExp(k.finalized).test(t), 3 * timeouts.content, "run finalized");
    await until(() => text(ctx.at(`${group}.steps`)), (t) => new RegExp(k.finalStep).test(t), timeouts.action, "final step line");
    await until(() => text(sim), (t) => new RegExp(k.acceptedResult).test(t), timeouts.action, "result accepted");
    await ctx.at(`${group}.toolRunStart`).first().click();
    const next = await until(() => latestToolRunGroup(ctx.page, ctx.pane), (g) => /^framework\.toolRun\.\d+$/.test(g) && g !== group, timeouts.action, "restarted run group");
    const abort = ctx.at(`${next}.toolRunAbort`).first();
    await until(() => abort.isEnabled(), Boolean, timeouts.action, "abort enabled");
    await abort.click();
    await until(() => text(ctx.at(`${next}.status`)), (t) => new RegExp(k.aborted).test(t), timeouts.action, "run aborted");
    await until(() => text(sim), (t) => new RegExp(k.noResult).test(t), timeouts.action, "no result accepted after abort");
  },

  async rowSelectPreview(ctx) {
    const k = ctx.cases;
    const preview = ctx.win(param(ctx, "preview"));
    const pool = ctx.win(param(ctx, "pool"));
    const placeholder = word(param<Words>(ctx, "placeholder"), ctx.language);
    await until(() => text(preview), (t) => t.includes(placeholder), timeouts.content, "no-selection placeholder");
    const stock = pool.locator(`.semio-table-host [data-row-id="${k.objectId}"]`).first();
    await until(() => stock.count(), (n) => n > 0, timeouts.content, "authored stock row");
    await stock.click({ timeout: 30_000 });
    await until(() => worldCount(preview), (n) => n > 0, timeouts.content, "preview scene after selection");
    await until(() => instanceIds(preview), (ids) => ids.length === 1 && ids[0] === k.objectId, timeouts.content, "preview shows the selected kind");
  },

  async paramScale(ctx) {
    const k = ctx.cases;
    const results = ctx.win(param(ctx, "window"));
    const caption = results.getByText(new RegExp(k.caption));
    const scale = ctx.at(param(ctx, "scaleId")).first();
    const phase = ctx.at(param(ctx, "phaseId")).getByRole("slider");
    await until(() => caption.isVisible(), Boolean, timeouts.content, "solved caption");
    await caption.click();
    await paneTab(ctx, k.panelCategoryId, k.panelTabId);
    await until(() => text(ctx.at(param(ctx, "transportId"))), (t) => tokens(param<Words>(ctx, "paused"), ctx.language).test(t), timeouts.action, "playback paused");
    await phase.press("Home");
    await phase.press("PageUp");
    await until(() => phase.getAttribute("aria-valuenow"), (v) => v === String(k.phase), timeouts.action, "phase value");
    await until(() => scale.inputValue(), (v) => v === String(k.initialScale), timeouts.action, "initial deformation scale");
    await until(() => worldCount(results), (n) => n > 0, timeouts.content, "results scene");
    const before = await projection(results);
    await scale.fill(String(k.changedScale));
    await scale.press("Tab");
    await until(() => projection(results), (v) => v !== before, timeouts.action, "deformation after scale change");
    await until(() => caption.isVisible(), Boolean, timeouts.action, "caption current after change");
    const changed = await projection(results);
    for (const [direction, value, geometry] of [["undo", k.initialScale, before], ["redo", k.changedScale, changed]] as const) {
      await caption.click();
      await ctx.page.keyboard.press(direction === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
      await until(() => scale.inputValue(), (v) => v === String(value), timeouts.action, `scale after ${direction}`);
      await until(() => projection(results), (v) => v === geometry, timeouts.action, `deformation after ${direction}`);
      await until(() => caption.isVisible(), Boolean, timeouts.action, `caption current after ${direction}`);
      await until(() => phase.getAttribute("aria-valuenow"), (v) => v === String(k.phase), timeouts.action, `phase kept after ${direction}`);
      await until(() => text(ctx.at(param(ctx, "transportId"))), (t) => tokens(param<Words>(ctx, "paused"), ctx.language).test(t), timeouts.action, `playback paused after ${direction}`);
    }
  },

  async curationUndoRedo(ctx) {
    const k = ctx.cases;
    const pool = ctx.win(param(ctx, "pool"));
    const curated = ctx.win(param(ctx, "curated"));
    const grid = ctx.win(param(ctx, "grid"));
    const stock = pool.locator(`[data-row-id="${k.objectId}"]`);
    const row = curated.locator(`[data-row-id="${k.objectId}"]`);
    const count = stock.getByRole("spinbutton");
    const gridCount = async (): Promise<number> => {
      const host = grid.locator(".semio-world-3d-host").first();
      if ((await host.count()) === 0) return 0;
      return (JSON.parse((await attribute(host, "data-instances-json")) ?? "[]") as { id: string }[]).filter((item) => item.id === k.objectId).length;
    };
    const scenes = new Map<number, string>();
    const scene = async (): Promise<string> => ((await grid.locator(".semio-world-3d-host").count()) === 0 ? "[]" : (await attribute(grid.locator(".semio-world-3d-host"), "data-instances-json")) ?? "[]");
    const assertCount = async (expected: number, label: string): Promise<void> => {
      await until(() => count.inputValue(), (v) => v === String(expected), timeouts.action, `pool count ${label}`);
      await until(() => curated.locator("[data-row-id]").count(), (n) => n === (expected === 0 ? 0 : 1), timeouts.action, `curated kinds ${label}`);
      if (expected > 0) await until(() => row.getByRole("spinbutton").inputValue(), (v) => v === String(expected), timeouts.action, `curated count ${label}`);
      await until(gridCount, (n) => n === expected, timeouts.content, `grid geometry ${label}`);
      const now = await scene();
      const seen = scenes.get(expected);
      if (seen === undefined) scenes.set(expected, now);
      else if (seen !== now) throw new Error(`grid projection for count ${expected} differs ${label}`);
    };
    await until(() => text(stock), (t) => t.includes(String(k.name)), timeouts.content, "stock row name");
    await until(() => count.getAttribute("aria-valuemax"), (v) => v === String(k.availability), timeouts.action, "availability maximum");
    await assertCount(0, "initially");
    const name = stock.getByText(String(k.name), { exact: true });
    for (const step of k.steps as { action: string; count: number }[]) {
      if (step.action === "increment") await stock.locator('[data-stepper-control="increment"]').click();
      else if (step.action === "keyboardIncrement") await count.press("ArrowUp");
      else if (step.action === "remove") await row.getByRole("button", { name: tokens(param<Words>(ctx, "remove"), ctx.language) }).click();
      else {
        await name.click();
        await ctx.page.keyboard.press(step.action === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
      }
      await assertCount(step.count, `after ${step.action}`);
    }
    const overview = ctx.page.locator("[data-layered-overview-button]");
    const open = (id: string): Locator => ctx.page.locator(`[data-layered-card="${id}"] [data-slot="demonstrator-pane-card-open-chip"]`);
    await overview.click();
    await dismissIntroduction(ctx.page, '[id="ui.introduction.skip"]');
    await open(k.revisit.viaPane).click();
    await until(() => shellOutcome(ctx.page, k.revisit.viaPane), (s) => s === "ready", timeouts.shellReady, "revisit pane ready");
    await dismissIntroduction(ctx.page, `[data-shell-id="${k.revisit.viaPane}"] [id="ui.introduction.skip"]`);
    await ctx.page.waitForTimeout(k.revisit.offscreenMs);
    if ((await ctx.page.locator(`[data-shell-id="${ctx.pane}"]`).count()) !== 1) throw new Error("edited pane was unmounted while off screen");
    await overview.click();
    await open(ctx.pane).click();
    await assertCount(k.revisit.afterUndoCount, "after revisiting the edited pane");
    await name.click();
    await ctx.page.keyboard.press("ControlOrMeta+Shift+z");
    await assertCount(k.revisit.afterRedoCount, "after revisited redo");
    await ctx.page.keyboard.press("ControlOrMeta+z");
    await assertCount(k.revisit.afterUndoCount, "after revisited undo");
  },
};

async function checkWindow(page: Page, pane: string, win: WindowSpec, language: string): Promise<void> {
  const ids = (win.instances ?? [win.kind]).map(windowId);
  const label = `window ${win.instances ? win.kind : win.kind}`;
  if (win.instances) await until(() => page.locator(paneSelector(pane, windowId(win.kind))).count(), (n) => n === ids.length, timeouts.shellReady, `${label} instance count`);
  for (const id of ids) {
    const box = page.locator(paneSelector(pane, id)).first();
    await box.waitFor({ state: "visible", timeout: timeouts.shellReady });
    const need = win.content;
    const wait = need ? timeouts.content : 3_000;
    if (win.surface === "world3d") {
      await until(() => box.locator(".semio-world-3d-host, .semio-world-3d-empty").count(), (n) => n > 0, timeouts.shellReady, `${label} surface`);
      if (need) await until(() => worldCount(box), (n) => n > 0, wait, `${label} meshes+instances`);
    } else if (win.surface === "nodeGraph") {
      await until(() => box.locator(".semio-node-graph-host, .semio-node-graph-empty").count(), (n) => n > 0, timeouts.shellReady, `${label} surface`);
      if (need) await until(() => graphWidgetCount(box), (n) => n > 0, wait, `${label} widgets`);
    } else if (win.surface === "table") {
      await until(() => box.locator(".semio-table-host, .semio-table-empty").count(), (n) => n > 0, timeouts.shellReady, `${label} surface`);
      if (need) await until(() => tableRowCount(box), (n) => n > 0, wait, `${label} rows`);
    } else if (win.surface === "tree") {
      await until(() => box.locator('[role="tree"]').count(), (n) => n > 0, timeouts.shellReady, `${label} tree`);
      if (need) await until(() => treeRowCount(box), (n) => n > 0, wait, `${label} tree rows`);
    } else if (win.surface === "placeholder") {
      const token = word(win.placeholder ?? {}, language);
      await until(() => text(box), (t) => t.includes(token), timeouts.shellReady, `${label} placeholder "${token}"`);
    } else {
      const canvas = box.locator(".semio-tiled-map-host canvas").first();
      await canvas.waitFor({ state: "visible", timeout: timeouts.shellReady });
      await canvas.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
      await until(() => distinctColors(page, canvas), (n) => n > 1, wait, `${label} painted map`);
    }
  }
}

async function runStage(row: Row, key: Stages, work: () => Promise<void>): Promise<boolean> {
  try { await work(); row[key] = "PASS"; return true; } catch (error) { row[key] = "FAIL"; row.detail.push(`${key}: ${firstLine(error)}`); return false; }
}
const newRow = (unit: string): Row => ({ unit, boot: "SKIP", content: "SKIP", action: "SKIP", console: "SKIP", ms: 0, detail: [] });
async function finishRow(row: Row, started: number, session: Session, shotDir: string | undefined): Promise<Row> {
  row.console = session.errors.length === 0 ? "PASS" : "FAIL";
  for (const error of [...new Set(session.errors)].slice(0, 3)) row.detail.push(error);
  row.ms = Date.now() - started;
  if (shotDir && [row.boot, row.content, row.action, row.console].includes("FAIL")) {
    mkdirSync(shotDir, { recursive: true });
    await session.page.screenshot({ path: join(shotDir, `${row.unit.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => undefined);
  }
  await session.context.close().catch(() => undefined);
  return row;
}

async function runPane(browser: Browser, base: string, pane: PaneSpec, lang: Language, all: Cases, shotDir: string | undefined, withAction: boolean): Promise<Row> {
  const row = newRow(`${pane.id}@${lang.language}`);
  const started = Date.now();
  const session = await openSession(browser, lang.browserLocale);
  const { page } = session;
  const booted = await runStage(row, "boot", async () => {
    await page.goto(new URL(`#${pane.id}`, base).toString(), { waitUntil: "domcontentloaded", timeout: 60_000 });
    await until(() => shellOutcome(page, pane.id), (s) => s !== "absent" && s !== "booting", timeouts.shellReady, "shell outcome");
    const outcome = await shellOutcome(page, pane.id);
    if (outcome !== "ready") throw new Error(`shell ${outcome}`);
    await dismissIntroduction(page, `[data-shell-id="${pane.id}"] [id="ui.introduction.skip"]`);
  });
  if (booted) {
    await runStage(row, "content", async () => {
      for (const win of pane.windows) await checkWindow(page, pane.id, win, lang.language);
    });
    if (withAction) await runStage(row, "action", async () => {
      for (const type of [pane.action.type, ...(pane.action.follow ?? [])]) {
        const run = actions[type];
        if (!run) throw new Error(`unknown action ${type}`);
        await run(mk(page, pane.id, lang.language, pane.action, all));
      }
    });
  }
  return finishRow(row, started, session, shotDir);
}

async function runLanding(browser: Browser, base: string, lang: Language, ids: readonly string[], shotDir: string | undefined): Promise<Row> {
  const row = newRow(`landing@${lang.language}`);
  const started = Date.now();
  const session = await openSession(browser, lang.browserLocale);
  const { page } = session;
  const texts = catalog();
  const say = (key: string, language: string): string => word(texts[key] ?? {}, language);
  const ok = await runStage(row, "boot", async () => {
    await page.goto(base, { waitUntil: "domcontentloaded", timeout: 60_000 });
    await until(() => page.locator("html").getAttribute("lang"), (v) => v === lang.language, timeouts.shellReady, "html lang");
    await until(() => page.title(), (t) => t === say(expectation.landing.titleKey, lang.language), 10_000, "document title");
    await dismissIntroduction(page, '[id="ui.introduction.skip"]');
  });
  if (ok) await runStage(row, "content", async () => {
    await until(() => page.locator("[data-layered-card]").count(), (n) => n === ids.length, timeouts.shellReady, "overview card count");
    for (const id of ids) {
      const wanted = say(expectation.landing.taglineKey.replace("{pane}", id), lang.language);
      const card = page.locator(`[data-layered-card="${id}"] [data-overview-card]`);
      await card.waitFor({ state: "visible", timeout: 10_000 });
      const slot = (name: string): Locator => card.locator(`[data-slot="${name}"]`);
      const counts: readonly [string, Locator, number][] = [
        ["silhouette", card.locator("[data-window-silhouette]"), 1],
        ["silhouette border", card.locator('[data-window-silhouette-border][data-kind="normal"]'), 1],
        ["title chip icon", card.locator('[data-slot="demonstrator-pane-card-title-chip"] svg'), 1],
        ["description paragraphs", slot("introduction-body-paragraph"), 2],
        ["drag handles", card.locator('[data-slot*="drag"], [data-drag-handle]'), 0],
      ];
      for (const [name, loc, want] of counts) await until(() => loc.count(), (n) => n === want, 10_000, `${id} ${name}`);
      for (const [name, wantGlass] of [["window-chrome-chip-cap", true], ["window-chrome-body-surface", true], ["window-chrome-body", false]] as const) {
        const klass = await attribute(slot(name), "class");
        if (/ui-glass/.test(klass ?? "") !== wantGlass && name !== "window-chrome-body") throw new Error(`${id} ${name} glass class: ${klass}`);
        if (name === "window-chrome-body" && /ui-surface/.test(klass ?? "")) throw new Error(`${id} ${name} must not be ui-surface: ${klass}`);
      }
      const opens = say("demonstrator.card.open.label.normal", lang.language);
      await until(() => text(slot("demonstrator-pane-card-open-chip")), (t) => t.includes(flat(opens)), 10_000, `${id} open chip`);
      await until(() => text(page.locator(`[data-layered-card="${id}"] [data-slot="demonstrator-pane-card-tagline"]`)), (t) => t === flat(wanted), 10_000, `tagline ${id}`);
    }
  });
  return finishRow(row, started, session, shotDir);
}

async function runPicker(browser: Browser, base: string, languages: readonly Language[], ids: readonly string[], shotDir: string | undefined): Promise<Row> {
  const row = newRow("picker");
  const started = Date.now();
  const session = await openSession(browser, expectation.unsupportedBrowserLocale);
  const { page } = session;
  const picker = page.locator('[data-slot="demonstrator-locale-picker"]');
  const texts = catalog();
  const chosen = expectation.landing.pickerChoose;
  const ok = await runStage(row, "boot", async () => {
    await page.goto(base, { waitUntil: "domcontentloaded", timeout: 60_000 });
    await picker.waitFor({ state: "visible", timeout: timeouts.shellReady });
    if ((await picker.locator("button").count()) !== languages.length) throw new Error("picker button count differs from the shipped languages");
    if ((await picker.locator("[aria-pressed], [aria-checked]").count()) !== 0) throw new Error("a language is preselected");
    if ((await page.locator("[data-layered-card]").count()) !== 0) throw new Error("cards visible before a language is chosen");
  });
  if (ok) {
    await runStage(row, "content", async () => {
      for (const { language } of languages) await picker.locator(`button[lang="${language}"]`).waitFor({ state: "visible", timeout: 5_000 });
    });
    await runStage(row, "action", async () => {
      await picker.locator(`button[data-locale="${chosen}"]`).click();
      await until(() => page.locator("html").getAttribute("lang"), (v) => v === chosen, timeouts.action, "html lang after choice");
      await until(() => page.title(), (t) => t === word(texts[expectation.landing.titleKey] ?? {}, chosen), 10_000, "title after choice");
      await until(() => page.locator("[data-layered-card]").count(), (n) => n === ids.length, timeouts.shellReady, "cards after choice");
      await page.reload({ waitUntil: "domcontentloaded" });
      await until(() => page.locator("html").getAttribute("lang"), (v) => v === chosen, timeouts.action, "choice kept across reload");
      if ((await picker.count()) !== 0) throw new Error("picker shown again after a stored choice");
    });
  }
  return finishRow(row, started, session, shotDir);
}

async function runSwitch(browser: Browser, base: string, languages: readonly Language[], ids: readonly string[], shotDir: string | undefined): Promise<Row> {
  const row = newRow("switch");
  const started = Date.now();
  const first = languages[0];
  const second = languages[1];
  const session = await openSession(browser, first?.browserLocale ?? "en-US");
  const { page } = session;
  const texts = catalog();
  const switcher = page.locator('[data-slot="demonstrator-locale-switcher"]');
  const ok = await runStage(row, "boot", async () => {
    await page.goto(base, { waitUntil: "domcontentloaded", timeout: 60_000 });
    await dismissIntroduction(page, '[id="ui.introduction.skip"]');
    await switcher.waitFor({ state: "visible", timeout: timeouts.shellReady });
  });
  if (ok && first && second) {
    const lead = ids[0] ?? "";
    await runStage(row, "action", async () => {
      for (const { language } of [second, first]) {
        const button = switcher.locator(`button[data-locale="${language}"]`);
        await button.click();
        await until(() => button.getAttribute("aria-pressed"), (v) => v === "true", timeouts.action, `${language} pressed`);
        await until(() => page.locator("html").getAttribute("lang"), (v) => v === language, timeouts.action, "html lang");
        await until(() => page.title(), (t) => t === word(texts[expectation.landing.titleKey] ?? {}, language), 10_000, "title");
        await until(() => text(page.locator(`[data-layered-card="${lead}"] [data-slot="demonstrator-pane-card-tagline"]`)), (t) => t === flat(word(texts[expectation.landing.taglineKey.replace("{pane}", lead)] ?? {}, language)), timeouts.shellReady, "tagline");
      }
    });
  }
  return finishRow(row, started, session, shotDir);
}

async function runRemount(browser: Browser, base: string, languages: readonly Language[], shotDir: string | undefined): Promise<Row> {
  const row = newRow("remount");
  const started = Date.now();
  const first = languages[0];
  const second = languages[1];
  const session = await openSession(browser, first?.browserLocale ?? "en-US");
  const { page } = session;
  const pane = "koordinator";
  const shell = page.locator(`[data-shell-id="${pane}"]`);
  const ok = await runStage(row, "boot", async () => {
    await page.goto(base, { waitUntil: "domcontentloaded", timeout: 60_000 });
    await dismissIntroduction(page, '[id="ui.introduction.skip"]');
    await page.locator(`[data-layered-card="${pane}"] [data-slot="demonstrator-pane-card-open-chip"]`).click();
    await until(() => shellOutcome(page, pane), (s) => s === "ready", timeouts.shellReady, "pane ready");
    await dismissIntroduction(page, `[data-shell-id="${pane}"] [id="ui.introduction.skip"]`);
    await shell.evaluate((element) => element.setAttribute("data-remount-probe", "kept"));
  });
  if (ok && first && second) await runStage(row, "action", async () => {
    await page.locator("[data-layered-overview-button]").click();
    const switcher = page.locator('[data-slot="demonstrator-locale-switcher"]');
    for (const { language } of [second, first]) {
      await switcher.locator(`button[data-locale="${language}"]`).click();
      await until(() => page.locator("html").getAttribute("lang"), (v) => v === language, timeouts.action, "html lang");
      await until(() => shell.getAttribute("lang"), (v) => v === language, timeouts.action, "shell lang");
      await until(() => shell.getAttribute("data-remount-probe"), (v) => v === "kept", timeouts.action, "pane kept mounted");
    }
  });
  return finishRow(row, started, session, shotDir);
}

async function runFunding(browser: Browser, base: string, lang: Language, shotDir: string | undefined): Promise<Row> {
  const row = newRow(`funding@${lang.language}`);
  const started = Date.now();
  const session = await openSession(browser, lang.browserLocale);
  const { page } = session;
  const texts = catalog();
  const ok = await runStage(row, "boot", async () => {
    await page.goto(base, { waitUntil: "networkidle", timeout: 60_000 });
    const next = page.locator('[data-slot="introduction-info-box"] button').filter({ hasText: new RegExp(lang.introNext, "i") });
    for (let step = 0; step < 2; step += 1) {
      await next.click({ timeout: 15_000 });
      await page.waitForTimeout(400);
    }
  });
  if (ok) await runStage(row, "content", async () => {
    await until(() => text(page.locator('[data-slot="introduction-info-box-title"]')), (t) => t === flat(word(texts["demonstrator.brand.shared.fundingNotice"] ?? {}, lang.language)), 10_000, "funding notice title");
    const sizes = await page.locator('[data-slot="introduction-info-box"] img').evaluateAll((imgs) =>
      imgs.filter((img) => getComputedStyle(img).display !== "none").map((img) => ({ width: img.clientWidth, height: img.clientHeight, naturalWidth: (img as HTMLImageElement).naturalWidth })));
    if (sizes.length < 3) throw new Error(`only ${sizes.length} visible funding logos`);
    for (const size of sizes) if (size.width <= 0 || size.height <= 0 || size.naturalWidth <= 0) throw new Error(`funding logo not laid out or decoded: ${JSON.stringify(size)}`);
  });
  return finishRow(row, started, session, shotDir);
}

const mark = (stage: Stage): string => (stage === "PASS" ? "PASS" : stage === "FAIL" ? "FAIL" : "-");
export function renderTable(rows: readonly Row[]): string {
  const head = ["UNIT", "BOOT", "CONTENT", "ACTION", "CONSOLE", "SEC", "DETAIL"];
  const body = rows.map((r) => [r.unit, mark(r.boot), mark(r.content), mark(r.action), mark(r.console), (r.ms / 1000).toFixed(0), r.detail[0]?.slice(0, 110) ?? ""]);
  const widths = head.map((h, i) => Math.max(h.length, ...body.map((b) => (b[i] ?? "").length)));
  const line = (cells: readonly string[]): string => cells.map((c, i) => c.padEnd(widths[i] ?? 0)).join("  ").trimEnd();
  const failed = rows.filter((r) => [r.boot, r.content, r.action, r.console].includes("FAIL")).length;
  const more = rows.flatMap((r) => r.detail.slice(1).map((d) => `  ${r.unit}: ${d}`));
  return [line(head), line(widths.map((w) => "-".repeat(w))), ...body.map(line), "", `VERDICT ${failed === 0 ? "GREEN" : "RED"} units=${rows.length} failed=${failed}`, ...(more.length ? ["", "more detail:", ...more] : [])].join("\n");
}

async function check(): Promise<number> {
  const results: [string, boolean, string][] = [];
  const note = (name: string, pass: boolean, detail = ""): void => { results.push([name, pass, detail]); };
  const guard = async (name: string, work: () => Promise<string | void>): Promise<void> => { try { note(name, true, (await work()) ?? ""); } catch (error) { note(name, false, firstLine(error)); } };
  const all = loadCases();
  const ids = brandPaneIds();
  await guard("schema", async () => { if (expectation.schema !== "demonstrator-e2e-ready/1") throw new Error(expectation.schema); });
  await guard("pane ids equal brand", async () => { if (JSON.stringify(expectation.panes.map((p) => p.id)) !== JSON.stringify(ids)) throw new Error(`${expectation.panes.map((p) => p.id)} vs ${ids}`); return ids.join(","); });
  await guard("window kinds equal acceptance spec", async () => {
    const source = readFileSync(join(root, expectation.sources.spec), "utf8");
    const block = source.slice(source.indexOf("const PANE_CASES"), source.indexOf("\n];", source.indexOf("const PANE_CASES")));
    for (const pane of expectation.panes) {
      const part = block.split(`paneId: "${pane.id}"`)[1]?.split(/paneId: "/)[0] ?? "";
      const kinds = [...part.matchAll(/kindId: "([^"]+)"/g)].map((m) => m[1]);
      if (JSON.stringify(kinds) !== JSON.stringify(pane.windows.map((w) => w.kind))) throw new Error(`${pane.id}: ${kinds} vs ${pane.windows.map((w) => w.kind)}`);
    }
  });
  await guard("every pane has a known action and cases", async () => {
    for (const pane of expectation.panes) {
      for (const type of [pane.action.type, ...(pane.action.follow ?? [])]) if (!actions[type]) throw new Error(`${pane.id}: unknown action ${type}`);
      if (dig(all, pane.action.cases) === undefined) throw new Error(`${pane.id}: no cases at ${pane.action.cases}`);
    }
    return `${expectation.panes.length} actions`;
  });
  await guard("language words complete", async () => {
    const texts = catalog();
    const keys = [expectation.landing.titleKey, ...ids.map((id) => expectation.landing.taglineKey.replace("{pane}", id))];
    for (const key of keys) for (const { language } of expectation.languages) if (!word(texts[key] ?? {}, language)) throw new Error(`${key} lacks ${language}`);
    for (const pane of expectation.panes) for (const words of [pane.action.idle, pane.action.paused, pane.action.buttonName, pane.action.placeholder, pane.action.remove, ...pane.windows.map((w) => w.placeholder)]) {
      if (words) for (const { language } of expectation.languages) if (!word(words as Words, language)) throw new Error(`${pane.id} lacks ${language} wording`);
    }
    return `${keys.length} catalog keys x ${expectation.languages.length} languages`;
  });
  await guard("no default language", async () => { if (JSON.stringify(expectation).includes('"defaultLanguage"')) throw new Error("expectation names a default language"); });
  await guard("headless chromium reads fixture DOM", async () => {
    const browser = await chromium.launch({ args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
    try {
      const page = await browser.newPage();
      await page.setContent(`<div data-shell-id="p" data-shell-ready>
        <div id="framework.window.w"><div class="semio-world-3d-host" data-meshes-json='[{"id":"m"}]' data-instances-json='[{"id":"b","selected":true},{"id":"a"},{"id":"a"}]'></div></div>
        <div id="framework.window.g"><div class="semio-node-graph-host" data-host-snapshot-json='{"widgets":[{"id":"h","value":3}],"synapses":[]}'></div></div>
        <div id="framework.window.t"><div class="semio-table-host"><table><tr data-row-id="1"></tr><tr data-row-id="2"></tr></table></div></div>
        <div id="framework.window.r"><div role="tree"><div role="treeitem"></div><div role="treeitem"></div><div role="treeitem"></div></div></div>
        <div id="framework.toolRun.2.status"></div><div id="framework.toolRun.10" ></div><div id="framework.toolRun.9.steps"></div>
        <div id="framework.window.inst" data-element-alias="framework.window.kind"></div></div>`);
      const at = (id: string): Locator => page.locator(paneSelector("p", id));
      const got = {
        shell: await shellOutcome(page, "p"),
        world: await worldCount(at("framework.window.w")),
        ids: await instanceIds(at("framework.window.w")),
        projection: JSON.parse(await projection(at("framework.window.w"))).instances.length,
        widgets: await graphWidgetCount(at("framework.window.g")),
        rows: await tableRowCount(at("framework.window.t")),
        tree: await treeRowCount(at("framework.window.r")),
        run: await latestToolRunGroup(page, "p"),
        alias: await at("framework.window.kind").count(),
        segment: windowId("energy.model.3d"),
      };
      const want = { shell: "ready", world: 4, ids: ["a", "b"], projection: 3, widgets: 1, rows: 2, tree: 3, run: "framework.toolRun.10", alias: 1, segment: "framework.window.energyModel3d" };
      if (JSON.stringify(got) !== JSON.stringify(want)) throw new Error(JSON.stringify(got));
      return "world, graph, table, tree, tool-run, alias readers agree";
    } finally { await browser.close(); }
  });
  await guard("verdict table renders", async () => {
    const row: Row = { unit: "demo@en", boot: "PASS", content: "FAIL", action: "SKIP", console: "PASS", ms: 1500, detail: ["content: x", "console y"] };
    const out = renderTable([row, newRow("other")]);
    if (!out.includes("VERDICT RED units=2 failed=1")) throw new Error(out);
  });
  for (const [name, pass, detail] of results) console.log(`${pass ? "PASS" : "FAIL"} ${name}${detail ? ` (${detail})` : ""}`);
  const bad = results.filter((r) => !r[1]).length;
  console.log(`CHECK ${bad === 0 ? "GREEN" : "RED"} ${results.length - bad}/${results.length}`);
  return bad === 0 ? 0 : 1;
}

async function main(): Promise<number> {
  const args = process.argv.slice(2);
  if (args.includes("--check")) return check();
  const flag = (name: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
  const base = args.find((a, i) => !a.startsWith("--") && !["--panes", "--languages", "--out"].includes(args[i - 1] ?? ""));
  if (!base) { console.error("usage: probe.ts <base-url> [--panes a,b] [--languages en,de] [--no-landing] [--out dir] | --check"); return 2; }
  const baseUrl = base.endsWith("/") ? base : `${base}/`;
  const paneFilter = (flag("--panes") ?? process.env.E2E_PANES)?.split(",").filter(Boolean);
  const langFilter = (flag("--languages") ?? process.env.E2E_LANGUAGES)?.split(",").filter(Boolean);
  const languages = expectation.languages.filter((l) => !langFilter || langFilter.includes(l.language));
  const out = flag("--out");
  const shotDir = out ? resolve(out) : undefined;
  const ids = brandPaneIds();
  const all = loadCases();
  const gpu = process.env.DEMONSTRATOR_E2E_GPU === "swiftshader" ? ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] : [process.platform === "darwin" ? "--use-angle=metal" : "--use-angle=gl", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"];
  const browser = await chromium.launch({ args: gpu });
  const adapter = await (async (): Promise<string> => {
    const probe = await browser.newPage();
    try {
      return await probe.evaluate(() => { const gl = document.createElement("canvas").getContext("webgl2"); const info = gl?.getExtension("WEBGL_debug_renderer_info"); return gl && info ? String(gl.getParameter(info.UNMASKED_RENDERER_WEBGL)) : "no webgl2"; });
    } finally { await probe.close(); }
  })();
  const software = /swiftshader|llvmpipe|software/i.test(adapter);
  console.log(`GPU adapter: ${adapter}${software && process.env.DEMONSTRATOR_E2E_GPU !== "swiftshader" ? " (SOFTWARE: the verfolgen wgpu map cannot boot here, expect its content FAIL)" : ""}`);
  const rows: Row[] = [];
  const emit = (row: Row): void => { rows.push(row); console.log(`done ${row.unit} boot=${row.boot} content=${row.content} action=${row.action} console=${row.console} ${(row.ms / 1000).toFixed(0)}s`); };
  try {
    if (!args.includes("--no-landing") && !paneFilter) {
      for (const lang of languages) emit(await runLanding(browser, baseUrl, lang, ids, shotDir));
      emit(await runPicker(browser, baseUrl, expectation.languages, ids, shotDir));
      emit(await runSwitch(browser, baseUrl, expectation.languages, ids, shotDir));
      emit(await runRemount(browser, baseUrl, expectation.languages, shotDir));
      for (const lang of languages) emit(await runFunding(browser, baseUrl, lang, shotDir));
    }
    for (const pane of expectation.panes.filter((p) => !paneFilter || paneFilter.includes(p.id))) {
      for (const [index, lang] of languages.entries()) emit(await runPane(browser, baseUrl, pane, lang, all, shotDir, index === 0 || process.env.E2E_ACTIONS_ALL_LANGUAGES === "1"));
    }
  } finally { await browser.close(); }
  const table = renderTable(rows);
  console.log(`\n${table}`);
  if (out) { mkdirSync(resolve(out), { recursive: true }); writeFileSync(join(resolve(out), "e2eready.txt"), `${table}\n`); writeFileSync(join(resolve(out), "e2eready.json"), `${JSON.stringify(rows, null, 2)}\n`); }
  return rows.some((r) => [r.boot, r.content, r.action, r.console].includes("FAIL")) ? 1 : 0;
}

if (import.meta.main) process.exit(await main());
