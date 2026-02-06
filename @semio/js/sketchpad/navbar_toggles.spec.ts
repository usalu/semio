import { expect, test } from "@playwright/test";

test.describe("Navbar Panel Toggles", () => {
  test.beforeEach(async ({ page }) => {
    page.on("console", (msg) => console.log(`[BROWSER] ${msg.text()}`));
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");
  });

  test("should show navbar", async ({ page }) => {
    const navbar = page.locator("#semio\\.sketchpad\\.navbar");
    await expect(navbar).toBeVisible();
  });

  test("should show footer", async ({ page }) => {
    const footer = page.locator("#semio\\.sketchpad\\.footer");
    await expect(footer).toBeVisible();
  });

  test("should show all toggles in Design app", async ({ page }) => {
    await page.goto("http://localhost:3000/kits/00000000-0000-0000-0000-000000000000/designs/00000000-0000-0000-0000-000000000000");
    await page.waitForLoadState("networkidle");

    const leftToggle = page.locator("#semio\\.sketchpad\\.navbar\\.panelToggle\\.leftSidePanel");
    const middleToggle = page.locator("#semio\\.sketchpad\\.navbar\\.panelToggle\\.hudPanel");
    const rightToggle = page.locator("#semio\\.sketchpad\\.navbar\\.panelToggle\\.rightSidePanel");

    await expect(leftToggle).toBeVisible({ timeout: 5000 });
    await expect(middleToggle).toBeVisible({ timeout: 5000 });
    await expect(rightToggle).toBeVisible({ timeout: 5000 });

    await leftToggle.click();
    await expect(leftToggle).toHaveAttribute("aria-checked", "true");
    
    await leftToggle.click();
    await expect(leftToggle).toHaveAttribute("aria-checked", "false");

  });
});
