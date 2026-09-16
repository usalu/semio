// #region 🧲️Header
// 💻️ ♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts
// Specs: End-to-end acceptance coverage for the "Entwerfen mit Bestand" demonstrator's six live panes.
// Summary: For every `DEMONSTRATOR_PANES` entry, deep-links straight to `/#<paneId>` (the fast path that
// boots exactly that one pane immediately instead of waiting through the 1.5s/35s sequential-boot queue —
// see `🟦️.tsx`'s `paneIdFromLocationHash`/`useSequentialPaneBoot`), waits for that pane's own
// `FrameworkOsShell` to report readiness via the per-shell `data-shell-ready`/`data-shell-error` beacon
// (`ShellHost/🟦️.tsx`'s `#region 🔖️ReadinessBeacon`), asserts its declared window(s) attach, and
// asserts each window actually carries rendered content (not an empty surface, not the "wird vorbereitet"
// `CanvasSkeleton`) by reading the same production `data-*-json`/`data-row-id` attributes each surface host
// already stamps on itself (`World3dHost`'s `data-meshes-json`/`data-instances-json`, `NodeGraph`'s
// `data-host-snapshot-json`, `Table`'s `data-row-id` rows) — no test-only instrumentation was added for
// this. `TiledMapHost` is the one surface with no such attribute, so it is graded from a composited
// element screenshot instead (see `tiledMapHasVisibleContent` for why a canvas readback cannot work).
// Known-defect windows are asserted exactly like every other window rather than being weakened or
// skipped, so a real defect fails loudly. Generator's edit-mode preview used to be expected to fail (it
// never got a synced flow-eval session — `📓️app-generator.md` §3); the guest half of that is fixed and
// proven natively (`📓️fix-2026-09-16-generator-eval-session.md`), so it now grades the served path only.
// Koordinator's four CAD windows were
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
/** @emoji 🪪️ Reads `🪧️brand.ts`'s pane ids as TEXT rather than importing it. Playwright loads specs
 * through Node's own ESM loader, and importing the brand module drags in the whole `@semio-tech/ui-react`
 * runtime — whose typed-scene catalog is a bare `.json` import that Node rejects without an
 * `import ... with { type: "json" }` attribute (`ERR_IMPORT_ATTRIBUTE_MISSING`). Vite rewrites that for the
 * app; Playwright does not, so this suite stays import-free of app modules exactly like `.storybook`'s specs. */
function brandPaneIds(): readonly string[] {
  const source = readFileSync(join(import.meta.dirname, "..", "..", "🪧️brand.ts"), "utf8");
  const block = /export const DEMONSTRATOR_PANES[^=]*=\s*\[(.*?)\n\];/s.exec(source);
  if (!block) throw new Error("🪧️brand.ts no longer declares a DEMONSTRATOR_PANES array literal");
  return [...block[1]!.matchAll(/\bid:\s*"([^"]+)"/g)].map((match) => match[1]!);
}
//#endregion 🪪️BrandPaneIds

/** @emoji ⏱️ Cold WASM plugin boots on a fresh page load can be slow (memory: "expect slow cold boots, be
 * patient") — generous on purpose so this suite reports real content defects, not infra flakiness. */
const SHELL_READY_TIMEOUT_MS = 120_000;
const TEST_TIMEOUT_MS = 240_000;

/** @emoji ⏳️ How long a window that IS expected to carry content may take to publish its first non-empty
 * frame after its surface host attaches — the shell reports "ready" as soon as the plugin's UI tree is
 * mounted, which is strictly before the guest's first scene/evaluation crosses the wire. */
const SURFACE_CONTENT_TIMEOUT_MS = 60_000;

/** 🔇️ Drops resource 404s and the repo's `[DEBUG] `-prefixed temporary diagnostics, which hosts emit at error level while a seam is being instrumented. */
function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b40[0-9]\b/i.test(text) && !text.startsWith("[DEBUG] "));
}

