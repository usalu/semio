// #region 🧲Header
// 💻 .storybook/cad-renderer.spec.ts
// Specs: End-to-end checks for the cad scope's `InteractionCanvas`/`InteractionSpatialView` box-interaction stories.
// Summary: Loads the `Idle` and `CommittedBox` stories inside the aggregated Storybook static build, asserting a clean
// boot (no page/console errors, canvas mounted) and, for `CommittedBox`, the `cad-box-debug` readout reaching the
// `committed` state with the story's `StoryBoxKernel` recording the scripted `(0,0,0)→(2,3,0)`/height-4 box.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Locator, type Page } from "@playwright/test";

type CadBoxDebug = {
  readonly state: string;
  readonly lastBox: { readonly cornerA: readonly number[]; readonly cornerB: readonly number[]; readonly height: number } | null;
};

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function readCadBoxDebug(debug: Locator): Promise<CadBoxDebug> {
  const text = await debug.innerText();
  return JSON.parse(text) as CadBoxDebug;
}

async function expectCadStory(page: Page, storyId: string): Promise<{ readonly debug: Locator }> {
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

  const debug = page.getByTestId("cad-box-debug");
  await expect(debug).toBeVisible();

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  return { debug };
}

test("cad idle: boots with the fresh primitive.box interaction, no committed solid yet", async ({ page }) => {
  const { debug } = await expectCadStory(page, "📐cad--idle");
  const state = await readCadBoxDebug(debug);
  expect(state.lastBox).toBeNull();
});

test("cad committed box: scripted corner → corner → height → confirm commits a real box through the pure-JS StoryBoxKernel", async ({ page }) => {
  const { debug } = await expectCadStory(page, "📐cad--committed-box");
  await expect.poll(async () => (await readCadBoxDebug(debug)).state).toBe("committed");
  const state = await readCadBoxDebug(debug);
  expect(state.lastBox).toEqual({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 });
});
