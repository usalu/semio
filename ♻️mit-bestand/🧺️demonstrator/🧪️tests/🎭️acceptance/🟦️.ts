// #region 🧲️Header
// 🎭️ Served acceptance for the eight demonstrator panes and their authored interactions.
// Every browser case collects page and console errors through the automatic health fixture.
// Production scene attributes expose geometry and table results; mutations use visible controls.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { expect, test as browserTest, type Locator, type Page } from "@playwright/test";
import { significantConsoleErrors } from "./🧬️console/🟦️.ts";
import { assertDemonstratorE2eArtifactPath } from "../../🔨️modules/🧪️e2e/📁️artifacts/🟦️.ts";
import artifactPolicy from "../../🔨️modules/🧪️e2e/📁️artifacts/🔣️.json" with { type: "json" };
// #endregion 🔌️Adapters

//#region 🪪️BrandPaneIds
/** 🪪️ Reads `🪧️brand.ts`'s pane ids as TEXT rather than importing it. Playwright loads specs
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

/** ⏱️ Cold WASM plugin boots on a fresh page load can be slow (memory: "expect slow cold boots, be
 * patient") — generous on purpose so this suite reports real content defects, not infra flakiness. */
const SHELL_READY_TIMEOUT_MS = 120_000;
const TEST_TIMEOUT_MS = 240_000;

/** ⏳️ How long a window that IS expected to carry content may take to publish its first non-empty
 * frame after its surface host attaches — the shell reports "ready" as soon as the plugin's UI tree is
 * mounted, which is strictly before the guest's first scene/evaluation crosses the wire. */
const SURFACE_CONTENT_TIMEOUT_MS = 60_000;

/** 🩺️ Enforces the same error policy for every browser case, including interaction and funding. */
const test = browserTest.extend<{ browserHealth: void }>({
  browserHealth: [async ({ page }, use, testInfo) => {
    expect(testInfo.title.length, "bounded diagnostic title").toBeLessThanOrEqual(artifactPolicy.maxTitleChars);
    for (const name of ["trace.zip", "error-context.md", "p.png"]) {
      assertDemonstratorE2eArtifactPath(join(testInfo.outputDir, name));
      assertDemonstratorE2eArtifactPath(testInfo.outputPath(name));
    }
    const pageErrors: string[] = [];
    const consoleErrors: string[] = [];
    page.on("pageerror", (error) => pageErrors.push(error.message));
    page.on("console", (message) => {
      if (message.type() === "error") consoleErrors.push(message.text());
    });
    try {
      await use();
    } finally {
      expect.soft(pageErrors, "unexpected browser page errors").toEqual([]);
      expect.soft(significantConsoleErrors(consoleErrors), "unexpected browser console errors").toEqual([]);
    }
  }, { auto: true }],
});

/** 📸️ Retains a short PNG and attaches its bytes without an extra copied attachment path. */
async function capturePaneArtifact(page: Page, paneId: string, name = "p.png"): Promise<void> {
  if (!/^[a-z][a-z0-9-]{0,15}\.png$/.test(name)) throw new Error("Use a short semantic PNG artifact name");
  assertDemonstratorE2eArtifactPath(join(test.info().outputDir, name));
  const body = await page.locator(`[data-shell-id="${paneId}"]`).screenshot({ path: test.info().outputPath(name), animations: "disabled" });
  await test.info().attach(`${paneId}-${name.slice(0, -4)}`, { contentType: "image/png", body });
}

//#region 🆔️ElementId
/** 🆔️ Local mirror of `framework/ui/elements/🆔️ElementId/🟦️.tsx`'s `elementIdSegment` — kept
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

/** 🪟️ Mirrors `framework/platform`'s `windowElementId(kindId)` → `"framework.window.<camelKindId>"`. */
function windowElementId(kindId: string): string {
  return `framework.window.${elementIdSegment(kindId)}`;
}

/** 🎯️ CSS selector for the element carrying `id` as either its real DOM id or a `data-element-alias`
 * token, scoped to one pane's shell root — mirrors `elementIdSelector`, scoped by `[data-shell-id]`. */
function paneElementSelector(paneId: string, elementId: string): string {
  return `[data-shell-id="${paneId}"] [id="${elementId}"], [data-shell-id="${paneId}"] [data-element-alias~="${elementId}"]`;
}
//#endregion 🆔️ElementId

//#region 🚦️ShellReadiness
type ShellOutcome = "ready" | "error" | "notFound";

/** 🚦️ Waits for the pane's own `[data-shell-id]` root to report an outcome via the per-shell
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

/** 👋️ Dismisses an introduction overlay, by pointer first and by its own documented keyboard
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

/** 👋️ Every brand replays its own app-level introduction on load once focused
 * (`suppressAutoIntroduction={!focused}` is false for a hash-deep-linked, already-focused pane) — dismiss
 * it so it never shadows a later interaction. Absence is not an error (already dismissed, or this brand
 * has none left to show). */
async function dismissIntroductionIfPresent(page: Page, paneId: string): Promise<void> {
  await dismissIntroduction(page, page.locator(paneElementSelector(paneId, "ui.introduction.skip")));
}
//#endregion 🚦️ShellReadiness

//#region 🪟️SurfaceContent
type WindowSurfaceKind = "world3d" | "nodeGraph" | "table" | "tree" | "tiledMap" | "placeholder";

