// #region 🧲Header
// 💻 .storybook/styling.spec.ts
// Specs: End-to-end checks for the `styling` Storybook scope's docs-gallery stories inside the aggregated Storybook static build.
// Summary: Loads one representative story per `stories/styling/*.stories.tsx` file and asserts it actually renders (no "couldn't find story", no page/console errors, expected gallery content visible), plus drives `ThemeRoundtrip`'s textarea interactively to exercise the real `parseUiTheme`/`serializeUiTheme` round-trip in the browser. Modeled on `puzzle-2d.spec.ts`; unlike that board host these stories are pure read-only data renders, so there is no canvas/WASM session to poke — content assertions replace the debug-readout polling.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Page } from "@playwright/test";

//#region 🔖Shared
function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

/** @emoji 🧪 Navigates to `iframe.html?id=…` and asserts the story actually mounted — a 200 status alone doesn't prove this since `iframe.html` is a static shell that renders its "couldn't find story" fallback with the same HTTP status. */
async function expectStylingStory(page: Page, storyId: string): Promise<void> {
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
//#endregion 🔖Shared

//#region 🔖Theme
test("styling/Theme AllThemes: renders every builtin theme's light+dark board/map/canvas/chrome swatch grids", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-theme--all-themes");
  await expect(page.getByText("semio", { exact: false }).first()).toBeVisible();
  await expect(page.getByText("board", { exact: false }).first()).toBeVisible();
  await expect(page.getByText("light", { exact: true }).first()).toBeVisible();
  await expect(page.getByText("dark", { exact: true }).first()).toBeVisible();
});

test("styling/Theme SemioOnly: renders only the semio theme's two appearance cards", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-theme--semio-only");
  await expect(page.getByText("semio", { exact: false }).first()).toBeVisible();
});
//#endregion 🔖Theme

//#region 🔖Tokens
test("styling/Tokens AllTokens: renders colors/spacing/radii/strokes/opacities tables", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-tokens--all-tokens");
  await expect(page.getByText("Colors", { exact: false }).first()).toBeVisible();
  await expect(page.getByText("primary", { exact: false }).first()).toBeVisible();
  await expect(page.getByText("Spacing", { exact: false }).first()).toBeVisible();
  await expect(page.getByText("Radii", { exact: false }).first()).toBeVisible();
});

test("styling/Tokens Colors: renders the STYLING_TOKENS palette as a table", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-tokens--colors");
  await expect(page.getByText("#ff344f", { exact: false }).first()).toBeVisible();
});
//#endregion 🔖Tokens

//#region 🔖Glass
test("styling/Glass AllLevels: renders every Level's ui-glass swatch", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-glass--all-levels");
  await expect(page.getByText("panel", { exact: true }).first()).toBeVisible();
  await expect(page.getByText("window", { exact: true }).first()).toBeVisible();
  await expect(page.getByText("menu", { exact: true }).first()).toBeVisible();
  await expect(page.getByText("pane", { exact: true }).first()).toBeVisible();
  await expect(page.getByText(".ui-glass", { exact: false }).first()).toBeVisible();
});

test("styling/Glass DefaultLevelFallback: useLevel() falls back to base outside any provider", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-glass--default-level-fallback");
  await expect(page.getByText("base", { exact: true }).first()).toBeVisible();
  await expect(page.getByText(".ui-glass", { exact: false }).first()).toBeVisible();
});
//#endregion 🔖Glass

//#region 🔖ThemeRoundtrip
test("styling/ThemeRoundtrip SemioTheme: parses cleanly and round-trips the input JSON", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-themeroundtrip--semio-theme");
  const errorReadout = page.getByTestId("theme-roundtrip-error");
  await expect(errorReadout).toHaveText("parses cleanly");
  const status = page.getByTestId("theme-roundtrip-status");
  await expect(status).toBeVisible();
  await expect(page.getByTestId("theme-roundtrip-output")).not.toHaveValue("");
});

test("styling/ThemeRoundtrip BrokenTokenReference: parseUiTheme surfaces the unknown token ref", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-themeroundtrip--broken-token-reference");
  const errorReadout = page.getByTestId("theme-roundtrip-error");
  await expect(errorReadout).toContainText("not-a-real-token");
  await expect(page.getByTestId("theme-roundtrip-output")).toHaveValue("");
});

test("styling/ThemeRoundtrip SemioTheme: editing the textarea to invalid JSON surfaces a parse error live", async ({ page }) => {
  await expectStylingStory(page, "🎨styling-themeroundtrip--semio-theme");
  const input = page.getByTestId("theme-roundtrip-input");
  await input.fill("{ not valid json");
  await expect(page.getByTestId("theme-roundtrip-error")).toContainText("parseUiTheme threw");
  await expect(page.getByTestId("theme-roundtrip-output")).toHaveValue("");
});
//#endregion 🔖ThemeRoundtrip
