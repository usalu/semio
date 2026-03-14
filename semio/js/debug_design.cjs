const { chromium } = require("@playwright/test");
const path = require("path");
(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", err => errors.push(err.message.substring(0, 200) + " | STACK: " + (err.stack||"").substring(0, 300)));
  
  await page.goto("http://localhost:5173/");
  await page.waitForTimeout(2000);
  const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
  await fileInput.setInputFiles(path.resolve(__dirname, "..", "assets/semio/metabolism.zip"));
  await fileInput.evaluate(el => el.dispatchEvent(new Event("change", { bubbles: true })));
  await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
  await page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first().dblclick({ force: true });
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await page.waitForTimeout(2000);
  await page.locator('tr[data-row-id^="design-"]').first().dblclick({ force: true });
  await page.waitForURL(/.*designs\/.+/, { timeout: 30000 });
  await page.waitForTimeout(8000);
  
  console.log("diagram count:", await page.locator("#diagram").count());
  console.log("react-flow count:", await page.locator(".react-flow").count());
  console.log("navbar count:", await page.locator('[id="semio.sketchpad.navbar"]').count());
  const rootLen = await page.evaluate(() => document.getElementById("root").innerHTML.length);
  console.log("root innerHTML len:", rootLen);
  console.log("Error count:", errors.length);
  for (let i = 0; i < Math.min(errors.length, 8); i++) console.log("  ERR " + i + ": " + errors[i].substring(0, 300));
  await browser.close();
  process.exit(0);
})().catch(e => { console.error(e); process.exit(1); });
