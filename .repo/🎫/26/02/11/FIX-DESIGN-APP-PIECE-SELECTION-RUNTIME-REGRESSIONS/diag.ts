import { chromium } from "playwright";

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  const debugLogs: string[] = [];
  page.on("console", (msg) => {
    const text = msg.text();
    if (text.includes("[DEBUG]")) {
      debugLogs.push(text);
      console.log("BROWSER:", text);
    }
  });

  await page.goto("http://localhost:5173");
  await page.waitForLoadState("networkidle");

  // Upload kit
  const fileInput = page.locator('input[type="file"]');
  const [fileChooser] = await Promise.all([
    page.waitForEvent("filechooser"),
    fileInput
      .first()
      .click()
      .catch(() => page.locator('button:has-text("Import"), button:has-text("Open"), [aria-label*="import"], [aria-label*="open"]').first().click()),
  ]);
  await fileChooser.setFiles("/workspaces/semio/assets/compose/metabolism.zip");
  await page.waitForTimeout(3000);

  // Navigate to design
  const nakaginText = page.getByText("Nakagin Capsule Tower");
  if (await nakaginText.isVisible({ timeout: 5000 }).catch(() => false)) {
    await nakaginText.dblclick();
    await page.waitForTimeout(5000);
  }

  console.log("URL:", page.url());

  const rf = page.locator(".react-flow").first();
  const pieces = rf.locator(".react-flow__node");
  const count = await pieces.count();
  console.log("Piece count:", count);

  if (count > 0) {
    const firstPiece = pieces.first();
    const box = await firstPiece.boundingBox();
    if (box) {
      console.log("Clicking piece at:", box.x + box.width / 2, box.y + box.height / 2);

      // Click with a 10s timeout
      const clickPromise = page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
      const timeoutPromise = new Promise((_, reject) => setTimeout(() => reject(new Error("CLICK TIMED OUT")), 10000));

      try {
        await Promise.race([clickPromise, timeoutPromise]);
        console.log("Click completed successfully");
      } catch (e: any) {
        console.error("Click failed:", e.message);
      }

      await page.waitForTimeout(2000);
      console.log("\n=== DEBUG LOGS ===");
      debugLogs.forEach((l) => console.log(l));
      console.log(`Total debug logs: ${debugLogs.length}`);
    }
  }

  await browser.close();
})();
