// #region 🧲️Header
// 💻️ .storybook/framework-hosts-no-wasm.spec.ts
// Specs: End-to-end checks for the `framework/hosts` scope's zero-WASM host stories (`TableHost`,
// `BlockListHost`, `GraphTimelineHost`, `IconRenderHost`, `InkCanvasHost`, `Canvas2dHost`, `UiInterpreter`).
// Summary: Every story id gets a load check (200-equivalent — no "Couldn't find story"/preview-file-failure
// text, no page/console errors); the reducer-backed hosts additionally get one real interaction assertion
// against their `<pre data-testid="…-host-debug">` readout, modeled on `.storybook/puzzle-2d.spec.ts`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

/** @emoji 🧪️ Navigates to one story's iframe and asserts it actually rendered (no missing-story/preview-failure text, no page/console errors) — the shared "loads cleanly" assertion every story id below gets. */
async function expectStoryLoads(page: Page, storyId: string): Promise<{ readonly pageErrors: Error[]; readonly consoleErrors: string[] }> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  return { pageErrors, consoleErrors };
}

async function readDebug<T>(page: Page, testId: string): Promise<T> {
  const text = await page.getByTestId(testId).innerText();
  return JSON.parse(text) as T;
}

//#region TableHost
// 🐛️ `@semio-tech/ui-react`'s `Table` component destructures `sortColumn`/`sortDirection`/`onSort` (and
// `TableColumn.sortable`) but its `<th>` header cell renders as plain `{column.header}` text with no
// `onClick` anywhere in the component body — column-header sorting is wired all the way down to
// `TableHost`'s `onSort` dispatch, but there is no way for a user to actually trigger it, so `sortTable`
// is unreachable through real interaction. `reduceStoryTableAction`'s `"sortTable"` case is kept (it's
// what a real host app would do once the header becomes clickable) but this spec only exercises the
// interaction that's actually wired end to end today: row click → `selectRow`.
test("TableHost sortable-with-actions: loads with the initial row order, row click selects it", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-tablehost--sortable-with-actions");
  type Debug = { readonly order: readonly string[]; readonly selectedIds: readonly string[]; readonly sort: unknown };
  const before = await readDebug<Debug>(page, "table-host-debug");
  expect(before.order).toEqual(["row-beta", "row-alpha", "row-gamma"]);
  expect(before.selectedIds).toEqual([]);

  await page.getByText("Gamma").click();
  await expect.poll(async () => (await readDebug<Debug>(page, "table-host-debug")).selectedIds).toEqual(["row-gamma"]);
});

test("TableHost empty-scene: renders the empty-scene fallback with zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-tablehost--empty-scene");
  await expect(page.locator(".semio-table-empty")).toBeVisible();
});
//#endregion TableHost

//#region BlockListHost
test("BlockListHost editable: loads with two steps, Add Step round-trips a third", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-blocklisthost--editable");
  type Debug = { readonly steps: readonly { readonly id: string; readonly blocks: readonly string[] }[] };
  const before = await readDebug<Debug>(page, "block-list-host-debug");
  expect(before.steps.map((step) => step.id)).toEqual(["step-1", "step-2"]);

  await page.getByRole("button", { name: "Add Step" }).click();
  await expect.poll(async () => (await readDebug<Debug>(page, "block-list-host-debug")).steps.map((step) => step.id)).toEqual(["step-1", "step-2", "step-3"]);
});

test("BlockListHost empty-scene: renders the empty-scene fallback with zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-blocklisthost--empty-scene");
  await expect(page.locator(".semio-block-list-empty")).toBeVisible();
});
//#endregion BlockListHost

//#region GraphTimelineHost
test("GraphTimelineHost branching: clicking a row dispatches checkoutCheckpoint", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-graphtimelinehost--branching");
  type Debug = { readonly lastAction: { readonly action: string; readonly args: { readonly checkpointId?: string } } | null };
  expect((await readDebug<Debug>(page, "graph-timeline-host-debug")).lastAction).toBeNull();

  await page.getByText("feature-hosts").click();
  const after = await readDebug<Debug>(page, "graph-timeline-host-debug");
  expect(after.lastAction?.action).toBe("checkoutCheckpoint");
  expect(after.lastAction?.args?.checkpointId).toBe("c4");
});

