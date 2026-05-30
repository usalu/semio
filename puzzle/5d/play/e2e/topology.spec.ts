import { expect, test } from "@playwright/test";

test("topology play mounts board and scene shells", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("[data-puzzle-5d-2d-root]")).toBeVisible({ timeout: 120_000 });
  await expect(page.locator('[data-measure-id="puzzle-5d-2d-lod"]')).toBeVisible({ timeout: 120_000 });
  await expect(page.locator("[data-puzzle-5d-3d-root]")).toBeVisible({ timeout: 120_000 });
  await expect(page.locator("[data-scene-root]")).toBeVisible({ timeout: 120_000 });
  await expect.poll(async () => (await page.locator('[data-measure-id="puzzle-5d-3d-lod"]').isVisible()) || (await page.locator('[data-measure-id="puzzle-5d-3d-auto"]').isVisible()), { timeout: 120_000 }).toBe(true);
});

test("topology play exposes paired FiveD surfaces with shared connect state", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator('[data-five-d-mode="flat"][data-five-d-instance="play-board"]')).toBeVisible({ timeout: 120_000 });
  await expect(page.locator('[data-five-d-mode="volume"][data-five-d-instance="play-volume"]')).toBeVisible({ timeout: 120_000 });
  await expect(page.locator('[data-five-d-mode="flat"][data-five-d-indirect-active="false"]')).toBeVisible();
  await expect(page.locator('[data-five-d-mode="volume"][data-five-d-indirect-active="false"]')).toBeVisible();
});
