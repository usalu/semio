// #region 🧲Header
// 💻 .storybook/puzzle-3d-5d-infinite.spec.ts
// Specs: End-to-end smoke + light-interaction checks for the `puzzle/3d`, `puzzle/5d`, and `infinite` Storybook scopes' new real-fixture stories, plus `puzzle/2d`'s `Fixtures.stories.tsx`.
// Summary: Every story gets the same base assertion as `./puzzle-2d.spec.ts`'s `expectBoardStory` (page loads, no page/console errors, `#storybook-root` mounts) via `expectStoryLoads`; a handful of stories get an additional readout/interaction assertion (fixture counts via each story's `data-testid` debug `<pre>`, the puzzle-5d timeline scrub, the mock GraphWasmCanvas pointer counter, ReferenceMedia's per-file load status).
// NOTE for whoever wires this into CI: this file is not yet listed in `playwright.config.ts`'s `testMatch` (currently hardcoded to `["puzzle-2d.spec.ts"]`) nor in `script.ts`'s `test storybook` pipeline — both are shared files outside this ticket's edit scope. Run directly with `bunx playwright test .storybook/puzzle-3d-5d-infinite.spec.ts --config .storybook/playwright.config.ts` against a running (or `storybook-static`-served) build with every scope active (`STORYBOOK_SCOPE` unset, or including `puzzle/3d,puzzle/5d,puzzle/2d,infinite`).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Page } from "@playwright/test";

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b40[0-9]\b/i.test(text));
}

/** @emoji 🧪 Base assertion shared by every story in this file — mirrors `./puzzle-2d.spec.ts`'s `expectBoardStory` minus the board-specific canvas lookup. */
async function expectStoryLoads(page: Page, storyId: string): Promise<void> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  // 📛 Story ids are the literal emoji-prefixed `meta.title` slug (Storybook does not strip non-ASCII from `index.json`'s `id`, unlike `./puzzle-2d.spec.ts`'s plain-ASCII "puzzle-2d--..." ids) — percent-encode for the query string.
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

//#region puzzle/2d Fixtures
test("puzzle2d fixtures nakagin: real 180-node/179-edge fixture round-trips through Board2dHost", async ({ page }) => {
  await expectStoryLoads(page, "🧩puzzle🩻2d-fixtures--nakagin-capsule-tower");
  const debug = JSON.parse(await page.getByTestId("puzzle2d-fixture-debug").innerText());
  expect(debug.nodeCount).toBe(180);
  expect(debug.edgeCount).toBe(179);
});

test("puzzle2d fixtures concrete forest: real fixture round-trips through Board2dHost", async ({ page }) => {
  await expectStoryLoads(page, "🧩puzzle🩻2d-fixtures--concrete-forest");
  const debug = JSON.parse(await page.getByTestId("puzzle2d-fixture-debug").innerText());
  expect(debug.nodeCount).toBe(1);
  expect(debug.edgeCount).toBe(0);
});
//#endregion puzzle/2d Fixtures

//#region puzzle/3d World
test("puzzle3d world concrete forest: real fixture round-trips through World3dHost", async ({ page }) => {
  await expectStoryLoads(page, "🧩puzzle🧊3d--concrete-forest");
  const debug = JSON.parse(await page.getByTestId("puzzle3d-world-debug").innerText());
  expect(debug.objectCount).toBe(1);
  expect(debug.selection).toEqual([]);
});

test("puzzle3d world nakagin: real 180-object fixture round-trips through World3dHost", async ({ page }) => {
  await expectStoryLoads(page, "🧩puzzle🧊3d--nakagin-capsule-tower");
  const debug = JSON.parse(await page.getByTestId("puzzle3d-world-debug").innerText());
  expect(debug.objectCount).toBe(180);
});
//#endregion puzzle/3d World

//#region puzzle/5d Timeline
test("puzzle5d timeline nakagin: boots fully assembled (revealCount === partCount), scrubbing to the oldest checkpoint reveals just 1 part", async ({ page }) => {
  await expectStoryLoads(page, "🧩puzzle🕐5d--nakagin-capsule-tower");
  const debug = page.getByTestId("puzzle5d-timeline-debug");
  const before = JSON.parse(await debug.innerText());
  expect(before.partCount).toBe(180);
  expect(before.revealCount).toBe(180);

  await page.getByTestId("graph-timeline-table").locator(".cursor-pointer").last().click();
  await expect.poll(async () => JSON.parse(await debug.innerText()).revealCount).toBe(1);
});
//#endregion puzzle/5d Timeline

//#region infinite/GraphWasmCanvas
test("infinite GraphWasmCanvas mock session: paints without WASM and counts pointer events", async ({ page }) => {
  await expectStoryLoads(page, "♾️infinite-graphwasmcanvas--mock-session");
  const canvas = page.locator(".semio-graph-wasm-canvas-story canvas");
  await expect(canvas).toBeVisible();
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await page.mouse.click(box!.x + box!.width / 2, box!.y + box!.height / 2);
  await expect.poll(async () => JSON.parse(await page.getByTestId("graph-wasm-canvas-debug").innerText()).pointerEvents).toBeGreaterThan(0);
});
//#endregion infinite/GraphWasmCanvas

//#region infinite/WorldR3f
test("infinite WorldR3f chunked field: renders a canvas with the chunked/LOD layer primitives, no console errors", async ({ page }) => {
  await expectStoryLoads(page, "♾️infinite-worldr3f--chunked-field");
  await expect(page.locator(".semio-world-r3f-story canvas")).toBeVisible();
});
//#endregion infinite/WorldR3f

//#region infinite/ReferenceMedia
test("infinite ReferenceMedia: every real infinite/fixture file reaches loaded status via referenceMediaPort", async ({ page }) => {
  await expectStoryLoads(page, "♾️infinite-referencemedia--all-fixtures");
  for (const label of ["sketch.png", "abbau-aufbau-masterarbeit-grundriss.jpg", "rathaus-ahlen-grundriss.png", "site.pdf"]) {
    await expect(page.getByTestId(`reference-media-status-${label}`)).toContainText("status: loaded", { timeout: 30000 });
  }
});
//#endregion infinite/ReferenceMedia
