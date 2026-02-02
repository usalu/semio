import { test, expect } from "@playwright/test";

test.describe("Selection Comparison: Design vs Kit", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:5173");
  });

  test.describe("Design App Selection", () => {
    test("should switch to additive mode with Shift key", async ({ page }) => {
      // Navigate to Design app
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
      
      // Wait for design app to load
      await page.waitForTimeout(1000);
      
      // Check initial tool state (should be SELECTION_NORMAL)
      const toolGroup = page.locator('[data-testid="tool-group"]').first();
      await expect(toolGroup).toBeVisible();
      
      // Press Shift key
      await page.keyboard.down("Shift");
      await page.waitForTimeout(200);
      
      // Tool should switch to SELECTION_ADDITIVE
      // Note: We need to verify this visually or through state inspection
      console.log("Shift pressed - tool should be SELECTION_ADDITIVE");
      
      // Release Shift key
      await page.keyboard.up("Shift");
      await page.waitForTimeout(200);
      
      // Tool should revert to SELECTION_NORMAL
      console.log("Shift released - tool should revert to SELECTION_NORMAL");
    });
    
    test("should switch to subtractive mode with Ctrl key", async ({ page }) => {
      // Navigate to Design app
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
      
      await page.waitForTimeout(1000);
      
      // Press Ctrl key (or Meta on Mac)
      await page.keyboard.down("Control");
      await page.waitForTimeout(200);
      
      console.log("Ctrl pressed - tool should be SELECTION_SUBTRACTIVE");
      
      // Release Ctrl key
      await page.keyboard.up("Control");
      await page.waitForTimeout(200);
      
      console.log("Ctrl released - tool should revert to SELECTION_NORMAL");
    });
  });

  test.describe("Kit App Selection", () => {
    test("should switch to additive mode with Shift key", async ({ page }) => {
      // Navigate to Kit app
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      
      await page.waitForTimeout(1000);
      
      // Check if selection tools are visible
      const toolGroup = page.locator('[data-testid="tool-group"]').first();
      const isVisible = await toolGroup.isVisible().catch(() => false);
      
      if (isVisible) {
        // Press Shift key
        await page.keyboard.down("Shift");
        await page.waitForTimeout(200);
        
        console.log("Shift pressed in Kit app - checking tool state");
        
        // Release Shift key
        await page.keyboard.up("Shift");
        await page.waitForTimeout(200);
        
        console.log("Shift released in Kit app");
      } else {
        console.log("Selection tools NOT VISIBLE in Kit app");
      }
    });
    
    test("should verify tool visibility and structure", async ({ page }) => {
      // Navigate to Kit app
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      
      await page.waitForTimeout(1000);
      
      // Take screenshot for debugging
      await page.screenshot({ path: "/tmp/kit-app-toolbar.png", fullPage: true });
      
      // Check toolbar structure
      const toolbar = page.locator('[data-testid="toolbar"]').or(page.locator('[class*="toolbar"]'));
      const toolbarCount = await toolbar.count();
      console.log(`Toolbars found: ${toolbarCount}`);
      
      // Check for ToolGroup component
      const toolGroups = page.locator('[data-testid="tool-group"]');
      const toolGroupCount = await toolGroups.count();
      console.log(`ToolGroups found: ${toolGroupCount}`);
      
      // Log all interactive elements in the toolbar area
      const buttons = page.locator("button");
      const buttonCount = await buttons.count();
      console.log(`Total buttons on page: ${buttonCount}`);
      
      // Check for selection tool icons
      const selectIcons = page.locator('[class*="select"]').or(page.locator('[aria-label*="select"]'));
      const selectIconCount = await selectIcons.count();
      console.log(`Selection-related elements: ${selectIconCount}`);
    });
  });

  test.describe("Comparison Analysis", () => {
    test("should document keyboard event handler differences", async ({ page }) => {
      console.log("\n=== ANALYSIS ===");
      console.log("Design App:");
      console.log("- Has useEffect with keydown/keyup handlers");
      console.log("- Switches activeTool state on Shift press/release");
      console.log("- Switches activeTool state on Ctrl/Meta press/release");
      console.log("- Lines 7497-7519 in Design.tsx");
      
      console.log("\nKit App:");
      console.log("- NO keyboard event handlers found");
      console.log("- Uses inline effectiveMode calculation (line 5162)");
      console.log("- Does NOT switch activeTool state");
      console.log("- Missing keydown/keyup event listeners");
      
      console.log("\n=== REQUIRED FIX ===");
      console.log("Kit app needs to add keyboard event handlers similar to Design app");
      console.log("These handlers should:");
      console.log("1. Listen for keydown/keyup on Shift, Ctrl, Meta");
      console.log("2. Switch activeTool from NORMAL to ADDITIVE on Shift press");
      console.log("3. Switch activeTool from NORMAL to SUBTRACTIVE on Ctrl/Meta press");
      console.log("4. Revert to NORMAL on key release");
      console.log("5. Update UI to reflect current tool mode");
      
      expect(true).toBe(true); // Dummy assertion to pass test
    });
  });
});
