// #region 🧲️Header
// 💻️ .storybook/framework-hosts-wasm.spec.ts
// Specs: End-to-end checks for the framework renderer hosts backed by prebuilt Rust/WASM engines, inside the
// aggregated Storybook static build: `NodeGraphHost` (workflow + flow-graph variants), `TextEditorHost`,
// `Paint2dHost`, `TiledMapHost`, `WorldTerrainLayer`, `World3dHost`.
// Summary: Loads each story's `iframe.html?id=...`, asserts a clean boot (no page/console errors, storybook-root
// populated) and a host-specific marker element/text. Tile/DEM fetches against intentionally-missing storybook
// paths produce benign `Failed to load resource … 404` console entries, which `significantConsoleErrors` filters
// out (mirrors `puzzle-2d.spec.ts`/`coda-trees.spec.ts`).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";

//#region Helpers
function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectHostStory(page: Page, storyId: string): Promise<void> {
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
}
//#endregion Helpers

//#region NodeGraphHost
test("NodeGraphHost workflow: boots the real GraphSession WASM engine and renders the host shell", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-nodegraphhost--workflow");
  await expect(page.locator(".semio-node-graph-host")).toBeVisible();
});

test("NodeGraphHost flow graph: boots the real FlowSession WASM engine (isFlowGraphScene routing)", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-nodegraphhost--flow-graph");
  await expect(page.locator(".semio-node-graph-host")).toBeVisible();
});
//#endregion NodeGraphHost

//#region TextEditorHost
test("TextEditorHost: boots the real EditorSession WASM engine and shows the document buffer", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-texteditorhost--jack-document");
  await expect(page.locator(".semio-text-editor-host")).toBeVisible();
  await expect(page.locator("body")).toContainText("MATCH");
});

test("TextEditorHost with diagnostics: renders the diagnostics readout", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-texteditorhost--with-diagnostics");
  await expect(page.locator("body")).toContainText("example diagnostic");
});
//#endregion TextEditorHost

//#region Paint2dHost
test("Paint2dHost composite view: boots the real RasterSession WASM engine", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-paint2dhost--composite-view");
  await expect(page.locator(".semio-paint-2d-canvas-surface")).toBeVisible();
  await expect(page.locator('[data-view-mode="composite"]').first()).toBeVisible();
});

test("Paint2dHost navigator view: shows the composite viewport overlay channel", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-paint2dhost--navigator-view");
  await expect(page.locator('[data-view-mode="navigator"]').first()).toBeVisible();
});
//#endregion Paint2dHost

//#region TiledMapHost
test("TiledMapHost vector render: boots the real MapSession WASM engine", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-tiledmaphost--vector-render");
});

test("TiledMapHost image render: boots with the image render mode", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-tiledmaphost--image-render");
});
//#endregion TiledMapHost

//#region World3dHost
test("World3dHost minimal viewport: renders the pure r3f viewport (no WASM engine)", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-world3dhost--minimal-viewport");
});

test("World3dHost terrain viewport: mounts WorldTerrainLayer against the real TerrainSession WASM engine", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-world3dhost--terrain-viewport");
});
//#endregion World3dHost

//#region WorldTerrainLayer
test("WorldTerrainLayer hypsometric ramp: boots the real TerrainSession WASM engine inside a standalone r3f Canvas", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-worldterrainlayer--hypsometric-ramp");
});
//#endregion WorldTerrainLayer
