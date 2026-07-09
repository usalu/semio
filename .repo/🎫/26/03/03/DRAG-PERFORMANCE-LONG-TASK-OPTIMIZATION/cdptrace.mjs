import { chromium } from "playwright";
import path from "node:path";
import fs from "node:fs";
const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || "http://127.0.0.1:5173";
const ZIP_PATH = path.resolve("/workspaces/semio/assets/compose/metabolism.zip");
const TRACE_DIR = path.dirname(new URL(import.meta.url).pathname);
async function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}
(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  await page.addInitScript(() => {
    window.__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
    const store = window.__COMPOSE_PERFORMANCE__;
    const obs = window.PerformanceObserver;
    if (!obs || !(obs.supportedEntryTypes || []).includes("longtask")) return;
    store.longTaskSupported = true;
    new obs((list) => {
      store.longTasks.push(...list.getEntries().map((e) => ({ duration: e.duration, startTime: e.startTime })));
    }).observe({ entryTypes: ["longtask"] });
  });
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
  await page.waitForLoadState("networkidle");
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
  await page.waitForLoadState("networkidle");
  await sleep(5000);
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
  const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
  if (await leftPanelToggle.isVisible().catch(() => false)) {
    const leftPanelOpen = await page
      .locator('[data-panel="leftSidePanel"]')
      .isVisible()
      .catch(() => false);
    if (leftPanelOpen) {
      await leftPanelToggle.click();
      await sleep(500);
    }
  }
  await page.evaluate(() => {
    window.__COMPOSE_PERFORMANCE__.longTasks = [];
  });
  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const sx = nodeBox.x + nodeBox.width / 2;
  const sy = nodeBox.y + nodeBox.height / 2;

  // Instrument: add detailed timing around drag
  await page.addScriptTag({
    content: `
    window.__DRAG_TRACE__ = [];
    const origObs = window.__COMPOSE_PERFORMANCE__;
    // Also inject timing into onNodesChange if possible
    const origPerf = performance;
  `,
  });

  // Use CDP tracing to capture a short trace around drag
  const cdp = await page.context().newCDPSession(page);

  await page.mouse.move(sx, sy);
  await sleep(100);
  await page.mouse.down();
  await sleep(100);

  // Start CDP trace
  await cdp.send("Tracing.start", {
    categories: "-*,devtools.timeline,v8.execute,disabled-by-default-devtools.timeline,disabled-by-default-devtools.timeline.stack",
    options: "sampling-frequency=1000",
  });

  await page.evaluate(() => performance.mark("drag-trace-start"));
  await page.mouse.move(sx + 50, sy, { steps: 1 });
  await page.mouse.up();
  await sleep(5000);
  await page.evaluate(() => performance.mark("drag-trace-end"));

  // Stop tracing
  const traceChunks = [];
  cdp.on("Tracing.dataCollected", ({ value }) => {
    traceChunks.push(...value);
  });
  await cdp.send("Tracing.end");
  await new Promise((r) => cdp.on("Tracing.tracingComplete", r));

  // Parse trace for RunTask events
  const bigTasks = traceChunks
    .filter((e) => e.cat?.includes("devtools.timeline") && (e.name === "RunTask" || e.name === "FunctionCall" || e.name === "EvaluateScript" || e.name === "EventDispatch"))
    .filter((e) => (e.dur || 0) > 50000) // >50ms in microseconds
    .sort((a, b) => (b.dur || 0) - (a.dur || 0))
    .slice(0, 10);

  console.log(`\n=== TOP 10 LONG TRACE EVENTS (>50ms) ===`);
  for (const e of bigTasks) {
    const durMs = ((e.dur || 0) / 1000).toFixed(0);
    const startMs = ((e.ts || 0) / 1000).toFixed(0);
    console.log(`  ${e.name} ${e.ph || ""}: dur=${durMs}ms start=${startMs}ms cat=${e.cat}`);
    if (e.args?.data) {
      const d = e.args.data;
      if (d.type) console.log(`    type=${d.type}`);
      if (d.functionName) console.log(`    fn=${d.functionName}`);
      if (d.url) console.log(`    url=${d.url}`);
      if (d.scriptName) console.log(`    script=${d.scriptName}`);
    }
  }

  // Also check EventDispatch events
  const eventDispatches = traceChunks
    .filter((e) => e.name === "EventDispatch" && (e.dur || 0) > 50000)
    .sort((a, b) => (b.dur || 0) - (a.dur || 0))
    .slice(0, 5);
  console.log(`\n=== EVENT DISPATCHES > 50ms ===`);
  for (const e of eventDispatches) {
    console.log(`  ${e.args?.data?.type || "unknown"}: dur=${((e.dur || 0) / 1000).toFixed(0)}ms`);
  }

  // Check Layout/Style events
  const layoutEvents = traceChunks
    .filter((e) => (e.name === "Layout" || e.name === "UpdateLayoutTree" || e.name === "RecalculateStyles" || e.name === "Paint") && (e.dur || 0) > 50000)
    .sort((a, b) => (b.dur || 0) - (a.dur || 0))
    .slice(0, 5);
  console.log(`\n=== LAYOUT/STYLE/PAINT > 50ms ===`);
  for (const e of layoutEvents) {
    console.log(`  ${e.name}: dur=${((e.dur || 0) / 1000).toFixed(0)}ms elementCount=${e.args?.elementCount || e.args?.data?.elementCount || "?"}`);
  }

  // Write full trace for analysis
  fs.writeFileSync(path.join(TRACE_DIR, "trace.json"), JSON.stringify(traceChunks, null, 0));
  console.log(`\nTrace written to trace.json (${traceChunks.length} events)`);

  const lt = await page.evaluate(() => {
    const s = window.__COMPOSE_PERFORMANCE__;
    const tasks = s.longTasks.sort((a, b) => b.duration - a.duration).slice(0, 3);
    return { count: s.longTasks.length, tasks };
  });
  console.log(`\n=== LONG TASKS ===`);
  console.log(`  Total: ${lt.count}`);
  for (const t of lt.tasks) console.log(`  dur=${t.duration.toFixed(0)}ms start=${t.startTime.toFixed(0)}ms`);

  await browser.close();
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
