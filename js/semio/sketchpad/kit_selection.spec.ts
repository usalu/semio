import { expect, test } from "@playwright/test";

test.describe("Kit App Selection Tools", () => {
  test.beforeEach(async ({ page }) => {
    page.on("console", (msg) => console.log(`[BROWSER] ${msg.text()}`));
    await page.goto("http://localhost:5173");
    await page.waitForLoadState("networkidle");
  });

  test("should navigate to kit app", async ({ page }) => {
    // Create a temporary kit or navigate to existing one
    await page.goto("http://localhost:5173/kits");
    await page.waitForLoadState("networkidle");

    // Look for a kit or create one
    const createKitButton = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
    if (await createKitButton.isVisible()) {
      await createKitButton.click();
      await page.waitForLoadState("networkidle");
    }
  });

  test("should show selection tools in toolbar", async ({ page }) => {
    // Navigate to kit app (assuming a test kit exists)
    await page.goto("http://localhost:5173/kits");
    await page.waitForLoadState("networkidle");

    // Create temporary kit
    const createKitButton = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
    await createKitButton.click();
    await page.waitForLoadState("networkidle");

    // Wait for Kit app to load
    await page.waitForTimeout(1000);

    // Check for toolbar
    const toolbar = page.locator('[id*="toolbar"]').first();
    await expect(toolbar).toBeVisible({ timeout: 10000 });

    // Check for selection tool toggle group
    const selectionTool = page.locator('[data-tool="selection"]');
    await expect(selectionTool).toBeVisible({ timeout: 5000 });

    console.log("Toolbar HTML:", await toolbar.innerHTML());
  });

  test("should cycle through selection modes", async ({ page }) => {
    // Navigate and create kit
    await page.goto("http://localhost:5173/kits");
    await page.waitForLoadState("networkidle");

    const createKitButton = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
    await createKitButton.click();
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(1000);

    // Find selection tool button
    const selectionButton = page.locator('button:has-text("Normal Selection"), button[data-tool="selection"]').first();

    if (await selectionButton.isVisible()) {
      // Click to cycle modes
      await selectionButton.click();
      await page.waitForTimeout(500);

      // Check if mode changed (look for different text/icon)
      const buttonText = await selectionButton.textContent();
      console.log("Selection tool button text:", buttonText);
    } else {
      console.log("Selection tool button not found");
      // Log all buttons in toolbar for debugging
      const allButtons = page.locator("button");
      const count = await allButtons.count();
      console.log(`Found ${count} buttons`);
      for (let i = 0; i < Math.min(count, 10); i++) {
        const text = await allButtons.nth(i).textContent();
        const id = await allButtons.nth(i).getAttribute("id");
        console.log(`Button ${i}: id="${id}", text="${text}"`);
      }
    }
  });

  test("should perform normal selection on table row", async ({ page }) => {
    // Navigate and create kit
    await page.goto("http://localhost:5173/kits");
    await page.waitForLoadState("networkidle");

    const createKitButton = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
    await createKitButton.click();
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(1000);

    // Create a type so we have something to select
    const createTypeButton = page.locator('[id*="createType"]').first();
    if (await createTypeButton.isVisible()) {
      await createTypeButton.click();
      await page.waitForTimeout(500);
    }

    // Find table rows
    const tableRows = page.locator("tr[data-row-id], table tbody tr").filter({ hasNot: page.locator("th") });
    const rowCount = await tableRows.count();
    console.log(`Found ${rowCount} table rows`);

    if (rowCount > 0) {
      // Click first row
      await tableRows.first().click();
      await page.waitForTimeout(500);

      // Check if row is selected (has selected class/attribute)
      const isSelected = await tableRows.first().evaluate((el) => {
        return el.classList.contains("selected") || el.getAttribute("aria-selected") === "true" || el.getAttribute("data-selected") === "true";
      });

      console.log(`Row selected: ${isSelected}`);
    }
  });

  test("should perform additive selection", async ({ page }) => {
    // Navigate and create kit
    await page.goto("http://localhost:5173/kits");
    await page.waitForLoadState("networkidle");

    const createKitButton = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
    await createKitButton.click();
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(1000);

    // Create multiple types
    const createTypeButton = page.locator('[id*="createType"]').first();
    if (await createTypeButton.isVisible()) {
      await createTypeButton.click();
      await page.waitForTimeout(500);
      await createTypeButton.click();
      await page.waitForTimeout(500);
    }

    // Switch to additive selection mode
    const selectionButton = page.locator('button:has-text("Add"), button[data-mode="additive"]').first();
    if (await selectionButton.isVisible()) {
      await selectionButton.click();
      await page.waitForTimeout(500);
    }

    // Find table rows
    const tableRows = page.locator("tr[data-row-id], table tbody tr").filter({ hasNot: page.locator("th") });
    const rowCount = await tableRows.count();
    console.log(`Found ${rowCount} table rows for additive selection`);

    if (rowCount >= 2) {
      // Click first row
      await tableRows.nth(0).click();
      await page.waitForTimeout(500);

      // Click second row (should add to selection, not replace)
      await tableRows.nth(1).click();
      await page.waitForTimeout(500);

      // Check if both rows are selected
      const firstSelected = await tableRows.nth(0).evaluate((el) => {
        return el.classList.contains("selected") || el.getAttribute("aria-selected") === "true" || el.getAttribute("data-selected") === "true";
      });

      const secondSelected = await tableRows.nth(1).evaluate((el) => {
        return el.classList.contains("selected") || el.getAttribute("aria-selected") === "true" || el.getAttribute("data-selected") === "true";
      });

      console.log(`First row selected: ${firstSelected}, Second row selected: ${secondSelected}`);
      console.log(`Expected both to be true for additive selection`);
    }
  });

  test("inspect kit app toolbar - direct navigation", async ({ page }) => {
    // Navigate directly to a kit URL (use a test GUID)
    const testKitGuid = "00000000-0000-0000-0000-000000000001";
    await page.goto(`http://localhost:5173/kits/${testKitGuid}`);
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    console.log(`\n=== Current URL: ${page.url()} ===`);

    await page.screenshot({ path: "/tmp/kit-app-direct.png", fullPage: true });
    console.log("=== Screenshot saved to /tmp/kit-app-direct.png ===");

    // Look for toolbar
    const toolbarPanel = page.locator('[data-slot="toolbar"], .toolbar, [role="toolbar"]');
    const toolbarCount = await toolbarPanel.count();
    console.log(`\n=== Found ${toolbarCount} toolbar panels ===`);

    // Look specifically for bottom panel/toolbar
    const bottomElements = page.locator('[class*="bottom"], [style*="bottom"]').filter({ hasText: /.+/ });
    const bottomCount = await bottomElements.count();
    console.log(`=== Found ${bottomCount} bottom elements ===`);

    // Search for toggle groups
    const toggleGroups = page.locator('[role="radiogroup"]');
    const toggleCount = await toggleGroups.count();
    console.log(`\n=== Found ${toggleCount} toggle groups (radio groups) ===`);

    for (let i = 0; i < Math.min(toggleCount, 5); i++) {
      const tg = toggleGroups.nth(i);
      const html = await tg.innerHTML();
      const visible = await tg.isVisible();
      console.log(`\nToggle group ${i} (visible=${visible}):`);
      console.log(html.substring(0, 400));
    }

    // Search for any buttons with "selection" or "tool" in text/id
    const selectionButtons = page.locator("button").filter({ hasText: /select|tool|normal|additive/i });
    const selCount = await selectionButtons.count();
    console.log(`\n=== Found ${selCount} selection/tool buttons ===`);

    for (let i = 0; i < selCount; i++) {
      const btn = selectionButtons.nth(i);
      const id = await btn.getAttribute("id");
      const text = (await btn.textContent())?.trim();
      const visible = await btn.isVisible();
      console.log(`Selection button ${i}: id="${id}", text="${text}", visible=${visible}`);
    }

    // Check footer
    const footer = page.locator('[id="semio.sketchpad.footer"]');
    if (await footer.isVisible()) {
      console.log("\n=== Footer visible ===");
      const footerHTML = await footer.innerHTML();
      console.log(`Footer HTML (first 500 chars): ${footerHTML.substring(0, 500)}`);
    }
  });
});