/** 🪟️ One window this pane is expected to open by default (per-app fixture/window research,
 * `.🧬semio/…/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️app-*.md`; the energie/statik entries were read off the
 * two standalone lanes instead, see `📓️fix-2026-09-17-energie-statik-acceptance-cases.md`).
 * `instanceIds` covers a window kind opened as
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
        note: "Was a KNOWN GAP: `build_world_scene_for_pane` hardcoded an empty `&[]` object slice. FIXED in commit f394df99d4 — `cad_pane_working_scene` now resolves the pane's composed child through `ArtifactChild::local_owner`, and `forest_play_document` no longer discards its fixture JSON. NOT yet compile-verified (`semio-hub-cad` is blocked by peer `🏪️store` E0119 errors), so this assertion is the first real proof of that fix.",
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
    paneId: "energie",
    windows: [
      {
        kindId: "energy.model.3d",
        surface: "world3d",
        expectContent: true,
        note: "`s.energy.model@1/*#editor`'s Model viewport — `✏️editor/🎭️modes/✏️edit/🦀️.rs`'s `layout()` gives it `MODEL_VIEWPORT_SHARE` (0.55) of the edit-mode row, with Structure/Zones/Energy simulation stacked in the remaining column. Measured 2026-09-17 on the live demonstrator serve: `/#energie` reached `ready` in 11.5 s and all four windows carried content by 15.3 s, with no page or console errors; this window read 9 meshes / 9 instances. The brand's `defaults.exampleId` is `bestest-600`, whose `artifactJson` is byte-identical to the plugin's `demo` example, so the standalone energy lane (:6106) grades the same document.",
      },
      {
        kindId: "framework.window.tree",
        surface: "tree",
        expectContent: true,
        note: "Structure. The energy editor reuses the framework's generic `TreeWindowKit` kind id verbatim, so the window element is `framework.window.frameworkWindowTree` (`elementIdSegment` folds the dots) — the pane-scoped selector keeps that generic id from colliding with any other shell on the page. The body is a plain framework `Tree`, NOT a surface host, so there is no `.semio-*-host` to grade; measured on :6106 it carries 46 `role=\"treeitem\"` rows headed `BESTEST 600 (vashrae-140-5.2)`. NOTE: the committed `✏️s/🔌️plugins/🔋️energy/🔣️.json` still declares this kind as `surfaceKind: \"block-list\"` and omits `energy.model.3d` entirely — that descriptor is stale relative to the editor source and the served build; the live DOM is what this expectation is written from.",
      },
      {
        kindId: "framework.window.table",
        surface: "table",
        expectContent: true,
        note: "Zones — the generic `TableWindowKit` kind id, so the element is `framework.window.frameworkWindowTable`. BESTEST 600 is a single-zone model, so exactly one `[data-row-id]` row (`id/name/volumeM3/multiplier/conditioned/partOfTotalFloorArea` → `1 / Zone / 129.6 / 1 / true / true`) is the correct, fully-loaded content — the assertion floor of one row is deliberately not raised.",
      },
      {
        kindId: "energy.simulation",
        surface: "tree",
        expectContent: true,
        note: "Energy simulation. Also a framework `Tree` body (run transport, live region, editable run settings, keybinding list) rather than a surface host: 15 `role=\"treeitem\"` rows on a fresh document, headed `Kein Energiesimulationslauf` under this brand's locked German locale. This grades that the simulation window's projection rendered at all — it deliberately does NOT require a completed run, which is a user-started tool run (`toolRun*` on `energySimulation`), not part of boot.",
      },
    ],
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
      {
        kindId: "sourcing-grid",
        surface: "world3d",
        expectContent: false,
        note: "Grid lays out the curated set only; on a fresh document curation is empty (same as sourcing-curated).",
      },
    ],
  },
  { paneId: "bearbeiten", windows: [{ kindId: "process-workpiece", surface: "world3d", expectContent: true }] },
  { paneId: "verfolgen", windows: [{ kindId: "gis2d-main", surface: "tiledMap", expectContent: true }] },
  {
    paneId: "statik",
    windows: [
      {
        kindId: "fem3d-model",
        surface: "world3d",
        expectContent: true,
        note: "`s.fem.fem3d@1/*#editor` opens exactly two windows, Model and Results, split 50/50 by its default layout (`🪟️windows/🧱️model` + `📊️results`, both `SurfaceKind::World3d`). Measured 2026-09-17 on the live demonstrator serve: `/#statik` reached `ready` in 4.6 s and both windows carried 3 meshes / 47 instances at 6.0 s, with no page or console errors. On the standalone fem3d lane (:6087) the `demo` example reads the same 3/47 while `concrete-forest` — the id this pane's brand sets as `defaults.exampleId` — reads 2 meshes / 234 instances, so the served pane looks like it is still booting `demo`; that is a brand/default question for the coordinator, not a reason to weaken this assertion, which only requires a non-empty scene and passes on either example.",
      },
      {
        kindId: "fem3d-results",
        surface: "world3d",
        expectContent: true,
        note: "Results draws the same solid set as Model until a solve lands and then recolours it, so it is non-empty from the first frame and is graded exactly like Model (identical counts measured in both windows, plus a `Case: dead` caption). The expensive path here is the SOLVE, not the boot: `📓️fem3d-interactive-2026-09-16.md` clocks the House example at ~11 s in wasm. Neither this assertion nor the Model one waits on a solve, and the whole pane was measured ready+filled in 6 s on the live serve, so `SURFACE_CONTENT_TIMEOUT_MS`/`TEST_TIMEOUT_MS` need no per-pane relaxation the way verfolgen's slow tile paint does.",
      },
    ],
  },
];

/** 📖️ Parses a `data-*-json` attribute into its array length, treating a missing/unparsable
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

/** ⏳️ A surface host attaches EMPTY and is filled by the first frame the guest publishes across
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

/** 🌍️ `World3dHost` stamps its live scene straight onto `.semio-world-3d-host` as
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

/** 🕸️ `NodeGraph` stamps its live flow document onto `.semio-node-graph-host` as
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

/** 📊️ `Table`'s generic row primitive stamps `data-row-id` on every real data `<tr>`
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

/** 🌳️ Two of energy's four windows (Structure, Energy simulation) have a plain framework `Tree`
 * for a body rather than a surface host — `TreeWindowKit` builds a `TreeView`, and `🌳️Tree/🟦️.tsx`
 * renders one `role="treeitem"` + `data-slot="tree-item-row"` element per row (lines ~1936/2013/2088).
 * There is no `.semio-…-host`/`.semio-…-empty` pair to look for, so "did this window resolve" is
 * answered by its `role="tree"` body attaching and "does it carry content" by the row count — both
 * production attributes the framework already emits, in the same spirit as `data-row-id` for tables. */
