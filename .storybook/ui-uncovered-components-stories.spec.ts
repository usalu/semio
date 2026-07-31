// #region 🧲️Header
// 💻️ .storybook/ui-uncovered-components-stories.spec.ts
// Specs: End-to-end smoke coverage for the "ui" scope's newly authored stories covering the previously-uncovered
// barrel exports BasicChatPanel, CanvasPickMenu/useCanvasPickInteraction, ContextMenu/ContextMenuController,
// DragHandle (DragAndDrop), Field, FileTree, Geometry, HistoryTable, IconSelector, IconShotFrame/iconShotFrameStyle/
// clipIconSvgMarkupToEllipse, and Label.
// Summary: Drives each story id through the aggregated Storybook static build's `iframe.html` and asserts a mounted
// root plus zero page/console errors — no visual/pixel assertions, this is boot-health coverage, not per-component
// behavior testing (each component's own barrel-level unit tests already cover behavior).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";

const UNCOVERED_UI_STORY_IDS: readonly string[] = [
  "🖱️ui⚛️react-basicchatpanel--default",
  "🖱️ui⚛️react-basicchatpanel--narrow",
  "🖱️ui⚛️react-canvaspickmenu--default",
  "🖱️ui⚛️react-canvaspickmenu--dismissed",
  "🖱️ui⚛️react-canvaspickmenu--pointer-interaction",
  "🖱️ui⚛️react-contextmenu--default",
  "🖱️ui⚛️react-contextmenu--no-items",
  "🖱️ui⚛️react-contextmenu--controlled",
  "🖱️ui⚛️react-draganddrop--default",
  "🖱️ui⚛️react-draganddrop--emphasized",
  "🖱️ui⚛️react-draganddrop--reorderable-rows",
  "🖱️ui⚛️react-field--default",
  "🖱️ui⚛️react-field--with-description-and-error",
  "🖱️ui⚛️react-filetree--default",
  "🖱️ui⚛️react-filetree--with-current-path",
  "🖱️ui⚛️react-filetree--navigable",
  "🖱️ui⚛️react-geometry--default",
  "🖱️ui⚛️react-geometry--selected-and-hovered",
  "🖱️ui⚛️react-historytable--default",
  "🖱️ui⚛️react-historytable--empty",
  "🖱️ui⚛️react-historytable--selectable",
  "🖱️ui⚛️react-iconselector--default",
  "🖱️ui⚛️react-iconselector--emoji",
  "🖱️ui⚛️react-iconselector--disabled",
  "🖱️ui⚛️react-iconshotframe--default",
  "🖱️ui⚛️react-iconshotframe--ellipse-shape",
  "🖱️ui⚛️react-iconshotframe--landscape-no-badge",
  "🖱️ui⚛️react-iconshotframe--clip-icon-svg-markup-to-ellipse-story",
  "🖱️ui⚛️react-iconshotframe--icon-shot-frame-style-story",
  "🖱️ui⚛️react-label--default",
  "🖱️ui⚛️react-label--tree-group-header",
  "🖱️ui⚛️react-label--fallback-from-id",
];

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectStoryMounts(page: Page, storyId: string): Promise<void> {
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

for (const storyId of UNCOVERED_UI_STORY_IDS) {
  test(`ui scope story "${storyId}" mounts with no console/page errors`, async ({ page }) => {
    await expectStoryMounts(page, storyId);
  });
}
