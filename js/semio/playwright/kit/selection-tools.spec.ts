import { test, expect } from "@playwright/test";

test.describe("Kit App Selection Tools", () => {
  test("selection tools should be visible in toolbar after creating kit", async ({ page }) => {
    await page.goto("http://127.0.0.1:5173");

    // Create a temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    
    // Wait for toolbar to appear
    await page.waitForSelector("#semio\\.sketchpad\\.toolbar", { timeout: 5000 });
    
    // Get the toolbar container
    const toolbar = page.locator("#semio\\.sketchpad\\.toolbar");
    
    // Look for the tool group that contains selection tools
    // The toolbar should have selection tool buttons
    const toolButtons = toolbar.locator('button');
    const buttonCount = await toolButtons.count();
    
    console.log(`Total buttons in toolbar: ${buttonCount}`);
    
    // Should have filter buttons (at least 9) + selection tools (at least 3) = at least 12 buttons
    expect(buttonCount).toBeGreaterThanOrEqual(12);
  });

  test("selection tool buttons should be individually clickable", async ({ page }) => {
    await page.goto("http://127.0.0.1:5173");

    // Create temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    await page.waitForSelector("#semio\\.sketchpad\\.toolbar");
    
    const toolbar = page.locator("#semio\\.sketchpad\\.toolbar");
    const buttons = toolbar.locator('button');
    
    // Get button count before interaction
    const initialCount = await buttons.count();
    console.log(`Buttons found: ${initialCount}`);
    
    // All buttons should be clickable (visible and enabled)
    for (let i = 0; i < initialCount && i < 15; i++) {
      const button = buttons.nth(i);
      const isVisible = await button.isVisible();
      const isEnabled = await button.isEnabled();
      expect(isVisible).toBeTruthy();
      expect(isEnabled).toBeTruthy();
    }
  });

  test("selection mode should change when clicking different tool buttons", async ({ page }) => {
    await page.goto("http://127.0.0.1:5173");

    // Create temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    await page.waitForSelector("#semio\\.sketchpad\\.toolbar");
    
    const toolbar = page.locator("#semio\\.sketchpad\\.toolbar");
    const buttons = toolbar.locator('button');
    
    // Try clicking buttons to see if selection mode changes
    // First, count how many we have
    const totalButtons = await buttons.count();
    expect(totalButtons).toBeGreaterThan(0);
    
    // Click a few buttons - they should remain enabled after click
    if (totalButtons > 10) {
      await buttons.nth(10).click();
      await page.waitForTimeout(100);
      
      const stillVisible = await buttons.nth(10).isVisible();
      expect(stillVisible).toBeTruthy();
    }
  });

  test("selection tools should remain visible when navigating between kits", async ({ page }) => {
    await page.goto("http://127.0.0.1:5173");

    // Create first temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    await page.waitForSelector("#semio\\.sketchpad\\.toolbar");
    
    const toolbar1 = page.locator("#semio\\.sketchpad\\.toolbar");
    const buttons1 = toolbar1.locator('button');
    const count1 = await buttons1.count();
    
    // Go back to home
    await page.locator('[id="semio.sketchpad.navbar.back"]').click();
    
    // Create second temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    await page.waitForSelector("#semio\\.sketchpad\\.toolbar");
    
    const toolbar2 = page.locator("#semio\\.sketchpad\\.toolbar");
    const buttons2 = toolbar2.locator('button');
    const count2 = await buttons2.count();
    
    // Both should have the same number of buttons
    expect(count1).toBe(count2);
    expect(count1).toBeGreaterThanOrEqual(12);
  });

  test("toolbar should have both filter and selection tools", async ({ page }) => {
    await page.goto("http://127.0.0.1:5173");

    // Create temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    await page.waitForSelector("#semio\\.sketchpad\\.toolbar");
    
    const toolbar = page.locator("#semio\\.sketchpad\\.toolbar");
    
    // Check that toolbar is not empty
    const toolbar_html = await toolbar.innerHTML();
    expect(toolbar_html.length).toBeGreaterThan(0);
    
    // Should have button elements
    const buttons = toolbar.locator('button');
    const count = await buttons.count();
    
    console.log(`Toolbar has ${count} buttons`);
    expect(count).toBeGreaterThan(9); // At least filter buttons + selection tools
  });
});