async function treeRowCount(container: Locator, requireContent = false): Promise<{ readonly hasScene: boolean; readonly rows: number }> {
  const body = container.locator('[role="tree"]');
  await expect(body.first()).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS });
  const rows = await settleContentCount(() => container.locator('[role="treeitem"]').count(), requireContent);
  return { hasScene: true, rows };
}

/** 🗺️ `TiledMapHost` has no content-count DOM attribute (unlike the other three surfaces), so
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
  await canvas.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
  const distinctColors = await settleContentCount(() => countCanvasColors(page, canvas), requireContent, 2);
  return { hasScene: true, painted: distinctColors > 1, distinctColors };
}

/** 🎨️ Distinct RGBA values in the centre 60% of one element's composited screenshot; `1` means
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
test("pane roster", () => {
  expect(PANE_CASES.map((entry) => entry.paneId)).toEqual(brandPaneIds());
});
//#endregion 🧪️PaneConsistency

//#region 🃏️OverviewCards
/** 🃏️ Landing overview cards use window silhouettes (icon title chips, no drag handles). */
test("overview chrome", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);

  await page.goto("/", { waitUntil: "domcontentloaded" });

  await dismissIntroduction(page, page.locator('[id="ui.introduction.skip"]'));

  const cards = page.locator("[data-layered-card]");
  await expect(cards).toHaveCount(brandPaneIds().length, { timeout: SHELL_READY_TIMEOUT_MS });

  for (const paneId of brandPaneIds()) {
    const card = page.locator(`[data-layered-card="${paneId}"] [data-overview-card]`);
    await expect(card).toBeVisible();
    await expect(card.locator("[data-window-silhouette]")).toHaveCount(1);
    await expect(card.locator('[data-window-silhouette-border][data-kind="normal"]')).toHaveCount(1);
    await expect(card.locator('[data-slot="window-chrome-chip-cap"]')).toHaveClass(/ui-glass/);
    await expect(card.locator('[data-slot="window-chrome-body-surface"]')).toHaveClass(/ui-glass/);
    await expect(card.locator('[data-slot="window-chrome-body"]')).not.toHaveClass(/ui-surface/);
    await expect(card.locator('[data-slot="demonstrator-pane-card-title-chip"] svg')).toHaveCount(1);
    await expect(card.locator('[data-slot="demonstrator-pane-card-tagline"]')).toHaveText(/.+/);
    await expect(card.locator('[data-slot="introduction-body-paragraph"]')).toHaveCount(2);
    await expect(card.locator('[data-slot="demonstrator-pane-card-open-chip"]')).toContainText("Demonstrator öffnen");
    await expect(card.locator('[data-slot*="drag"], [data-drag-handle]')).toHaveCount(0);
  }

});
//#endregion 🃏️OverviewCards

for (const paneCase of PANE_CASES) {
  test(`${paneCase.paneId} boot`, async ({ page }) => {
    test.setTimeout(TEST_TIMEOUT_MS);

    await page.goto(`/#${paneCase.paneId}`, { waitUntil: "domcontentloaded" });
    await expect(page.locator(`[data-shell-id="${paneCase.paneId}"]`)).toHaveCount(1, { timeout: SHELL_READY_TIMEOUT_MS });

    const outcome = await waitForPaneShellOutcome(page, paneCase.paneId);
    expect(outcome, `pane "${paneCase.paneId}": expected its shell to reach "ready"`).toBe("ready");

    await dismissIntroductionIfPresent(page, paneCase.paneId);

    for (const win of paneCase.windows) {
      const instanceIds = win.instanceIds ?? [windowElementId(win.kindId)];
      if (win.instanceIds) {
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
        } else if (win.surface === "tree") {
          const { hasScene, rows } = await treeRowCount(container, win.expectContent);
          expect(hasScene, `${label}: tree body never attached`).toBe(true);
          if (win.expectContent) expect(rows, `${label}: expected at least one role="treeitem" row`).toBeGreaterThan(0);
        } else if (win.surface === "placeholder") {
          await expect(container, `${label}: expected its documented text placeholder`).toHaveText(win.placeholderPattern!, { timeout: SHELL_READY_TIMEOUT_MS });
        } else {
          const { hasScene, painted, distinctColors } = await tiledMapHasVisibleContent(page, container, win.expectContent);
          expect(hasScene, `${label}: tiled-map surface never resolved past its empty placeholder`).toBe(true);
          if (win.expectContent) expect(painted, `${label}: expected the map canvas to paint more than one flat color (distinct colors=${distinctColors})`).toBe(true);
        }
      }
    }

    await capturePaneArtifact(page, paneCase.paneId);
  });
}

//#region 🎯️AussuchenSelection
/** 🎯️ The other half of aussuchen's Preview window: picking a stock row must turn the
 * `built_text_node(labels.no_selection)` placeholder into a real World3d scene.
 *
 * This is the served proof of the interaction-view threading wave
 * (`📓️fix-2026-09-16-interaction-view-threading.md`): `✏️editor/🦀️.rs:1091` feeds
 * `preview::render(doc.snapshot, &selected_ids, …)` from `interaction.selection(SOURCING_ROWS_DOMAIN)`,
 * so "what the rows domain has selected" has to travel guest→host→surface for this to pass. Measured
 * on the 6029 serve 2026-09-16: before the click the window reads `Keine Auswahl` with no world host;
 * after it, `.semio-world-3d-host` carries an 804-byte `data-meshes-json`. */
