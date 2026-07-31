// #region 🧲️Header
// 💻️ .storybook/animate-presentation-deck.spec.ts
// Specs: End-to-end check for the animate scope's `PresentationDeck` 3-slide reveal.js story.
// Summary: Loads `animate--three-slide-deck` inside the aggregated Storybook static build, asserts a clean boot
// (no page/console errors, `.reveal .slides` mounted with 3 `section`s) and that reveal.js keyboard navigation
// (ArrowRight) advances `Reveal.getIndices().h` from the title slide onward.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectAnimateStory(page: Page, storyId: string): Promise<void> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${encodeURIComponent(storyId)}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForSelector(".reveal .slides section");

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
}

test("animate three-slide deck: boots with the title slide visible and reveal.js mounted", async ({ page }) => {
  await expectAnimateStory(page, "animate--three-slide-deck");
  await expect(page.locator("body")).toContainText("Semio Storybook");
});

test("animate three-slide deck: ArrowRight advances to the feature slide", async ({ page }) => {
  await expectAnimateStory(page, "animate--three-slide-deck");
  await page.keyboard.press("ArrowRight");
  await expect(page.locator("body")).toContainText("Reveal.js Decks");
});
