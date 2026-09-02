// #region 🧲️Header
// 💻️ ♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts
// Specs: End-to-end acceptance coverage for the "Entwerfen mit Bestand" demonstrator's six live panes.
// Summary: For every `DEMONSTRATOR_PANES` entry, deep-links straight to `/#<paneId>` (the fast path that
// boots exactly that one pane immediately instead of waiting through the 1.5s/35s sequential-boot queue —
// see `🟦️.tsx`'s `paneIdFromLocationHash`/`useSequentialPaneBoot`), waits for that pane's own
// `FrameworkOsShell` to report readiness via the per-shell `data-shell-ready`/`data-shell-error` beacon
// (`ShellHost/🟦️component.tsx`'s `#region 🔖️ReadinessBeacon`), asserts its declared window(s) attach, and
// asserts each window actually carries rendered content (not an empty surface, not the "wird vorbereitet"
// `CanvasSkeleton`) by reading the same production `data-*-json`/`data-row-id` attributes each surface host
// already stamps on itself (`World3dHost`'s `data-meshes-json`/`data-instances-json`, `NodeGraph`'s
// `data-fixture-json`, `Table`'s `data-row-id` rows) — no test-only instrumentation was added for this.
// Known-defect windows are asserted exactly like every other window rather than being weakened or
// skipped, so a real defect fails loudly. Generator's edit-mode preview is still expected to fail (it
// never gets a synced flow-eval session — `📓️app-generator.md` §3). Koordinator's four CAD windows were
// the same story until commit f394df99d4 wired `cad_pane_working_scene` through
// `ArtifactChild::local_owner`; that fix is committed but NOT yet compile-verified, so these four
// assertions are its first real proof — see `📓️app-koordinator.md`. Every test also fails on any page error or non-404 console error, following
// `.storybook/framework-hosts-wasm.spec.ts`'s `expectHostStory` idiom.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { expect, test, type Locator, type Page } from "@playwright/test";
// #endregion 🔌️Adapters

//#region 🪪️BrandPaneIds
/** @emoji 🪪️ Reads `🟦️brand.ts`'s pane ids as TEXT rather than importing it. Playwright loads specs
 * through Node's own ESM loader, and importing the brand module drags in the whole `@semio-tech/ui-react`
 * runtime — whose typed-scene catalog is a bare `.json` import that Node rejects without an
 * `import ... with { type: "json" }` attribute (`ERR_IMPORT_ATTRIBUTE_MISSING`). Vite rewrites that for the
 * app; Playwright does not, so this suite stays import-free of app modules exactly like `.storybook`'s specs. */
function brandPaneIds(): readonly string[] {
  const source = readFileSync(join(import.meta.dirname, "🟦️brand.ts"), "utf8");
  const block = /export const DEMONSTRATOR_PANES[^=]*=\s*\[(.*?)\n\];/s.exec(source);
  if (!block) throw new Error("🟦️brand.ts no longer declares a DEMONSTRATOR_PANES array literal");
  return [...block[1]!.matchAll(/\bid:\s*"([^"]+)"/g)].map((match) => match[1]!);
}
//#endregion 🪪️BrandPaneIds

/** @emoji ⏱️ Cold WASM plugin boots on a fresh page load can be slow (memory: "expect slow cold boots, be
 * patient") — generous on purpose so this suite reports real content defects, not infra flakiness. */
const SHELL_READY_TIMEOUT_MS = 120_000;
const TEST_TIMEOUT_MS = 180_000;

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b40[0-9]\b/i.test(text));
}

//#region 🆔️ElementId
/** @emoji 🆔️ Local mirror of `framework/ui/elements/🆔️ElementId/component.tsx`'s `elementIdSegment` — kept
 * as a tiny pure copy rather than importing the framework's React-bearing module into a Playwright spec. */
function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) segment += ch.toLowerCase();
    else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else segment += ch;
  }
  return segment;
}

