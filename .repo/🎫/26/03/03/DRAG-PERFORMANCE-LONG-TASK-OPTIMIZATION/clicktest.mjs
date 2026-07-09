import { chromium } from "playwright";
import path from "node:path";
const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || "http://127.0.0.1:5173";
const ZIP_PATH = path.resolve("/workspaces/semio/assets/compose/metabolism.zip");
async function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}
(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  await page.goto(BASE_URL);
  await page.waitForLoadState("domcontentloaded");
  await sleep(2000);
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: "attached", timeout: 10000 });
  const [fc] = await Promise.all([page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent("click")]);
  if (fc) await fc.setFiles(ZIP_PATH);
  else await fileInput.setInputFiles(ZIP_PATH);
  await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
  await sleep(500);
  const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
  if (await tableRow.isVisible().catch(() => false)) await tableRow.dblclick({ force: true });
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await sleep(3000);
  const allRowIds = await page.evaluate(() =>
    Array.from(document.querySelectorAll("[data-row-id]"))
      .map((el) => el.getAttribute("data-row-id"))
      .slice(0, 20),
  );
  const designRowIds = allRowIds.filter((id) => id?.startsWith("design-"));
  const nakaginRowId = designRowIds.find((id) => id?.includes("9a890dd4")) || designRowIds[designRowIds.length - 1];
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
  await sleep(8000);
  const diagram = page.locator("#diagram .react-flow").first();
  await diagram.waitFor({ state: "visible", timeout: 60000 });
  const nodes = diagram.locator(".react-flow__node");
  await nodes.first().waitFor({ state: "attached", timeout: 60000 });
  let lastPos = "";
  for (let i = 0; i < 30; i++) {
    await sleep(500);
    const pos = await page.evaluate(() =>
      Array.from(document.querySelectorAll(".react-flow__node"))
        .slice(0, 5)
        .map((n) => n.getAttribute("style"))
        .join("|"),
    );
    if (pos === lastPos && pos.length > 0) break;
    lastPos = pos;
  }
  console.log(`Nodes: ${await nodes.count()}`);

  // Add click listener to track clicks on nodes
  await page.evaluate(() => {
    window.__CLICK_LOG__ = [];
    document.addEventListener(
      "click",
      (e) => {
        const node = e.target.closest(".react-flow__node");
        window.__CLICK_LOG__.push({
          target: e.target.tagName + (e.target.className ? "." + e.target.className.toString().substring(0, 50) : ""),
          node: node ? node.getAttribute("data-id") : null,
          time: performance.now(),
          phase: e.eventPhase,
          defaultPrevented: e.defaultPrevented,
        });
      },
      true,
    ); // capture phase
    document.addEventListener(
      "click",
      (e) => {
        const node = e.target.closest(".react-flow__node");
        window.__CLICK_LOG__.push({
          target: "BUBBLE:" + e.target.tagName,
          node: node ? node.getAttribute("data-id") : null,
          time: performance.now(),
          phase: e.eventPhase,
          defaultPrevented: e.defaultPrevented,
        });
      },
      false,
    ); // bubble phase
  });

  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const sx = nodeBox.x + nodeBox.width / 2;
  const sy = nodeBox.y + nodeBox.height / 2;

  console.log("=== Test: Click without drag ===");
  await page.evaluate(() => {
    window.__CLICK_LOG__ = [];
  });
  await page.mouse.click(sx, sy);
  await sleep(500);
  let clicks = await page.evaluate(() => window.__CLICK_LOG__);
  console.log(`Clicks after click: ${clicks.length}`);
  for (const c of clicks) console.log(`  ${c.target} node=${c.node} phase=${c.phase} time=${c.time.toFixed(0)}`);

  console.log("\n=== Test: Drag then check for click ===");
  await page.evaluate(() => {
    window.__CLICK_LOG__ = [];
  });
  await page.mouse.move(sx, sy);
  await sleep(50);
  await page.mouse.down();
  await sleep(50);
  await page.mouse.move(sx + 100, sy, { steps: 5 });
  await page.mouse.up();
  await sleep(1000);
  clicks = await page.evaluate(() => window.__CLICK_LOG__);
  console.log(`Clicks after drag: ${clicks.length}`);
  for (const c of clicks) console.log(`  ${c.target} node=${c.node} phase=${c.phase} time=${c.time.toFixed(0)}`);

  await browser.close();
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
