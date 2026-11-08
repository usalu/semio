// #region 🧲️Header
// 🔁️ Served behaviour acceptance, one cycle per pane: boot, authored fixture visible, one act, undo, redo, in `en` and `de`.
// Written, not run: no activation had served the demonstrator when this file was authored; every selector is read from a producer or the baseline suite.
// Run apart from the baseline: PLAYWRIGHT_BASE_URL=<served demonstrator> playwright test --config <🎚️config> --grep @cycle [--grep @<pane>]
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";

import { expect, type Locator, type Page } from "@playwright/test";
import {
  SURFACE_CONTENT_TIMEOUT_MS, TEST_TIMEOUT_MS,
  capturePaneArtifact, documentWorldProjection, enterPane, historyGesture, openDocumentTab, paneElementSelector, paneWindow, test, waitWorldSettled, windowElementId, wording, worldInstanceIds,
  type Wording,
} from "./🧰️pane/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🧬️Cases
const read = <T>(path: string): T => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8")) as T;
const edits = read<{
  readonly generator: { readonly heightId: string; readonly initial: number; readonly changed: number; readonly widgetIds: readonly string[] };
  readonly koordinator: { readonly catalogueId: string; readonly windowIds: readonly string[]; readonly rowText: Wording };
  readonly aggregator: { readonly kindId: string; readonly seedId: string; readonly windowIds: readonly string[] };
  readonly bearbeiten: { readonly stepId: string; readonly stepIds: readonly string[] };
  readonly verfolgen: { readonly label: string; readonly delta: number; readonly longitude: number; readonly latitude: number };
}>("../🧬️edits/🧫️cases.json");
const curation = read<{ readonly objectId: string; readonly name: string; readonly availability: number }>("../🧬️curation/🧫️cases.json");
const simulation = read<{ readonly fem: { readonly paneId: string; readonly panelTabId: string; readonly panelCategoryId: string; readonly caption: string; readonly phase: number; readonly initialScale: number; readonly changedScale: number } }>("../🧬️simulation/🧫️cases.json");
const behaviours = read<{
  readonly texts: Readonly<Record<"renameZone", Wording>>;
  readonly cycle: { readonly energie: { readonly paneId: string; readonly zoneWindowId: string; readonly modelWindowId: string; readonly actionId: string; readonly newName: string } };
}>("./🧫️cases.json");
const enabledLabel: Wording = { en: "Enabled", de: "Aktiviert" };
const removeLabel: Wording = { en: "Remove", de: "Entfernen" };
const captionPattern: Wording = { en: "^Case: dead$", de: "^Lastfall: dead$" };
//#endregion 🧬️Cases

//#region 🔁️Cycle
const tags = (pane: string): { tag: string[] } => ({ tag: ["@behaviour", "@cycle", `@${pane}`] });
const budget = (): number => TEST_TIMEOUT_MS + 4 * SURFACE_CONTENT_TIMEOUT_MS;

/** 🔁️ The one act-undo-redo law of every pane: the fingerprint moves on the act, returns on undo and returns again on redo. */
async function actUndoRedo(fingerprint: () => Promise<string>, act: () => Promise<void>, history: (direction: "undo" | "redo") => Promise<void>): Promise<void> {
  const before = await fingerprint();
  await act();
  await expect.poll(fingerprint, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "the act must change the document projection" }).not.toBe(before);
  const after = await fingerprint();
  await history("undo");
  await expect.poll(fingerprint, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "undo must restore the projection before the act" }).toBe(before);
  await history("redo");
  await expect.poll(fingerprint, { timeout: SURFACE_CONTENT_TIMEOUT_MS, message: "redo must restore the projection after the act" }).toBe(after);
}

/** 🕸️ The node-graph snapshot of a window without its ephemeral fields. */
async function graphFingerprint(graph: Locator): Promise<string> {
  const raw = await graph.locator(".semio-node-graph-host").first().getAttribute("data-host-snapshot-json");
  const snapshot = JSON.parse(raw ?? "{}") as { readonly widgets?: readonly unknown[]; readonly synapses?: readonly unknown[] };
  return JSON.stringify({ widgets: snapshot.widgets ?? [], synapses: snapshot.synapses ?? [] });
}