test("GraphTimelineHost linear/empty: both load with zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-graphtimelinehost--linear");
  await page.goto(`iframe.html?id=🛠️framework🔌️hosts-graphtimelinehost--empty&viewMode=story`);
  await expect(page.getByTestId("graph-timeline-table")).toContainText("—");
});
//#endregion GraphTimelineHost

//#region IconRenderHost
test("IconRenderHost toolbar-format/svg-fixed/png-fixed: all load and render a preview with zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-iconrenderhost--toolbar-format");
  await expect(page.getByText(/^format: (png|svg)$/)).toBeVisible();

  await expectStoryLoads(page, "🛠️framework🔌️hosts-iconrenderhost--svg-fixed");
  await expect(page.getByText("format: svg")).toBeVisible();

  await expectStoryLoads(page, "🛠️framework🔌️hosts-iconrenderhost--png-fixed");
  await expect(page.getByText("format: png")).toBeVisible();
});
//#endregion IconRenderHost

//#region InkCanvasHost
test("InkCanvasHost editable: clicking the text block selects it", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-inkcanvashost--editable");
  type Debug = { readonly blockCount: number; readonly selection: readonly string[] };
  const before = await readDebug<Debug>(page, "ink-canvas-host-debug");
  expect(before.blockCount).toBe(3);
  expect(before.selection).toEqual([]);

  await page.getByText("Design notes").click();
  await expect.poll(async () => (await readDebug<Debug>(page, "ink-canvas-host-debug")).selection).toEqual(["text-1"]);
});

test("InkCanvasHost navigator-preview: loads read-only with zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-inkcanvashost--navigator-preview");
});
//#endregion InkCanvasHost

//#region Canvas2dHost
test("Canvas2dHost scene: wheel-zoom round-trips the camera into the debug readout", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-canvas2dhost--scene");
  type Debug = { readonly camera: { readonly zoom: number }; readonly layerCount: number };
  const before = await readDebug<Debug>(page, "canvas-2d-host-debug");
  expect(before.layerCount).toBe(3);

  const canvas = page.locator(".semio-canvas-2d-host canvas");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
  for (let index = 0; index < 8; index += 1) await page.mouse.wheel(0, -120);
  await expect.poll(async () => (await readDebug<Debug>(page, "canvas-2d-host-debug")).camera.zoom).toBeGreaterThan(before.camera.zoom);
});

test("Canvas2dHost empty-canvas: loads with zero layers and zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-canvas2dhost--empty-canvas");
  type Debug = { readonly layerCount: number };
  expect((await readDebug<Debug>(page, "canvas-2d-host-debug")).layerCount).toBe(0);
});
//#endregion Canvas2dHost

//#region UiInterpreter
test("UiInterpreter button: clicking the button dispatches addItem", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-uiinterpreter--button");
  type Debug = { readonly lastAction: { readonly action: string } | null };
  expect((await readDebug<Debug>(page, "ui-interpreter-debug")).lastAction).toBeNull();

  await page.getByRole("button", { name: "Add Item" }).click();
  await expect.poll(async () => (await readDebug<Debug>(page, "ui-interpreter-debug")).lastAction?.action).toBe("addItem");
});

test("UiInterpreter tree: toggling the Visible control dispatches toggleVisible", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-uiinterpreter--tree");
  await expect(page.getByText("Background")).toBeVisible();
  await expect(page.getByText("Foreground")).toBeVisible();

  type Debug = { readonly lastAction: { readonly action: string } | null };
  await page.getByText("Visible").click();
  await expect.poll(async () => (await readDebug<Debug>(page, "ui-interpreter-debug")).lastAction?.action).toBe("toggleVisible");
});

test("UiInterpreter panel: renders the field/section/slider/keyValue shape with zero console errors", async ({ page }) => {
  await expectStoryLoads(page, "🛠️framework🔌️hosts-uiinterpreter--panel");
  await expect(page.getByText("Properties")).toBeVisible();
  await expect(page.getByText("node-42")).toBeVisible();
});
//#endregion UiInterpreter
