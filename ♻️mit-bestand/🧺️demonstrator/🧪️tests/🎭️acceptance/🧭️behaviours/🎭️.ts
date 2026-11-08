// #region 🧲️Header
// 🧭️ Served behaviour acceptance: documented pane behaviours the 25-case baseline (`../🟦️.ts`) does not cover.
// Run apart from the baseline: this file is outside the default `testMatch`; every case carries `@behaviour` and a pane tag.
// Each case runs in the `en` and `de` projects. Selectors and wordings are read from the producers named at each case.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { expect, type Page } from "@playwright/test";
import {
  SHELL_READY_TIMEOUT_MS, SURFACE_CONTENT_TIMEOUT_MS, TEST_TIMEOUT_MS,
  capturePaneArtifact, catalogText, documentWorldProjection, enterPane, historyGesture, latestToolRunGroup,
  openDocumentTab, openPaneTab, openWindowOptions, paneElementSelector, paneWindow, pickSelectValue, test, vortexCount, waitForPaneShellOutcome, waitWorldSettled, windowElementId, wording, worldInstanceIds,
  type Wording,
} from "./🧰️pane/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🧬️Cases
const cases = JSON.parse(readFileSync(new URL("./🧫️cases.json", import.meta.url), "utf8")) as {
  readonly texts: Readonly<Record<"playTutorial" | "vortexShow" | "always" | "selected" | "move" | "rotate" | "searchPlaceholder" | "allTypologies" | "routes" | "nodesSection" | "caption", Wording>>;
  readonly tour: { readonly paneId: string; readonly tutorialTitleKey: string; readonly descriptionKey: string; readonly describedChapterIds: readonly string[]; readonly seekChapterId: string; readonly seekPanelTabId: string };
  readonly transform: { readonly paneId: string; readonly windowId: string; readonly utilityId: string; readonly flagIds: readonly string[] };
  readonly vortexShow: { readonly paneId: string; readonly windowId: string; readonly measureId: string; readonly initial: string; readonly changed: string };
  readonly fill: { readonly paneId: string; readonly windowId: string; readonly toolCategoryId: string; readonly toolTabId: string; readonly countId: string; readonly extraCount: number };
  readonly filter: {
    readonly paneId: string; readonly poolRowIds: readonly string[];
    readonly query: { readonly text: string; readonly rowIds: readonly string[] };
    readonly module: { readonly id: string; readonly rowIds: readonly string[] };
    readonly typology: { readonly path: string; readonly rowIds: readonly string[] };
  };
  readonly chain: { readonly paneId: string; readonly catalogueRowId: string; readonly stepIds: readonly string[] };
  readonly routes: { readonly paneId: string; readonly collection: string; readonly label: string; readonly delta: number };
  readonly model: { readonly paneId: string; readonly modelWindowId: string; readonly resultsWindowId: string; readonly coordinates: Readonly<Record<"x" | "y" | "z", number>>; readonly delta: number };
  readonly lifecycle: { readonly firstPaneId: string; readonly warmPaneId: string; readonly openedPaneId: string; readonly otherPaneId: string };
};

/** 📖️ The tour's chapters read from `🪧️brand.ts` as text (the baseline's technique: Node cannot load the brand module). */
function brandTourChapters(): readonly { readonly id: string; readonly titleKey: string }[] {
  const source = readFileSync(join(import.meta.dirname, "..", "..", "..", "🪧️brand.ts"), "utf8");
  const block = /chapters:\s*\[(.*?)\n\s*\],/s.exec(source);
  if (!block) throw new Error("🪧️brand.ts no longer declares the tutorial chapters array");
  return [...block[1]!.matchAll(/\{\s*id:\s*"([^"]+)",\s*at:\s*[\d_]+,\s*title:\s*demonstratorLabel\("([^"]+)"\)/g)].map((match) => ({ id: match[1]!, titleKey: match[2]! }));
}

