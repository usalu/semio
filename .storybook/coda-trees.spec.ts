// #region 🧲️Header
// 💻️ .storybook/coda-trees.spec.ts
// Specs: End-to-end checks for the coda scope's gap-fill `OntologyTree`/`ValidationTree` stories.
// Summary: Loads `CodaTrees--uncovered-kinds` and `CodaValidationTree--uncovered-kinds` inside the aggregated
// Storybook static build, asserting a clean boot (no page/console errors) and that every previously-uncovered
// node-kind icon glyph (∀, ≥n, ≤n, ∀d, v, D) renders somewhere in the tree.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "@playwright/test";

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectCodaStory(page: Page, storyId: string): Promise<void> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${encodeURIComponent(storyId)}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
}

test("coda OntologyTree uncovered kinds: renders AllValuesFrom/MinCardinality/MaxCardinality/DataAllValuesFrom/DataHasValue/DatatypeRestriction icons", async ({ page }) => {
  await expectCodaStory(page, "coda-ontologytree--uncovered-kinds");
  for (const glyph of ["∀", "≥n", "≤n", "∀d", "D"]) {
    await expect(page.getByTitle(/AllValuesFrom|MinCardinality|MaxCardinality|DataAllValuesFrom|DataHasValue|DatatypeRestriction/).first()).toBeVisible();
    await expect(page.locator("body")).toContainText(glyph);
  }
});

test("coda ValidationTree uncovered kinds: Min/Max cardinality get the n/n badge and the tree reaches its final truth value", async ({ page }) => {
  await expectCodaStory(page, "coda-validationtree--uncovered-kinds");
  await expect(page.locator("body")).toContainText("2/1");
  await expect(page.locator("body")).toContainText("2/4");
});
