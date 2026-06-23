import { test } from "@playwright/test";

test("seed", async ({ page }) => {
  await page.goto("http://localhost:6007/iframe.html?id=compose-algorithms-design-cluster--default&viewMode=story");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(3000);
});