test('aussuchen preview', async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  await page.goto("/#aussuchen", { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, "aussuchen"), 'pane "aussuchen": expected its shell to reach "ready"').toBe("ready");
  await dismissIntroductionIfPresent(page, "aussuchen");

  const preview = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-preview")));
  await expect(preview, "aussuchen preview starts on its documented no-selection placeholder").toHaveText(/Keine Auswahl|No selection/, { timeout: SHELL_READY_TIMEOUT_MS });

  const rows = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-pool"))).locator(".semio-table-host [data-row-id]");
  const stock = rows.filter({ hasText: curationCase.name });
  await expect(stock, "aussuchen stock pool must load the authored demo kind").toHaveAttribute("data-row-id", curationCase.objectId, { timeout: SHELL_READY_TIMEOUT_MS });
  await stock.click({ timeout: 30_000 });

  const { hasScene, meshes, instances } = await worldContentCount(preview, true);
  expect(hasScene, "aussuchen preview: selecting a row must resolve it to a world3d surface").toBe(true);
  expect(meshes + instances, `aussuchen preview: expected the selected kind's geometry (meshes=${meshes}, instances=${instances})`).toBeGreaterThan(0);
  await expect.poll(async () => {
    const selected = JSON.parse(await instanceSnapshot(preview)) as { readonly id: string }[];
    return selected.map((instance) => instance.id);
  }, { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toEqual([curationCase.objectId]);
});
//#endregion 🎯️AussuchenSelection

interface CurationStep {
  readonly action: "increment" | "keyboardIncrement" | "undo" | "redo" | "remove";
  readonly count: number;
}
const curationCase = JSON.parse(readFileSync(new URL("./🧬️curation/🧫️cases.json", import.meta.url), "utf8")) as {
  readonly objectId: string;
  readonly name: string;
  readonly availability: number;
  readonly steps: readonly CurationStep[];
  readonly revisit: {
    readonly viaPane: string;
    readonly offscreenMs: number;
    readonly afterRedoCount: number;
    readonly afterUndoCount: number;
  };
};

/** 🧩️ Reads the production world projection so undo/redo must restore the actual scene. */
async function instanceSnapshot(container: Locator): Promise<string> {
  const host = container.locator(".semio-world-3d-host");
  return (await host.count()) === 0 ? "[]" : (await host.getAttribute("data-instances-json")) ?? "[]";
}

test('aussuchen undo-redo', async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + SHELL_READY_TIMEOUT_MS + curationCase.revisit.offscreenMs);
  await page.goto("/#aussuchen", { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, "aussuchen")).toBe("ready");
  await dismissIntroductionIfPresent(page, "aussuchen");

  const pool = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-pool")));
  const stock = pool.locator(`[data-row-id="${curationCase.objectId}"]`);
  const curated = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-curated")));
  const row = curated.locator(`[data-row-id="${curationCase.objectId}"]`);
  const grid = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-grid")));
  const count = stock.getByRole("spinbutton");
  await expect(stock).toContainText(curationCase.name, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(count).toHaveAttribute("aria-valuemax", String(curationCase.availability));
  await expect(count).toHaveValue("0");
  await expect(curated.locator("[data-row-id]")).toHaveCount(0);
  await expect.poll(() => instanceSnapshot(grid)).toBe("[]");
  const scenes = new Map<number, string>([[0, "[]"]]);
  const assertCount = async (expected: number, label: string): Promise<void> => {
    await expect(count, `pool count ${label}`).toHaveValue(String(expected));
    await expect(curated.locator("[data-row-id]"), `curated kinds ${label}`).toHaveCount(expected === 0 ? 0 : 1);
    if (expected > 0) await expect(row.getByRole("spinbutton")).toHaveValue(String(expected));
    await expect.poll(async () => {
      const instances = JSON.parse(await instanceSnapshot(grid)) as { readonly id: string }[];
      return instances.filter((instance) => instance.id === curationCase.objectId).length;
    }, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: `grid geometry ${label}` }).toBe(expected);
    const scene = await instanceSnapshot(grid);
    if (scenes.has(expected)) expect(scene, `complete grid projection for count ${expected} ${label}`).toBe(scenes.get(expected));
    else scenes.set(expected, scene);
  };

  for (const step of curationCase.steps) {
    if (step.action === "increment") await stock.locator('[data-stepper-control="increment"]').click();
    else if (step.action === "keyboardIncrement") await count.press("ArrowUp");
    else if (step.action === "remove") await row.getByRole("button", { name: /^(Remove|Entfernen)$/ }).click();
    else {
      await stock.getByText(curationCase.name, { exact: true }).click();
      await page.keyboard.press(step.action === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
    }
    await assertCount(step.count, `after ${step.action}`);
  }

  await page.locator("[data-layered-overview-button]").click();
  await dismissIntroduction(page, page.locator('[id="ui.introduction.skip"]'));
  await page.locator(`[data-layered-card="${curationCase.revisit.viaPane}"] [data-slot="demonstrator-pane-card-open-chip"]`).click();
  await expect(page).toHaveURL(new RegExp(`#${curationCase.revisit.viaPane}$`));
  expect(await waitForPaneShellOutcome(page, curationCase.revisit.viaPane)).toBe("ready");
  await dismissIntroductionIfPresent(page, curationCase.revisit.viaPane);
  await page.waitForTimeout(curationCase.revisit.offscreenMs);
  await expect(page.locator('[data-shell-id="aussuchen"]'), "edited pane stays mounted beyond the pristine-pane suspension threshold").toHaveCount(1);
  await page.locator("[data-layered-overview-button]").click();
  await page.locator('[data-layered-card="aussuchen"] [data-slot="demonstrator-pane-card-open-chip"]').click();
  await expect(page).toHaveURL(/#aussuchen$/);
  await assertCount(curationCase.revisit.afterUndoCount, "after revisiting the edited pane");
  await stock.getByText(curationCase.name, { exact: true }).click();
  await page.keyboard.press("ControlOrMeta+Shift+z");
  await assertCount(curationCase.revisit.afterRedoCount, "after revisited redo");
  await page.keyboard.press("ControlOrMeta+z");
  await assertCount(curationCase.revisit.afterUndoCount, "after revisited undo");
  await capturePaneArtifact(page, "aussuchen");
});

const simulationCases = JSON.parse(readFileSync(new URL("./🧬️simulation/🧫️cases.json", import.meta.url), "utf8")) as {
  readonly energy: {
    readonly paneId: string;
    readonly toolTabId: string;
    readonly toolCategoryId: string;
    readonly complete: string;
    readonly finalized: string;
    readonly aborted: string;
    readonly finalStep: string;
    readonly noResult: string;
  };
  readonly fem: {
    readonly paneId: string;
    readonly panelTabId: string;
    readonly panelCategoryId: string;
    readonly caption: string;
    readonly phase: number;
    readonly initialScale: number;
    readonly changedScale: number;
    readonly steps: readonly { readonly action: "change" | "undo" | "redo"; readonly scale: number }[];
  };
};

/** 🧭️ Opens authored progressive panel chrome through the same tab buttons a user operates. */
async function openPaneTab(page: Page, paneId: string, categoryId: string, tabId: string): Promise<void> {
  const shell = page.locator(`[data-shell-id="${paneId}"]`);
  const tab = shell.locator(`[data-tab-id="${tabId}"]:visible`).first();
  if (!(await tab.isVisible())) await shell.locator(`[data-tab-id="${categoryId}"]:visible`).first().click();
  if ((await tab.getAttribute("aria-pressed")) !== "true") await tab.click();
}

/** 🪪️ Finds the current framework run identity from actual rendered transport IDs, never injected state. */
async function latestToolRunGroup(page: Page, paneId: string): Promise<string> {
  return page.locator(`[data-shell-id="${paneId}"] [id], [data-shell-id="${paneId}"] [data-element-alias]`).evaluateAll((elements) => {
    const runs = elements.flatMap((element) => [element.id, ...(element.getAttribute("data-element-alias") ?? "").split(/\s+/)])
      .flatMap((id) => /^framework\.toolRun\.(\d+)(?:\.|$)/.exec(id)?.[1] ?? []);
    const latest = runs.reduce((highest, run) => BigInt(run) > BigInt(highest) ? run : highest, "0");
    return runs.length ? `framework.toolRun.${latest}` : "";
  });
}

test('energie run-cancel', async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 4 * SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = simulationCases.energy;
  await page.goto(`/#${scenario.paneId}`, { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, scenario.paneId)).toBe("ready");
  await dismissIntroductionIfPresent(page, scenario.paneId);
  const simulation = page.locator(paneElementSelector(scenario.paneId, windowElementId("energy.simulation")));
  await expect(simulation).toContainText(/No energy simulation run|Kein Energiesimulationslauf/);
  await simulation.getByRole("tree").click();
  await openPaneTab(page, scenario.paneId, scenario.toolCategoryId, scenario.toolTabId);
  await simulation.getByRole("tree").click();
  await page.keyboard.press("ControlOrMeta+Enter");
  await expect.poll(() => latestToolRunGroup(page, scenario.paneId), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).not.toBe("");
  const completedGroup = await latestToolRunGroup(page, scenario.paneId);
  const completed = page.locator(paneElementSelector(scenario.paneId, completedGroup));
  const completedStatus = page.locator(paneElementSelector(scenario.paneId, `${completedGroup}.status`));
  await expect(completed.getByRole("progressbar")).toBeVisible();
  await expect(completedStatus).toHaveAttribute("aria-live", /^(polite|off|assertive)$/);
  await expect(completedStatus).toHaveText(new RegExp(scenario.complete), { timeout: 3 * SURFACE_CONTENT_TIMEOUT_MS });
  await expect(page.locator(paneElementSelector(scenario.paneId, `${completedGroup}.steps`))).toContainText(new RegExp(scenario.finalStep));
  await page.locator(paneElementSelector(scenario.paneId, `${completedGroup}.toolRunFinalize`)).click();
  await expect(completedStatus).toHaveText(new RegExp(scenario.finalized), { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await capturePaneArtifact(page, scenario.paneId);

  await page.locator(paneElementSelector(scenario.paneId, `${completedGroup}.toolRunStart`)).click();
  await expect.poll(() => latestToolRunGroup(page, scenario.paneId), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).not.toBe(completedGroup);
  const abortedGroup = await latestToolRunGroup(page, scenario.paneId);
  const abort = page.locator(paneElementSelector(scenario.paneId, `${abortedGroup}.toolRunAbort`));
  await expect(abort).toBeEnabled();
  await abort.click();
  await expect(page.locator(paneElementSelector(scenario.paneId, `${abortedGroup}.status`))).toHaveText(new RegExp(scenario.aborted), { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(simulation).toContainText(new RegExp(scenario.noResult));
});

/** 🌐️ Captures both geometry lanes because a deformation can change mesh vertices or instance transforms. */
async function worldProjectionSnapshot(container: Locator): Promise<string> {
  return container.locator(".semio-world-3d-host").evaluate((host) => JSON.stringify({
    meshes: JSON.parse(host.getAttribute("data-meshes-json") ?? "[]"),
    instances: JSON.parse(host.getAttribute("data-instances-json") ?? "[]"),
  }));
}

test('statik undo-redo', async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = simulationCases.fem;
  await page.goto(`/#${scenario.paneId}`, { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, scenario.paneId)).toBe("ready");
  await dismissIntroductionIfPresent(page, scenario.paneId);
  const results = page.locator(paneElementSelector(scenario.paneId, windowElementId("fem3d-results")));
  const caption = results.getByText(scenario.caption, { exact: true });
  await expect(caption, "initial automatic solve must settle without a pending/fault suffix").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await caption.click();
  await openPaneTab(page, scenario.paneId, scenario.panelCategoryId, scenario.panelTabId);
  const transport = page.locator(paneElementSelector(scenario.paneId, "fem3d-play-results.transport.play"));
  await expect(transport, "deformation comparisons require paused playback").toHaveText(/^(Play|Abspielen)$/);
  const fullAmplitude = await worldProjectionSnapshot(results);
  const phase = page.locator(paneElementSelector(scenario.paneId, "fem3d-play-results.phase.control")).getByRole("slider");
  await phase.press("Home");
  await expect(phase).toHaveAttribute("aria-valuenow", "0");
  await expect.poll(() => worldProjectionSnapshot(results), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "zero phase must reach the result scene before advancing" }).not.toBe(fullAmplitude);
  const zeroPhase = await worldProjectionSnapshot(results);
  await phase.press("PageUp");
  await expect(phase).toHaveAttribute("aria-valuenow", String(scenario.phase));
  await expect.poll(() => worldProjectionSnapshot(results), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "the chosen nonzero phase must reach the result scene before comparison" }).not.toBe(zeroPhase);
  const scale = page.locator(paneElementSelector(scenario.paneId, "fem3d-play-results.deformation-scale.control"));
  await expect(scale).toHaveValue(String(scenario.initialScale));
  const content = await worldContentCount(results, true);
  expect(content.meshes + content.instances).toBeGreaterThan(0);
  const initial = await worldProjectionSnapshot(results);
  const projections = new Map<number, string>([[scenario.initialScale, initial]]);

  for (const step of scenario.steps) {
    const previous = await worldProjectionSnapshot(results);
    if (step.action === "change") {
      await scale.fill(String(step.scale));
      await scale.press("Tab");
    } else {
      await caption.click();
      await page.keyboard.press(step.action === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
    }
    await expect(scale).toHaveValue(String(step.scale));
    await expect.poll(() => worldProjectionSnapshot(results), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: `deformation must change after ${step.action}` }).not.toBe(previous);
    await expect(caption, `replacement analysis after ${step.action} must be current, without a progress suffix`).toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
    const projection = await worldProjectionSnapshot(results);
    if (projections.has(step.scale)) expect(projection, `${step.action} restores the solved projection`).toBe(projections.get(step.scale));
    else projections.set(step.scale, projection);
    await expect(phase).toHaveAttribute("aria-valuenow", String(scenario.phase));
    await expect(transport).toHaveText(/^(Play|Abspielen)$/);
  }
  await capturePaneArtifact(page, scenario.paneId);
});

const editCases = JSON.parse(readFileSync(new URL("./🧬️edits/🧫️cases.json", import.meta.url), "utf8")) as {
  readonly generator: { readonly heightId: string; readonly initial: number; readonly changed: number; readonly widgetIds: readonly string[]; readonly synapseIds: readonly string[] };
  readonly koordinator: { readonly catalogueId: string; readonly windowIds: readonly string[]; readonly rowPrefix: string };
  readonly aggregator: { readonly kindId: string; readonly seedId: string; readonly vortexCount: number; readonly meshUrl: string; readonly windowIds: readonly string[] };
  readonly bearbeiten: { readonly stepId: string; readonly label: string; readonly machine: string; readonly capability: string; readonly stepIds: readonly string[] };
  readonly verfolgen: { readonly label: string; readonly delta: number; readonly collection: string; readonly longitude: number; readonly latitude: number };
};

/** 🚪️ Enters a pane through its supported hash route before a document gesture. */
async function enterPane(page: Page, paneId: string): Promise<void> {
  await page.goto(`/#${paneId}`, { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, paneId)).toBe("ready");
  await dismissIntroductionIfPresent(page, paneId);
}

/** 🗂️ Uses the authored top-level document tabs without changing application state through an API. */
async function openDocumentTab(page: Page, paneId: string, tabId: string): Promise<Locator> {
  const tab = page.locator(`[data-shell-id="${paneId}"] [data-tab-id="framework.panel.${tabId}"]:visible`).first();
  if ((await tab.getAttribute("aria-pressed")) !== "true") await tab.click();
  return page.locator(paneElementSelector(paneId, `framework.panelTab.framework.panel.${tabId}`));
}

/** ⏱️ Waits for production compute chrome to leave its busy state before comparing geometry. */
async function waitWorldSettled(container: Locator): Promise<void> {
  await worldContentCount(container, true);
  await expect(container.locator('[data-slot="world-compute-status"][aria-busy="true"]')).toHaveCount(0, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(container.locator('[data-slot="world-compute-status"]')).not.toHaveAttribute("data-compute-phase", /cancel|fault|fail|error/i);
}

/** 🧱️ Removes ephemeral interaction paint from a document geometry comparison. */
async function documentWorldProjection(container: Locator): Promise<string> {
  return container.locator(".semio-world-3d-host").evaluate((host) => JSON.stringify({
    meshes: JSON.parse(host.getAttribute("data-meshes-json") ?? "[]"),
    instances: (JSON.parse(host.getAttribute("data-instances-json") ?? "[]") as Record<string, unknown>[]).map(({ selected, hovered, highlighted, ...instance }) => instance),
  }));
}

/** 🆔️ Reads the visible instance roster to tie document rows to the actually projected objects. */
async function worldInstanceIds(container: Locator): Promise<readonly string[]> {
  return [...new Set((JSON.parse(await instanceSnapshot(container)) as { readonly id: string }[]).map((instance) => instance.id))].sort();
}

/** ↩️ Drives the real pane keyboard route after focusing its production scene canvas. */
async function historyGesture(page: Page, container: Locator, direction: "undo" | "redo"): Promise<void> {
  await container.locator("canvas").first().click({ position: { x: 8, y: 8 } });
  await page.keyboard.press(direction === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
}

test("generator undo-redo", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  await enterPane(page, "generator");
  const scenario = editCases.generator;
  const graph = page.locator(paneElementSelector("generator", windowElementId("procedural-main")));
  const preview = page.locator(paneElementSelector("generator", windowElementId("procedural-preview")));
  await nodeGraphWidgetCount(graph, true);
  const snapshot = async () => JSON.parse((await graph.locator(".semio-node-graph-host").getAttribute("data-host-snapshot-json"))!) as {
    readonly widgets: readonly { readonly id: string; readonly value?: number }[];
    readonly synapses: readonly { readonly id: string }[];
  };
  const graphProjection = async () => {
    const value = await snapshot();
    return JSON.stringify({ widgets: value.widgets, synapses: value.synapses });
  };
  const initialGraph = await snapshot();
  expect(initialGraph.widgets.map((widget) => widget.id).sort()).toEqual([...scenario.widgetIds].sort());
  expect(initialGraph.synapses.map((synapse) => synapse.id).sort()).toEqual([...scenario.synapseIds].sort());
  expect(initialGraph.widgets.find((widget) => widget.id === scenario.heightId)?.value).toBe(scenario.initial);
  await waitWorldSettled(preview);
  const initialGeometry = await documentWorldProjection(preview);
  const initial = await graphProjection();
  await openDocumentTab(page, "generator", "artifact");
  await page.locator(paneElementSelector("generator", scenario.heightId)).locator('[data-slot="tree-label"]').click();
  await openDocumentTab(page, "generator", "inspection");
  const input = page.locator(paneElementSelector("generator", "procedural-play-inspector.value.input"));
  await expect(input).toHaveValue(String(scenario.initial));
  await input.fill(String(scenario.changed));
  await input.press("Tab");
  await expect.poll(async () => (await snapshot()).widgets.find((widget) => widget.id === scenario.heightId)?.value).toBe(scenario.changed);
  await expect.poll(() => documentWorldProjection(preview), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).not.toBe(initialGeometry);
  await waitWorldSettled(preview);
  const changed = await graphProjection();
  const changedGeometry = await documentWorldProjection(preview);
  expect((await snapshot()).widgets.map((widget) => widget.id).sort()).toEqual([...scenario.widgetIds].sort());
  expect((await snapshot()).synapses.map((synapse) => synapse.id).sort()).toEqual([...scenario.synapseIds].sort());
  for (const [direction, expectedGraph, expectedGeometry] of [["undo", initial, initialGeometry], ["redo", changed, changedGeometry]] as const) {
    await historyGesture(page, graph, direction);
    await expect.poll(graphProjection, { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(expectedGraph);
    await expect.poll(() => documentWorldProjection(preview), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(expectedGeometry);
    await waitWorldSettled(preview);
  }
  await capturePaneArtifact(page, "generator");
});

test("koordinator undo-redo", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  await enterPane(page, "koordinator");
  const scenario = editCases.koordinator;
  const windows = scenario.windowIds.map((id) => page.locator(paneElementSelector("koordinator", windowElementId(id))));
  for (const window of windows) await waitWorldSettled(window);
  const initial = await Promise.all(windows.map(documentWorldProjection));
  const initialIds = await Promise.all(windows.map(worldInstanceIds));
  await windows[0]!.locator("canvas").first().click({ position: { x: 8, y: 8 } });
  await openDocumentTab(page, "koordinator", "catalogue");
  await page.locator(paneElementSelector("koordinator", scenario.catalogueId)).locator('[data-slot="tree-label"]').click();
  await expect.poll(async () => (await worldInstanceIds(windows[0]!)).filter((id) => !initialIds[0]!.includes(id)).length, { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(1);
  const newId = (await worldInstanceIds(windows[0]!)).find((id) => !initialIds[0]!.includes(id))!;
  await openDocumentTab(page, "koordinator", "artifact");
  const row = page.locator(paneElementSelector("koordinator", newId));
  await expect(row).toContainText(/Box \d+/);
  await waitWorldSettled(windows[0]!);
  const changed = await documentWorldProjection(windows[0]!);
  for (let index = 1; index < windows.length; index += 1) expect(await worldInstanceIds(windows[index]!)).toEqual(initialIds[index]);
  for (const [direction, projection, present] of [["undo", initial[0]!, false], ["redo", changed, true]] as const) {
    await historyGesture(page, windows[0]!, direction);
    await expect.poll(() => documentWorldProjection(windows[0]!), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(projection);
    await expect(row).toHaveCount(present ? 1 : 0);
    for (let index = 1; index < windows.length; index += 1) expect(await documentWorldProjection(windows[index]!)).toBe(initial[index]);
  }
  await capturePaneArtifact(page, "koordinator");
});

test("aggregator undo-redo", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  await enterPane(page, "aggregator");
  const scenario = editCases.aggregator;
  const windows = scenario.windowIds.map((id) => page.locator(paneElementSelector("aggregator", `framework.window.${elementIdSegment(id)}`)));
  for (const window of windows) await waitWorldSettled(window);
  const initial = await Promise.all(windows.map(documentWorldProjection));
  const initialIds = await worldInstanceIds(windows[0]!);
  expect(initialIds).toContain(scenario.seedId);
  const catalogue = await openDocumentTab(page, "aggregator", "catalogue");
  await catalogue.locator(`[id="${scenario.kindId}"] [data-slot="tree-label"]`).click();
  await expect.poll(async () => (await worldInstanceIds(windows[0]!)).filter((id) => !initialIds.includes(id)).length, { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(1);
  const newId = (await worldInstanceIds(windows[0]!)).find((id) => !initialIds.includes(id))!;
  expect(newId).toMatch(/^puzzle3d\.object\.[0-9a-f]+$/);
  await openDocumentTab(page, "aggregator", "artifact");
  const row = page.locator(paneElementSelector("aggregator", newId));
  await expect(row).toContainText(scenario.kindId);
  for (const window of windows) {
    await expect.poll(() => worldInstanceIds(window), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toEqual([...initialIds, newId].sort());
    const host = window.locator(".semio-world-3d-host");
    await expect(host).toHaveAttribute("data-meshes-json", new RegExp(scenario.meshUrl.replaceAll(".", "\\.")));
    const instances = JSON.parse((await host.getAttribute("data-instances-json"))!) as { readonly id: string; readonly meshId: string }[];
    expect(instances.find((instance) => instance.id === newId)?.meshId).toBe(instances.find((instance) => instance.id === scenario.seedId)?.meshId);
    const vortices = JSON.parse((await host.getAttribute("data-vortices-json"))!) as { readonly objectId: string }[];
    expect(vortices.filter((vortex) => vortex.objectId === newId)).toHaveLength(scenario.vortexCount);
  }
  const changed = await Promise.all(windows.map(documentWorldProjection));
  for (const [direction, projections, present] of [["undo", initial, false], ["redo", changed, true]] as const) {
    await historyGesture(page, windows[0]!, direction);
    for (let index = 0; index < windows.length; index += 1) await expect.poll(() => documentWorldProjection(windows[index]!), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(projections[index]);
    await expect(row).toHaveCount(present ? 1 : 0);
  }
  await capturePaneArtifact(page, "aggregator");
});

test("bearbeiten undo-redo", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  await enterPane(page, "bearbeiten");
  const scenario = editCases.bearbeiten;
  const workpiece = page.locator(paneElementSelector("bearbeiten", windowElementId("process-workpiece")));
  await waitWorldSettled(workpiece);
  await openDocumentTab(page, "bearbeiten", "artifact");
  for (const id of scenario.stepIds) await expect(page.locator(paneElementSelector("bearbeiten", id))).toHaveCount(1);
  const row = page.locator(paneElementSelector("bearbeiten", scenario.stepId));
  await row.locator('[data-slot="tree-label"]').click();
  await openDocumentTab(page, "bearbeiten", "inspection");
  const field = (id: string) => page.locator(paneElementSelector("bearbeiten", `process3d-play-inspector.step.${id}`));
  await expect(field("id")).toContainText(scenario.stepId);
  await expect(field("label")).toContainText(scenario.label);
  await expect(field("machine")).toContainText(scenario.machine);
  await expect(field("capability")).toContainText(scenario.capability);
  await expect(field("enabled")).toContainText("true");
  const initial = await documentWorldProjection(workpiece);
  await openDocumentTab(page, "bearbeiten", "artifact");
  await row.getByRole("button", { name: /^(Enabled|Aktiviert)$/ }).click();
  await openDocumentTab(page, "bearbeiten", "inspection");
  await expect(field("enabled")).toContainText("false");
  await expect.poll(() => documentWorldProjection(workpiece), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).not.toBe(initial);
  await waitWorldSettled(workpiece);
  const changed = await documentWorldProjection(workpiece);
  for (const [direction, enabled, projection] of [["undo", "true", initial], ["redo", "false", changed]] as const) {
    await historyGesture(page, workpiece, direction);
    await openDocumentTab(page, "bearbeiten", "artifact");
    await row.locator('[data-slot="tree-label"]').click();
    for (const id of scenario.stepIds) await expect(page.locator(paneElementSelector("bearbeiten", id))).toHaveCount(1);
    await openDocumentTab(page, "bearbeiten", "inspection");
    await expect(field("enabled")).toContainText(enabled);
    await expect.poll(() => documentWorldProjection(workpiece), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBe(projection);
    await waitWorldSettled(workpiece);
  }
  await capturePaneArtifact(page, "bearbeiten");
});

test("verfolgen undo-redo", async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  await enterPane(page, "verfolgen");
  const scenario = editCases.verfolgen;
  const map = page.locator(paneElementSelector("verfolgen", windowElementId("gis2d-main")));
  await openDocumentTab(page, "verfolgen", "inspection");
  const summary = (kind: string) => page.locator(paneElementSelector("verfolgen", `gis2d-play-inspector.${kind}-count`)).locator('[data-slot="tree-description"]');
  const positions = Number(await summary("positions").innerText());
  const routes = await summary("routes").innerText();
  const regions = await summary("regions").innerText();
  expect(Number.isSafeInteger(positions)).toBe(true);
  await page.locator(paneElementSelector("verfolgen", "framework.window.gis2dMain.engagement.toggle")).click();
  const actions = map.locator('[data-slot="window-action-pane"]');
  await actions.locator('[id="action.addFeature"] [data-slot="tree-label"]').click();
  await actions.locator('[id="action.addFeature.arg.label"] input').fill(scenario.label);
  await expect(actions.locator('[id="action.addFeature.arg.lon"] input')).toHaveValue(String(scenario.longitude));
  await expect(actions.locator('[id="action.addFeature.arg.lat"] input')).toHaveValue(String(scenario.latitude));
  await page.locator(paneElementSelector("verfolgen", "framework.window.gis2dMain.action.addFeature.execute")).click();
  await expect(summary("positions")).toHaveText(String(positions + scenario.delta));
  await expect(summary("routes")).toHaveText(routes);
  await expect(summary("regions")).toHaveText(regions);
  for (const [direction, expected] of [["undo", positions], ["redo", positions + scenario.delta]] as const) {
    await historyGesture(page, map, direction);
    await expect(summary("positions")).toHaveText(String(expected));
    await expect(summary("routes")).toHaveText(routes);
    await expect(summary("regions")).toHaveText(regions);
  }
  await capturePaneArtifact(page, "verfolgen");
});

test("funding logos", async ({ page }) => {
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