//#region 🆔️ElementId
/** @emoji 🆔️ Local mirror of `framework/ui/elements/🆔️ElementId/🟦️.tsx`'s `elementIdSegment` — kept
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
 * one ready" — see `ShellHost/🟦️.tsx`'s `#region 🔖️ReadinessBeacon`). */
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

/** @emoji 👋️ Dismisses an introduction overlay, by pointer first and by its own documented keyboard
 * parity route second.
 *
 * `ui.introduction.skip` anchors itself to the surface the current tour step highlights
 * (`registerIntroductionSurfaceResolver`), so while a cold page is still booting shells the box
 * re-anchors and the button's box moves — Playwright's actionability "stable" gate then never settles
 * and a bare `click()` times out mid-boot even though the button is visible and enabled (measured
 * 2026-09-16: on a warm server the box holds for 300 straight frames and the click lands in ~230 ms;
 * on the cold acceptance server run 1 the same click never became stable inside 5 s). So: one generous
 * click, then `escape` — the keybinding this very control declares
 * (`🕹️control-keybinding-context/🟦️.tsx`'s `"ui.introduction.skip": "escape"`), which needs no
 * actionability at all — and finally proof that the overlay really went away. */
async function dismissIntroduction(page: Page, skip: Locator): Promise<void> {
  if (!(await skip.first().isVisible({ timeout: 3_000 }).catch(() => false))) return;
  const clicked = await skip
    .first()
    .click({ timeout: 20_000 })
    .then(() => true)
    .catch(() => false);
  if (!clicked) await page.keyboard.press("Escape");
  await expect(skip, "the introduction overlay must be dismissable").toHaveCount(0, { timeout: 15_000 });
}

/** @emoji 👋️ Every brand replays its own app-level introduction on load once focused
 * (`suppressAutoIntroduction={!focused}` is false for a hash-deep-linked, already-focused pane) — dismiss
 * it so it never shadows a later interaction. Absence is not an error (already dismissed, or this brand
 * has none left to show). */
async function dismissIntroductionIfPresent(page: Page, paneId: string): Promise<void> {
  await dismissIntroduction(page, page.locator(paneElementSelector(paneId, "ui.introduction.skip")));
}
//#endregion 🚦️ShellReadiness

//#region 🪟️SurfaceContent
type WindowSurfaceKind = "world3d" | "nodeGraph" | "table" | "tiledMap" | "placeholder";

/** @emoji 🪟️ One window this pane is expected to open by default (per-app fixture/window research,
 * `.🧬semio/…/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️app-*.md`). `instanceIds` covers a window kind opened as
 * several simultaneous instances (aggregator's split top/perspective puzzle3d-main views) — each instance
 * carries its OWN `id` plus a shared `data-element-alias` back to the kind id (`ShellHost/🟦️.tsx`
 * lines ~6687-6717). `expectContent` is false only for windows that are documented, distinct, *non-empty-
 * surface* gaps unrelated to "did the fixture load" (aussuchen's Curated table starts genuinely empty by
 * design) — every other window is asserted to carry real, non-empty content, including the two known
 * regressions this suite is meant to catch (koordinator's four windows, generator's edit-mode preview).
 * `surface: "placeholder"` is for a window whose *declared* surface kind is a scene host but whose body
 * on a fresh document is deliberately a plain text node instead (aussuchen's Preview renders
 * `built_text_node(labels.no_selection)` until something is selected) — that window is graded against
 * `placeholderPattern`, and the scene it is supposed to grow is graded by its own interaction test. */
