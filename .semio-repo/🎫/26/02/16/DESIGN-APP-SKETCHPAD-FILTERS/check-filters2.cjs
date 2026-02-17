const { chromium } = require("playwright");
(async () => {
  const browser = await chromium.launch({ executablePath: "/usr/bin/google-chrome-stable", args: ["--no-sandbox"] });
  const page = await browser.newPage();
  
  // Listen for console messages
  page.on("console", (msg) => {
    if (msg.text().includes("[DEBUG]")) console.log("[BROWSER]", msg.text());
  });
  
  await page.goto("http://localhost:5173/");
  await page.waitForTimeout(3000);
  
  // Upload metabolism kit
  const fs = require("fs");
  const kitPath = "/workspaces/semio/semio/sketchpad/public/metabolism.zip";
  if (!fs.existsSync(kitPath)) {
    console.log("[DEBUG] Kit file not found, trying to find it");
    const { execSync } = require("child_process");
    const result = execSync("find /workspaces/semio -name 'metabolism.zip' -type f 2>/dev/null | head -5").toString();
    console.log("[DEBUG] Found:", result);
    process.exit(1);
  }
  
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles(kitPath);
  await page.waitForTimeout(5000);
  
  console.log("[DEBUG] After kit upload, URL:", page.url());
  
  // Check if we're on a kit page now
  const title = await page.title();
  console.log("[DEBUG] Title after upload:", title);
  
  // Find kit/design navigation
  // Look for design links 
  const allLinks = await page.locator("a").evaluateAll(els => els.map(e => ({href: e.href, text: e.textContent?.trim()?.substring(0, 50)})).filter(e => e.text));
  console.log("[DEBUG] Links:", JSON.stringify(allLinks.slice(0, 20)));
  
  // Navigate to a design app
  const designLinks = allLinks.filter(l => l.href && l.href.includes("design"));
  console.log("[DEBUG] Design links:", JSON.stringify(designLinks));
  
  await page.screenshot({ path: "/workspaces/semio/.semio-repo/🎫/26/02/16/DESIGN-APP-SKETCHPAD-FILTERS/after-upload.png" });
  
  // Click on a design link if found
  if (designLinks.length > 0) {
    await page.goto(designLinks[0].href);
    await page.waitForTimeout(3000);
    console.log("[DEBUG] Navigated to design, URL:", page.url());
  }

  // Now check for the filter toolbar group
  const filterGroup = page.locator('[id="semio.sketchpad.toolbar.group.filter"]');
  const filterGroupCount = await filterGroup.count();
  console.log("[DEBUG] Filter group toggle count:", filterGroupCount);
  
  if (filterGroupCount > 0) {
    // Click filter group to activate it
    await filterGroup.click();
    await page.waitForTimeout(1000);
    
    // Check settings zone
    const settingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
    const settingsCount = await settingsZone.count();
    console.log("[DEBUG] Settings zone visible:", settingsCount);
    
    // Check for individual filter toggles
    const pieceToggle = page.locator('[id="semio.sketchpad.app.design.toolbar.showPieces"]');
    const connToggle = page.locator('[id="semio.sketchpad.app.design.toolbar.showConnections"]');
    const portToggle = page.locator('[id="semio.sketchpad.app.design.toolbar.showPorts"]');
    
    const pCount = await pieceToggle.count();
    const cCount = await connToggle.count();
    const poCount = await portToggle.count();
    console.log("[DEBUG] Piece toggle:", pCount, "Connection toggle:", cCount, "Port toggle:", poCount);
    
    if (pCount > 0) {
      // Check initial pressed state
      const pieceState = await pieceToggle.getAttribute("data-state");
      console.log("[DEBUG] Pieces toggle initial state:", pieceState);
      
      // Click pieces toggle to disable pieces
      await pieceToggle.click();
      await page.waitForTimeout(500);
      
      const newUrl = page.url();
      console.log("[DEBUG] URL after toggling pieces off:", newUrl);
      
      const pieceStateAfter = await pieceToggle.getAttribute("data-state");
      console.log("[DEBUG] Pieces toggle state after click:", pieceStateAfter);
      
      // Check URL params
      const url = new URL(page.url().replace("#", "?hash="));
      const hashPart = page.url().split("#")[1] || "";
      console.log("[DEBUG] Hash part:", hashPart);
      
      // Click pieces again to enable
      await pieceToggle.click();
      await page.waitForTimeout(500);
      
      const urlAfterReenable = page.url();
      console.log("[DEBUG] URL after re-enabling pieces:", urlAfterReenable);
      
      await page.screenshot({ path: "/workspaces/semio/.semio-repo/🎫/26/02/16/DESIGN-APP-SKETCHPAD-FILTERS/filter-test.png" });
    }
  }
  
  await browser.close();
  console.log("[DEBUG] Test complete");
})();
