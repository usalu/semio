import { chromium } from "playwright";
import path from "path";
import { fileURLToPath } from "url";
import fs from "fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const logFile = path.join(__dirname, "debug.log");
const zipPath = "/workspaces/semio/assets/compose/metabolism.zip";

function log(msg) {
  const line = `${new Date().toISOString()} ${msg}\n`;
  fs.appendFileSync(logFile, line);
  process.stdout.write(line);
}

fs.writeFileSync(logFile, "");
log("Starting debug script");

const browser = await chromium.launch({ executablePath: "/usr/bin/google-chrome-stable", args: ["--no-sandbox", "--disable-gpu", "--disable-dev-shm-usage"] });
const page = await browser.newPage();

const debugLogs = [];
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("DEBUG") || text.includes("addSection")) {
    debugLogs.push(text);
    console.log("BROWSER:", text);
  }
});

log("--- Navigating to home ---");
await page.goto("http://localhost:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(3000);

log("--- Importing zip ---");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
await fileInput.setInputFiles(zipPath);
await page.waitForTimeout(2000);

log("--- Waiting for Metabolism ---");
const metabolismText = page.getByText("Metabolism", { exact: true }).first();
await metabolismText.waitFor({ state: "visible", timeout: 60000 });
log("Metabolism visible!");

const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
const isTableRowVisible = await tableRow.isVisible().catch(() => false);
log("Table row visible:", isTableRowVisible);
if (isTableRowVisible) {
  await tableRow.dblclick({ force: true });
} else {
  await metabolismText.dblclick({ force: true });
}

await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
log("Kit URL:", page.url());
await page.waitForTimeout(2000);

log("--- Finding and navigating to design ---");
const allRowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll("[data-row-id]"))
    .map((el) => el.getAttribute("data-row-id"))
    .slice(0, 20),
);
log("Row IDs:", JSON.stringify(allRowIds));

const designRowIds = allRowIds.filter((id) => id?.startsWith("design-"));
if (designRowIds.length > 0) {
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, designRowIds[0]);
} else {
  const designElement = page.getByText("Nakagin Capsule Tower", { exact: true }).first();
  if (await designElement.isVisible({ timeout: 5000 }).catch(() => false)) {
    await designElement.dblclick({ force: true });
  }
}

await page.waitForLoadState("networkidle");
await page.waitForTimeout(8000);
log("Design URL:", page.url());

const rfNodes = await page.locator(".react-flow__node").count();
log("ReactFlow nodes:", rfNodes);

log("--- Checking all debug flags ---");
const debugState = await page.evaluate(() => ({
  appRender: window.__DEBUG_APP_RENDER__,
  effectTop: window.__DEBUG_EFFECT_TOP__,
  panelState: window.__DEBUG_PANEL_STATE__,
  sectionContent: window.__DEBUG_DESIGN_SECTION__,
}));
log("DEBUG STATE:", JSON.stringify(debugState, null, 2));

log("--- Opening right panel ---");
const rightToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.rightSidePanel"]').first();
if (await rightToggle.isVisible({ timeout: 3000 }).catch(() => false)) {
  await rightToggle.click();
  await page.waitForTimeout(1000);
}

const detailsTab = page.locator('[id="compose.sketchpad.navbar.panelToggle.details.show"]').first();
if (await detailsTab.isVisible({ timeout: 3000 }).catch(() => false)) {
  await detailsTab.click();
  await page.waitForTimeout(1000);
}

log("--- After opening panel, checking flags again ---");
const debugState2 = await page.evaluate(() => ({
  appRender: window.__DEBUG_APP_RENDER__,
  effectTop: window.__DEBUG_EFFECT_TOP__,
  panelState: window.__DEBUG_PANEL_STATE__,
  sectionContent: window.__DEBUG_DESIGN_SECTION__,
}));
log("DEBUG STATE AFTER:", JSON.stringify(debugState2, null, 2));

const designNameExists = await page.evaluate(() => !!document.querySelector('[id="compose.sketchpad.app.design.panel.details.section.design.name"]'));
log("Design name input exists:", designNameExists);

const rightPanelInfo = await page.evaluate(() => {
  const rp = document.querySelector('[data-panel="rightSidePanel"]');
  if (!rp) return "no rightSidePanel found";
  const ids = Array.from(rp.querySelectorAll("[id]"))
    .map((el) => el.id)
    .filter((id) => id.includes("compose"));
  const sectionButtons = Array.from(rp.querySelectorAll('[role="button"]')).map((el) => el.id || el.textContent?.slice(0, 40));
  return { ids: ids.slice(0, 30), sectionButtons: sectionButtons.slice(0, 20) };
});
log("Right panel info:", JSON.stringify(rightPanelInfo, null, 2));

log("--- Collected debug logs from browser ---");
debugLogs.forEach((l) => console.log("  ", l));

await browser.close();