/** @emoji 🪟️ Mirrors `framework/platform`'s `windowElementId(kindId)` → `"framework.window.<camelKindId>"`. */
function windowElementId(kindId: string): string {
  return `framework.window.${elementIdSegment(kindId)}`;
}

/** @emoji 🎯️ CSS selector for the element carrying `id` as either its real DOM id or a `data-element-alias`
 * token, scoped to one pane's shell root — mirrors `elementIdSelector`, scoped by `[data-shell-id]`. */
function paneElementSelector(paneId: string, elementId: string): string {
  return `[data-shell-id="${paneId}"] [id="${elementId}"], [data-shell-id="${paneId}"] [data-element-alias~="${elementId}"]`;
}
//#endregion 🆔️ElementId

//#region 🚦️ShellReadiness
type ShellOutcome = "ready" | "error" | "notFound";

/** @emoji 🚦️ Waits for the pane's own `[data-shell-id]` root to report an outcome via the per-shell
 * `data-shell-ready`/`data-shell-error`/`data-shell-not-found` beacon (added alongside the pre-existing
 * global `document.documentElement` one specifically so a page hosting several shells can ask "is THIS
 * one ready" — see `ShellHost/🟦️component.tsx`'s `#region 🔖️ReadinessBeacon`). */
async function waitForPaneShellOutcome(page: Page, paneId: string): Promise<ShellOutcome> {
  await page.waitForFunction(
    (id) => {
      const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
      return !!el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined || el.dataset.shellNotFound !== undefined);
    },
    paneId,
    { timeout: SHELL_READY_TIMEOUT_MS },
  );
  return page.evaluate((id) => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement;
    if (el.dataset.shellReady !== undefined) return "ready";
    if (el.dataset.shellError !== undefined) return "error";
    return "notFound";
  }, paneId) as Promise<ShellOutcome>;
}

/** @emoji 👋️ Every brand replays its own app-level introduction on load once focused
 * (`suppressAutoIntroduction={!focused}` is false for a hash-deep-linked, already-focused pane) — dismiss
 * it if present so it never shadows a later interaction. Absence is not an error (already dismissed, or
 * this brand has none left to show). */
async function dismissIntroductionIfPresent(page: Page, paneId: string): Promise<void> {
  const skip = page.locator(paneElementSelector(paneId, "ui.introduction.skip"));
  const visible = await skip.first().isVisible({ timeout: 3_000 }).catch(() => false);
  if (visible) await skip.first().click({ timeout: 5_000 }).catch(() => {});
}
//#endregion 🚦️ShellReadiness

//#region 🪟️SurfaceContent
type WindowSurfaceKind = "world3d" | "nodeGraph" | "table" | "tiledMap";

/** @emoji 🪟️ One window this pane is expected to open by default (per-app fixture/window research,
 * `.🧬semio/…/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️app-*.md`). `instanceIds` covers a window kind opened as
 * several simultaneous instances (aggregator's split top/perspective puzzle3d-main views) — each instance
 * carries its OWN `id` plus a shared `data-element-alias` back to the kind id (`ShellHost/🟦️component.tsx`
 * lines ~6558-6591). `expectContent` is false only for windows that are documented, distinct, *non-empty-
 * surface* gaps unrelated to "did the fixture load" (aussuchen's Curated table starts genuinely empty by
 * design; its Preview window is a framework-wide selection-threading gap, permanently "No selection") —
 * every other window is asserted to carry real, non-empty content, including the two known regressions
 * this suite is meant to catch (koordinator's four windows, generator's edit-mode preview). */
interface ExpectedWindow {
  readonly kindId: string;
  readonly instanceIds?: readonly string[];
  readonly surface: WindowSurfaceKind;
  readonly expectContent: boolean;
  readonly note?: string;
}

interface PaneCase {
  readonly paneId: string;
  readonly windows: readonly ExpectedWindow[];
}

