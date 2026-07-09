import { chromium } from "playwright";
import { writeFileSync } from "fs";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu"] });
const page = await browser.newPage();
await page.goto("http://127.0.0.1:5173", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(2000);
const kitRow = page.locator("table tbody tr").first();
await kitRow.dblclick();
await page.waitForTimeout(2000);
const designRows = page.locator('table tbody tr[id^="design-"]');
const designRow = designRows.first();
await designRow.waitFor({ state: "visible", timeout: 60000 });
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
const pane = diag.locator(".react-flow__pane").first();
const paneBox = await pane.boundingBox();
const cx = paneBox.x + paneBox.width / 2,
  cy = paneBox.y + paneBox.height / 2;
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
const firstNode = nodes.first();
const box = await firstNode.boundingBox();
const sx = box.x + box.width / 2,
  sy = box.y + box.height / 2;
await page.mouse.move(sx, sy);
await page.waitForTimeout(200);
const client = await page.context().newCDPSession(page);
await client.send("Profiler.enable");
await client.send("Profiler.start");
await page.mouse.down();
await page.waitForTimeout(50);
await page.mouse.move(sx + 100, sy, { steps: 20 });
await page.mouse.up();
await page.waitForTimeout(6000);
const { profile } = await client.send("Profiler.stop");
const fnSelf = new Map();
for (const node of profile.nodes) {
  const fn = node.callFrame;
  const file = fn.url ? fn.url.split("/").pop().split("?")[0] : "?";
  const key = `${fn.functionName || "(anon)"}@${file}:${fn.lineNumber}`;
  fnSelf.set(key, (fnSelf.get(key) || 0) + (node.hitCount || 0));
}
const sorted = [...fnSelf.entries()].sort((a, b) => b[1] - a[1]).slice(0, 40);
console.log("\n=== TOP 40 HOTTEST FUNCTIONS (self time) ===");
for (const [key, hits] of sorted) console.log(`  ${hits} hits: ${key}`);
await browser.close();