/** 🎬️ Stages and executes one window action through the window's action pane, as a user does. */
async function runWindowAction(page: Page, paneId: string, windowId: string, actionId: string, args: Readonly<Record<string, string>>): Promise<void> {
  const window = paneWindow(page, paneId, windowId);
  const toggle = page.locator(paneElementSelector(paneId, `${windowElementId(windowId)}.engagement.toggle`));
  await toggle.click();
  const actions = window.locator('[data-slot="window-action-pane"]');
  await actions.locator(`[id="action.${actionId}"] [data-slot="tree-label"]`).click();
  for (const [name, value] of Object.entries(args)) await actions.locator(`[id="action.${actionId}.arg.${name}"] input`).fill(value);
  await page.locator(paneElementSelector(paneId, `${windowElementId(windowId)}.action.${actionId}.execute`)).click();
}
//#endregion 🔁️Cycle

test("generator cycle", tags("generator"), async ({ page }) => {
  test.setTimeout(budget());
  await enterPane(page, "generator");
  const graph = paneWindow(page, "generator", "procedural-main");
  const preview = paneWindow(page, "generator", "procedural-preview");
  await waitWorldSettled(preview);
  await expect.poll(async () => (JSON.parse((await graph.locator(".semio-node-graph-host").first().getAttribute("data-host-snapshot-json")) ?? "{}") as { widgets?: { id: string }[] }).widgets?.map((widget) => widget.id).sort(), { timeout: SURFACE_CONTENT_TIMEOUT_MS }).toEqual([...edits.generator.widgetIds].sort());
  await openDocumentTab(page, "generator", "artifact");
  await page.locator(paneElementSelector("generator", edits.generator.heightId)).locator('[data-slot="tree-label"]').click();
  await openDocumentTab(page, "generator", "inspection");
  const input = page.locator(paneElementSelector("generator", "procedural-play-inspector.value.input"));
  await expect(input).toHaveValue(String(edits.generator.initial));
  await actUndoRedo(
    async () => JSON.stringify([await graphFingerprint(graph), await documentWorldProjection(preview)]),
    async () => {
      await input.fill(String(edits.generator.changed));
      await input.press("Tab");
    },
    (direction) => historyGesture(page, graph, direction),
  );
  await expect(input).toHaveValue(String(edits.generator.changed));
  await capturePaneArtifact(page, "generator");
});

test("koordinator cycle", tags("koordinator"), async ({ page }) => {
  test.setTimeout(budget());
  await enterPane(page, "koordinator");
  const windows = edits.koordinator.windowIds.map((id) => paneWindow(page, "koordinator", id));
  for (const window of windows) await waitWorldSettled(window);
  const idle = await Promise.all(windows.slice(1).map(documentWorldProjection));
  await windows[0]!.locator("canvas").first().click({ position: { x: 8, y: 8 } });
  await openDocumentTab(page, "koordinator", "catalogue");
  await actUndoRedo(
    async () => documentWorldProjection(windows[0]!),
    async () => {
      await page.locator(paneElementSelector("koordinator", edits.koordinator.catalogueId)).locator('[data-slot="tree-label"]').click();
      await openDocumentTab(page, "koordinator", "artifact");
      await expect(page.locator(`[data-shell-id="koordinator"] [data-slot="tree-label"]`).filter({ hasText: new RegExp(wording(edits.koordinator.rowText)) }).first()).toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
    },
    (direction) => historyGesture(page, windows[0]!, direction),
  );
  expect(await Promise.all(windows.slice(1).map(documentWorldProjection)), "the other three models stay unchanged").toEqual(idle);
  await capturePaneArtifact(page, "koordinator");
});

