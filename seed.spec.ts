import { expect, test } from "@playwright/test";

test.describe("Performance tests", () => {
  test("kit UI interactions are responsive", async ({ page }) => {
    await page.goto("http://localhost:5174");
    await page.locator('[id="semio\\.sketchpad\\.app\\.home\\.createTemporary"]').click();
    await expect(page.locator('[id="semio\\.sketchpad\\.app\\.kit\\.filter\\.strip"]')).toBeVisible({ timeout: 10000 });
    await page.waitForTimeout(200);
    for (let i = 0; i < 5; i++) {
      const start = Date.now();
      await page.locator('[id="semio\\.sketchpad\\.app\\.kit\\.sortByKind"]').click();
      await page.waitForTimeout(50);
      const elapsed = Date.now() - start;
      expect(elapsed).toBeLessThan(500);
    }
  });
});