/** ⏳️ The landing's lifecycle thresholds read from `🟦️.tsx` as text, so the cases follow the constants instead of restating them. */
function landingLifecycle(): Readonly<Record<"warmStartMs" | "warmIntervalMs" | "suspendIdleMs" | "suspendOffscreenMs" | "suspendHiddenMs", number>> {
  const source = readFileSync(join(import.meta.dirname, "..", "..", "..", "🟦️.tsx"), "utf8");
  const block = /DEMONSTRATOR_LIFECYCLE\s*=\s*\{([^}]*)\}/.exec(source);
  if (!block) throw new Error("🟦️.tsx no longer declares DEMONSTRATOR_LIFECYCLE");
  const read = (name: string): number => {
    const match = new RegExp(`${name}:\\s*([\\d_]+)(?:\\s*\\*\\s*([\\d_]+))?`).exec(block[1]!);
    if (!match) throw new Error(`DEMONSTRATOR_LIFECYCLE no longer declares ${name}`);
    return Number(match[1]!.replaceAll("_", "")) * Number((match[2] ?? "1").replaceAll("_", ""));
  };
  return { warmStartMs: read("warmStartMs"), warmIntervalMs: read("warmIntervalMs"), suspendIdleMs: read("suspendIdleMs"), suspendOffscreenMs: read("suspendOffscreenMs"), suspendHiddenMs: read("suspendHiddenMs") };
}

const escapeRegExp = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const tags = (pane: string): { tag: string[] } => ({ tag: ["@behaviour", `@${pane}`] });
//#endregion 🧬️Cases

//#region 🎬️AggregatorTour
/** 🎬️ Documented (`demonstrator.brand.tutorial.aggregatorTour.description`): a guided tour "view, catalogue, transform, connection points and fill".
 * Starts through the command palette (`mod+p`, `os.playTutorial` with its `tutorialId` argument; `🛠️ShellHelpers/🎛️command-registry`), then grades the
 * tutorial bar (`ui.tutorial.*`, `🎥️tutorial-ui`): the 12 authored chapters in order with their localized titles, play/pause, a chapter seek that opens
 * the Catalogue tab (the authored `panelTab` delta at 76 s, `🪧️brand.ts` tracks.ui), and stop. The tour's document track is empty by authors' note. */