test("aggregator cycle", tags("aggregator"), async ({ page }) => {
  test.setTimeout(budget());
  await enterPane(page, "aggregator");
  const windows = edits.aggregator.windowIds.map((id) => paneWindow(page, "aggregator", id));
  for (const window of windows) {
    await waitWorldSettled(window);
    expect(await worldInstanceIds(window), "the authored seed instance is visible").toContain(edits.aggregator.seedId);
  }
  const catalogueTab = await openDocumentTab(page, "aggregator", "catalogue");
  await actUndoRedo(
    async () => JSON.stringify(await Promise.all(windows.map(worldInstanceIds))),
    async () => {
      await catalogueTab.locator(`[id="${edits.aggregator.kindId}"] [data-slot="tree-label"]`).click();
    },
    (direction) => historyGesture(page, windows[0]!, direction),
  );
  await capturePaneArtifact(page, "aggregator");
});

test("energie cycle", tags("energie"), async ({ page }) => {
  test.setTimeout(budget());
  const scenario = behaviours.cycle.energie;
  await enterPane(page, scenario.paneId);
  const model = paneWindow(page, scenario.paneId, scenario.modelWindowId);
  const zones = paneWindow(page, scenario.paneId, scenario.zoneWindowId);
  await waitWorldSettled(model);
  const rows = zones.locator(".semio-table-host [data-row-id]");
  await expect(rows.first(), "the authored zone is listed").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  const zoneId = ((await rows.first().locator("td").first().innerText()) ?? "").trim();
  expect(zoneId, "the zone id is the first cell; data-row-id is only the row position").toMatch(/^\d+$/);
  await actUndoRedo(
    async () => (await rows.allInnerTexts()).join("\n"),
    () => runWindowAction(page, scenario.paneId, scenario.zoneWindowId, scenario.actionId, { zone: zoneId, newName: scenario.newName }),
    (direction) => historyGesture(page, model, direction),
  );
  await expect(rows.first()).toContainText(scenario.newName);
  await capturePaneArtifact(page, scenario.paneId);
});

