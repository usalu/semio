import fs from "fs";
import path from "path";
import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
const page = await context.newPage();

await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);

const zipPath = path.resolve("/workspaces/semio/compose/assets/compose/metabolism.zip");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
const [fileChooser] = await Promise.all([
  page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
  fileInput.dispatchEvent("click"),
]);
if (fileChooser) await fileChooser.setFiles(zipPath);
else await fileInput.setInputFiles(zipPath);
await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(3000);

const designRowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll("[data-row-id]")).map(el => el.getAttribute("data-row-id")).filter(id => id?.startsWith("design-"))
);
const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
await page.evaluate((rowId) => {
  const row = document.querySelector(`[data-row-id="${rowId}"]`);
  if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
}, nakaginRowId);
await page.waitForLoadState("networkidle");
await page.waitForTimeout(10000);

const nodeCount = await page.locator("#diagram .react-flow__node").count();
console.log(`Nodes: ${nodeCount}`);

const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await leftPanelToggle.isVisible().catch(() => false)) {
  const open = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
  if (open) { await leftPanelToggle.click(); await page.waitForTimeout(500); }
}

const firstNode = page.locator("#diagram .react-flow__node").first();
const nb = await firstNode.boundingBox();
if (!nb) { console.log("No node"); await browser.close(); process.exit(1); }

await page.mouse.move(nb.x + nb.width / 2, nb.y + nb.height / 2);
await page.waitForTimeout(50);
await page.mouse.down();
await page.waitForTimeout(50);
await page.mouse.move(nb.x + nb.width / 2 + 100, nb.y + nb.height / 2, { steps: 20 });
await page.waitForTimeout(100);

const cdp = await context.newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.start");

console.log("Profiling mouseup + settle...");
await page.mouse.up();
await page.waitForTimeout(5000);

const profile = await cdp.send("Profiler.stop");

const topFunctions = new Map<string, number>();
for (const node of profile.profile.nodes) {
  if (node.hitCount && node.hitCount > 0 && node.callFrame.functionName) {
    const url = node.callFrame.url?.split("/").pop() || "?";
    const key = `${node.callFrame.functionName} (${url}:${node.callFrame.lineNumber})`;
    topFunctions.set(key, (topFunctions.get(key) || 0) + node.hitCount);
  }
}
const sorted = [...topFunctions.entries()].sort((a, b) => b[1] - a[1]).slice(0, 40);
console.log("\nTop functions during mouseup + 5s settle:");
let totalHits = 0;
for (const [, hits] of sorted) totalHits += hits;
for (const [fn, hits] of sorted) {
  const pct = (hits / totalHits * 100).toFixed(1);
  console.log(`  ${hits} hits (${pct}%): ${fn}`);
}

const ticketDir = "/workspaces/semio/.repo/🎫/26/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION";
fs.writeFileSync(`${ticketDir}/mouseup-profile.cpuprofile`, JSON.stringify(profile.profile));
console.log(`\nProfile saved to ${ticketDir}/mouseup-profile.cpuprofile`);

await cdp.detach();
await browser.close();
