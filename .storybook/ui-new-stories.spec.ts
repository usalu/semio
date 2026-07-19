// #region 🧲Header
// 💻 .storybook/ui-new-stories.spec.ts
// Specs: End-to-end smoke coverage for the "ui" scope's newly authored stories (MobilePanel, PanelTabBar/PanelChromeTabBar, Scene, SelectionMarquee, Shell*Panel/Dialog, Skeletons, SortableTreeItems, UIDialog, UIIntroduction, UnifiedGumball, NavbarExampleSelect, ActionDropdown, Providers).
// Summary: Drives each story id through the aggregated Storybook static build's `iframe.html` and asserts a mounted root plus zero page/console errors — no visual/pixel assertions, this is boot-health coverage, not per-component behavior testing (each component's own barrel-level unit tests already cover behavior).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Page } from "@playwright/test";

const NEW_UI_STORY_IDS: readonly string[] = [
  "🖱️ui⚛️react-mobilepanel--default",
  "🖱️ui⚛️react-mobilepanel--hidden",
  "🖱️ui⚛️react-paneltabbar--panel-variant",
  "🖱️ui⚛️react-paneltabbar--mobile-variant",
  "🖱️ui⚛️react-paneltabbar--chrome-hosted",
  "🖱️ui⚛️react-scene--default",
  "🖱️ui⚛️react-scene--orthographic",
  "🖱️ui⚛️react-scene--without-chrome",
  "🖱️ui⚛️react-selectionmarquee--rect-full",
  "🖱️ui⚛️react-selectionmarquee--rect-partial",
  "🖱️ui⚛️react-selectionmarquee--polygon",
  "🖱️ui⚛️react-shelldisplaypanel--default",
  "🖱️ui⚛️react-shelldisplaypanel--compact-on",
  "🖱️ui⚛️react-shellfinddialog--default",
  "🖱️ui⚛️react-shellfinddialog--filtered",
  "🖱️ui⚛️react-shellsearchdialog--default",
  "🖱️ui⚛️react-shellsearchdialog--filtered",
  "🖱️ui⚛️react-shellsearchdialog--empty",
  "🖱️ui⚛️react-shellsettingspanel--default",
  "🖱️ui⚛️react-shellsettingspanel--expert-mode",
  "🖱️ui⚛️react-skeletons--table",
  "🖱️ui⚛️react-skeletons--diagram",
  "🖱️ui⚛️react-skeletons--loading-row-story",
  "🖱️ui⚛️react-skeletons--scene",
  "🖱️ui⚛️react-sortabletreeitems--default",
  "🖱️ui⚛️react-sortabletreeitems--single-item",
  "🖱️ui⚛️react-uidialog--staged-form",
  "🖱️ui⚛️react-uidialog--confirm-only",
  "🖱️ui⚛️react-uiintroduction--first-step",
  "🖱️ui⚛️react-uiintroduction--last-step",
  "🖱️ui⚛️react-unifiedgumball--move-rotate-scale",
  "🖱️ui⚛️react-unifiedgumball--move-only",
  "🖱️ui⚛️react-navbarexampleselect--default",
  "🖱️ui⚛️react-navbarexampleselect--without-no-example-option",
  "🖱️ui⚛️react-actiondropdown--default",
  "🖱️ui⚛️react-actiondropdown--with-transaction",
  "🖱️ui⚛️react-providers--ghost",
  "🖱️ui⚛️react-providers--interaction",
  "🖱️ui⚛️react-providers--panel-dock",
  "🖱️ui⚛️react-providers--transaction",
  "🖱️ui⚛️react-providers--tree-state",
  "🖱️ui⚛️react-providers--chrome-compact",
  "🖱️ui⚛️react-providers--chrome-label-policy",
  "🖱️ui⚛️react-providers--flow",
  "🖱️ui⚛️react-providers--glass-tier",
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

for (const storyId of NEW_UI_STORY_IDS) {
  test(`ui scope story "${storyId}" mounts with no console/page errors`, async ({ page }) => {
    await expectStoryMounts(page, storyId);
  });
}