interface ExpectedWindow {
  readonly kindId: string;
  readonly instanceIds?: readonly string[];
  readonly surface: WindowSurfaceKind;
  readonly expectContent: boolean;
  readonly placeholderPattern?: RegExp;
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
        note: "Was a KNOWN GAP (📓️app-generator.md §3): edit-mode render() built a fresh, never-ticked FlowEvalSession per call. The session is retained per app instance now and the evaluation is the progress/cancel-capable `previewEval` tool run; the guest half is proven natively by `the_demonstrator_boot_example_renders_a_non_empty_preview_scene` (generation3d edit preview window unit tests, 📓️fix-2026-09-16-generator-eval-session.md). This assertion therefore grades the SERVED path only.",
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
        surface: "placeholder",
        expectContent: false,
        placeholderPattern: /Keine Auswahl|No selection/,
        note: "Declared `SurfaceKind::World3d`, but on a fresh document nothing is selected and `👁️preview/🦀️.rs`'s `render` deliberately returns `built_text_node(labels.no_selection)` — a text body, so there is no `.semio-world-3d-host`/`.semio-world-3d-empty` to grade at all. The old `📓️app-aussuchen.md §5` gap (`preview::render(snapshot, &[], labels)` hardcoded at the interaction-less delegate) is CLOSED: `✏️editor/🦀️.rs:1091` now feeds it `interaction.selection(SOURCING_ROWS_DOMAIN)`, and the scene that produces is graded by the aussuchen selection test below.",
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

/** @emoji ⏳️ A surface host attaches EMPTY and is filled by the first frame the guest publishes across
 * the wire, so a single sample right after the host becomes visible grades the boot gap, not the app: on
 * the 2026-09-16 serve generator's preview read `meshes=0` at 2.8 s and a 3689-byte `data-meshes-json`
 * once its `previewEval` run landed. This re-reads until the count is non-zero — it only ever waits on
 * the windows that are *expected* to carry content, so a genuinely empty surface still fails, just
 * `SURFACE_CONTENT_TIMEOUT_MS` later instead of instantly. */
async function settleContentCount(read: () => Promise<number>, requireContent: boolean, minimum = 1): Promise<number> {
  let value = await read();
  const deadline = Date.now() + SURFACE_CONTENT_TIMEOUT_MS;
  while (requireContent && value < minimum && Date.now() < deadline) {
    await new Promise((resolve) => setTimeout(resolve, 500));
    value = await read();
  }
  return value;
}

/** @emoji 🌍️ `World3dHost` stamps its live scene straight onto `.semio-world-3d-host` as
 * `data-meshes-json`/`data-instances-json` (`World3dHost/🟦️.tsx` ~line 5063) — reading those is
 * strictly more reliable than sampling canvas pixels (no readback-timing/`preserveDrawingBuffer` gotchas). */
async function worldContentCount(container: Locator, requireContent = false): Promise<{ readonly hasScene: boolean; readonly meshes: number; readonly instances: number }> {
  const host = container.locator(".semio-world-3d-host");
  const empty = container.locator(".semio-world-3d-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if (requireContent) await host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(() => {});
  if ((await host.count()) === 0) return { hasScene: false, meshes: 0, instances: 0 };
  const readAttrs = async (): Promise<{ readonly meshes: number; readonly instances: number }> => {
    const attrs = await host.evaluate((el) => ({ meshes: el.getAttribute("data-meshes-json"), instances: el.getAttribute("data-instances-json") }));
    return { meshes: jsonArrayLength(attrs.meshes), instances: jsonArrayLength(attrs.instances) };
  };
  let counts = await readAttrs();
  await settleContentCount(async () => {
    counts = await readAttrs();
    return counts.meshes + counts.instances;
  }, requireContent);
  return { hasScene: true, ...counts };
}

/** @emoji 🕸️ `NodeGraph` stamps its live flow document onto `.semio-node-graph-host` as
 * `data-host-snapshot-json` (`NodeGraph/🟦️.tsx`'s `NodeGraphHost` render) — a real snapshot parses to a
 * JSON object carrying a `widgets[]`. That attribute was called `data-fixture-json` until commit
 * 8773331d23 ("Align plugin schemas, mutations, panels, and tests to snapshot-fixture-asset
 * terminology") renamed the scene field `fixtureJson` → `hostSnapshotJson`; the graph engine is a wasm
 * canvas with no per-node DOM, so this attribute is the only DOM-level evidence of graph content there
 * is. Measured on the 6029 serve 2026-09-16: generator's `proceduralMain` carries a 1867-byte snapshot
 * with 7 widgets and its `synapses[]` wiring. */
async function nodeGraphWidgetCount(container: Locator, requireContent = false): Promise<{ readonly hasScene: boolean; readonly widgets: number }> {
  const host = container.locator(".semio-node-graph-host");
  const empty = container.locator(".semio-node-graph-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if (requireContent) await host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(() => {});
  if ((await host.count()) === 0) return { hasScene: false, widgets: 0 };
  const widgets = await settleContentCount(async () => {
    const snapshotJson = await host.getAttribute("data-host-snapshot-json");
    if (!snapshotJson) return 0;
    try {
      return (JSON.parse(snapshotJson) as { readonly widgets?: readonly unknown[] }).widgets?.length ?? 0;
    } catch {
      return 0;
    }
  }, requireContent);
  return { hasScene: true, widgets };
}

/** @emoji 📊️ `Table`'s generic row primitive stamps `data-row-id` on every real data `<tr>`
 * (`framework/ui/elements/📊️Table/🟦️.tsx` lines 201/262) — counting them is a direct, DOM-level
 * "does this table have rows" check with no reliance on cell text/locale. */
async function tableRowCount(container: Locator, requireContent = false): Promise<{ readonly hasScene: boolean; readonly rows: number }> {
  const host = container.locator(".semio-table-host");
  const empty = container.locator(".semio-table-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if (requireContent) await host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(() => {});
  if ((await host.count()) === 0) return { hasScene: false, rows: 0 };
  const rows = await settleContentCount(() => host.locator("[data-row-id]").count(), requireContent);
  return { hasScene: true, rows };
}

/** @emoji 🗺️ `TiledMapHost` has no content-count DOM attribute (unlike the other three surfaces), so
 * "did the map paint" has to be answered from pixels — but NOT by reading the live canvas back.
 *
 * The map is presented by the wasm/wgpu surface session, whose swap-chain is not a preserved drawing
 * buffer: `drawImage(mapCanvas, …)` + `getImageData` answers a fully transparent `0,0,0,0` image even
 * when the map is visibly drawn (measured 2026-09-16 against the 6029 serve, whose map paints
 * continents, labels and the marker — the readback was flat-transparent on both the GPU and the
 * software stack). Playwright's own element screenshot is a composited grab, so it sees what the user
 * sees; it is decoded back into an `ImageData` in the page, and only the CENTRE box is counted so the
 * window's own chrome/veil at the edges can never stand in for map content.
 *
 * Map tiles are proxied same-origin by the dev server (`framework/ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`'s
 * `createTileProxyMiddleware`, routes `/osm` + `/vt` off `.🧬semio/🗺️map`), so nothing here is
 * cross-origin-tainted and no network egress is needed. */
async function tiledMapHasVisibleContent(page: Page, container: Locator, requireContent = false): Promise<{ readonly hasScene: boolean; readonly painted: boolean; readonly distinctColors: number }> {
  const host = container.locator(".semio-tiled-map-host");
  const empty = container.locator(".semio-tiled-map-empty");
  await expect(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  if (requireContent) await host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(() => {});
  if ((await host.count()) === 0) return { hasScene: false, painted: false, distinctColors: 0 };
  const canvas = host.locator("canvas");
  await expect(canvas).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  // 🐢️ Give the map session a couple of animation frames to actually paint after mount.
  await canvas.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
  // 🎨️ A flat grab still counts ONE colour, so "painted" needs a floor of two — not merely non-zero.
  const distinctColors = await settleContentCount(() => countCanvasColors(page, canvas), requireContent, 2);
  return { hasScene: true, painted: distinctColors > 1, distinctColors };
}

/** @emoji 🎨️ Distinct RGBA values in the centre 60% of one element's composited screenshot; `1` means
 * one flat colour and `0` means the grab could not be decoded. */
async function countCanvasColors(page: Page, canvas: Locator): Promise<number> {
  const shot = (await canvas.screenshot()).toString("base64");
  const distinct = await page.evaluate(async (base64) => {
    const binary = atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
    const bitmap = await createImageBitmap(new Blob([bytes], { type: "image/png" }));
    const probe = document.createElement("canvas");
    probe.width = bitmap.width;
    probe.height = bitmap.height;
    const ctx = probe.getContext("2d");
    if (!ctx) return 0;
    ctx.drawImage(bitmap, 0, 0);
    const x0 = Math.floor(bitmap.width * 0.2);
    const y0 = Math.floor(bitmap.height * 0.2);
    const w = Math.max(1, Math.floor(bitmap.width * 0.6));
    const h = Math.max(1, Math.floor(bitmap.height * 0.6));
    const { data } = ctx.getImageData(x0, y0, w, h);
    const seen = new Set<number>();
    for (let i = 0; i < data.length; i += 4) seen.add((data[i]! << 24) | (data[i + 1]! << 16) | (data[i + 2]! << 8) | data[i + 3]!);
    return seen.size;
  }, shot);
  return distinct;
}
//#endregion 🪟️SurfaceContent

//#region 🧪️PaneConsistency
test("DEMONSTRATOR_PANES matches the pane ids this suite covers (drift guard)", () => {
  expect(PANE_CASES.map((entry) => entry.paneId)).toEqual(brandPaneIds());
});
//#endregion 🧪️PaneConsistency

//#region 🃏️OverviewCards
/** @emoji 🃏️ Landing overview cards use window silhouettes (icon title chips, no drag handles). */
test("demonstrator overview: pane cards use window-silhouette chrome without drag handles", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto("/", { waitUntil: "domcontentloaded" });

  await dismissIntroduction(page, page.locator('[id="ui.introduction.skip"]'));

  const cards = page.locator("[data-demonstrator-pane-card]");
  await expect(cards).toHaveCount(brandPaneIds().length, { timeout: SHELL_READY_TIMEOUT_MS });

  for (const paneId of brandPaneIds()) {
    const card = page.locator(`[data-demonstrator-pane-card][data-pane-id="${paneId}"]`);
    await expect(card).toBeVisible();
    await expect(card.locator("[data-window-silhouette]")).toHaveCount(1);
    await expect(card.locator('[data-slot="demonstrator-pane-card-title-chip"] svg')).toHaveCount(1);
    await expect(card.locator('[data-slot*="drag"], [data-drag-handle]')).toHaveCount(0);
  }

  expect(pageErrors.map((error) => error.message), "unexpected page errors on overview").toEqual([]);
  expect(significantConsoleErrors(consoleErrors), "unexpected console errors on overview").toEqual([]);
});
//#endregion 🃏️OverviewCards

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
          const { hasScene, meshes, instances } = await worldContentCount(container, win.expectContent);
          expect(hasScene, `${label}: world3d surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(meshes + instances, `${label}: expected non-empty meshes/instances JSON (meshes=${meshes}, instances=${instances})`).toBeGreaterThan(0);
        } else if (win.surface === "nodeGraph") {
          const { hasScene, widgets } = await nodeGraphWidgetCount(container, win.expectContent);
          expect(hasScene, `${label}: node-graph surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(widgets, `${label}: expected a non-empty widgets[] in data-host-snapshot-json`).toBeGreaterThan(0);
        } else if (win.surface === "table") {
          const { hasScene, rows } = await tableRowCount(container, win.expectContent);
          expect(hasScene, `${label}: table surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(rows, `${label}: expected at least one [data-row-id] row`).toBeGreaterThan(0);
        } else if (win.surface === "placeholder") {
          await expect(container, `${label}: expected its documented text placeholder`).toHaveText(win.placeholderPattern!, { timeout: SHELL_READY_TIMEOUT_MS });
        } else {
          const { hasScene, painted, distinctColors } = await tiledMapHasVisibleContent(page, container, win.expectContent);
          expect(hasScene, `${label}: tiled-map surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(painted, `${label}: expected the map canvas to paint more than one flat color (distinct colors=${distinctColors})`).toBe(true);
        }
      }
    }

    expect(pageErrors.map((error) => error.message), `pane "${paneCase.paneId}": unexpected page errors`).toEqual([]);
    expect(significantConsoleErrors(consoleErrors), `pane "${paneCase.paneId}": unexpected console errors`).toEqual([]);
  });
}

