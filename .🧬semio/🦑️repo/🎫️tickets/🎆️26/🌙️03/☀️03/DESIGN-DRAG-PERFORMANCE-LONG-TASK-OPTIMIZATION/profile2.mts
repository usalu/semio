import fs from "fs";
import path from "path";
import { CDPSession, chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox"] });
const page = await browser.newPage();
await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);
const zipPath = path.resolve(process.cwd(), "assets/compose/metabolism.zip");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
const [fileChooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent("click")]);
if (fileChooser) await fileChooser.setFiles(zipPath);
else {
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el) => {
    el.dispatchEvent(new Event("change", { bubbles: true }));
  });
}
await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(2000);
const designRowIds = await page.evaluate(() => Array.from(document.querySelectorAll('[data-row-id^="design-"]')).map((el) => el.getAttribute("data-row-id")));
const nakaginRowId = designRowIds.find((id) => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
if (nakaginRowId) {
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(10000);
// Close left panel
const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await leftPanelToggle.isVisible().catch(() => false)) {
  const leftPanelOpen = await page
    .locator('[data-panel="leftSidePanel"]')
    .isVisible()
    .catch(() => false);
  if (leftPanelOpen) {
    await leftPanelToggle.click();
    await page.waitForTimeout(500);
  }
}
await page.waitForTimeout(3000);
console.log("Ready for profiling...");
// Setup drag
const firstNode = page.locator(".react-flow__node").first();
const nodeBox = await firstNode.boundingBox();
const nx = nodeBox!.x + nodeBox!.width / 2;
const ny = nodeBox!.y + nodeBox!.height / 2;
// Zoom first
const pane = page.locator("#diagram .react-flow__pane").first();
const paneBox = await pane.boundingBox();
await page.mouse.move(paneBox!.x + paneBox!.width / 2, paneBox!.y + paneBox!.height / 2);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// Start drag (without profiling)
await page.mouse.move(nx, ny);
await page.mouse.down();
await page.waitForTimeout(50);
await page.mouse.move(nx + 100, ny, { steps: 20 });
// NOW profile the mouseup + settle
const cdp: CDPSession = await page.context().newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.start");
console.log("Profile started, releasing mouse...");
await page.mouse.up();
await page.waitForTimeout(3000);
const { profile } = await cdp.send("Profiler.stop");
const outPath = path.resolve(process.cwd(), ".repo/tickets/2026/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/mouseup2.cpuprofile");
fs.writeFileSync(outPath, JSON.stringify(profile));
console.log(`Profile saved to ${outPath}`);
// Analyze top functions
const nodes2 = profile.nodes;
const totalSamples = profile.samples?.length ?? 0;
const hitCounts: Record<string, number> = {};
for (const node of nodes2) {
  const fn = node.callFrame.functionName || "(anonymous)";
  const url = node.callFrame.url || "";
  const line = node.callFrame.lineNumber;
  const key = fn === "(anonymous)" ? `(anonymous)@${url.split("/").pop()}:${line}` : `${fn}@${url.split("/").pop()}:${line}`;
  hitCounts[key] = (hitCounts[key] ?? 0) + (node.hitCount ?? 0);
}
const sorted = Object.entries(hitCounts)
  .sort((a, b) => b[1] - a[1])
  .slice(0, 30);
console.log("\n=== TOP 30 FUNCTIONS (by hit count) ===");
for (const [fn, hits] of sorted) {
  const pct = ((hits / totalSamples) * 100).toFixed(1);
  console.log(`${pct}% (${hits} hits) - ${fn}`);
}
await browser.close();
