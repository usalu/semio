import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu"] });
const page = await browser.newPage();
const setNodesLogs = [];
page.on("console", (msg) => {
  const txt = msg.text();
  if (txt.includes("[DEBUG] setNodes")) setNodesLogs.push({ t: Date.now(), txt: txt.slice(0, 300) });
});
await page.goto("http://127.0.0.1:5173", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(2000);
await page.locator("table tbody tr").first().dblclick();
await page.waitForTimeout(2000);
const designRow = page.locator('table tbody tr[id^="design-"]').first();
await designRow.waitFor({ state: "visible", timeout: 30000 });
await designRow.dblclick();
await page.waitForTimeout(5000);
const diag = page.locator("#diagram .react-flow").first();
await diag.waitFor({ state: "visible", timeout: 60000 });
const nodes = diag.locator(".react-flow__node");
await nodes.first().waitFor({ state: "attached", timeout: 60000 });
for (let i = 0; i < 10; i++) {
  await page.waitForTimeout(2000);
  if ((await nodes.count()) >= 170) break;
}
await page.waitForTimeout(3000);
console.log("nodes:", await nodes.count());
// Close panel
const toggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await toggle.isVisible().catch(() => false)) {
  const leftOpen = await page
    .locator('[data-panel="leftSidePanel"]')
    .isVisible()
    .catch(() => false);
  if (leftOpen) {
    await toggle.click();
    await page.waitForTimeout(500);
  }
}
// Zoom
const pane = diag.locator(".react-flow__pane").first();
const paneBox = await pane.boundingBox();
const cx = paneBox.x + paneBox.width / 2,
  cy = paneBox.y + paneBox.height / 2;
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// Clear logs, drag
setNodesLogs.length = 0;
const firstNode = nodes.first();
const box = await firstNode.boundingBox();
const sx = box.x + box.width / 2,
  sy = box.y + box.height / 2;
await page.mouse.move(sx, sy);
await page.waitForTimeout(100);
const t0 = Date.now();
await page.mouse.down();
await page.waitForTimeout(50);
await page.mouse.move(sx + 100, sy, { steps: 20 });
await page.mouse.up();
await page.waitForTimeout(6000);
console.log(`\n=== setNodes calls during drag (${setNodesLogs.length}) ===`);
for (const log of setNodesLogs) {
  console.log(`  +${log.t - t0}ms: ${log.txt}`);
}
await browser.close();