//#region 🎯️AussuchenSelection
/** @emoji 🎯️ The other half of aussuchen's Preview window: picking a stock row must turn the
 * `built_text_node(labels.no_selection)` placeholder into a real World3d scene.
 *
 * This is the served proof of the interaction-view threading wave
 * (`📓️fix-2026-09-16-interaction-view-threading.md`): `✏️editor/🦀️.rs:1091` feeds
 * `preview::render(doc.snapshot, &selected_ids, …)` from `interaction.selection(SOURCING_ROWS_DOMAIN)`,
 * so "what the rows domain has selected" has to travel guest→host→surface for this to pass. Measured
 * on the 6029 serve 2026-09-16: before the click the window reads `Keine Auswahl` with no world host;
 * after it, `.semio-world-3d-host` carries an 804-byte `data-meshes-json`. */
test('demonstrator pane "aussuchen": selecting a stock row renders the preview scene', async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  await page.goto("/#aussuchen", { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, "aussuchen"), 'pane "aussuchen": expected its shell to reach "ready"').toBe("ready");
  await dismissIntroductionIfPresent(page, "aussuchen");

  const preview = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-preview")));
  await expect(preview, "aussuchen preview starts on its documented no-selection placeholder").toHaveText(/Keine Auswahl|No selection/, { timeout: SHELL_READY_TIMEOUT_MS });

  const rows = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-pool"))).locator(".semio-table-host [data-row-id]");
  await expect(rows.first(), "aussuchen stock pool must offer a row to select").toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  await rows.first().click({ timeout: 30_000 });

  const { hasScene, meshes, instances } = await worldContentCount(preview, true);
  expect(hasScene, "aussuchen preview: selecting a row must resolve it to a world3d surface").toBe(true);
  expect(meshes + instances, `aussuchen preview: expected the selected kind's geometry (meshes=${meshes}, instances=${instances})`).toBeGreaterThan(0);
});
//#endregion 🎯️AussuchenSelection