test("aussuchen cycle", tags("aussuchen"), async ({ page }) => {
  test.setTimeout(budget());
  await enterPane(page, "aussuchen");
  const pool = paneWindow(page, "aussuchen", "sourcing-pool");
  const curated = paneWindow(page, "aussuchen", "sourcing-curated");
  const grid = paneWindow(page, "aussuchen", "sourcing-grid");
  const stock = pool.locator(`[data-row-id="${curation.objectId}"]`);
  const count = stock.getByRole("spinbutton");
  await expect(stock).toContainText(curation.name, { timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await expect(count).toHaveAttribute("aria-valuemax", String(curation.availability));
  await expect(count).toHaveValue("0");
  await expect(curated.locator("[data-row-id]")).toHaveCount(0);
  await actUndoRedo(
    async () => JSON.stringify([await count.inputValue(), await curated.locator("[data-row-id]").count(), await worldInstanceIds(grid)]),
    async () => {
      await count.press("ArrowUp");
      await expect(curated.locator(`[data-row-id="${curation.objectId}"]`).getByRole("button", { name: wording(removeLabel), exact: true })).toBeVisible();
    },
    async (direction) => {
      await stock.getByText(curation.name, { exact: true }).click();
      await page.keyboard.press(direction === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
    },
  );
  await expect(count).toHaveValue("1");
  await capturePaneArtifact(page, "aussuchen");
});

test("bearbeiten cycle", tags("bearbeiten"), async ({ page }) => {
  test.setTimeout(budget());
  await enterPane(page, "bearbeiten");
  const workpiece = paneWindow(page, "bearbeiten", "process-workpiece");
  await waitWorldSettled(workpiece);
  await openDocumentTab(page, "bearbeiten", "artifact");
  for (const id of edits.bearbeiten.stepIds) await expect(page.locator(paneElementSelector("bearbeiten", id))).toHaveCount(1);
  const row = page.locator(paneElementSelector("bearbeiten", edits.bearbeiten.stepId));
  await actUndoRedo(
    async () => documentWorldProjection(workpiece),
    async () => {
      await row.getByRole("button", { name: wording(enabledLabel), exact: true }).click();
      await waitWorldSettled(workpiece);
    },
    async (direction) => {
      await historyGesture(page, workpiece, direction);
      await waitWorldSettled(workpiece);
    },
  );
  await capturePaneArtifact(page, "bearbeiten");
});

test("verfolgen cycle", tags("verfolgen"), async ({ page }) => {
  test.setTimeout(TEST_TIMEOUT_MS);
  await enterPane(page, "verfolgen");
  const map = paneWindow(page, "verfolgen", "gis2d-main");
  await expect(map.locator(".semio-tiled-map-host canvas")).toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await openDocumentTab(page, "verfolgen", "inspection");
  const summary = (kind: string): Locator => page.locator(paneElementSelector("verfolgen", `gis2d-play-inspector.${kind}-count`)).locator('[data-slot="tree-description"]');
  expect(Number.isSafeInteger(Number(await summary("positions").innerText())), "the authored positions are counted").toBe(true);
  const untouched = JSON.stringify([await summary("routes").innerText(), await summary("regions").innerText()]);
  await actUndoRedo(
    async () => summary("positions").innerText(),
    async () => {
      await page.locator(paneElementSelector("verfolgen", "framework.window.gis2dMain.engagement.toggle")).click();
      const actions = map.locator('[data-slot="window-action-pane"]');
      await actions.locator('[id="action.addFeature"] [data-slot="tree-label"]').click();
      await actions.locator('[id="action.addFeature.arg.label"] input').fill(edits.verfolgen.label);
      await expect(actions.locator('[id="action.addFeature.arg.lon"] input')).toHaveValue(String(edits.verfolgen.longitude));
      await expect(actions.locator('[id="action.addFeature.arg.lat"] input')).toHaveValue(String(edits.verfolgen.latitude));
      await page.locator(paneElementSelector("verfolgen", "framework.window.gis2dMain.action.addFeature.execute")).click();
    },
    (direction) => historyGesture(page, map, direction),
  );
  expect(JSON.stringify([await summary("routes").innerText(), await summary("regions").innerText()]), "routes and regions stay unchanged").toBe(untouched);
  await capturePaneArtifact(page, "verfolgen");
});

test("statik cycle", tags("statik"), async ({ page }) => {
  test.setTimeout(budget());
  const scenario = simulation.fem;
  await enterPane(page, scenario.paneId);
  const model = paneWindow(page, scenario.paneId, "fem3d-model");
  const results = paneWindow(page, scenario.paneId, "fem3d-results");
  await waitWorldSettled(model);
  await waitWorldSettled(results);
  const caption = results.getByText(new RegExp(wording(captionPattern)));
  await expect(caption, "the initial solve settles without a pending suffix").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await caption.click();
  const shell = page.locator(`[data-shell-id="${scenario.paneId}"]`);
  const tab = shell.locator(`[data-tab-id="${scenario.panelTabId}"]:visible`).first();
  if (!(await tab.isVisible())) await shell.locator(`[data-tab-id="${scenario.panelCategoryId}"]:visible`).first().click();
  if ((await tab.getAttribute("aria-pressed")) !== "true") await tab.click();
  const scale = page.locator(paneElementSelector(scenario.paneId, "fem3d-play-results.deformation-scale.control"));
  await expect(scale).toHaveValue(String(scenario.initialScale));
  await actUndoRedo(
    async () => JSON.stringify([await scale.inputValue(), await documentWorldProjection(results)]),
    async () => {
      await scale.fill(String(scenario.changedScale));
      await scale.press("Tab");
    },
    async (direction) => {
      await caption.click();
      await page.keyboard.press(direction === "undo" ? "ControlOrMeta+z" : "ControlOrMeta+Shift+z");
    },
  );
  await expect(scale).toHaveValue(String(scenario.changedScale));
  await expect(caption, "the replacement analysis is current").toBeVisible({ timeout: SURFACE_CONTENT_TIMEOUT_MS });
  await capturePaneArtifact(page, scenario.paneId);
});
