import { chromium } from "playwright";
import path from "path";

async function testTooltipPositioning() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
  await page.goto("http://localhost:5173/", { waitUntil: "networkidle", timeout: 30000 });
  await page.waitForTimeout(3000);
  const zipPath = path.resolve("/workspaces/semio/assets/compose/metabolism.zip");
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  const isAttached = (await fileInput.count()) > 0;
  console.log(`[DEBUG] importKit input attached: ${isAttached}`);
  if (isAttached) {
    await fileInput.setInputFiles(zipPath);
    await fileInput.evaluate((el) => el.dispatchEvent(new Event("change", { bubbles: true })));
    const metabolismText = page.getByText("Metabolism", { exact: true }).first();
    await metabolismText.waitFor({ state: "visible", timeout: 60000 }).catch(() => console.log("[DEBUG] Metabolism text didn't appear"));
    await page.waitForTimeout(1000);
    const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
    const rowVisible = await tableRow.isVisible().catch(() => false);
    console.log(`[DEBUG] Metabolism row visible: ${rowVisible}`);
    if (rowVisible) {
      await tableRow.dblclick({ force: true });
      await page.waitForURL(/.*kits\/.+/, { timeout: 30000 }).catch(() => console.log("[DEBUG] Kit URL didn't appear"));
      console.log(`[DEBUG] URL after opening kit: ${page.url()}`);
      await page.waitForTimeout(3000);
    }
  }
  console.log(`[DEBUG] Current URL: ${page.url()}`);
  const designsTab = page.getByText("Designs", { exact: true }).first();
  if (await designsTab.isVisible().catch(() => false)) {
    await designsTab.click();
    await page.waitForTimeout(2000);
    console.log(`[DEBUG] Switched to Designs tab`);
  }
  const designRow = page.locator("tr[data-row-id]").first();
  if (await designRow.isVisible().catch(() => false)) {
    await designRow.dblclick({ force: true });
    await page.waitForTimeout(5000);
    console.log(`[DEBUG] URL after dblclick: ${page.url()}`);
  }
  console.log(`[DEBUG] Final URL: ${page.url()}`);
  const tooltipTriggers = await page.locator('[data-slot="toggle-group-item"], [data-slot="button-group-item"], [data-slot="action-group-item"], [data-slot="property-label"]').all();
  console.log(`[DEBUG] Found ${tooltipTriggers.length} tooltip trigger elements`);
  for (let i = 0; i < Math.min(tooltipTriggers.length, 30); i++) {
    const trigger = tooltipTriggers[i];
    const isVisible = await trigger.isVisible({ timeout: 1000 }).catch(() => false);
    if (!isVisible) continue;
    const box = await trigger.boundingBox({ timeout: 1000 }).catch(() => null);
    if (!box) continue;
    console.log(`[DEBUG] Trigger ${i}: visible at (${box.x.toFixed(0)}, ${box.y.toFixed(0)}) ${box.width.toFixed(0)}x${box.height.toFixed(0)}`);
    await trigger.hover({ force: true, timeout: 2000 }).catch(() => {});
    await page.waitForTimeout(600);
    const tooltipContent = page.locator('[data-slot="tooltip-content"]');
    const tooltipCount = await tooltipContent.count();
    if (tooltipCount > 0) {
      const tooltipBox = await tooltipContent.first().boundingBox();
      if (tooltipBox) {
        console.log(`[DEBUG] Tooltip ${i}: at (${tooltipBox.x.toFixed(0)}, ${tooltipBox.y.toFixed(0)}) ${tooltipBox.width.toFixed(0)}x${tooltipBox.height.toFixed(0)}`);
        const dx = Math.abs(tooltipBox.x - box.x);
        const dy = Math.abs(tooltipBox.y - box.y);
        const atTopLeft = tooltipBox.x < 10 && tooltipBox.y < 10;
        if (atTopLeft) {
          console.log(`[DEBUG] ❌ TOOLTIP AT TOP-LEFT! Trigger at (${box.x.toFixed(0)},${box.y.toFixed(0)}) but tooltip at (${tooltipBox.x.toFixed(0)},${tooltipBox.y.toFixed(0)})`);
        } else {
          console.log(`[DEBUG] ✅ Tooltip near trigger (dx=${dx.toFixed(0)}, dy=${dy.toFixed(0)})`);
        }
      } else {
        console.log(`[DEBUG] Tooltip ${i}: visible but no boundingBox`);
      }
    } else {
      console.log(`[DEBUG] Trigger ${i}: no tooltip appeared`);
    }
    await page.mouse.move(0, 0);
    await page.waitForTimeout(300);
  }
  const allTooltipContent = await page.locator('[data-slot="tooltip-content"]').all();
  for (const tc of allTooltipContent) {
    const style = await tc.evaluate((el) => {
      const cs = window.getComputedStyle(el);
      const parent = el.parentElement;
      const parentStyle = parent ? window.getComputedStyle(parent) : null;
      return {
        position: cs.position,
        top: cs.top,
        left: cs.left,
        transform: cs.transform,
        parentPosition: parentStyle?.position,
        parentTransform: parentStyle?.transform,
      };
    });
    console.log(`[DEBUG] Tooltip computed style:`, JSON.stringify(style));
  }
  await browser.close();
  console.log("[DEBUG] Done");
}

testTooltipPositioning().catch((e) => {
  console.error("[DEBUG] Error:", e);
  process.exit(1);
});
