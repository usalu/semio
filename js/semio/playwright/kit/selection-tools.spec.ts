import { expect, test } from "@playwright/test";

test.describe("Kit App Selection Tools", () => {
  test("selection tools should be visible in toolbar after creating kit", async ({ page }) => {
    await page.goto("http://localhost:5173");

    // Create a temporary kit - this should create a kit and navigate to it
    const createBtn = page.locator('[id="semio.sketchpad.app.home.toolbar.createTemporary"]');
    await createBtn.click();
    
    // Check navigation to Kit pages (plural)
    await expect(page).toHaveURL(/\/kits\//);
    
    // Wait for KIT toolbar element to ensure we are fully loaded
    await page.waitForSelector('[id="semio.sketchpad.app.kit.toolbar.showDesigns"]', { timeout: 10000 });
    
    // Get the toolbar container - use attribute selector to avoid dot escaping issues
    const toolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    await expect(toolbar).toBeVisible();
    
    // Look for filter buttons inside the toolbar
    const showDesigns = toolbar.locator('[id="semio.sketchpad.app.kit.toolbar.showDesigns"]');
    await expect(showDesigns).toBeVisible();

    // Verify presence of standard Kit filters
    const filters = [
      'semio.sketchpad.app.kit.toolbar.showTypes',
      'semio.sketchpad.app.kit.toolbar.showDesigns',
      'semio.sketchpad.app.kit.toolbar.showQualities',
      'semio.sketchpad.app.kit.toolbar.showFiles',
      'semio.sketchpad.app.kit.toolbar.showAuthors'
    ];
    
    for (const id of filters) {
       await expect(page.locator(`[id="${id}"]`)).toBeVisible();
    }
  });
});