test("aggregator tour", tags("aggregator"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = cases.tour;
  await enterPane(page, scenario.paneId);
  await page.keyboard.press("ControlOrMeta+p");
  const search = page.locator('[data-slot="command-input"]');
  await expect(search, "the command palette opens with mod+p").toBeVisible();
  await search.fill(wording(cases.texts.playTutorial));
  await page.getByRole("option", { name: new RegExp(escapeRegExp(wording(cases.texts.playTutorial))) }).first().click();
  const tutorialArg = page.locator('[id^="command."][id$=".arg.tutorialId"]');
  await expect(tutorialArg, "the play-tutorial command asks for the tutorial").toBeVisible({ timeout: 30_000 });
  await tutorialArg.getByRole("combobox").click();
  await page.getByRole("option", { name: new RegExp(escapeRegExp(catalogText(scenario.tutorialTitleKey))) }).first().click();
  await page.locator('[id^="command-"][id$="-execute"]').click();

  const bar = page.locator('[id="ui.tutorial.bar"]');
  await expect(bar).toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  const chapters = brandTourChapters();
  for (const id of scenario.describedChapterIds) expect(chapters.map((chapter) => chapter.id), `the description names chapter ${id}`).toContain(id);
  expect(catalogText(scenario.descriptionKey).length).toBeGreaterThan(0);
  const ticks = bar.locator('[data-slot="tutorial-chapter-tick"]');
  await expect(ticks).toHaveCount(chapters.length);
  for (const chapter of chapters) await expect(bar.locator(`[id="ui.tutorial.chapter.${chapter.id}"]`), `chapter ${chapter.id}`).toHaveAttribute("title", catalogText(chapter.titleKey));
  expect(await ticks.evaluateAll((elements) => elements.map((element) => element.id.replace("ui.tutorial.chapter.", ""))), "chapters keep their authored order").toEqual(chapters.map((chapter) => chapter.id));

  const time = bar.locator('[id="ui.tutorial.time"]');
  const play = bar.locator('[id="ui.tutorial.play"]');
  const start = await time.innerText();
  await play.click();
  await expect.poll(() => time.innerText(), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "playback advances the playhead" }).not.toBe(start);
  await play.click();

  const tab = page.locator(`[data-shell-id="${scenario.paneId}"] [data-tab-id="${scenario.seekPanelTabId}"]:visible`).first();
  if ((await tab.getAttribute("aria-pressed")) === "true") await tab.click();
  await expect(tab).not.toHaveAttribute("aria-pressed", "true");
  await bar.locator(`[id="ui.tutorial.chapter.${scenario.seekChapterId}"]`).click({ force: true });
  await expect(tab, "seeking past the authored panelTab delta opens the Catalogue tab").toHaveAttribute("aria-pressed", "true", { timeout: SURFACE_CONTENT_TIMEOUT_MS });

  await bar.locator('[id="ui.tutorial.stop"]').click();
  await expect(bar).toHaveCount(0, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🎬️AggregatorTour

//#region 🔄️AggregatorTransform
/** 🔄️ Documented (`demonstrator.brand.tutorial.transformUtility.text`): "The Transform utility moves and rotates building components."
 * Producer: `puzzle3d …/🪛️utilities/🔄️transform/🦀️.rs` (utility id `transform`, label Transform/Transformieren; options `puzzle3d-transform-move` and
 * `puzzle3d-transform-rotate`, reuse labels Move/Verschieben and Rotate/Drehen). Activating the utility is a toggle with `aria-pressed`. */
test("aggregator transform", tags("aggregator"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  const scenario = cases.transform;
  await enterPane(page, scenario.paneId);
  await waitWorldSettled(paneWindow(page, scenario.paneId, scenario.windowId));
  await openWindowOptions(page, scenario.paneId, scenario.windowId);
  const utility = page.locator(paneElementSelector(scenario.paneId, scenario.utilityId)).filter({ visible: true }).first();
  await expect(utility, "the Transform utility is offered").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  if ((await utility.getAttribute("aria-pressed")) === "true") await utility.click();
  await expect(utility).toHaveAttribute("aria-pressed", "false");
  await utility.click();
  await expect(utility).toHaveAttribute("aria-pressed", "true", { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  const shell = page.locator(`[data-shell-id="${scenario.paneId}"]`);
  for (const [flagId, label] of [[scenario.flagIds[0]!, cases.texts.move], [scenario.flagIds[1]!, cases.texts.rotate]] as const) {
    await expect(page.locator(paneElementSelector(scenario.paneId, flagId)).first(), `option ${flagId}`).toBeAttached({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
    await expect(shell.getByRole("checkbox", { name: wording(label), exact: true }).first(), `option ${flagId} is named ${wording(label)}`).toBeVisible();
  }
  await utility.click();
  await expect(utility).toHaveAttribute("aria-pressed", "false", { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🔄️AggregatorTransform

//#region 🌀️AggregatorVortexShow
/** 🌀️ Documented (`demonstrator.brand.tutorial.verbindungspunkte.text`): setting "Show connection points" to "Always" makes the connections of all building
 * components visible. Producer: `puzzle3d …/☑️options/🌀️vortex/🦀️.rs` (`puzzle3d-play-vortex-show`, items always/selected, default selected; markers are emitted
 * for every object only on `always`, `🧊️main/🦀️.rs` vortex publish rule). Observed as the length of `data-vortices-json` on the window's world host, and as
 * the combobox's `data-published-value` (`🎚️measure-controls`). The option is per window instance. */
test("aggregator vortex show", tags("aggregator"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 2 * SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = cases.vortexShow;
  await enterPane(page, scenario.paneId);
  const win = paneWindow(page, scenario.paneId, scenario.windowId);
  await waitWorldSettled(win);
  await openWindowOptions(page, scenario.paneId, scenario.windowId);
  const select = win.locator(`[id="${scenario.measureId}"]`).first();
  await expect(select, "the connection-point option is in the window options").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(select).toHaveAttribute("aria-label", wording(cases.texts.vortexShow));
  await expect(select).toHaveAttribute("data-published-value", scenario.initial);
  const before = await vortexCount(win);
  await pickSelectValue(page, select, scenario.changed);
  await expect(select).toHaveAttribute("data-published-value", scenario.changed, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(select).toContainText(wording(cases.texts.always));
  await expect.poll(() => vortexCount(win), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "Always shows the connection points of all building components" }).toBeGreaterThan(before);
  await pickSelectValue(page, select, scenario.initial);
  await expect(select).toHaveAttribute("data-published-value", scenario.initial, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(select).toContainText(wording(cases.texts.selected));
  await expect.poll(() => vortexCount(win), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "Selected returns to the first count" }).toBe(before);
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🌀️AggregatorVortexShow

//#region 🪣️AggregatorFill
/** 🪣️ Documented (`demonstrator.brand.tutorial.fillTool.text` and `fillDistribution.text`): the "Fill" tool fills the design with building components
 * automatically; "Set the count with the slider". Producer: `puzzle3d …/🛠️tools/🪣️fill/🦀️.rs` (tool `fill`, count measure `puzzle3d-fill-count`, a number stepper
 * since the count is unbounded, `WindowMeasureNumber`). Opens the `tool.fill` tab, commits a count above the current instance count, starts the framework
 * tool run with its documented chord (`ControlOrMeta+Enter`) if none is running, and expects more instances in the window. */
test("aggregator fill", tags("aggregator"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = cases.fill;
  await enterPane(page, scenario.paneId);
  const win = paneWindow(page, scenario.paneId, scenario.windowId);
  await waitWorldSettled(win);
  const initial = await worldInstanceIds(win);
  await openPaneTab(page, scenario.paneId, scenario.toolCategoryId, scenario.toolTabId);
  const count = page.locator(paneElementSelector(scenario.paneId, scenario.countId)).first();
  await expect(count, "the fill count entry is in the Fill tool tab").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  const wanted = initial.length + scenario.extraCount;
  await count.fill(String(wanted));
  await count.press("Tab");
  const wrapper = count.locator("xpath=ancestor::*[@data-slot='window-measure-number'][1]");
  await expect(wrapper, "the count is committed to the document config").toHaveAttribute("data-published-value", String(wanted), { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  if ((await latestToolRunGroup(page, scenario.paneId)) === "") {
    await count.focus();
    await page.keyboard.press("ControlOrMeta+Enter");
  }
  await expect.poll(() => latestToolRunGroup(page, scenario.paneId), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "Fill runs as a framework tool run" }).not.toBe("");
  await expect.poll(async () => (await worldInstanceIds(win)).length, { timeout: 3 * SURFACE_CONTENT_TIMEOUT_MS, message: "Fill adds building components to the design" }).toBeGreaterThan(initial.length);
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🪣️AggregatorFill

//#region 🔎️AussuchenFilter
/** 🔎️ Documented (`demonstrator.brand.pane.aussuchen.description`): "filter by module and typology". Producer: `sourcing …/🏊️pool/🦀️.rs` `filter_bar`
 * (row `sourcing-pool-filters`: input `sourcing-filter-query`, one toggle `sourcing-filter-module-<module>` per module, select `sourcing-filter-typology`
 * whose items are `/`-joined typology paths and "" for all). The fixture is the demo stock of `📚️examples/🎬️demo` (12 kinds, modules beams, windows, slabs, reuse). */
test("aussuchen filter", tags("aussuchen"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  const scenario = cases.filter;
  await enterPane(page, scenario.paneId);
  const pool = paneWindow(page, scenario.paneId, "sourcing-pool");
  const rowIds = () => pool.locator(".semio-table-host [data-row-id]").evaluateAll((rows) => rows.map((row) => row.getAttribute("data-row-id")!).sort());
  const expectRows = async (expected: readonly string[], label: string): Promise<void> => {
    await expect.poll(rowIds, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: label }).toEqual([...expected].sort());
  };
  await expectRows(scenario.poolRowIds, "the unfiltered pool lists the whole stock");

  const query = pool.locator('[id="sourcing-filter-query"]').locator("xpath=self::input|.//input").first();
  await expect(query).toHaveAttribute("placeholder", wording(cases.texts.searchPlaceholder));
  await query.fill(scenario.query.text);
  await query.press("Tab");
  await expectRows(scenario.query.rowIds, "the text query narrows the pool");
  await query.fill("");
  await query.press("Tab");
  await expectRows(scenario.poolRowIds, "clearing the query restores the pool");

  const module = pool.locator(`[id="sourcing-filter-module-${scenario.module.id}"]`).first();
  await module.click();
  await expectRows(scenario.module.rowIds, "the module toggle narrows the pool to that module");
  await module.click();
  await expectRows(scenario.poolRowIds, "releasing the module toggle restores the pool");

  const typology = pool.locator('[id="sourcing-filter-typology"]').first();
  await pickSelectValue(page, typology, scenario.typology.path);
  await expectRows(scenario.typology.rowIds, "the typology select narrows the pool to that path");
  await pickSelectValue(page, typology, "");
  await expect(typology).toContainText(wording(cases.texts.allTypologies));
  await expectRows(scenario.poolRowIds, "choosing all typologies restores the pool");
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🔎️AussuchenFilter

//#region ⛓️BearbeitenChain
/** ⛓️ Documented (`demonstrator.brand.pane.bearbeiten.description`): "define and visualise processing chains on the workpiece". Producer: `process3d …/📌️panels/🛍️catalogue/🦀️.rs`
 * (a capability row `process3d-catalogue.<machine>.<capability>` binds `addStep`; `🎮️commands/🪜️step/🦀️.rs` inserts a step) and the artifact tab's step rows. Adding the
 * Wall Saw's Wall Cut extends the chain by one step, changes the workpiece, and undo and redo restore both. */
test("bearbeiten chain", tags("bearbeiten"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = cases.chain;
  await enterPane(page, scenario.paneId);
  const workpiece = paneWindow(page, scenario.paneId, "process-workpiece");
  await waitWorldSettled(workpiece);
  const artifact = await openDocumentTab(page, scenario.paneId, "artifact");
  for (const id of scenario.stepIds) await expect(page.locator(paneElementSelector(scenario.paneId, id))).toHaveCount(1);
  const rows = () => artifact.locator('[role="treeitem"]').count();
  const before = await rows();
  const initial = await documentWorldProjection(workpiece);

  const catalogue = await openDocumentTab(page, scenario.paneId, "catalogue");
  const capability = catalogue.locator(`[id="${scenario.catalogueRowId}"] [data-slot="tree-label"]`);
  await expect(capability, "the Wall Saw capability is offered for this workpiece").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await capability.click();
  await openDocumentTab(page, scenario.paneId, "artifact");
  await expect.poll(rows, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "the chain gains one step" }).toBe(before + 1);
  await expect.poll(() => documentWorldProjection(workpiece), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "the new step is visualised on the workpiece" }).not.toBe(initial);
  await waitWorldSettled(workpiece);
  const changed = await documentWorldProjection(workpiece);
  for (const [direction, count, projection] of [["undo", before, initial], ["redo", before + 1, changed]] as const) {
    await historyGesture(page, workpiece, direction);
    await openDocumentTab(page, scenario.paneId, "artifact");
    await expect.poll(rows, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: `${direction} restores the chain length` }).toBe(count);
    await expect.poll(() => documentWorldProjection(workpiece), { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: `${direction} restores the workpiece` }).toBe(projection);
    await waitWorldSettled(workpiece);
  }
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion ⛓️BearbeitenChain

//#region 🛣️VerfolgenRoutes
/** 🛣️ Documented (`demonstrator.brand.pane.verfolgen.description`): "where existing components come from and where they go" and "explore the connections". Producer:
 * `gis gismap …/🎮️commands/🗺️features/🦀️.rs` `add_feature` (collection `routes` mints a two-vertex route) with the staged `collection` select of `🧾️manifest/🦀️.rs`
 * (Routes/Routen) and the inspector summaries `gis2d-play-inspector.routes-count`. The baseline adds a position and proves routes unchanged; this adds a route and proves
 * positions and regions unchanged, with undo and redo. */
test("verfolgen routes", tags("verfolgen"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  const scenario = cases.routes;
  await enterPane(page, scenario.paneId);
  const map = paneWindow(page, scenario.paneId, "gis2d-main");
  await openDocumentTab(page, scenario.paneId, "inspection");
  const summary = (kind: string) => page.locator(paneElementSelector(scenario.paneId, `gis2d-play-inspector.${kind}-count`)).locator('[data-slot="tree-description"]');
  const routes = Number(await summary("routes").innerText());
  const positions = await summary("positions").innerText();
  const regions = await summary("regions").innerText();
  expect(Number.isSafeInteger(routes)).toBe(true);
  await page.locator(paneElementSelector(scenario.paneId, `${windowElementId("gis2d-main")}.engagement.toggle`)).click();
  const actions = map.locator('[data-slot="window-action-pane"]');
  await actions.locator('[id="action.addFeature"] [data-slot="tree-label"]').click();
  const collection = actions.locator('[id="action.addFeature.arg.collection"]').getByRole("combobox");
  await pickSelectValue(page, collection, scenario.collection);
  await expect(collection).toContainText(wording(cases.texts.routes));
  await actions.locator('[id="action.addFeature.arg.label"] input').fill(scenario.label);
  await page.locator(paneElementSelector(scenario.paneId, `${windowElementId("gis2d-main")}.action.addFeature.execute`)).click();
  await expect(summary("routes")).toHaveText(String(routes + scenario.delta));
  await expect(summary("positions")).toHaveText(positions);
  await expect(summary("regions")).toHaveText(regions);
  for (const [direction, expected] of [["undo", routes], ["redo", routes + scenario.delta]] as const) {
    await historyGesture(page, map, direction);
    await expect(summary("routes")).toHaveText(String(expected));
    await expect(summary("positions")).toHaveText(positions);
    await expect(summary("regions")).toHaveText(regions);
  }
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🛣️VerfolgenRoutes

//#region 🏗️StatikModelEdit
/** 🏗️ Documented (`demonstrator.brand.pane.statik.description`): "you edit the FEM model and check the structural response". Producer: `fem 3d …/⚪️add-node/🦀️.rs`
 * (`addNode` with required numeric x, y, z; `📋️manifest/🦀️.rs`), the window action pane (as in the verfolgen baseline) and the artifact tab's section
 * `fem3d-play-artifact.nodes` headed "Nodes (N)" / "Knoten (N)" (`📌️panels/🗿️artifact/🦀️.rs`). Adds a node, expects N + 1, the result caption to settle again
 * (the structural response), and undo and redo to restore N and N + 1. */
test("statik model edit", tags("statik"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS + 3 * SURFACE_CONTENT_TIMEOUT_MS);
  const scenario = cases.model;
  await enterPane(page, scenario.paneId);
  const model = paneWindow(page, scenario.paneId, scenario.modelWindowId);
  const results = paneWindow(page, scenario.paneId, scenario.resultsWindowId);
  const caption = results.getByText(new RegExp(wording(cases.texts.caption)));
  await expect(caption, "the initial solve settles").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  const artifact = await openDocumentTab(page, scenario.paneId, "artifact");
  const nodes = async (): Promise<number> => Number(/\((\d+)\)/.exec(await artifact.getByText(new RegExp(wording(cases.texts.nodesSection))).first().innerText())?.[1] ?? Number.NaN);
  await expect.poll(nodes, { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toBeGreaterThan(0);
  const before = await nodes();
  await page.locator(paneElementSelector(scenario.paneId, `${windowElementId(scenario.modelWindowId)}.engagement.toggle`)).click();
  const actions = model.locator('[data-slot="window-action-pane"]');
  await actions.locator('[id="action.addNode"] [data-slot="tree-label"]').click();
  for (const axis of ["x", "y", "z"] as const) await actions.locator(`[id="action.addNode.arg.${axis}"] input`).fill(String(scenario.coordinates[axis]));
  await page.locator(paneElementSelector(scenario.paneId, `${windowElementId(scenario.modelWindowId)}.action.addNode.execute`)).click();
  await expect.poll(nodes, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "the model gains one node" }).toBe(before + scenario.delta);
  await expect(caption, "the replacement analysis settles after the edit").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  for (const [direction, expected] of [["undo", before], ["redo", before + scenario.delta]] as const) {
    await historyGesture(page, model, direction);
    await expect.poll(nodes, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: `${direction} restores the node count` }).toBe(expected);
    await expect(caption).toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  }
  await capturePaneArtifact(page, scenario.paneId);
});
//#endregion 🏗️StatikModelEdit

//#region ⏳️Lifecycle
/** ⏳️ Documented (`🟦️.tsx` DEMONSTRATOR_LIFECYCLE comment): "boots start 1.5 s after load, one per 35 s plugin-load budget. A pristine app is released to its poster
 * after 30 s offscreen while another is opened, 5 min idle on the overview or 60 s in a hidden tab". Producer: `LayeredOverview/🧵️pane` `usePaneLifecycle` (sweep every
 * `sweepMs` 5 s; `data-layered-pane` holds a `[data-shell-id]` only while the pane is live; the first pane is never released by time while the overview shows).
 * The thresholds are read from the landing source. Pristine means: no pointer or key input inside the pane, so these cases do not dismiss introductions by click. */
const shellOf = (page: Page, paneId: string) => page.locator(`[data-layered-pane="${paneId}"] [data-shell-id]`);

test("lifecycle stagger", tags("lifecycle"), async ({ page }) => {
  const policy = landingLifecycle();
  test.setTimeout(TEST_TIMEOUT_MS + policy.warmIntervalMs);
  const scenario = cases.lifecycle;
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await expect(shellOf(page, scenario.firstPaneId), "the first warm boot is the first pane").toHaveCount(1, { timeout: policy.warmStartMs + SURFACE_CONTENT_TIMEOUT_MS });
  const bootedAt = Date.now();
  await expect(shellOf(page, scenario.warmPaneId), "no second pane boots with the first").toHaveCount(0);
  await expect(shellOf(page, scenario.warmPaneId), "the next pane boots one interval later").toHaveCount(1, { timeout: policy.warmIntervalMs + SURFACE_CONTENT_TIMEOUT_MS });
  expect(Date.now() - bootedAt, "boots are spaced by the warm interval").toBeGreaterThanOrEqual(policy.warmIntervalMs - 5_000);
});

test("lifecycle offscreen", tags("lifecycle"), async ({ page }) => {
  const policy = landingLifecycle();
  test.setTimeout(TEST_TIMEOUT_MS + 2 * SHELL_READY_TIMEOUT_MS + policy.suspendOffscreenMs);
  const scenario = cases.lifecycle;
  await page.goto(`/#${scenario.openedPaneId}`, { waitUntil: "domcontentloaded" });
  expect(await waitForPaneShellOutcome(page, scenario.openedPaneId)).toBe("ready");
  await page.keyboard.press("Escape");
  await page.locator("[data-layered-overview-button]").click();
  await dismissLandingIntroduction(page);
  const leftAt = Date.now();
  await page.locator(`[data-layered-card="${scenario.otherPaneId}"] [data-slot="demonstrator-pane-card-open-chip"]`).click();
  await expect(page).toHaveURL(new RegExp(`#${scenario.otherPaneId}$`));
  expect(await waitForPaneShellOutcome(page, scenario.otherPaneId)).toBe("ready");
  await expect(shellOf(page, scenario.openedPaneId), "a pristine pane is released while another is opened").toHaveCount(0, { timeout: policy.suspendOffscreenMs + SHELL_READY_TIMEOUT_MS });
  expect(Date.now() - leftAt, "not before the offscreen threshold").toBeGreaterThanOrEqual(policy.suspendOffscreenMs - 2_000);
  await expect(shellOf(page, scenario.otherPaneId), "the opened pane stays").toHaveCount(1);
});

/** 🧭️ Dismisses the landing's own introduction, which is not inside any pane. */
async function dismissLandingIntroduction(page: Page): Promise<void> {
  const skip = page.locator('[id="ui.introduction.skip"]').first();
  if (await skip.isVisible({ timeout: 3_000 }).catch(() => false)) await skip.click();
}

test("lifecycle idle", tags("lifecycle"), async ({ page }) => {
  const policy = landingLifecycle();
  test.setTimeout(TEST_TIMEOUT_MS + SHELL_READY_TIMEOUT_MS);
  const scenario = cases.lifecycle;
  await page.clock.install();
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await expect(shellOf(page, scenario.firstPaneId)).toHaveCount(1, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await page.clock.fastForward(policy.warmIntervalMs + 1_000);
  await expect(shellOf(page, scenario.warmPaneId), "the warm queue boots the second pane").toHaveCount(1, { timeout: SHELL_READY_TIMEOUT_MS });
  expect(await waitForPaneShellOutcome(page, scenario.warmPaneId)).toBe("ready");
  await page.clock.fastForward(policy.suspendIdleMs + 10_000);
  await expect(shellOf(page, scenario.warmPaneId), "a pristine pane is released after the idle threshold on the overview").toHaveCount(0, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(shellOf(page, scenario.firstPaneId), "the most recently opened pane (initially the first) is never released by time").toHaveCount(1);
});

test("lifecycle hidden", tags("lifecycle"), async ({ page }) => {
  const policy = landingLifecycle();
  test.setTimeout(TEST_TIMEOUT_MS + SHELL_READY_TIMEOUT_MS);
  const scenario = cases.lifecycle;
  await page.addInitScript(() => {
    Object.defineProperty(Document.prototype, "hidden", { configurable: true, get: () => (window as unknown as { __semioHidden?: boolean }).__semioHidden === true });
  });
  await page.clock.install();
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await expect(shellOf(page, scenario.firstPaneId)).toHaveCount(1, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await page.clock.fastForward(policy.warmIntervalMs + 1_000);
  await expect(shellOf(page, scenario.warmPaneId)).toHaveCount(1, { timeout: SHELL_READY_TIMEOUT_MS });
  expect(await waitForPaneShellOutcome(page, scenario.warmPaneId)).toBe("ready");
  await page.clock.fastForward(policy.suspendHiddenMs + 10_000);
  await page.waitForTimeout(12_000);
  await expect(shellOf(page, scenario.warmPaneId), "a visible tab keeps a pristine pane until the idle threshold").toHaveCount(1);
  await page.evaluate(() => { (window as unknown as { __semioHidden?: boolean }).__semioHidden = true; });
  await expect(shellOf(page, scenario.warmPaneId), "a hidden tab releases a pristine pane after the hidden threshold").toHaveCount(0, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(shellOf(page, scenario.firstPaneId)).toHaveCount(1);
});
//#endregion ⏳️Lifecycle
