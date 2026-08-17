// #region 🧲️Header
// 💻️ .storybook/ui-new-stories.spec.ts
// Specs: End-to-end smoke coverage for the "ui" scope's newly authored stories (PanelTabBar/PanelChromeTabBar, Scene, SelectionMarquee, Shell*Panel/Dialog, Skeletons, SortableTreeItems, UIDialog, UIIntroduction, UnifiedGumball, NavbarExampleSelect, ActionDropdown, Providers).
// Summary: Drives each story id through the aggregated Storybook static build's `iframe.html` and asserts a mounted root plus zero page/console errors — no visual/pixel assertions, this is boot-health coverage, not per-component behavior testing (each component's own barrel-level unit tests already cover behavior).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

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
  "🖱️ui⚛️react-shellsettingspanel--default",
  "🖱️ui⚛️react-shellsettingspanel--expert-mode",
  "🖱️ui⚛️react-skeletons--table",
  "🖱️ui⚛️react-skeletons--diagram",
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
  "🖱️ui⚛️react-providers--panel-dock-story",
  "🖱️ui⚛️react-providers--transaction",
  "🖱️ui⚛️react-providers--tree-state",
  "🖱️ui⚛️react-providers--chrome-compact",
  "🖱️ui⚛️react-providers--chrome-label-policy",
  "🖱️ui⚛️react-providers--flow",
  "🖱️ui⚛️react-providers--glass-tier",
  "🖱️ui⚛️react-mode--content-through-glass",
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

test("celebrated panel tab icons paint conic ink through --icon-mask, not a rectangular fill", async ({ page }) => {
  await expectStoryMounts(page, "🖱️ui⚛️react-paneltabbar--panel-variant");
  const paint = await page.evaluate(() => {
    const tab = document.querySelector('[data-slot="panel-tab-button"]') as HTMLElement | null;
    if (!tab) return null;
    tab.setAttribute("data-celebrated", "true");
    const icon = tab.querySelector("[data-icon]") as HTMLElement | null;
    const label = tab.querySelector('[data-slot="inline-label"]') as HTMLElement | null;
    if (!icon || !label) return null;
    const iconStyle = getComputedStyle(icon);
    const labelStyle = getComputedStyle(label);
    const beforeStyle = getComputedStyle(icon, "::before");
    return {
      maskImage: iconStyle.maskImage,
      backgroundImage: iconStyle.backgroundImage,
      beforeBackgroundImage: beforeStyle.backgroundImage,
      labelBackgroundImage: labelStyle.backgroundImage,
      labelBackgroundClip: labelStyle.backgroundClip,
      labelWebkitTextFillColor: labelStyle.webkitTextFillColor,
    };
  });
  expect(paint).not.toBeNull();
  expect(paint!.maskImage.startsWith('url("data:image/svg+xml')).toBe(true);
  expect(paint!.backgroundImage).toContain("conic-gradient");
  expect(paint!.beforeBackgroundImage === "none" || paint!.beforeBackgroundImage === "").toBe(true);
  expect(paint!.labelBackgroundImage).toContain("conic-gradient");
  expect(paint!.labelBackgroundClip).toBe("text");
  expect(paint!.labelWebkitTextFillColor === "rgba(0, 0, 0, 0)" || paint!.labelWebkitTextFillColor === "transparent").toBe(true);
});

// #region 🪟️SilhouetteAccessibilityFallbacks
type SilhouetteFallbackPaint = {
  readonly chipBackdrop: string;
  readonly chipBackground: string;
  readonly gapBackdrop: string;
  readonly gapBackground: string;
};

async function readSilhouetteFallbackPaint(page: Page): Promise<SilhouetteFallbackPaint | null> {
  return page.evaluate(() => {
    const chip = document.querySelector("[data-window-silhouette-chip]");
    const gap = document.querySelector("[data-window-silhouette-gap]");
    if (!(chip instanceof HTMLElement) || !(gap instanceof HTMLElement)) return null;
    const chipStyle = getComputedStyle(chip);
    const gapStyle = getComputedStyle(gap);
    return {
      chipBackdrop: chipStyle.backdropFilter || chipStyle.getPropertyValue("-webkit-backdrop-filter"),
      chipBackground: chipStyle.backgroundColor,
      gapBackdrop: gapStyle.backdropFilter || gapStyle.getPropertyValue("-webkit-backdrop-filter"),
      gapBackground: gapStyle.backgroundColor,
    };
  });
}

test("window silhouette keeps its gap transparent in reduced-transparency mode", async ({ page, context }) => {
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-transparency", value: "reduce" }] });
  await expectStoryMounts(page, "🖱️ui⚛️react-mode--content-through-glass");
  const paint = await readSilhouetteFallbackPaint(page);
  expect(paint).not.toBeNull();
  expect(paint!.chipBackdrop).toBe("none");
  expect(paint!.chipBackground).not.toBe("rgba(0, 0, 0, 0)");
  expect(paint!.gapBackdrop).toBe("none");
  expect(paint!.gapBackground).toBe("rgba(0, 0, 0, 0)");
});

test("window silhouette uses system paint without filling its gap in forced-colors mode", async ({ page }) => {
  await page.emulateMedia({ forcedColors: "active" });
  await expectStoryMounts(page, "🖱️ui⚛️react-mode--content-through-glass");
  const paint = await readSilhouetteFallbackPaint(page);
  expect(paint).not.toBeNull();
  expect(paint!.chipBackdrop).toBe("none");
  expect(paint!.chipBackground).not.toBe("rgba(0, 0, 0, 0)");
  expect(paint!.gapBackdrop).toBe("none");
  expect(paint!.gapBackground).toBe("rgba(0, 0, 0, 0)");
});
// #endregion 🪟️SilhouetteAccessibilityFallbacks
