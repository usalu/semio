const { chromium } = require("playwright");
(async () => {
  const browser = await chromium.launch({ executablePath: "/usr/bin/google-chrome-stable", args: ["--no-sandbox"] });
  const page = await browser.newPage();
  await page.goto("http://localhost:5173/");
  await page.waitForTimeout(3000);
  console.log("[DEBUG] Page title:", await page.title());

  // Navigate to a design - we need to check the toolbar
  // First check if filter toolbar group exists
  const filterToggle = await page.locator('[id="compose.sketchpad.toolbar.group.filter"]').count();
  console.log("[DEBUG] Filter toggle present on home:", filterToggle);

  // Upload a kit to get to design view
  const fileInput = page.locator('input[type="file"]');
  const fileCount = await fileInput.count();
  console.log("[DEBUG] File input count:", fileCount);

  // Navigate to a design URL directly with query params
  await page.goto("http://localhost:5173/#/kit/local/test?app=design&design=test");
  await page.waitForTimeout(2000);

  const filterToggle2 = await page.locator('[id="compose.sketchpad.toolbar.group.filter"]').count();
  console.log("[DEBUG] Filter toggle present on design:", filterToggle2);

  // Take a screenshot to see what we have
  await page.screenshot({ path: "/workspaces/semio/.repo/🎫️/26/02/16/DESIGN-APP-SKETCHPAD-FILTERS/state.png" });

  // Check what toolbar buttons exist
  const toolbarButtons = await page.locator('[id^="compose.sketchpad.toolbar.group."]').allInnerTexts();
  console.log("[DEBUG] Toolbar buttons:", toolbarButtons);
  const toolbarIds = await page.locator('[id^="compose.sketchpad.toolbar.group."]').evaluateAll((els) => els.map((e) => e.id));
  console.log("[DEBUG] Toolbar IDs:", toolbarIds);

  await browser.close();
  console.log("[DEBUG] Done");
})();