test("landing Förderhinweis renders non-empty funding logos", async ({ page }) => {
  test.setTimeout(60_000);
  await page.goto("/", { waitUntil: "networkidle" });
  const next = page.locator('[data-slot="introduction-info-box"] button').filter({ hasText: /Weiter/i });
  for (let step = 0; step < 2; step += 1) {
    await next.click({ timeout: 15_000 });
    await page.waitForTimeout(400);
  }
  await expect(page.locator('[data-slot="introduction-info-box-title"]')).toHaveText("Förderhinweis");
  const visibleLogoSizes = await page.locator('[data-slot="introduction-info-box"] img').evaluateAll((imgs) =>
    imgs
      .filter((img) => getComputedStyle(img).display !== "none")
      .map((img) => ({ width: img.clientWidth, height: img.clientHeight, naturalWidth: (img as HTMLImageElement).naturalWidth })),
  );
  expect(visibleLogoSizes.length).toBeGreaterThanOrEqual(3);
  for (const logo of visibleLogoSizes) {
    expect(logo.width, "introduction funding logo should layout with width").toBeGreaterThan(0);
    expect(logo.height, "introduction funding logo should layout with height").toBeGreaterThan(0);
    expect(logo.naturalWidth, "introduction funding logo should decode as an image").toBeGreaterThan(0);
  }
});