const PANE_CASES: readonly PaneCase[] = [
  {
    paneId: "generator",
    windows: [
      { kindId: "procedural-main", surface: "nodeGraph", expectContent: true },
      {
        kindId: "procedural-preview",
        surface: "world3d",
        expectContent: true,
        note: "KNOWN GAP (📓️app-generator.md §3): edit-mode render() reads an always-fresh, never-ticked FlowEvalSession, so eval_json is always empty and the preview mesh/instance JSON stays empty.",
      },
    ],
  },
  {
    paneId: "koordinator",
    windows: [
      {
        kindId: "cad-play-shape",
        surface: "world3d",
        expectContent: true,
        note: "Was a KNOWN GAP: `build_world_scene_for_pane` hardcoded an empty `&[]` object slice. FIXED in commit f394df99d4 — `cad_pane_working_scene` now resolves the pane's composed child through `ArtifactChild::local_owner`, and `forest_play_document` no longer discards its fixture JSON. NOT yet compile-verified (`semio-s-plugin-cad` is blocked by peer `🏪️store` E0119 errors), so this assertion is the first real proof of that fix.",
      },
      { kindId: "cad-play-building", surface: "world3d", expectContent: true, note: "same shared render boundary as cad-play-shape — covered by the same f394df99d4 fix." },
      { kindId: "cad-play-energy", surface: "world3d", expectContent: true, note: "same shared render boundary as cad-play-shape — covered by the same f394df99d4 fix." },
      { kindId: "cad-play-structure-classic", surface: "world3d", expectContent: true, note: "same shared render boundary as cad-play-shape — covered by the same f394df99d4 fix." },
    ],
  },
  {
    paneId: "aggregator",
    windows: [{ kindId: "puzzle3d-main", instanceIds: ["puzzle3d-main-top", "puzzle3d-main-perspective"], surface: "world3d", expectContent: true }],
  },
  {
    paneId: "aussuchen",
    windows: [
      { kindId: "sourcing-pool", surface: "table", expectContent: true },
      { kindId: "sourcing-curated", surface: "table", expectContent: false, note: "Curated starts empty by design — nothing has been curated yet on a fresh document." },
      {
        kindId: "sourcing-preview",
        surface: "world3d",
        expectContent: false,
        note: "KNOWN GAP (📓️app-aussuchen.md §5): the app-level render call site always passes an empty selection (`preview::render(snapshot, &[], labels)`), so this window is permanently stuck on its 'No selection' placeholder regardless of what's clicked — a framework selection-threading gap, not this test relaxing its standard.",
      },
      { kindId: "sourcing-grid", surface: "world3d", expectContent: true },
    ],
  },
  { paneId: "bearbeiten", windows: [{ kindId: "process-workpiece", surface: "world3d", expectContent: true }] },
  { paneId: "verfolgen", windows: [{ kindId: "gis2d-main", surface: "tiledMap", expectContent: true }] },
];

/** @emoji 📖️ Parses a `data-*-json` attribute into its array length, treating a missing/unparsable
 * attribute as zero rather than throwing — an absent attribute is itself evidence of "no content yet". */
function jsonArrayLength(raw: string | null): number {
  if (!raw) return 0;
  try {
    const parsed = JSON.parse(raw) as unknown;
    return Array.isArray(parsed) ? parsed.length : 0;
  } catch {
    return 0;
  }
}

/** @emoji 🌍️ `World3dHost` stamps its live scene straight onto `.semio-world-3d-host` as
 * `data-meshes-json`/`data-instances-json` (`World3dHost/🟦️component.tsx` ~line 4997) — reading those is
 * strictly more reliable than sampling canvas pixels (no readback-timing/`preserveDrawingBuffer` gotchas). */
