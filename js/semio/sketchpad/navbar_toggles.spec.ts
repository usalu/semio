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
    // Navigate to a design
    await page.goto("http://localhost:3000/kits/00000000-0000-0000-0000-000000000000/designs/00000000-0000-0000-0000-000000000000");
    await page.waitForLoadState("networkidle");

    // Check for toggles
    const leftToggle = page.locator("#semio\\.sketchpad\\.navbar\\.panelToggle\\.leftSidePanel");
    const middleToggle = page.locator("#semio\\.sketchpad\\.navbar\\.panelToggle\\.hudPanel");
    const rightToggle = page.locator("#semio\\.sketchpad\\.navbar\\.panelToggle\\.rightSidePanel");

    await expect(leftToggle).toBeVisible({ timeout: 5000 });
    await expect(middleToggle).toBeVisible({ timeout: 5000 });
    await expect(rightToggle).toBeVisible({ timeout: 5000 });

    // Test toggle functionality - Left Panel
    await leftToggle.click();
    // Assuming the panel opens and has some identifiable element or width change. 
    // Checking for aria-pressed or visual indication if possible, or side panel visibility.
    // The test description asks to check for functionality.
    
    // Check if aria-pressed is now true (if implemented) or check side panel visibility
    await expect(leftToggle).toHaveAttribute("aria-pressed", "true");
    
    await leftToggle.click();
    await expect(leftToggle).toHaveAttribute("aria-pressed", "false");

  });
});
