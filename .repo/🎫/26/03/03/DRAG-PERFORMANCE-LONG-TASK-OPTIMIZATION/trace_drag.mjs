import { chromium } from "playwright";
const zipPath = "/workspaces/semio/compose/assets/compose/metabolism.zip";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu"] });
const ctx = await browser.newContext();
const page = await ctx.newPage();
await page.goto("http://127.0.0.1:5173/", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(2000);
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
const [fc] = await Promise.all([page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent("click")]);
if (fc) await fc.setFiles(zipPath);
else {
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el) => el.dispatchEvent(new Event("change", { bubbles: true })));
}
await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
if (await tableRow.isVisible().catch(() => false)) await tableRow.dblclick({ force: true });
else await page.getByText("Metabolism").first().dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForTimeout(5000);
const dids = await page.evaluate(() =>
  Array.from(document.querySelectorAll("[data-row-id]"))
    .map((el) => el.getAttribute("data-row-id"))
    .filter((id) => id?.startsWith("design-"))
    .slice(0, 5),
);
if (dids.length > 0) {
  const rid = dids.find((id) => id?.includes("9a890dd4")) ?? dids[dids.length - 1];
  await page.evaluate((r) => {
    document.querySelector(`[data-row-id="${r}"]`)?.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, rid);
}
await page.waitForTimeout(8000);
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
  if (
    await page
      .locator('[data-panel="leftSidePanel"]')
      .isVisible()
      .catch(() => false)
  ) {
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
await page.waitForTimeout(2000);
// Start CDP trace
const cdp = await ctx.newCDPSession(page);
await cdp.send("Tracing.start", { categories: "devtools.timeline,v8.execute,disabled-by-default-devtools.timeline,disabled-by-default-devtools.timeline.frame", options: "sampling-frequency=10000" });
// Drag
const firstNode = nodes.first();
const box = await firstNode.boundingBox();
const sx = box.x + box.width / 2,
  sy = box.y + box.height / 2;
await page.mouse.move(sx, sy);
await page.waitForTimeout(100);
await page.mouse.down();
await page.waitForTimeout(50);
await page.mouse.move(sx + 100, sy, { steps: 20 });
await page.mouse.up();
await page.waitForTimeout(3000);
// Stop trace
const traceChunks = [];
cdp.on("Tracing.dataCollected", (data) => traceChunks.push(...data.value));
await cdp.send("Tracing.end");
await new Promise((resolve) => cdp.on("Tracing.tracingComplete", resolve));
// Write trace
const fs = await import("fs");
const tracePath = "/workspaces/semio/.repo/tickets/2026/03/03/DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/trace.json";
fs.writeFileSync(tracePath, JSON.stringify(traceChunks));
console.log(`Trace saved to ${tracePath} (${traceChunks.length} events)`);
// Quick analysis: find top functions by duration
const functionDurations = new Map();
for (const event of traceChunks) {
  if (event.ph === "X" && event.dur > 1000 && event.name) {
    const name = event.name;
    functionDurations.set(name, (functionDurations.get(name) || 0) + event.dur);
  }
}
const sorted = [...functionDurations.entries()].sort((a, b) => b[1] - a[1]);
console.log("\n=== Top functions by total duration (>1ms events) ===");
for (const [name, dur] of sorted.slice(0, 20)) {
  console.log(`  ${name}: ${(dur / 1000).toFixed(1)}ms`);
}
await browser.close();