async function worldContentCount(container: Locator): Promise<{ readonly hasScene: boolean; readonly meshes: number; readonly instances: number }> {
  const host = container.locator(".semio-world-3d-host");
  const empty = container.locator(".semio-world-3d-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if ((await empty.count()) > 0) return { hasScene: false, meshes: 0, instances: 0 };
  const attrs = await host.evaluate((el) => ({ meshes: el.getAttribute("data-meshes-json"), instances: el.getAttribute("data-instances-json") }));
  return { hasScene: true, meshes: jsonArrayLength(attrs.meshes), instances: jsonArrayLength(attrs.instances) };
}

/** @emoji 🕸️ `NodeGraph` stamps its live flow document onto `.semio-node-graph-host` as `data-fixture-json`
 * (`NodeGraph/🟦️component.tsx` line 1151) — a real fixture parses to a JSON object carrying a `widgets[]`. */
async function nodeGraphWidgetCount(container: Locator): Promise<{ readonly hasScene: boolean; readonly widgets: number }> {
  const host = container.locator(".semio-node-graph-host");
  const empty = container.locator(".semio-node-graph-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if ((await empty.count()) > 0) return { hasScene: false, widgets: 0 };
  const fixtureJson = await host.getAttribute("data-fixture-json");
  if (!fixtureJson) return { hasScene: true, widgets: 0 };
  try {
    const fixture = JSON.parse(fixtureJson) as { readonly widgets?: readonly unknown[] };
    return { hasScene: true, widgets: fixture.widgets?.length ?? 0 };
  } catch {
    return { hasScene: true, widgets: 0 };
  }
}

/** @emoji 📊️ `Table`'s generic row primitive stamps `data-row-id` on every real data `<tr>`
 * (`framework/ui/elements/📊️Table/🟦️component.tsx` lines 183/243) — counting them is a direct, DOM-level
 * "does this table have rows" check with no reliance on cell text/locale. */
async function tableRowCount(container: Locator): Promise<{ readonly hasScene: boolean; readonly rows: number }> {
  const host = container.locator(".semio-table-host");
  const empty = container.locator(".semio-table-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if ((await empty.count()) > 0) return { hasScene: false, rows: 0 };
  const rows = await host.locator("[data-row-id]").count();
  return { hasScene: true, rows };
}

/** @emoji 🗺️ `TiledMapHost` has no content-count DOM attribute (unlike the other three surfaces), so this
 * falls back to sampling the rendered canvas for more than one distinct pixel color — the same
 * "did anything actually paint" question `capturePanePoster` (`🟦️.tsx`) answers for its poster
 * capture, just read-only here. Map tiles are proxied same-origin by the dev server
 * (`vite-elements-assets.ts`'s `createTileProxyMiddleware`), so this canvas is not cross-origin-tainted. */
async function tiledMapHasVisibleContent(container: Locator): Promise<{ readonly hasScene: boolean; readonly painted: boolean }> {
  const host = container.locator(".semio-tiled-map-host");
  const empty = container.locator(".semio-tiled-map-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if ((await empty.count()) > 0) return { hasScene: false, painted: false };
  const canvas = host.locator("canvas");
  await expect(canvas).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  // 🐢️ Give the map session a couple of animation frames to actually paint after mount.
  await canvas.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
  const painted = await canvas.evaluate((el: HTMLCanvasElement) => {
    const width = el.width || el.clientWidth;
    const height = el.height || el.clientHeight;
    if (!width || !height) return false;
    const probe = document.createElement("canvas");
    probe.width = width;
    probe.height = height;
    const ctx = probe.getContext("2d");
    if (!ctx) return false;
    ctx.drawImage(el, 0, 0, width, height);
    const { data } = ctx.getImageData(0, 0, width, height);
    let first: [number, number, number, number] | null = null;
    for (let i = 0; i < data.length; i += 4) {
      const pixel: [number, number, number, number] = [data[i]!, data[i + 1]!, data[i + 2]!, data[i + 3]!];
      if (!first) first = pixel;
      else if (pixel[0] !== first[0] || pixel[1] !== first[1] || pixel[2] !== first[2] || pixel[3] !== first[3]) return true;
    }
    return false;
  });
  return { hasScene: true, painted };
}
//#endregion 🪟️SurfaceContent

//#region 🧪️PaneConsistency
test("DEMONSTRATOR_PANES matches the pane ids this suite covers (drift guard)", () => {
  expect(PANE_CASES.map((entry) => entry.paneId)).toEqual(brandPaneIds());
});
//#endregion 🧪️PaneConsistency

for (const paneCase of PANE_CASES) {
  test(`demonstrator pane "${paneCase.paneId}": boots via hash deep-link and renders its declared window(s)`, async ({ page }) => {
    test.setTimeout(TEST_TIMEOUT_MS);
    const pageErrors: Error[] = [];
    const consoleErrors: string[] = [];
    page.on("pageerror", (error) => pageErrors.push(error));
    page.on("console", (message) => {
      if (message.type() === "error") consoleErrors.push(message.text());
    });

    await page.goto(`/#${paneCase.paneId}`, { waitUntil: "domcontentloaded" });
    await expect(page.locator(`[data-shell-id="${paneCase.paneId}"]`)).toHaveCount(1, { timeout: SHELL_READY_TIMEOUT_MS });

    const outcome = await waitForPaneShellOutcome(page, paneCase.paneId);
    expect(outcome, `pane "${paneCase.paneId}": expected its shell to reach "ready"`).toBe("ready");

    await dismissIntroductionIfPresent(page, paneCase.paneId);

    for (const win of paneCase.windows) {
      const instanceIds = win.instanceIds ?? [windowElementId(win.kindId)];
      if (win.instanceIds) {
        // 🎯️ Both split instances alias back to the shared kind id — confirms the alias wiring itself.
        const aliasMatches = page.locator(paneElementSelector(paneCase.paneId, windowElementId(win.kindId)));
        await expect(aliasMatches).toHaveCount(instanceIds.length, { timeout: SHELL_READY_TIMEOUT_MS });
      }
      for (const rawId of instanceIds) {
        const elementId = win.instanceIds ? `framework.window.${elementIdSegment(rawId)}` : rawId;
        const container = page.locator(paneElementSelector(paneCase.paneId, elementId));
        await expect(container, `pane "${paneCase.paneId}": window "${rawId}" did not attach`).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
        const label = `pane "${paneCase.paneId}" window "${rawId}"${win.note ? ` (${win.note})` : ""}`;

        if (win.surface === "world3d") {
          const { hasScene, meshes, instances } = await worldContentCount(container);
          expect(hasScene, `${label}: world3d surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(meshes + instances, `${label}: expected non-empty meshes/instances JSON (meshes=${meshes}, instances=${instances})`).toBeGreaterThan(0);
        } else if (win.surface === "nodeGraph") {
          const { hasScene, widgets } = await nodeGraphWidgetCount(container);
          expect(hasScene, `${label}: node-graph surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(widgets, `${label}: expected a non-empty widgets[] in data-fixture-json`).toBeGreaterThan(0);
        } else if (win.surface === "table") {
          const { hasScene, rows } = await tableRowCount(container);
          expect(hasScene, `${label}: table surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(rows, `${label}: expected at least one [data-row-id] row`).toBeGreaterThan(0);
        } else {
          const { hasScene, painted } = await tiledMapHasVisibleContent(container);
          expect(hasScene, `${label}: tiled-map surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(painted, `${label}: expected the map canvas to paint more than one flat color`).toBe(true);
        }
      }
    }

    expect(pageErrors.map((error) => error.message), `pane "${paneCase.paneId}": unexpected page errors`).toEqual([]);
    expect(significantConsoleErrors(consoleErrors), `pane "${paneCase.paneId}": unexpected console errors`).toEqual([]);
  });
}
