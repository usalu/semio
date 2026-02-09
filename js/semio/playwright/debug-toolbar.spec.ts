import { test, expect } from "@playwright/test";

test("toolbar visibility in apps", async ({ page }) => {
  page.on("console", (msg) => console.log(msg.text()));
  page.on("pageerror", (exception) => console.log(`Uncaught exception: "${exception}"`));

  // 1. Go to Home
  await page.goto("/");
  await expect(page.locator("#semio\\.sketchpad\\.navbar\\.navigationButtons")).toBeVisible();
  const toolbar = page.locator("#semio\\.sketchpad\\.toolbar");
  await expect(toolbar).toBeVisible();

  // 2. Open Create Group and Create Temporary Kit
  await page.locator("#semio\\.sketchpad\\.toolbar\\.group\\.create").click();
  await page.locator("#semio\\.sketchpad\\.app\\.home\\.toolbar\\.createTemporary").click();

  // Wait for Kit App
  // Look for "kits" in URL or check for Kit-specific element
  await expect(page).toHaveURL(/\/kits\//);
  await expect(toolbar).toBeVisible();

  // 3. Create Quality
  // Need to ensure "Create" category is active or click it
  // But usually Home/Kit apps default to Filter or something.
  // Let's print visible buttons to debug

  // Try to click Create toggle if it exists
  const createToggle = page.locator('button[id="semio.sketchpad.toolbar.group.create"]');
  await expect(createToggle).toBeVisible();

  const isPressed = (await createToggle.getAttribute("data-state")) === "on";
  if (!isPressed) {
    console.log("Create group not active, clicking to open...");
    await createToggle.click();
  } else {
    console.log("Create group already active.");
  }

  // Debug toolbar content
  console.log("Toolbar HTML after click:", await page.locator("#semio\\.sketchpad\\.toolbar").innerHTML());
  console.log("Create Toggle pressed:", await createToggle.getAttribute("aria-pressed"));

  const createQualityBtn = page.locator("#semio\\.sketchpad\\.app\\.kit\\.toolbar\\.createQuality");
  await expect(createQualityBtn).toBeVisible();
  await createQualityBtn.click();

  // 4. Navigate to Quality
  // Quality should appear in the table.
  // Let's assume it's created and visible.
  // Click on a row that has "Quality" kind or just click the first row if table was empty before?
  // Kit table has artifacts.

  // Let's wait a bit for creation
  // await page.waitForTimeout(500);

  // Find a row. The row usually has a cell with the name. Default name might be "New Quality"
  const firstRow = page.locator("tbody tr").first();
  await firstRow.waitFor({ state: "visible", timeout: 5000 });

  const qualityRow = page.locator("tbody tr").filter({ hasText: "New Quality" }).first();
  if (await qualityRow.isVisible()) {
    // Use evaluate to bypass detachment checks if the row is re-rendering rapidly
    await qualityRow.evaluate((node) => (node as HTMLElement).click());
  } else {
    console.log("Could not find 'New Quality' row, clicking first available row");
    await firstRow.evaluate((node) => (node as HTMLElement).click());
  }

  // 5. Verify Quality App
  await expect(page).toHaveURL(/\/qualities\//);

  // 6. Check Toolbar (This should FAIL currently)
  await expect(toolbar).toBeVisible({ timeout: 2000 });
});
