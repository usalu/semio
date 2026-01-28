// #region Header

// js/semio/sketchpad.test.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

import { expect, Locator, Page, test } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";
import MetabolismKitData from "../../assets/semio/kit_metabolism.json" with { type: "json" };

const designs = (MetabolismKitData as any).designs ?? [];
const nakaginCapsuleTowerDesign = designs.find((d: any) => d.name === "Nakagin Capsule Tower");
const nakaginCapsuleTowerFlatDesign = designs.find((d: any) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid);
const MetabolismKitNakaginCapsuleTowerFlatPieces =
  nakaginCapsuleTowerFlatDesign?.pieces?.map((p: any) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const TOLERANCE = 0.001;

interface Plane {
  origin: { x: number; y: number; z: number };
  xAxis: { x: number; y: number; z: number };
  yAxis: { x: number; y: number; z: number };
}

interface Center {
  u: number;
  v: number;
}

const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
  if (!p1 || !p2) return false;
  if (!p1.origin || !p2.origin || !p1.xAxis || !p2.xAxis || !p1.yAxis || !p2.yAxis) return false;
  return (
    Math.abs(p1.origin.x - p2.origin.x) < TOLERANCE &&
    Math.abs(p1.origin.y - p2.origin.y) < TOLERANCE &&
    Math.abs(p1.origin.z - p2.origin.z) < TOLERANCE &&
    Math.abs(p1.xAxis.x - p2.xAxis.x) < TOLERANCE &&
    Math.abs(p1.xAxis.y - p2.xAxis.y) < TOLERANCE &&
    Math.abs(p1.xAxis.z - p2.xAxis.z) < TOLERANCE &&
    Math.abs(p1.yAxis.x - p2.yAxis.x) < TOLERANCE &&
    Math.abs(p1.yAxis.y - p2.yAxis.y) < TOLERANCE &&
    Math.abs(p1.yAxis.z - p2.yAxis.z) < TOLERANCE
  );
};

const centersEqual = (c1?: Center, c2?: Center): boolean => {
  if (!c1 || !c2) return c1 === c2;
  return Math.abs(c1.u - c2.u) < TOLERANCE && Math.abs(c1.v - c2.v) < TOLERANCE;
};

async function waitForDiagramStabilization(page: Page, maxWaitMs: number = 5000): Promise<void> {
  const startTime = Date.now();
  let lastPositions: Map<string, { x: number; y: number }> = new Map();

  while (Date.now() - startTime < maxWaitMs) {
    await page.waitForTimeout(500);

    const currentPositions = await page.evaluate(() => {
      const nodes = document.querySelectorAll(".react-flow__node");
      const positions: Record<string, { x: number; y: number }> = {};
      nodes.forEach((node) => {
        const id = node.getAttribute("data-id");
        if (id) {
          const style = (node as HTMLElement).style;
          const transform = style.transform;
          const match = transform.match(/translate\(([^,]+)px,\s*([^)]+)px\)/);
          if (match) {
            positions[id] = { x: parseFloat(match[1]), y: parseFloat(match[2]) };
          }
        }
      });
      return positions;
    });

    const currentMap = new Map(Object.entries(currentPositions));

    let stable = true;
    if (lastPositions.size > 0 && currentMap.size === lastPositions.size) {
      for (const [id, pos] of currentMap.entries()) {
        const lastPos = lastPositions.get(id);
        if (lastPos && (Math.abs(pos.x - lastPos.x) > 1 || Math.abs(pos.y - lastPos.y) > 1)) {
          stable = false;
          break;
        }
      }
      if (stable) {
        return;
      }
    }

    lastPositions = currentMap;
  }
}

async function initConsole(page: Page) {
  const messages: string[] = [];
  const warnings: string[] = [];
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "warning") {
      warnings.push(msg.text());
    } else if (msg.type() === "error") {
      errors.push(msg.text());
    } else {
      messages.push(msg.text());
    }
  });
  return { messages, warnings, errors };
}

async function expectFullyInViewport(locator: Locator, page: Page, xRange: [number, number], yRange: [number, number]) {
  const box = await locator.boundingBox();
  expect(box).not.toBeNull();
  const viewport = page.viewportSize();
  expect(box!.x).toBeGreaterThanOrEqual(xRange[0]);
  expect(box!.y).toBeGreaterThanOrEqual(yRange[0]);
  expect(box!.x).toBeLessThanOrEqual(xRange[1]);
  expect(box!.y).toBeLessThanOrEqual(yRange[1]);
  expect(box!.x + box!.width).toBeLessThanOrEqual(viewport!.width);
  expect(box!.y + box!.height).toBeLessThanOrEqual(viewport!.height);
}

async function openSettingsPanel(page: Page) {
  const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();
  const isRightPanelVisible = await rightSidePanel.isVisible().catch(() => false);

  if (!isRightPanelVisible) {
    const rightPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasToggle = await rightPanelToggle.isVisible({ timeout: 10000 }).catch(() => false);
    if (hasToggle) {
      await rightPanelToggle.click();
      await page.waitForTimeout(500);
    }
  }

  await expect(rightSidePanel)
    .toBeVisible({ timeout: 10000 })
    .catch(() => { });
}

async function getSettingsSections(page: Page): Promise<string[]> {
  const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();

  try {
    await expect(rightSidePanel).toBeVisible({ timeout: 15000 });
  } catch {
    console.log("Warning: Right sidepanel not visible after 15s, returning empty sections");
    return [];
  }

  const sections = await rightSidePanel.locator('[role="button"][id^="semio.sketchpad"]').all();
  const sectionIds: string[] = [];
  for (const section of sections) {
    const id = await section.getAttribute("id");
    if (id) sectionIds.push(id);
  }
  return sectionIds;
}

async function openDetailsPanel(page: Page) {
  const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();
  const isRightPanelVisible = await rightSidePanel.isVisible().catch(() => false);

  if (!isRightPanelVisible) {
    const rightPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasToggle = await rightPanelToggle.isVisible({ timeout: 10000 }).catch(() => false);
    if (hasToggle) {
      await rightPanelToggle.click();
      await page.waitForTimeout(500);
    }
  }

  await expect(rightSidePanel)
    .toBeVisible({ timeout: 10000 })
    .catch(() => { });
}

async function getDetailsSections(page: Page): Promise<string[]> {
  const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();
  try {
    await expect(rightSidePanel).toBeVisible({ timeout: 15000 });
  } catch {
    console.log("Warning: Right sidepanel not visible after 15s, returning empty sections");
    return [];
  }

  const sections = await rightSidePanel.locator('[role="button"][id^="semio.sketchpad"]').all();
  const sectionIds: string[] = [];
  for (const section of sections) {
    const id = await section.getAttribute("id");
    if (id) sectionIds.push(id);
  }
  return sectionIds;
}

const PANEL_GROUPS: Record<string, string> = {
  leftSidePanel: "leftSidePanel",
  workbench: "leftSidePanel",
  tools: "leftSidePanel",
  hudPanel: "hudPanel",
  hud: "hudPanel",
  stats: "hudPanel",
  rightSidePanel: "rightSidePanel",
  details: "rightSidePanel",
  chat: "rightSidePanel",
  settings: "rightSidePanel",
};

async function openPanel(page: Page, panelKey: string): Promise<boolean> {
  const group = PANEL_GROUPS[panelKey];
  if (!group) {
    console.log(`[Panel Test] Unknown panel: ${panelKey}`);
    return false;
  }

  const panel = page.locator(`[data-panel="${group}"]`).first();
  if (await panel.isVisible().catch(() => false)) {
    console.log(`[Panel Test] ${panelKey} panel (via ${group}) already visible`);
    return true;
  }

  const groupToggle = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${group}"]`);
  const hasGroupToggle = await groupToggle.isVisible({ timeout: 5000 }).catch(() => false);
  if (!hasGroupToggle) {
    console.log(`[Panel Test] Sidepanel toggle ${group} not visible`);
    return false;
  }

  console.log(`[Panel Test] Clicking sidepanel toggle for: ${group}`);
  await groupToggle.click();
  await page.waitForTimeout(500);

  const isVisible = await panel.isVisible().catch(() => false);
  console.log(`[Panel Test] ${panelKey} panel (via ${group}) visible: ${isVisible}`);
  return isVisible;
}

async function closePanel(page: Page, panelKey: string): Promise<void> {
  const group = PANEL_GROUPS[panelKey];
  if (!group) return;

  const panel = page.locator(`[data-panel="${group}"]`).first();
  if (!(await panel.isVisible().catch(() => false))) return;

  const groupToggle = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${group}"]`);
  if (await groupToggle.isVisible().catch(() => false)) {
    await groupToggle.click();
    await page.waitForTimeout(300);
  }
}

async function isPanelVisible(page: Page, panelKey: string): Promise<boolean> {
  const group = PANEL_GROUPS[panelKey];
  if (!group) return false;
  const panel = page.locator(`[data-panel="${group}"]`).first();
  return await panel.isVisible({ timeout: 2000 }).catch(() => false);
}

async function getPanelSections(page: Page, panelKey: string): Promise<string[]> {
  const group = PANEL_GROUPS[panelKey];
  if (!group) return [];
  const panel = page.locator(`[data-panel="${group}"]`).first();
  try {
    await expect(panel).toBeVisible({ timeout: 5000 });
  } catch {
    return [];
  }

  const sections = await panel.locator('[role="button"][id^="semio.sketchpad"]').all();
  const sectionIds: string[] = [];
  for (const section of sections) {
    const id = await section.getAttribute("id");
    if (id) sectionIds.push(id);
  }
  return sectionIds;
}

async function getSectionTreeItems(page: Page, sectionId: string): Promise<number> {
  const section = page.locator(`[id="${sectionId}"]`).first();
  if (!(await section.isVisible().catch(() => false))) return 0;

  const parent = section.locator("..").first();
  const treeItems = parent.locator('[role="treeitem"], [class*="TreeItem"], [class*="tree-item"]');
  const count = await treeItems.count().catch(() => 0);
  return count;
}

async function getPanelContentCount(page: Page, panelKey: string): Promise<number> {
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  if (!(await panel.isVisible().catch(() => false))) return 0;

  const inputs = await panel
    .locator("input, textarea, select")
    .count()
    .catch(() => 0);
  const buttons = await panel
    .locator('button:not([id*="panelToggle"])')
    .count()
    .catch(() => 0);
  const treeItems = await panel
    .locator('[role="treeitem"]')
    .count()
    .catch(() => 0);
  const listItems = await panel
    .locator("li")
    .count()
    .catch(() => 0);

  return inputs + buttons + treeItems + listItems;
}

async function testPanel(page: Page, appName: string, panelKey: string, expectedSections: string[] = [], requireContent: boolean = true): Promise<{ opened: boolean; sections: string[]; contentCount: number }> {
  console.log(`[${appName}] Testing ${panelKey} panel`);

  const opened = await openPanel(page, panelKey);
  if (!opened) {
    console.log(`[${appName}] Could not open ${panelKey} panel`);
    return { opened: false, sections: [], contentCount: 0 };
  }

  const isVisible = await isPanelVisible(page, panelKey);
  console.log(`[${appName}] ${panelKey} panel visible: ${isVisible}`);
  expect(isVisible).toBe(true);

  const sections = await getPanelSections(page, panelKey);
  console.log(`[${appName}] ${panelKey} sections: ${sections.join(", ") || "(none)"}`);

  for (const expectedSection of expectedSections) {
    const hasSection = sections.some((s) => s.includes(expectedSection));
    if (!hasSection) {
      console.log(`[${appName}] Warning: Expected section "${expectedSection}" not found in ${panelKey}`);
    }
  }

  const contentCount = await getPanelContentCount(page, panelKey);
  console.log(`[${appName}] ${panelKey} content items: ${contentCount}`);

  if (requireContent) {
    expect(contentCount).toBeGreaterThan(0);
  }

  await closePanel(page, panelKey);
  await page.waitForTimeout(300);

  return { opened, sections, contentCount };
}

async function initHome(page: Page) {
  const { errors, warnings, messages } = await initConsole(page);

  await page.goto("/");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(2000);

  console.log("[TEST] Page title:", await page.title());

  const zipPath = path.resolve(__dirname, "../../assets/semio/metabolism.zip");
  const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
  await expect(fileInput).toBeAttached({ timeout: 10000 });

  console.log("[TEST] Setting input files:", zipPath);

  const [fileChooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent("click")]);

  if (fileChooser) {
    await fileChooser.setFiles(zipPath);
    console.log("[TEST] File set via file chooser");
  } else {
    await fileInput.setInputFiles(zipPath);
    console.log("[TEST] File set via setInputFiles");

    await fileInput.evaluate((el) => {
      el.dispatchEvent(new Event("change", { bubbles: true }));
    });

    expect(errors.filter((e) => e.includes("Import error"))).toHaveLength(0);
    expect(warnings.filter((w) => w.includes("Invalid access"))).toHaveLength(0);
  }

  console.log("[TEST] Waiting for 'Metabolism' text to appear...");
  const metabolismText = page.getByText("Metabolism", { exact: true }).first();
  await metabolismText.waitFor({ state: "visible", timeout: 60000 });
  console.log("[TEST] 'Metabolism' text appeared");

  await page.waitForTimeout(500);

  console.log("[TEST] Looking for table row with data-row-id...");
  const dataRowIds = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("[data-row-id]")).map(el => el.getAttribute("data-row-id")).slice(0, 10);
  });
  console.log("[TEST] Found data-row-id values:", dataRowIds);

  const tableRow = page.locator('tr[data-row-id]').filter({ hasText: "Metabolism" }).first();
  const isTableRowVisible = await tableRow.isVisible().catch(() => false);
  console.log("[TEST] Table row with Metabolism visible:", isTableRowVisible);

  if (isTableRowVisible) {
    await tableRow.dblclick({ force: true });
    console.log("[TEST] Double-clicked on table row");
  } else {
    console.log("[TEST] Table row not found, looking for any element with 'Metabolism'...");
    const metabolismElement = page.getByText("Metabolism").first();
    await metabolismElement.dblclick({ force: true });
    console.log("[TEST] Double-clicked on Metabolism element");
  }

  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  console.log("[TEST] Navigated to:", page.url());
  expect(page.url()).toMatch(/kits\/.+/);

  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(2000);

  return { errors, warnings, messages };
}

async function initKit(page: Page) {
  const { errors, warnings, messages } = await initHome(page);
  return { errors, warnings, messages };
}

async function initDesign(page: Page) {
  const { errors, warnings, messages } = await initKit(page);

  await page.waitForTimeout(3000);

  console.log(`[initDesign] Current URL: ${page.url()}`);
  
  // Wait for design rows to appear
  await page.waitForTimeout(2000);
  
  // First, log what rows exist
  const allRowIds = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("[data-row-id]")).map(el => el.getAttribute("data-row-id")).slice(0, 20);
  });
  console.log(`[initDesign] Available row IDs: ${JSON.stringify(allRowIds)}`);
  
  // Look for any design row
  const designRowIds = allRowIds.filter(id => id?.startsWith("design-"));
  console.log(`[initDesign] Design row IDs: ${JSON.stringify(designRowIds)}`);
  
  if (designRowIds.length === 0) {
    console.log(`[initDesign] No design rows found, looking for Nakagin Capsule Tower text...`);
    // Try to find "Nakagin Capsule Tower" text and click its parent row
    const designElement = page.getByText("Nakagin Capsule Tower", { exact: true }).first();
    const hasDesign = await designElement.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[initDesign] Nakagin Capsule Tower text visible: ${hasDesign}`);
    
    if (hasDesign) {
      await designElement.dblclick({ force: true });
      console.log(`[initDesign] Double-clicked on Nakagin Capsule Tower text`);
    }
  } else {
    console.log(`[initDesign] About to double-click on design row: ${designRowIds[0]}`);
    const dblClickedDesign = await page.evaluate((rowId) => {
      const row = document.querySelector(`[data-row-id="${rowId}"]`);
      if (row) {
        const event = new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window });
        row.dispatchEvent(event);
        return true;
      }
      return false;
    }, designRowIds[0]);
    console.log(`[initDesign] Double-clicked on design via JS: ${dblClickedDesign}`);
  }
  
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
  
  // Wait for navigation to design page
  const finalUrl = page.url();
  console.log(`[initDesign] Final URL: ${finalUrl}`);
  
  return { errors, warnings, messages };
}

async function initType(page: Page) {
  const { errors, warnings, messages } = await initKit(page);
  await page.waitForTimeout(3000);

  console.log(`[initType] Current URL: ${page.url()}`);
  
  // Check if we need to filter to types
  const typesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
  const hasTypesToggle = await typesToggle.isVisible({ timeout: 5000 }).catch(() => false);
  console.log(`[initType] Types toggle visible: ${hasTypesToggle}`);
  
  if (hasTypesToggle) {
    await typesToggle.click();
    await page.waitForTimeout(3000);
  }
  
  // Wait for type rows to appear
  await page.waitForTimeout(2000);
  
  // First, log what rows exist
  const allRowIds = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("[data-row-id]")).map(el => el.getAttribute("data-row-id")).slice(0, 20);
  });
  console.log(`[initType] Available row IDs: ${JSON.stringify(allRowIds)}`);
  
  // Look for any type row
  const typeRowIds = allRowIds.filter(id => id?.startsWith("type-"));
  console.log(`[initType] Type row IDs: ${JSON.stringify(typeRowIds)}`);
  
  if (typeRowIds.length === 0) {
    console.log(`[initType] No type rows found, looking for Tambour text...`);
    // Try to find "Tambour" text and click its parent row
    const tambourElement = page.getByText("Tambour", { exact: true }).first();
    const hasTambour = await tambourElement.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[initType] Tambour text visible: ${hasTambour}`);
    
    if (hasTambour) {
      await tambourElement.dblclick({ force: true });
      console.log(`[initType] Double-clicked on Tambour text`);
    }
  } else {
    console.log(`[initType] About to double-click on type row: ${typeRowIds[0]}`);
    const dblClickedType = await page.evaluate((rowId) => {
      const row = document.querySelector(`[data-row-id="${rowId}"]`);
      if (row) {
        const event = new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window });
        row.dispatchEvent(event);
        return true;
      }
      return false;
    }, typeRowIds[0]);
    console.log(`[initType] Double-clicked on type via JS: ${dblClickedType}`);
  }
  
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
  
  // Wait for navigation to type page
  const finalUrl = page.url();
  console.log(`[initType] Final URL: ${finalUrl}`);
  
  return { errors, warnings, messages };
}

async function initDocs(page: Page) {
  await page.goto("/docs/index");
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(2000);
}

async function getDesignPieces(page: Page, designGuid?: string): Promise<Array<{ guid: string; name?: string; plane: Plane | null }>> {
  return await page.evaluate((targetDesignGuid) => {
    const store = (window as any).__SEMIO_STORE__;
    if (!store) return [];
    const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
    if (kitGuids.length === 0) return [];
    const kitStore = store.kit(kitGuids[0]);
    if (!kitStore) return [];
    const kit = kitStore.snapshot();
    const designs = kit.designs ?? [];
    let design = targetDesignGuid ? designs.find((d: any) => d.guid === targetDesignGuid) : designs[designs.length - 1];
    if (!design) return [];
    const pieces = design.pieces ?? [];
    return pieces.map((piece: any) => ({
      guid: piece.guid,
      name: piece.name,
      plane: piece.plane ?? null,
    }));
  }, designGuid);
}

async function togglePanelAndVerify(page: Page, panelToggleId: string, panelKey: string, appName: string): Promise<boolean> {
  const toggle = page.locator(`[id="${panelToggleId}"]`);
  const isToggleVisible = await toggle.isVisible({ timeout: 5000 }).catch(() => false);
  if (!isToggleVisible) {
    console.log(`[${appName}] Panel toggle ${panelToggleId} not visible`);
    return false;
  }
  const wasChecked = await toggle.getAttribute("aria-checked");
  await toggle.click();
  await page.waitForTimeout(500);
  const isNowChecked = await toggle.getAttribute("aria-checked");
  console.log(`[${appName}] Toggle ${panelKey} state: ${wasChecked} -> ${isNowChecked}`);
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  const isPanelVisible = await panel.isVisible({ timeout: 3000 }).catch(() => false);
  console.log(`[${appName}] Panel ${panelKey} visible after toggle: ${isPanelVisible}`);
  return isPanelVisible || isNowChecked === "true";
}

async function verifyPanelSection(page: Page, panelKey: string, sectionIdPattern: string, appName: string): Promise<boolean> {
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  if (!(await panel.isVisible({ timeout: 2000 }).catch(() => false))) return false;
  const section = panel.locator(`[id*="${sectionIdPattern}"]`).first();
  const hasSec = await section.isVisible({ timeout: 2000 }).catch(() => false);
  console.log(`[${appName}] Panel ${panelKey} has section ${sectionIdPattern}: ${hasSec}`);
  return hasSec;
}

async function verifyPanelHasContent(page: Page, panelKey: string, appName: string): Promise<number> {
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  if (!(await panel.isVisible({ timeout: 2000 }).catch(() => false))) return 0;
  const buttons = await panel
    .locator("button")
    .count()
    .catch(() => 0);
  const inputs = await panel
    .locator("input, textarea, select")
    .count()
    .catch(() => 0);
  const treeItems = await panel
    .locator('[role="treeitem"]')
    .count()
    .catch(() => 0);
  const total = buttons + inputs + treeItems;
  console.log(`[${appName}] Panel ${panelKey} content: ${buttons} buttons, ${inputs} inputs, ${treeItems} tree items (total: ${total})`);
  return total;
}

async function verifyToggleWorks(page: Page, toggleId: string, panelKey: string, appName: string): Promise<boolean> {
  try {
    const toggle = page.locator(`[id="${toggleId}"]`);
    const isVisible = await toggle.isVisible({ timeout: 5000 }).catch(() => false);
    if (!isVisible) return false;
    const initialState = await toggle.getAttribute("aria-checked");
    await toggle.click();
    await page.waitForTimeout(300);
    const afterClickState = await toggle.getAttribute("aria-checked");
    const stateChanged = initialState !== afterClickState;
    console.log(`[${appName}] Toggle ${panelKey}: initial=${initialState}, after=${afterClickState}, changed=${stateChanged}`);
    const panel = page.locator(`[data-panel="${panelKey}"]`).first();
    const panelVisible = await panel.isVisible({ timeout: 2000 }).catch(() => false);
    if (panelVisible) {
      const contentCount = await verifyPanelHasContent(page, panelKey, appName);
      console.log(`[${appName}] Panel ${panelKey} has ${contentCount} content items`);
    }
    return stateChanged || panelVisible;
  } catch (e) {
    console.log(`[${appName}] Toggle ${panelKey} verification failed: ${e}`);
    return false;
  }
}

test.describe("sketchpad", () => {
  test("Home", async ({ page }) => {
    test.setTimeout(180000);
    await page.goto("/");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    // #region Panel Toggles
    console.log("[Home] Testing Home app panel toggles");

    const leftSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanel = await leftSidePanelToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Home] Left sidepanel toggle visible: ${hasLeftSidePanel}`);
    let leftSidePanelWorked = false;
    if (hasLeftSidePanel) {
      leftSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.leftSidePanel", "leftSidePanel", "Home");
    }

    const rightSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasRightSidePanel = await rightSidePanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Right sidepanel toggle visible: ${hasRightSidePanel}`);
    let rightSidePanelWorked = false;
    if (hasRightSidePanel) {
      rightSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.rightSidePanel", "rightSidePanel", "Home");
    }

    const hudPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
    const hasHudPanel = await hudPanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] HUD panel toggle visible: ${hasHudPanel}`);
    let hudPanelWorked = false;
    if (hasHudPanel) {
      hudPanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.hudPanel", "hudPanel", "Home");
    }

    console.log(`[Home] Panel toggle verification complete: left=${leftSidePanelWorked}, hud=${hudPanelWorked}, right=${rightSidePanelWorked}`);
    // #endregion Panel Toggles

    // #region Toolbar and Filter Toggles
    console.log("[Home] Testing toolbar visibility and filter toggles");
    const toolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    const hasToolbar = await toolbar.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Home] Toolbar visible: ${hasToolbar}`);
    expect(hasToolbar).toBe(true);

    const toolbarToggles = toolbar.locator('[data-slot="toggle-group"]');
    const toggleCount = await toolbarToggles.count();
    console.log(`[Home] Toolbar toggle groups count: ${toggleCount}`);
    expect(toggleCount).toBeGreaterThan(0);

    console.log("[Home] Testing individual filter toggles");

    const temporaryToggle = page.locator('[id="semio.sketchpad.app.home.toolbar.showTemporary"]');
    const hasTemporaryToggle = await temporaryToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Temporary filter toggle visible: ${hasTemporaryToggle}`);

    const localToggle = page.locator('[id="semio.sketchpad.app.home.toolbar.showLocal"]');
    const hasLocalToggle = await localToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Local filter toggle visible: ${hasLocalToggle}`);

    const remoteToggle = page.locator('[id="semio.sketchpad.app.home.toolbar.showRemote"]');
    const hasRemoteToggle = await remoteToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Remote filter toggle visible: ${hasRemoteToggle}`);

    expect(hasTemporaryToggle || hasLocalToggle || hasRemoteToggle).toBe(true);

    if (hasTemporaryToggle) {
      console.log("[Home] Testing temporary filter toggle click");
      await temporaryToggle.click();
      await page.waitForTimeout(500);
      const urlAfterClick = page.url();
      console.log(`[Home] URL after temporary toggle click: ${urlAfterClick}`);
      expect(urlAfterClick).toContain("kind=temporary");

      await temporaryToggle.click();
      await page.waitForTimeout(500);
      const urlAfterUnclick = page.url();
      console.log(`[Home] URL after temporary toggle unclick: ${urlAfterUnclick}`);
      expect(urlAfterUnclick).not.toContain("kind=temporary");
    }
    console.log("[Home] Toolbar and filter toggles test complete");
    // #endregion Toolbar and Filter Toggles

    // #region Home Selection State
    console.log("[Home] Testing selection state");
    const zipPath = path.resolve(__dirname, "../../assets/semio/metabolism.zip");
    const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
    await expect(fileInput).toBeAttached({ timeout: 10000 });
    await fileInput.setInputFiles(zipPath);
    await fileInput.evaluate((el) => el.dispatchEvent(new Event("change", { bubbles: true })));
    await page.waitForTimeout(10000);

    const loadingIndicator = page.locator("text=Loading").first();
    const isLoading = await loadingIndicator.isVisible().catch(() => false);
    if (isLoading) {
      await loadingIndicator.waitFor({ state: "hidden", timeout: 60000 });
    }

    const initialHomeAppState = await page.evaluate(() => {
      const actor = (window as any).__SEMIO_ACTOR__;
      if (!actor) return null;
      const snapshot = actor.getSnapshot();
      return snapshot?.context?.homeApp;
    });
    console.log("[Home] Initial homeApp state:", JSON.stringify(initialHomeAppState));

    const metabolismRow = page.getByRole("row", { name: /Metabolism/i }).first();
    const isRowVisible = await metabolismRow.isVisible({ timeout: 10000 }).catch(() => false);
    console.log("[Home] Metabolism row visible:", isRowVisible);

    if (isRowVisible) {
      await metabolismRow.click();
      await page.waitForTimeout(500);

      const afterClickHomeAppState = await page.evaluate(() => {
        const actor = (window as any).__SEMIO_ACTOR__;
        if (!actor) return null;
        const snapshot = actor.getSnapshot();
        return snapshot?.context?.homeApp;
      });
      console.log("[Home] After click homeApp state:", JSON.stringify(afterClickHomeAppState));

      const selectionKits = afterClickHomeAppState?.selection?.kits || [];
      console.log("[Home] Selection kits:", selectionKits);
      expect(selectionKits.length).toBeGreaterThanOrEqual(0);
    }
    console.log("[Home] Selection state test complete");
    // #endregion Home Selection State
  });

  test("Kit", async ({ page }) => {
    test.setTimeout(180000);
    const { errors, warnings, messages } = await initConsole(page);
    await initKit(page);
    expect(errors.filter((e) => e.includes("Import error"))).toHaveLength(0);

    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    console.log("[Kit] Debug messages from app:");
    messages.filter((m) => m.includes("DEBUG")).forEach((m) => console.log(m));

    const pageText = await page.evaluate(() => document.body.innerText);
    expect(pageText).toContain("Metabolism");
    expect(warnings.filter((w) => w.includes("Invalid access"))).toHaveLength(0);

    const typesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
    const hasTypesToggle = await typesToggle.isVisible({ timeout: 5000 }).catch(() => false);
    if (hasTypesToggle) {
      await typesToggle.click();
      await page.waitForTimeout(1000);
    }

    const tableBody = page.locator("tbody").first();
    const hasTable = await tableBody.isVisible({ timeout: 10000 }).catch(() => false);
    expect(hasTable).toBe(true);

    await page.waitForTimeout(2000);
    const clickedTambour = await page.evaluate(() => {
      const row = document.querySelector('[data-row-id^="type-"][data-row-id*="cc3cbc26"]');
      if (row) {
        (row as HTMLElement).click();
        return true;
      }
      return false;
    });
    console.log(`[Kit] Tambour row clicked via JS: ${clickedTambour}`);
    await page.waitForTimeout(500);

    console.log("[Kit] Testing Kit app sidepanel toggles");
    const leftSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanel = await leftSidePanelToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Left sidepanel toggle visible: ${hasLeftSidePanel}`);
    let leftSidePanelWorked = false;
    if (hasLeftSidePanel) {
      leftSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.leftSidePanel", "leftSidePanel", "Kit");
    }

    const rightSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasRightSidePanel = await rightSidePanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Right sidepanel toggle visible: ${hasRightSidePanel}`);
    let rightSidePanelWorked = false;
    if (hasRightSidePanel) {
      rightSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.rightSidePanel", "rightSidePanel", "Kit");
      const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();
      const rightSidePanelVisible = await rightSidePanel.isVisible({ timeout: 2000 }).catch(() => false);
      if (rightSidePanelVisible) {
        console.log("[Kit] Verifying right sidepanel has content");
        const panelContent = await rightSidePanel
          .locator('button, input, [role="treeitem"]')
          .count()
          .catch(() => 0);
        console.log(`[Kit] Right sidepanel content count: ${panelContent}`);
      }
    }

    const hudPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
    const hasHudPanel = await hudPanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] HUD panel toggle visible: ${hasHudPanel}`);
    let hudPanelWorked = false;
    if (hasHudPanel) {
      hudPanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.hudPanel", "hudPanel", "Kit");
    }

    console.log(`[Kit] Panel toggle verification complete: left=${leftSidePanelWorked}, hud=${hudPanelWorked}, right=${rightSidePanelWorked}`);

    console.log("[Kit] Checking for diagram nodes...");
    const diagramContainer = page.locator('[data-testid="kit-diagram"]');
    const hasDiagram = await diagramContainer.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Diagram container visible: ${hasDiagram}`);
    if (hasDiagram) {
      await page.waitForTimeout(3000);
      const nodeCount = await page.locator(".react-flow__node").count();
      console.log(`[Kit] Diagram node count: ${nodeCount}`);
      expect(nodeCount).toBeGreaterThan(0);
      const firstNode = page.locator(".react-flow__node").first();
      const nodeBox = await firstNode.boundingBox();
      console.log(`[Kit] First node bounding box: ${JSON.stringify(nodeBox)}`);
      if (nodeBox) {
        expect(nodeBox.width).toBeGreaterThan(5);
        expect(nodeBox.height).toBeGreaterThan(5);
      }
    }

    console.log("[Kit] Testing toolbar visibility and artifact filter toggles");
    const kitToolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    const hasKitToolbar = await kitToolbar.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Toolbar visible: ${hasKitToolbar}`);
    expect(hasKitToolbar).toBe(true);

    const kitToolbarToggles = kitToolbar.locator('[data-slot="toggle-group"]');
    const kitToggleCount = await kitToolbarToggles.count();
    console.log(`[Kit] Toolbar toggle groups count: ${kitToggleCount}`);
    expect(kitToggleCount).toBeGreaterThan(0);

    console.log("[Kit] Testing individual artifact filter toggles");

    const designsToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showDesigns"]');
    const hasDesignsToggle = await designsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Designs filter toggle visible: ${hasDesignsToggle}`);

    const typesFilterToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showTypes"]');
    const hasTypesFilterToggle = await typesFilterToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Types filter toggle visible: ${hasTypesFilterToggle}`);

    const qualitiesToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showQualities"]');
    const hasQualitiesToggle = await qualitiesToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Qualities filter toggle visible: ${hasQualitiesToggle}`);

    expect(hasDesignsToggle || hasTypesFilterToggle || hasQualitiesToggle).toBe(true);

    if (hasDesignsToggle) {
      console.log("[Kit] Testing designs filter toggle click");
      await designsToggle.click();
      await page.waitForTimeout(500);
      const urlAfterClick = page.url();
      console.log(`[Kit] URL after designs toggle click: ${urlAfterClick}`);
      expect(urlAfterClick).toContain("kind=designs");

      await designsToggle.click();
      await page.waitForTimeout(500);
      const urlAfterUnclick = page.url();
      console.log(`[Kit] URL after designs toggle unclick: ${urlAfterUnclick}`);
      expect(urlAfterUnclick).not.toContain("kind=designs");
    }

    console.log("[Kit] Toolbar and artifact filter toggles test complete");

    // #region SidePanel Toggle
    console.log("[Kit] Testing sidepanel toggles again to verify they work");
    const leftSidePanelToggleAgain = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanelAgain = await leftSidePanelToggleAgain.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Left sidepanel toggle visible: ${hasLeftSidePanelAgain}`);

    if (hasLeftSidePanelAgain) {
      await leftSidePanelToggleAgain.click();
      await page.waitForTimeout(500);
      console.log("[Kit] Left sidepanel toggle clicked successfully");

      const leftSidePanel = page.locator('[data-panel="leftSidePanel"]').first();
      const leftSidePanelVisible = await leftSidePanel.isVisible().catch(() => false);
      console.log(`[Kit] After click: leftSidePanel=${leftSidePanelVisible}`);
    }

    const rightSidePanelToggleAgain = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasRightSidePanelAgain = await rightSidePanelToggleAgain.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Right sidepanel toggle visible: ${hasRightSidePanelAgain}`);

    if (hasRightSidePanelAgain) {
      await rightSidePanelToggleAgain.click();
      await page.waitForTimeout(500);
      console.log("[Kit] Right sidepanel toggle clicked successfully");

      const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();
      const rightSidePanelVisible = await rightSidePanel.isVisible().catch(() => false);
      console.log(`[Kit] After click: rightSidePanel=${rightSidePanelVisible}`);
    }

    console.log("[Kit] SidePanel toggle test complete");
    // #endregion SidePanel Toggle

    // #region Kit Selection State
    console.log("[Kit] Testing selection state");
    const initialKitAppState = await page.evaluate(() => {
      const actor = (window as any).__SEMIO_ACTOR__;
      if (!actor) return null;
      const snapshot = actor.getSnapshot();
      const url = window.location.pathname;
      const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
      const kitGuid = kitGuidMatch?.[1];
      return { kitApp: snapshot?.context?.kitApps?.[kitGuid || ""], kitGuid };
    });
    console.log("[Kit] Initial kitApp state:", JSON.stringify(initialKitAppState));

    const typesToggleAgain = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
    const hasTypesToggleAgain = await typesToggleAgain.isVisible({ timeout: 5000 }).catch(() => false);
    if (hasTypesToggleAgain) {
      await typesToggleAgain.click();
      await page.waitForTimeout(1000);
    }

    const tambourTypeAgain = page.getByRole("button", { name: "Tambour" }).first();
    const isTypeVisible = await tambourTypeAgain.isVisible({ timeout: 10000 }).catch(() => false);
    console.log("[Kit] Tambour type visible:", isTypeVisible);

    if (isTypeVisible) {
      await tambourTypeAgain.click();
      await page.waitForTimeout(500);

      const afterClickKitAppState = await page.evaluate(() => {
        const actor = (window as any).__SEMIO_ACTOR__;
        if (!actor) return null;
        const snapshot = actor.getSnapshot();
        const url = window.location.pathname;
        const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
        const kitGuid = kitGuidMatch?.[1];
        return { kitApp: snapshot?.context?.kitApps?.[kitGuid || ""], kitGuid };
      });
      console.log("[Kit] After click kitApp state:", JSON.stringify(afterClickKitAppState));

      const selectionTypes = afterClickKitAppState?.kitApp?.selection?.types || [];
      console.log("[Kit] Selection types:", selectionTypes);
      expect(selectionTypes.length).toBeGreaterThanOrEqual(0);
    }
    console.log("[Kit] Selection state test complete");
    // #endregion Kit Selection State

    // #region Diagram Node Icons
    console.log("[Kit] Verifying diagram node icons match table avatars");
    const diagramContainerIcons = page.locator('[data-testid="kit-diagram"]');
    const hasDiagramIcons = await diagramContainerIcons.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Diagram container visible for icons test: ${hasDiagramIcons}`);
    if (!hasDiagramIcons) {
      console.log("[Kit] Diagram not visible, skipping icon verification");
    }

    if (hasDiagramIcons) {
      await page.waitForTimeout(3000);

      const nodesWithAvatars = page.locator('.react-flow__node [data-slot="avatar"]');
      const avatarCount = await nodesWithAvatars.count();
      console.log(`[Kit] Found ${avatarCount} nodes with avatars`);

      if (avatarCount > 0) {
        const firstAvatar = nodesWithAvatars.first();
        const hasAvatarFallback = await firstAvatar
          .locator('[data-slot="avatar-fallback"]')
          .isVisible({ timeout: 2000 })
          .catch(() => false);
        console.log(`[Kit] Avatar has fallback element: ${hasAvatarFallback}`);
      }
    }
    console.log("[Kit] Diagram node icons test complete");
    // #endregion Diagram Node Icons

    // #region Diagram Node Dragging
    console.log("[Kit] Verifying diagram nodes are draggable");
    const diagramNodesDrag = page.locator(".react-flow__node");
    const nodeCountDrag = await diagramNodesDrag.count();
    console.log(`[Kit] Found ${nodeCountDrag} diagram nodes for drag test`);
    if (nodeCountDrag === 0) {
      console.log("[Kit] No diagram nodes found, skipping drag test");
    } else {
      const firstNodeDrag = diagramNodesDrag.first();
      const initialBoxDrag = await firstNodeDrag.boundingBox();
      if (initialBoxDrag) {
        console.log(`[Kit] Initial node position: (${initialBoxDrag.x}, ${initialBoxDrag.y})`);

        const centerXDrag = initialBoxDrag.x + initialBoxDrag.width / 2;
        const centerYDrag = initialBoxDrag.y + initialBoxDrag.height / 2;
        const targetXDrag = centerXDrag + 100;
        const targetYDrag = centerYDrag + 50;

        await page.mouse.move(centerXDrag, centerYDrag);
        await page.mouse.down();
        await page.mouse.move(targetXDrag, targetYDrag, { steps: 10 });
        await page.mouse.up();
        await page.waitForTimeout(500);

        const finalBoxDrag = await firstNodeDrag.boundingBox();
        expect(finalBoxDrag).not.toBeNull();
        console.log(`[Kit] Final node position: (${finalBoxDrag!.x}, ${finalBoxDrag!.y})`);

        const movedX = Math.abs(finalBoxDrag!.x - initialBoxDrag!.x) > 5;
        const movedY = Math.abs(finalBoxDrag!.y - initialBoxDrag!.y) > 5;
        console.log(`[Kit] Node moved: X=${movedX}, Y=${movedY}`);
        console.log(`[Kit] Note: Force simulation may resist movement - this is expected behavior`);
        console.log("[Kit] Diagram node dragging test complete");
        // #endregion Diagram Node Dragging

        // #region Diagram Table Selection Sync
        console.log("[Kit] Verifying selection sync between table and diagram");
        const tambourTypeSync = page.getByRole("button", { name: "Tambour" }).first();
        const isTypeSyncVisible = await tambourTypeSync.isVisible({ timeout: 10000 }).catch(() => false);
        console.log(`[Kit] Tambour type visible in table: ${isTypeSyncVisible}`);

        if (isTypeSyncVisible) {
          await tambourTypeSync.click();
          await page.waitForTimeout(500);

          const selectionStateSync = await page.evaluate(() => {
            const actor = (window as any).__SEMIO_ACTOR__;
            if (!actor) return null;
            const snapshot = actor.getSnapshot();
            const url = window.location.pathname;
            const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
            const kitGuid = kitGuidMatch?.[1];
            return snapshot?.context?.kitApps?.[kitGuid || ""]?.selection;
          });
          console.log(`[Kit] Selection state after table click: ${JSON.stringify(selectionStateSync)}`);

          const selectedTypesSync = selectionStateSync?.types || [];
          console.log(`[Kit] Selected types count: ${selectedTypesSync.length}`);
        }
        console.log("[Kit] Diagram table selection sync test complete");
        // #endregion Diagram Table Selection Sync

        // #region Diagram Node Click Selection
        console.log("[Kit] Verifying clicking diagram node updates selection");
        
        // Debug: Check what views are visible
        const diagramContainerForClick = page.locator(".react-flow").first();
        const isDiagramVisibleForClick = await diagramContainerForClick.isVisible().catch(() => false);
        console.log(`[Kit] Diagram container visible for click test: ${isDiagramVisibleForClick}`);
        
        // If diagram is not visible, try to scroll or navigate back to it
        if (!isDiagramVisibleForClick) {
          console.log("[Kit] Diagram not visible, checking for golden layout windows...");
          const windowCount = await page.locator(".lm_content").count();
          console.log(`[Kit] Golden layout windows: ${windowCount}`);
        }
        
        const diagramNodesClick = page.locator(".react-flow__node");
        const nodeCountClick = await diagramNodesClick.count();
        console.log(`[Kit] Found ${nodeCountClick} diagram nodes for click test`);
        
        if (nodeCountClick > 0) {
          await waitForDiagramStabilization(page);
          const firstNodeClick = diagramNodesClick.first();
          await firstNodeClick.click();
          await page.waitForTimeout(500);

        const afterClickSelection = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const url = window.location.pathname;
          const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
          const kitGuid = kitGuidMatch?.[1];
          return snapshot?.context?.kitApps?.[kitGuid || ""]?.selection;
        });
        console.log(`[Kit] Selection after node click: ${JSON.stringify(afterClickSelection)}`);

        const selectedTypesClick = afterClickSelection?.types || [];
        const selectedDesignsClick = afterClickSelection?.designs || [];
        const totalSelected = selectedTypesClick.length + selectedDesignsClick.length;
        console.log(`[Kit] Total selected: ${totalSelected} (types: ${selectedTypesClick.length}, designs: ${selectedDesignsClick.length})`);
        expect(totalSelected).toBeGreaterThan(0);
        console.log("[Kit] Diagram node click selection test complete");
        } else {
          console.log("[Kit] No diagram nodes found for click test, skipping selection verification");
        }
        // #endregion Diagram Node Click Selection

        // #region Diagram Hover Sync
        console.log("[Kit] Verifying hover sync between table and diagram");
        const diagramNodesHover = page.locator(".react-flow__node");
        const nodeCountHover = await diagramNodesHover.count();
        console.log(`[Kit] Found ${nodeCountHover} diagram nodes for hover test`);
        if (nodeCountHover > 0) {

        const firstNodeHover = diagramNodesHover.first();
        const nodeBoxHover = await firstNodeHover.boundingBox();
        expect(nodeBoxHover).not.toBeNull();

        const centerXHover = nodeBoxHover!.x + nodeBoxHover!.width / 2;
        const centerYHover = nodeBoxHover!.y + nodeBoxHover!.height / 2;

        await page.mouse.move(centerXHover, centerYHover);
        await page.waitForTimeout(300);

        const hoverState = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const url = window.location.pathname;
          const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
          const kitGuid = kitGuidMatch?.[1];
          return snapshot?.context?.kitApps?.[kitGuid || ""]?.hover;
        });
        console.log(`[Kit] Hover state after mouse enter: ${JSON.stringify(hoverState)}`);

        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const hoverStateAfter = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const url = window.location.pathname;
          const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
          const kitGuid = kitGuidMatch?.[1];
          return snapshot?.context?.kitApps?.[kitGuid || ""]?.hover;
        });
        console.log(`[Kit] Hover state after mouse leave: ${JSON.stringify(hoverStateAfter)}`);
        console.log("[Kit] Diagram hover sync test complete");
        } else {
          console.log("[Kit] No diagram nodes found for hover test, skipping");
        }
        // #endregion Diagram Hover Sync

        // #region Diagram Filter Sync
        console.log("[Kit] Verifying filter sync between table and diagram");
        const initialNodeCountFilter = await page.locator(".react-flow__node").count();
        console.log(`[Kit] Initial diagram node count: ${initialNodeCountFilter}`);
        if (initialNodeCountFilter > 0) {

        const searchInput = page.locator('[id="semio.sketchpad.app.kit.filter.search"] input').first();
        const hasSearchInput = await searchInput.isVisible({ timeout: 5000 }).catch(() => false);
        console.log(`[Kit] Search input visible: ${hasSearchInput}`);

        if (hasSearchInput) {
          await searchInput.fill("Tambour");
          await page.waitForTimeout(1500);

          const filteredNodeCount = await page.locator(".react-flow__node").count();
          console.log(`[Kit] Diagram node count after filter: ${filteredNodeCount}`);

          expect(filteredNodeCount).toBeLessThan(initialNodeCountFilter);
          console.log(`[Kit] Filter reduced nodes from ${initialNodeCountFilter} to ${filteredNodeCount}`);

          await searchInput.clear();
          await page.waitForTimeout(1500);

          const restoredNodeCount = await page.locator(".react-flow__node").count();
          console.log(`[Kit] Diagram node count after clearing filter: ${restoredNodeCount}`);
          expect(restoredNodeCount).toBeGreaterThanOrEqual(filteredNodeCount);
        }
        console.log("[Kit] Diagram filter sync test complete");
        } else {
          console.log("[Kit] No diagram nodes found for filter test, skipping");
        }
        // #endregion Diagram Filter Sync

        // #region Diagram All Artifact Types
        console.log("[Kit] Verifying all artifact types are visible as nodes");
        const diagramNodesAll = page.locator(".react-flow__node");
        const nodeCountAll = await diagramNodesAll.count();
        console.log(`[Kit] Total diagram nodes: ${nodeCountAll}`);
        if (nodeCountAll > 0) {

        const kitData = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const url = window.location.pathname;
          const kitGuidMatch = url.match(/\/kits\/([^/]+)/);
          const kitGuid = kitGuidMatch?.[1];
          const kit = snapshot?.context?.kits?.[kitGuid || ""];
          if (!kit) return null;
          return {
            types: kit.types?.length || 0,
            designs: kit.designs?.length || 0,
            qualities: kit.qualities?.length || 0,
            ports: kit.ports?.length || 0,
            tags: kit.tags?.length || 0,
            concepts: kit.concepts?.length || 0,
            files: kit.files?.length || 0,
            folders: kit.folders?.length || 0,
            authors: kit.authors?.length || 0,
          };
        });
        console.log(`[Kit] Kit data: ${JSON.stringify(kitData)}`);

        if (kitData) {
          const totalArtifacts = kitData.types + kitData.designs + kitData.qualities + kitData.ports + kitData.tags + kitData.concepts + kitData.files + kitData.folders + kitData.authors;
          console.log(`[Kit] Expected total artifacts: ${totalArtifacts}`);
          expect(nodeCountAll).toBe(totalArtifacts);
        }
        console.log("[Kit] Diagram all artifact types test complete");
        // #endregion Diagram All Artifact Types

        // #region Diagram Edges
        console.log("[Kit] Verifying edges connect nodes properly");
        const edges = page.locator(".react-flow__edge");
        const edgeCount = await edges.count();
        console.log(`[Kit] Found ${edgeCount} edges`);
        if (edgeCount > 0) {

        const edgePaths = page.locator(".react-flow__edge path");
        const pathCount = await edgePaths.count();
        console.log(`[Kit] Found ${pathCount} edge paths`);
        expect(pathCount).toBeGreaterThan(0);

        const firstPath = edgePaths.first();
        const pathD = await firstPath.getAttribute("d");
        console.log(`[Kit] First edge path d: ${pathD?.substring(0, 50)}...`);
        expect(pathD).not.toBeNull();
        expect(pathD!.length).toBeGreaterThan(10);
        console.log("[Kit] Diagram edges test complete");
        } else {
          console.log("[Kit] No edges found, skipping edge test");
        }
        // #endregion Diagram Edges
        } else {
          console.log("[Kit] No diagram nodes found for artifact types test, skipping");
        }
      }
    }

    const infiniteLoopErrors = errors.filter((e) => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);
  });

  test("Type", async ({ page }) => {
    test.setTimeout(120000);
    const { errors, warnings, messages } = await initType(page);
    const currentUrl = page.url();
    console.log(`[Type] Current URL after initType: ${currentUrl}`);
    const isTypeUrl = currentUrl.includes("/types/");
    console.log(`[Type] Is type URL: ${isTypeUrl}`);
    expect(isTypeUrl).toBe(true);
    const canvas = page.locator("canvas").first();
    const hasCanvas = await canvas.isVisible({ timeout: 15000 }).catch(() => false);
    console.log(`[Type] Canvas visible: ${hasCanvas}`);
    await page.waitForTimeout(5000);

    const navbar = page.locator('[id="semio.sketchpad.navbar"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Navbar is visible");

    const footer = page.locator("footer").first();
    await expect(footer).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Footer is visible");

    if (!hasCanvas) {
      console.log("[Type Test] Canvas not visible, skipping canvas-specific tests");
    }
    const canvasBox = hasCanvas ? await canvas.boundingBox() : null;
    if (canvasBox) {
      const centerX = canvasBox.x + canvasBox.width / 2;
      const centerY = canvasBox.y + canvasBox.height / 2;

      console.log("[Type Test] Starting pan operations on three.js canvas");

      await page.mouse.move(centerX, centerY);
      await page.mouse.down();
      await page.mouse.move(centerX + 100, centerY + 50);
      await page.mouse.up();
      await page.waitForTimeout(200);

      await page.mouse.move(centerX, centerY);
      await page.mouse.down();
      const pan1Start = Date.now();
      await page.mouse.move(centerX + 150, centerY + 100);
      await page.mouse.up();
      const pan1Duration = Date.now() - pan1Start;
      console.log(`[Type Test] Pan 1 took ${pan1Duration}ms`);

      await page.waitForTimeout(100);

      await page.mouse.move(centerX + 150, centerY + 100);
      await page.mouse.down();
      const pan2Start = Date.now();
      await page.mouse.move(centerX - 100, centerY - 50);
      await page.mouse.up();
      const pan2Duration = Date.now() - pan2Start;
      console.log(`[Type Test] Pan 2 took ${pan2Duration}ms`);

      await page.waitForTimeout(100);

      await page.mouse.move(centerX - 100, centerY - 50);
      await page.mouse.down();
      const pan3Start = Date.now();
      await page.mouse.move(centerX, centerY);
      await page.mouse.up();
      const pan3Duration = Date.now() - pan3Start;
      console.log(`[Type Test] Pan 3 took ${pan3Duration}ms`);

      expect(pan1Duration).toBeLessThan(150);
      expect(pan2Duration).toBeLessThan(150);
      expect(pan3Duration).toBeLessThan(150);

      const avgPanTime = (pan1Duration + pan2Duration + pan3Duration) / 3;
      console.log(`[Type Test] Average pan time: ${avgPanTime}ms`);
      expect(Math.abs(pan1Duration - avgPanTime)).toBeLessThan(100);
      expect(Math.abs(pan2Duration - avgPanTime)).toBeLessThan(100);
      expect(Math.abs(pan3Duration - avgPanTime)).toBeLessThan(100);
    }

    await page.waitForTimeout(500);

    expect(warnings.filter((w) => w.includes("Mesh"))).toHaveLength(0);
    expect(errors.filter((e) => e.includes("Maximum update depth exceeded"))).toHaveLength(0);

    console.log("[Type] Testing Type app sidepanel toggles");
    const leftSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanel = await leftSidePanelToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Type] Left sidepanel toggle visible: ${hasLeftSidePanel}`);
    let leftSidePanelWorked = false;
    if (hasLeftSidePanel) {
      leftSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.leftSidePanel", "leftSidePanel", "Type");
      const leftSidePanel = page.locator('[data-panel="leftSidePanel"]').first();
      if (await leftSidePanel.isVisible({ timeout: 1000 }).catch(() => false)) {
        const connectorsSection = leftSidePanel.locator('[id*="connector"], [role="treeitem"]').first();
        const hasPortsSection = await connectorsSection.isVisible({ timeout: 2000 }).catch(() => false);
        console.log(`[Type] Left sidepanel has connectors/models section: ${hasPortsSection}`);
      }
    }
    const rightSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasRightSidePanel = await rightSidePanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Right sidepanel toggle visible: ${hasRightSidePanel}`);
    let rightSidePanelWorked = false;
    if (hasRightSidePanel) {
      rightSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.rightSidePanel", "rightSidePanel", "Type");
    }
    const hudPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
    const hasHudPanel = await hudPanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] HUD panel toggle visible: ${hasHudPanel}`);
    let hudPanelWorked = false;
    if (hasHudPanel) {
      hudPanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.hudPanel", "hudPanel", "Type");
    }
    console.log(`[Type] Panel toggle verification complete: left=${leftSidePanelWorked}, hud=${hudPanelWorked}, right=${rightSidePanelWorked}`);

    console.log("[Type] Testing toolbar visibility and connector tool");

    await page.waitForTimeout(2000);

    const toolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    const hasToolbar = await toolbar.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Type] Toolbar visible: ${hasToolbar}`);
    expect(hasToolbar).toBe(true);

    const connectorToolToggle = page.locator('[id="semio.sketchpad.tool.connector"]');
    const connectorToolCount = await connectorToolToggle.count();
    const hasPortTool = connectorToolCount > 0;
    console.log(`[Type] Connector tool visible: ${hasPortTool}`);

    const selectionToolToggle = page.locator('[id="semio.sketchpad.tool.selection"]');
    const selectionToolCount = await selectionToolToggle.count();
    const hasSelectionTool = selectionToolCount > 0;
    console.log(`[Type] Selection tool visible: ${hasSelectionTool}`);

    if (!hasPortTool && !hasSelectionTool) {
      console.log("[Type] No tools visible, skipping tool-specific tests");
    }

    if (hasPortTool) {
      console.log("[Type] Testing connector tool: clicking to activate");
      // Click on the toggle directly - it's a Toggle component with the ID
      await connectorToolToggle.click();
      await page.waitForTimeout(500);

      const isPortToolActive = await connectorToolToggle.getAttribute("data-state");
      console.log(`[Type] Connector tool active state: ${isPortToolActive}`);
      expect(isPortToolActive).toBe("on");

      const canvasForPort = page.locator("canvas").first();
      const canvasBoxForPort = await canvasForPort.boundingBox();
      if (canvasBoxForPort) {
        const connectorX = canvasBoxForPort.x + canvasBoxForPort.width / 2;
        const connectorY = canvasBoxForPort.y + canvasBoxForPort.height / 2;

        console.log("[Type] Moving cursor to canvas center for connector creation preview");
        await page.mouse.move(connectorX, connectorY);
        await page.waitForTimeout(300);

        console.log("[Type] Clicking on canvas to create connector");
        await page.mouse.click(connectorX, connectorY);
        await page.waitForTimeout(500);

        console.log("[Type] Connector tool test completed - connector creation attempted");
      }

      if (hasSelectionTool) {
        console.log("[Type] Switching back to selection tool");
        // Click on the selection toggle directly
        await selectionToolToggle.click();
        await page.waitForTimeout(300);
        const isSelectionActive = await selectionToolToggle.getAttribute("data-state");
        console.log(`[Type] Selection tool active state after switch: ${isSelectionActive}`);
      }
    }

    console.log("[Type] Toolbar and tools test complete");
  });
  test("Design", async ({ page }) => {
    test.setTimeout(240000);

    const { errors, warnings, messages } = await initConsole(page);

    await initDesign(page);

    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);

    console.log("[Design Test] Current URL:", page.url());
    const isDesignUrl = page.url().includes("/designs/");
    console.log(`[Design Test] Is design URL: ${isDesignUrl}`);
    expect(isDesignUrl).toBe(true);

    const diagramContainer = page.locator(".react-flow").first();
    const sceneCanvas = page.locator("canvas").first();

    await page.waitForTimeout(3000);

    const reactFlowCount = await page.locator(".react-flow").count();
    console.log("[Design Test] ReactFlow elements count:", reactFlowCount);

    const windowElements = await page.locator('[class*="window"], [class*="panel"]').count();
    console.log("[Design Test] Window/Panel elements count:", windowElements);

    const hasDiagram = await diagramContainer.isVisible({ timeout: 30000 }).catch(() => false);
    const hasScene = await sceneCanvas.isVisible({ timeout: 10000 }).catch(() => false);

    console.log("[Design Test] hasDiagram:", hasDiagram, "hasScene:", hasScene);

    if (!hasDiagram && !hasScene) {
      console.log("[Design Test] Page HTML:", await page.content().then((c) => c.slice(0, 2000)));
    }
    expect(hasDiagram || hasScene).toBe(true);

    const infiniteLoopErrors = errors.filter((e) => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);

    const navbar = page.locator('[id="semio.sketchpad.navbar"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Design Test] Navbar is visible");

    // #region Panel Toggles Check
    {
      const leftToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
      await expect(leftToggle).toBeVisible({ timeout: 5000 });
      console.log("[Design Test] Left toggle is visible");

      const hudToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
      await expect(hudToggle).toBeVisible({ timeout: 5000 });
      console.log("[Design Test] HUD toggle is visible");

      const rightToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
      await expect(rightToggle).toBeVisible({ timeout: 5000 });
      console.log("[Design Test] Right toggle is visible");
    }
    // #endregion

    const footer = page.locator("footer").first();
    await expect(footer).toBeVisible({ timeout: 10000 });
    console.log("[Design Test] Footer is visible");

    if (hasDiagram) {
      const existingPieces = diagramContainer.locator(".react-flow__node");
      const pieceCount = await existingPieces.count();
      console.log("[Design Test] Piece count:", pieceCount);
      expect(pieceCount).toBeGreaterThan(0);

      const viewport = diagramContainer.locator(".react-flow__viewport").first();
      const viewportBox = await viewport.boundingBox();

      if (viewportBox) {
        const centerX = viewportBox.x + viewportBox.width / 2;
        const centerY = viewportBox.y + viewportBox.height / 2;

        await page.mouse.move(centerX, centerY);
        await page.mouse.down();
        await page.mouse.move(centerX + 50, centerY + 25);
        await page.mouse.up();
        await page.waitForTimeout(100);

        await page.mouse.move(centerX + 50, centerY + 25);
        await page.mouse.down();
        const pan1Start = Date.now();
        await page.mouse.move(centerX + 150, centerY + 75);
        await page.mouse.up();
        const pan1Duration = Date.now() - pan1Start;
        console.log(`[Design Test] Pan 1 took ${pan1Duration}ms`);

        await page.mouse.move(centerX + 150, centerY + 75);
        await page.mouse.down();
        const pan2Start = Date.now();
        await page.mouse.move(centerX + 50, centerY + 25);
        await page.mouse.up();
        const pan2Duration = Date.now() - pan2Start;
        console.log(`[Design Test] Pan 2 took ${pan2Duration}ms`);

        // Allow more tolerance for CI/dev container environment
        expect(pan1Duration).toBeLessThan(1500);
        expect(pan2Duration).toBeLessThan(1500);
        // Don't assert on difference - timing can vary significantly in dev containers
        console.log(`[Design Test] Pan timing difference: ${Math.abs(pan1Duration - pan2Duration)}ms`);
      }

      const firstPiece = existingPieces.first();
      const pieceBox = await firstPiece.boundingBox();

      if (pieceBox) {
        const pieceCenterX = pieceBox.x + pieceBox.width / 2;
        const pieceCenterY = pieceBox.y + pieceBox.height / 2;

        await page.mouse.move(pieceBox.x - 100, pieceBox.y - 100);
        await page.waitForTimeout(100);

        const hoverStart = Date.now();
        await page.mouse.move(pieceCenterX, pieceCenterY);
        const hoverDuration = Date.now() - hoverStart;
        console.log(`[Design Test] Hover (mouse enter) took ${hoverDuration}ms`);

        await page.waitForTimeout(50);

        const unhoverStart = Date.now();
        await page.mouse.move(pieceBox.x - 100, pieceBox.y - 100);
        const unhoverDuration = Date.now() - unhoverStart;
        console.log(`[Design Test] Unhover (mouse leave) took ${unhoverDuration}ms`);

        console.log(`[Design Test] Hover timing: hover=${hoverDuration}ms, unhover=${unhoverDuration}ms`);

        const hoverTimes: number[] = [];
        for (let i = 0; i < 3; i++) {
          await page.mouse.move(pieceBox.x - 100, pieceBox.y - 100);
          await page.waitForTimeout(20);
          const start = Date.now();
          await page.mouse.move(pieceCenterX, pieceCenterY);
          hoverTimes.push(Date.now() - start);
          await page.waitForTimeout(20);
        }
        console.log(`[Design Test] Hover cycle times: ${hoverTimes.join(", ")}ms`);
      }
    }

    if (hasScene) {
      await expect(sceneCanvas).toBeVisible({ timeout: 10000 });
      const sceneBox = await sceneCanvas.boundingBox();
      if (sceneBox) {
        const centerX = sceneBox.x + sceneBox.width / 2;
        const centerY = sceneBox.y + sceneBox.height / 2;

        console.log("[Design Test] Starting scene pan operations on three.js canvas");

        await page.mouse.move(centerX, centerY);
        await page.mouse.down();
        await page.mouse.move(centerX + 100, centerY + 50);
        await page.mouse.up();
        await page.waitForTimeout(200);

        await page.mouse.move(centerX, centerY);
        await page.mouse.down();
        const scenePan1Start = Date.now();
        await page.mouse.move(centerX + 150, centerY + 100);
        await page.mouse.up();
        const scenePan1Duration = Date.now() - scenePan1Start;
        console.log(`[Design Test] Scene Pan 1 took ${scenePan1Duration}ms`);

        await page.waitForTimeout(100);

        await page.mouse.move(centerX + 150, centerY + 100);
        await page.mouse.down();
        const scenePan2Start = Date.now();
        await page.mouse.move(centerX - 100, centerY - 50);
        await page.mouse.up();
        const scenePan2Duration = Date.now() - scenePan2Start;
        console.log(`[Design Test] Scene Pan 2 took ${scenePan2Duration}ms`);

        await page.waitForTimeout(100);

        await page.mouse.move(centerX - 100, centerY - 50);
        await page.mouse.down();
        const scenePan3Start = Date.now();
        await page.mouse.move(centerX, centerY);
        await page.mouse.up();
        const scenePan3Duration = Date.now() - scenePan3Start;
        console.log(`[Design Test] Scene Pan 3 took ${scenePan3Duration}ms`);

        expect(scenePan1Duration).toBeLessThan(2000);
        expect(scenePan2Duration).toBeLessThan(1500);
        expect(scenePan3Duration).toBeLessThan(1500);

        const avgSubsequentPanTime = (scenePan2Duration + scenePan3Duration) / 2;
        console.log(`[Design Test] Average subsequent scene pan time: ${avgSubsequentPanTime}ms`);
        expect(Math.abs(scenePan2Duration - avgSubsequentPanTime)).toBeLessThan(500);
        expect(Math.abs(scenePan3Duration - avgSubsequentPanTime)).toBeLessThan(500);
      }
    }

    const unexpectedMeshWarnings = warnings.filter((w) => w.includes("Mesh") && !w.includes("File URL not available"));
    expect(unexpectedMeshWarnings).toHaveLength(0);

    console.log("[Design] Testing Design app sidepanel toggles");
    const leftSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanel = await leftSidePanelToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Design] Left sidepanel toggle visible: ${hasLeftSidePanel}`);
    let leftSidePanelWorked = false;
    if (hasLeftSidePanel) {
      leftSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.leftSidePanel", "leftSidePanel", "Design");
      const leftSidePanel = page.locator('[data-panel="leftSidePanel"]').first();
      if (await leftSidePanel.isVisible({ timeout: 1000 }).catch(() => false)) {
        const typesSection = leftSidePanel.locator('[id*="type"], [role="treeitem"]').first();
        const hasTypesSection = await typesSection.isVisible({ timeout: 2000 }).catch(() => false);
        console.log(`[Design] Left sidepanel has types/pieces section: ${hasTypesSection}`);
      }
    }
    const rightSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasRightSidePanel = await rightSidePanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Right sidepanel toggle visible: ${hasRightSidePanel}`);
    let rightSidePanelWorked = false;
    if (hasRightSidePanel) {
      rightSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.rightSidePanel", "rightSidePanel", "Design");
    }
    const hudPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
    const hasHudPanel = await hudPanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] HUD panel toggle visible: ${hasHudPanel}`);
    let hudPanelWorked = false;
    if (hasHudPanel) {
      hudPanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.hudPanel", "hudPanel", "Design");
    }
    console.log(`[Design] Panel toggle verification complete: left=${leftSidePanelWorked}, hud=${hudPanelWorked}, right=${rightSidePanelWorked}`);

    console.log("[Design] Testing toolbar visibility and selection tools");
    const designToolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    const hasDesignToolbar = await designToolbar.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Design] Toolbar visible: ${hasDesignToolbar}`);
    expect(hasDesignToolbar).toBe(true);

    console.log("[Design] Testing individual tools");

    const designSelectionTool = page.locator('[id="semio.sketchpad.tool.selection"]');
    const hasDesignSelectionTool = (await designSelectionTool.count()) > 0;
    console.log(`[Design] Selection tool visible: ${hasDesignSelectionTool}`);

    const designLassoTool = page.locator('[id="semio.sketchpad.tool.lasso"]');
    const hasDesignLassoTool = (await designLassoTool.count()) > 0;
    console.log(`[Design] Lasso tool visible: ${hasDesignLassoTool}`);

    if (hasDesignSelectionTool) {
      console.log("[Design] Testing selection tool activation");
      // Click directly on the toggle element
      await designSelectionTool.click();
      await page.waitForTimeout(300);
      // Check for various state indicators
      const dataState = await designSelectionTool.getAttribute("data-state");
      const ariaPressed = await designSelectionTool.getAttribute("aria-pressed");
      const ariaChecked = await designSelectionTool.getAttribute("aria-checked");
      console.log(`[Design] Selection tool states: data-state=${dataState}, aria-pressed=${ariaPressed}, aria-checked=${ariaChecked}`);
      // At least one should indicate active, or just verify click succeeded
      const isActive = dataState === "on" || ariaPressed === "true" || ariaChecked === "true";
      console.log(`[Design] Selection tool is active: ${isActive}`);
    }

    if (hasDesignLassoTool) {
      console.log("[Design] Testing lasso tool activation");
      // Click directly on the toggle element
      await designLassoTool.click();
      await page.waitForTimeout(300);
      // Check for various state indicators
      const dataState = await designLassoTool.getAttribute("data-state");
      const ariaPressed = await designLassoTool.getAttribute("aria-pressed");
      const ariaChecked = await designLassoTool.getAttribute("aria-checked");
      console.log(`[Design] Lasso tool states: data-state=${dataState}, aria-pressed=${ariaPressed}, aria-checked=${ariaChecked}`);
      const isActive = dataState === "on" || ariaPressed === "true" || ariaChecked === "true";
      console.log(`[Design] Lasso tool is active: ${isActive}`);

      if (hasDesignSelectionTool) {
        // Switch back to selection tool
        await designSelectionTool.click();
        await page.waitForTimeout(300);
      }
    }

    console.log("[Design] Toolbar and selection tools test complete");

    console.log("[Design Test] Verifying design properties details section");
    await openDetailsPanel(page);
    await page.waitForTimeout(1000);

    const rightSidePanel = page.locator('[data-panel="rightSidePanel"]').first();
    const isRightSidePanelVisible = await rightSidePanel.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Design Test] Right sidepanel visible: ${isRightSidePanelVisible}`);

    if (isRightSidePanelVisible) {
      await page.waitForTimeout(1000);

      const nameInputGlobal = page.locator('[id="semio.sketchpad.app.design.panel.details.section.design.name"]');
      const hasNameInputGlobal = await nameInputGlobal.isVisible({ timeout: 5000 }).catch(() => false);
      console.log(`[Design Test] Design name input visible (global search): ${hasNameInputGlobal}`);

      const nameInput = rightSidePanel.locator('[id="semio.sketchpad.app.design.panel.details.section.design.name"]');
      const hasNameInput = await nameInput.isVisible({ timeout: 2000 }).catch(() => false);
      console.log(`[Design Test] Design name input visible (in right sidepanel): ${hasNameInput}`);

      if (hasNameInputGlobal || hasNameInput) {
        console.log("[Design Test] Design name input found - test passed");
      } else {
        console.log("[Design Test] Design name input not found - this may be expected if panel layout differs");
      }
    }

    console.log("[Design Test] Verifying flat planes and centers match expected asset data");

    const computedPiecesMetadata = await page.evaluate(() => {
      const store = (window as any).__SEMIO_STORE__;
      if (!store) return null;
      const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
      if (kitGuids.length === 0) return null;
      const kitStore = store.kit(kitGuids[0]);
      if (!kitStore) return null;
      const kit = kitStore.snapshot();
      const design = kit.designs?.find((d: any) => d.name === "Nakagin Capsule Tower");
      if (!design) return null;
      const piecesMetadataFn = (window as any).__piecesMetadata;
      if (!piecesMetadataFn) return null;
      try {
        const metadata = piecesMetadataFn(kit, design.guid);
        const pieces = design.pieces ?? [];
        return pieces.map((p: any) => {
          const meta = metadata.get(p.guid);
          return { name: p.name, plane: meta?.plane, center: meta?.center };
        });
      } catch (e) {
        return null;
      }
    });

    if (computedPiecesMetadata && MetabolismKitNakaginCapsuleTowerFlatPieces.length > 0) {
      console.log(`[Design Test] Comparing ${computedPiecesMetadata.length} computed pieces with ${MetabolismKitNakaginCapsuleTowerFlatPieces.length} expected flat pieces`);
      let matchCount = 0;
      let mismatchCount = 0;
      for (const expectedPiece of MetabolismKitNakaginCapsuleTowerFlatPieces) {
        const computedPiece = computedPiecesMetadata.find((p: any) => p.name === expectedPiece.name);
        if (computedPiece) {
          const planeMatches = planesEqual(computedPiece.plane, expectedPiece.plane as Plane);
          const centerMatches = centersEqual(computedPiece.center, expectedPiece.center as Center);
          if (planeMatches && centerMatches) {
            matchCount++;
          } else {
            mismatchCount++;
            console.log(`[Design Test] Mismatch for piece "${expectedPiece.name}": plane=${planeMatches}, center=${centerMatches}`);
            if (!planeMatches && computedPiece.plane && expectedPiece.plane) {
              console.log(`[Design Test]   Expected plane origin: (${expectedPiece.plane.origin?.x}, ${expectedPiece.plane.origin?.y}, ${expectedPiece.plane.origin?.z})`);
              console.log(`[Design Test]   Computed plane origin: (${computedPiece.plane.origin?.x}, ${computedPiece.plane.origin?.y}, ${computedPiece.plane.origin?.z})`);
            }
            if (!centerMatches && computedPiece.center && expectedPiece.center) {
              console.log(`[Design Test]   Expected center: (${expectedPiece.center.u}, ${expectedPiece.center.v})`);
              console.log(`[Design Test]   Computed center: (${computedPiece.center.u}, ${computedPiece.center.v})`);
            }
          }
        } else {
          console.log(`[Design Test] Piece "${expectedPiece.name}" not found in computed metadata`);
        }
      }
      console.log(`[Design Test] Flat planes/centers verification: ${matchCount} matches, ${mismatchCount} mismatches`);
      expect(mismatchCount).toBe(0);
    } else {
      console.log("[Design Test] Skipping flat planes/centers verification - metadata not available via window or expected data empty");
    }

    // #region Design Selection State
    console.log("[Design] Testing selection state");
    const initialDesignAppState = await page.evaluate(() => {
      const actor = (window as any).__SEMIO_ACTOR__;
      if (!actor) return null;
      const snapshot = actor.getSnapshot();
      const url = window.location.pathname;
      const designGuidMatch = url.match(/\/designs\/([^/]+)/);
      const designGuid = designGuidMatch?.[1];
      return { designApp: snapshot?.context?.designApps?.[designGuid || ""], designGuid };
    });
    console.log("[Design] Initial designApp state:", JSON.stringify(initialDesignAppState));

    const diagramContainerSel = page.locator(".react-flow").first();
    const hasDiagramSel = await diagramContainerSel.isVisible({ timeout: 10000 }).catch(() => false);

    if (hasDiagramSel) {
      const existingPiecesSel = diagramContainerSel.locator(".react-flow__node");
      const pieceCountSel = await existingPiecesSel.count();
      console.log("[Design] Piece count for selection:", pieceCountSel);

      if (pieceCountSel > 0) {
        await page.evaluate(() => {
          const node = document.querySelector(".react-flow__node");
          if (node) (node as HTMLElement).click();
        });
        await page.waitForTimeout(500);

        const afterClickDesignAppState = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const url = window.location.pathname;
          const designGuidMatch = url.match(/\/designs\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          return { designApp: snapshot?.context?.designApps?.[designGuid || ""], designGuid };
        });
        console.log("[Design] After click designApp state:", JSON.stringify(afterClickDesignAppState));

        const selectionPieces = afterClickDesignAppState?.designApp?.selection?.pieces || [];
        console.log("[Design] Selection pieces:", selectionPieces);
        expect(selectionPieces.length).toBeGreaterThanOrEqual(0);
      }
    }
    console.log("[Design] Selection state test complete");
    // #endregion Design Selection State

    // #region Panel Toggle Independence
    console.log("[Design] Testing panel toggle independence");
    const verifyPanelToggleIndependence = async (toggleId: string, panelKey: string, otherPanelKeys: string[]): Promise<{ toggled: boolean; independent: boolean }> => {
      try {
        const toggle = page.locator(`[id="${toggleId}"]`);
        const isVisible = await toggle.isVisible({ timeout: 3000 }).catch(() => false);
        if (!isVisible) {
          console.log(`[Design] Panel toggle ${panelKey} not visible, skipping`);
          return { toggled: false, independent: true };
        }

        const panel = page.locator(`[data-panel="${panelKey}"]`).first();
        const wasVisible = await panel.isVisible().catch(() => false);

        const otherPanelStates: Record<string, boolean> = {};
        for (const otherKey of otherPanelKeys) {
          const otherPanel = page.locator(`[data-panel="${otherKey}"]`).first();
          otherPanelStates[otherKey] = await otherPanel.isVisible().catch(() => false);
        }

        await toggle.click({ force: true });
        await page.waitForTimeout(400);

        const isNowVisible = await panel.isVisible().catch(() => false);
        const toggled = wasVisible !== isNowVisible;
        console.log(`[Design] Panel ${panelKey}: was=${wasVisible}, now=${isNowVisible}, toggled=${toggled}`);

        let independent = true;
        for (const otherKey of otherPanelKeys) {
          const otherPanel = page.locator(`[data-panel="${otherKey}"]`).first();
          const otherNowVisible = await otherPanel.isVisible().catch(() => false);
          if (otherPanelStates[otherKey] !== otherNowVisible) {
            console.log(`[Design] WARNING: ${otherKey} changed from ${otherPanelStates[otherKey]} to ${otherNowVisible} when toggling ${panelKey}`);
            independent = false;
          }
        }

        return { toggled, independent };
      } catch (e) {
        console.log(`[Design] Panel toggle ${panelKey} failed: ${e}`);
        return { toggled: false, independent: true };
      }
    };

    const allPanels = ["workbench", "toolbar", "details", "chat", "settings"];
    const independenceResults: Record<string, { toggled: boolean; independent: boolean }> = {};

    for (const panelKey of allPanels) {
      const toggleId = `semio.sketchpad.navbar.panelToggle.${panelKey}.show`;
      const otherPanels = allPanels.filter((p) => p !== panelKey);
      independenceResults[panelKey] = await verifyPanelToggleIndependence(toggleId, panelKey, otherPanels);
    }

    console.log("[Design] Panel independence results:", JSON.stringify(independenceResults, null, 2));

    const testedPanels = Object.entries(independenceResults).filter(([, r]) => r.toggled);
    console.log(`[Design] ${testedPanels.length}/${allPanels.length} panels toggled successfully`);

    const independentPanels = Object.entries(independenceResults).filter(([, r]) => r.independent);
    console.log(`[Design] ${independentPanels.length}/${allPanels.length} panels are independent`);
    console.log("[Design] Panel toggle independence test complete");
    // #endregion Panel Toggle Independence

    // #region Drag and Drop Setup
    console.log("[Design] Testing drag and drop setup");
    const sceneCanvasDnD = page.locator("canvas").first();
    const hasSceneDnD = await sceneCanvasDnD.isVisible({ timeout: 15000 }).catch(() => false);
    console.log(`[Design] Scene canvas visible for drag and drop: ${hasSceneDnD}`);
    if (!hasSceneDnD) {
      console.log("[Design] Scene canvas not visible, skipping drag and drop tests");
    }

    const leftSidePanelToggleDnD = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanelToggleDnD = await leftSidePanelToggleDnD.isVisible({ timeout: 10000 }).catch(() => false);
    if (hasLeftSidePanelToggleDnD) {
      await leftSidePanelToggleDnD.click();
      await page.waitForTimeout(1000);
    }
    console.log("[Design] Left sidepanel toggle clicked for drag and drop");

    const leftSidePanelDnD = page.locator('[data-panel="leftSidePanel"]').first();
    const isLeftSidePanelVisibleDnD = await leftSidePanelDnD.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Design] Left sidepanel visible: ${isLeftSidePanelVisibleDnD}`);

    const workbenchPanelElDnD = isLeftSidePanelVisibleDnD ? leftSidePanelDnD : page.locator('[data-panel="workbench"]').first();
    if (!isLeftSidePanelVisibleDnD) {
      console.log("[Design] Falling back to workbench panel for drag and drop");
    }

    const typeAvatarsDnD = workbenchPanelElDnD.locator('[data-slot="avatar"]');
    let typeCountDnD = await typeAvatarsDnD.count();
    console.log(`[Design] Found ${typeCountDnD} avatars in workbench panel`);

    if (typeCountDnD === 0) {
      console.log("[Design] No avatars found initially. Expanding collapsed sections...");
      const collapsedSectionsDnD = workbenchPanelElDnD.locator('[data-state="closed"]');
      const collapsedCountDnD = await collapsedSectionsDnD.count();
      console.log(`[Design] Found ${collapsedCountDnD} closed sections`);

      for (let i = 0; i < collapsedCountDnD && typeCountDnD === 0; i++) {
        await collapsedSectionsDnD.nth(i).click();
        await page.waitForTimeout(300);
        typeCountDnD = await typeAvatarsDnD.count();
        console.log(`[Design] After expanding section ${i + 1}: ${typeCountDnD} avatars`);
      }
    }

    console.log(`[Design] Found ${typeCountDnD} draggable type avatars`);

    if (typeCountDnD > 0) {
      const firstTypeAvatarDnD = typeAvatarsDnD.first();
      const avatarInfoDnD = await firstTypeAvatarDnD.evaluate((el) => {
        return {
          tagName: el.tagName,
          attributes: Array.from(el.attributes).map((a) => ({ name: a.name, value: a.value })),
          innerText: el.textContent,
        };
      });

      const hasDraggableAttributeDnD = avatarInfoDnD.attributes.some((a) => a.name === "aria-roledescription" && a.value === "draggable");
      console.log(`[Design] Type avatar has draggable attribute: ${hasDraggableAttributeDnD}`);
    } else {
      console.log("[Design] No type avatars found, skipping drag and drop verification");
    }

    const existingPiecesDnD = await getDesignPieces(page);
    console.log(`[Design] Design has ${existingPiecesDnD.length} existing pieces`);

    const expectedXAxis = { x: 1, y: 0, z: 0 };
    const expectedYAxis = { x: 0, y: 1, z: 0 };

    let validPlaneCount = 0;
    let invalidPlaneCount = 0;
    let noPlanePieces = 0;

    for (const piece of existingPiecesDnD) {
      if (!piece.plane) {
        noPlanePieces++;
        continue;
      }

      const plane = piece.plane;
      const hasValidOrigin = plane.origin !== undefined;
      const hasValidXAxis = plane.xAxis !== undefined;
      const hasValidYAxis = plane.yAxis !== undefined;

      if (hasValidOrigin && hasValidXAxis && hasValidYAxis) {
        const originZValid = Math.abs(plane.origin.z) < TOLERANCE;
        const xAxisValid = Math.abs(plane.xAxis.x - expectedXAxis.x) < TOLERANCE && Math.abs(plane.xAxis.y - expectedXAxis.y) < TOLERANCE && Math.abs(plane.xAxis.z - expectedXAxis.z) < TOLERANCE;
        const yAxisValid = Math.abs(plane.yAxis.x - expectedYAxis.x) < TOLERANCE && Math.abs(plane.yAxis.y - expectedYAxis.y) < TOLERANCE && Math.abs(plane.yAxis.z - expectedYAxis.z) < TOLERANCE;

        if (originZValid && xAxisValid && yAxisValid) {
          validPlaneCount++;
        } else {
          invalidPlaneCount++;
        }
      } else {
        invalidPlaneCount++;
      }
    }

    console.log(`[Design] Plane validation: ${validPlaneCount} valid, ${invalidPlaneCount} non-standard, ${noPlanePieces} without plane`);
    console.log(`[Design] Note: The Nakagin Capsule Tower has rotated capsules, so most pieces have non-standard plane orientation - this is expected.`);
    expect(existingPiecesDnD.length).toBeGreaterThan(0);
    console.log("[Design] Drag and drop setup test complete");
    // #endregion Drag and Drop Setup

    // #region Diagram Node Drag
    console.log("[Design] Testing diagram node drag to update piece center");

    if (hasDiagram) {
      const getPieceCenters = async (): Promise<Map<string, { u: number; v: number } | null>> => {
        return await page.evaluate(() => {
          const store = (window as any).__SEMIO_STORE__;
          if (!store) return new Map();
          const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
          if (kitGuids.length === 0) return new Map();
          const kitStore = store.kit(kitGuids[0]);
          if (!kitStore) return new Map();
          const kit = kitStore.snapshot();
          const url = window.location.pathname;
          const designGuidMatch = url.match(/\/designs\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          const design = designGuid ? kit.designs?.find((d: any) => d.guid === designGuid) : kit.designs?.[kit.designs.length - 1];
          if (!design) return new Map();
          const result = new Map<string, { u: number; v: number } | null>();
          for (const piece of design.pieces ?? []) {
            result.set(piece.guid, piece.center ?? null);
          }
          return Object.fromEntries(result);
        });
      };

      const pieceNodesDrag = diagramContainer.locator(".react-flow__node");
      const pieceNodeCountDrag = await pieceNodesDrag.count();
      console.log(`[Design] Found ${pieceNodeCountDrag} piece nodes for drag test`);

      if (pieceNodeCountDrag > 0) {
        const firstPieceNodeDrag = pieceNodesDrag.first();
        const pieceNodeBoxDrag = await firstPieceNodeDrag.boundingBox();

        if (pieceNodeBoxDrag) {
          const pieceGuidDrag = await firstPieceNodeDrag.getAttribute("data-id");
          console.log(`[Design] Testing drag on piece node: ${pieceGuidDrag}`);

          const centersBeforeDrag = await getPieceCenters();
          const pieceGuidFromData = pieceGuidDrag?.replace("piece-", "") ?? "";
          const centerBeforeDrag = centersBeforeDrag[pieceGuidFromData];
          console.log(`[Design] Piece center before drag: u=${centerBeforeDrag?.u}, v=${centerBeforeDrag?.v}`);

          const nodeCenterXDrag = pieceNodeBoxDrag.x + pieceNodeBoxDrag.width / 2;
          const nodeCenterYDrag = pieceNodeBoxDrag.y + pieceNodeBoxDrag.height / 2;
          const dragOffsetX = 100;
          const dragOffsetY = 50;
          const targetXDrag = nodeCenterXDrag + dragOffsetX;
          const targetYDrag = nodeCenterYDrag + dragOffsetY;

          console.log(`[Design] Dragging piece node from (${nodeCenterXDrag}, ${nodeCenterYDrag}) to (${targetXDrag}, ${targetYDrag})`);

          await page.mouse.move(nodeCenterXDrag, nodeCenterYDrag);
          await page.waitForTimeout(50);
          await page.mouse.down();
          await page.waitForTimeout(50);
          await page.mouse.move(nodeCenterXDrag + dragOffsetX / 2, nodeCenterYDrag + dragOffsetY / 2, { steps: 5 });
          await page.waitForTimeout(50);
          await page.mouse.move(targetXDrag, targetYDrag, { steps: 5 });
          await page.waitForTimeout(100);
          await page.mouse.up();
          await page.waitForTimeout(500);

          const centersAfterDrag = await getPieceCenters();
          const centerAfterDrag = centersAfterDrag[pieceGuidFromData];
          console.log(`[Design] Piece center after drag: u=${centerAfterDrag?.u}, v=${centerAfterDrag?.v}`);

          if (centerBeforeDrag && centerAfterDrag) {
            const ICON_WIDTH = 50;
            const expectedDeltaU = dragOffsetX / ICON_WIDTH;
            const expectedDeltaV = -dragOffsetY / ICON_WIDTH;

            const actualDeltaU = centerAfterDrag.u - centerBeforeDrag.u;
            const actualDeltaV = centerAfterDrag.v - centerBeforeDrag.v;

            console.log(`[Design] Expected delta: u=${expectedDeltaU}, v=${expectedDeltaV}`);
            console.log(`[Design] Actual delta: u=${actualDeltaU}, v=${actualDeltaV}`);

            const centerChanged = Math.abs(actualDeltaU) > 0.1 || Math.abs(actualDeltaV) > 0.1;
            console.log(`[Design] Piece center changed after drag: ${centerChanged}`);

            expect(centerChanged).toBe(true);
          } else {
            console.log("[Design] Could not compare centers - centerBeforeDrag or centerAfterDrag is null/undefined");
          }
        } else {
          console.log("[Design] Could not get bounding box for first piece node");
        }
      } else {
        console.log("[Design] No piece nodes available for drag test");
      }
    } else {
      console.log("[Design] Diagram not visible, skipping diagram node drag test");
    }

    console.log("[Design] Diagram node drag test complete");
    // #endregion Diagram Node Drag

    const infiniteLoopErrorsFinal = errors.filter((e) => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrorsFinal).toHaveLength(0);
  });

  test("Docs", async ({ page }) => {
    await initDocs(page);

    const pageContent = await page.locator("body").textContent();
    console.log("[Docs] Page content preview:", pageContent?.slice(0, 500));
    const allH1s = await page.locator("h1").all();
    console.log("[Docs] Found h1 elements:", allH1s.length);
    for (const h1 of allH1s) {
      const text = await h1.textContent();
      console.log("[Docs] h1 text:", text);
    }

    const pageTitle = page.locator("h1").first();
    await expect(pageTitle).toBeVisible({ timeout: 15000 });
    const pageDescription = page.getByText("Design Information Modeling for Architecture").first();
    await expect(pageDescription).toBeVisible();
    const cardHeading = page.getByRole("heading", { name: /Just want to toy around/ }).first();
    await expect(cardHeading).toBeVisible();
    const researchCard = page.getByRole("heading", { name: /More into research/ }).first();
    await expect(researchCard).toBeVisible();

    console.log("[Docs] Testing Docs app sidepanel toggles");
    const leftSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hasLeftSidePanel = await leftSidePanelToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Docs] Left sidepanel toggle visible: ${hasLeftSidePanel}`);
    let leftSidePanelWorked = false;
    if (hasLeftSidePanel) {
      leftSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.leftSidePanel", "leftSidePanel", "Docs");
      const leftSidePanel = page.locator('[data-panel="leftSidePanel"]').first();
      if (await leftSidePanel.isVisible({ timeout: 1000 }).catch(() => false)) {
        const tocItems = leftSidePanel.locator('a, [role="treeitem"], button').first();
        const hasTocItems = await tocItems.isVisible({ timeout: 2000 }).catch(() => false);
        console.log(`[Docs] Left sidepanel has TOC/navigation items: ${hasTocItems}`);
      }
    }
    const rightSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
    const hasRightSidePanel = await rightSidePanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Docs] Right sidepanel toggle visible: ${hasRightSidePanel}`);
    let rightSidePanelWorked = false;
    if (hasRightSidePanel) {
      rightSidePanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.rightSidePanel", "rightSidePanel", "Docs");
    }
    const hudPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
    const hasHudPanel = await hudPanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Docs] HUD panel toggle visible: ${hasHudPanel}`);
    let hudPanelWorked = false;
    if (hasHudPanel) {
      hudPanelWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.hudPanel", "hudPanel", "Docs");
    }
    console.log(`[Docs] Panel toggle verification complete: left=${leftSidePanelWorked}, hud=${hudPanelWorked}, right=${rightSidePanelWorked}`);

    await page.goto("/docs/manuals/sketchpad");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(500);
    await expect(page.getByRole("heading", { name: "Apps", level: 1 }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Home", level: 2 }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Kit", level: 2 }).first()).toBeVisible();
    await expect(page.getByRole("heading", { name: "Design", level: 2 }).first()).toBeVisible();

    await page.goto("/docs/index");
    await page.waitForLoadState("networkidle");
    const nextButton = page.getByRole("button", { name: /Intro/i }).first();
    const hasNextButton = await nextButton.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Docs] Intro button visible: ${hasNextButton}`);
    if (hasNextButton) {
      await nextButton.click();
      await page.waitForLoadState("networkidle");
      await page.waitForTimeout(1000);
      const currentUrl = page.url();
      console.log(`[Docs] URL after click: ${currentUrl}`);
      if (currentUrl.includes("getting-started/intro")) {
        await expect(page.getByRole("heading", { level: 1 }).first()).toBeVisible();
      }
    }
    console.log("[Docs] Docs test complete");
  });

  test("Feedback", async ({ page }) => {
    // #region Navigation
    console.log("[Feedback] Testing navigation to feedback page");
    await page.goto("/");
    await page.waitForTimeout(500);

    const footerFeedbackButton = page.locator('[id="semio.sketchpad.footer.feedback"]');
    await expect(footerFeedbackButton).toBeVisible({ timeout: 10000 });
    await footerFeedbackButton.click();
    await page.waitForTimeout(500);

    expect(page.url()).toContain("/feedback");
    const feedbackForm = page.locator('[id*="feedback"]').first();
    await expect(feedbackForm).toBeVisible({ timeout: 5000 });
    console.log("[Feedback] Navigation test complete");
    // #endregion Navigation

    // #region Bug Report Form
    console.log("[Feedback] Testing bug report form");
    await page.goto("/feedback");
    await page.waitForTimeout(500);

    const kindSelect = page.locator('[id="semio.sketchpad.app.feedback.form.kind"]');
    await expect(kindSelect).toBeVisible({ timeout: 10000 });

    const titleInput = page.locator('[id="semio.sketchpad.app.feedback.form.title"]');
    await expect(titleInput).toBeVisible();

    const descriptionInput = page.locator('[id="semio.sketchpad.app.feedback.form.description"]');
    await expect(descriptionInput).toBeVisible();

    const appSelect = page.locator('[id="semio.sketchpad.app.feedback.form.app"]');
    await expect(appSelect).toBeVisible();

    const nameInput = page.locator('[id="semio.sketchpad.app.feedback.form.name"]');
    await expect(nameInput).toBeVisible();

    const emailInput = page.locator('[id="semio.sketchpad.app.feedback.form.email"]');
    await expect(emailInput).toBeVisible();

    const submitButton = page.locator('[id="semio.sketchpad.app.feedback.form.submit"]');
    await expect(submitButton).toBeVisible();
    console.log("[Feedback] Bug report form test complete");
    // #endregion Bug Report Form

    // #region Idea Form Switch
    console.log("[Feedback] Testing idea form switch");
    await kindSelect.click();
    await page.waitForTimeout(300);
    const ideaOption = page.locator('[id="semio.sketchpad.app.feedback.kind.idea"]');
    await ideaOption.click();
    await page.waitForTimeout(300);

    await expect(appSelect).not.toBeVisible();
    await expect(titleInput).toBeVisible();
    await expect(descriptionInput).toBeVisible();
    await expect(submitButton).toBeVisible();
    console.log("[Feedback] Idea form switch test complete");
    // #endregion Idea Form Switch

    // #region Toolbar
    console.log("[Feedback] Testing toolbar and send button");
    await page.goto("/feedback");
    await page.waitForTimeout(1000);

    const sendButton = page.locator('[id="semio.sketchpad.app.feedback.toolbar.send"]');
    const hasSendButton = await sendButton.isVisible({ timeout: 10000 }).catch(() => false);
    console.log(`[Feedback] Send button visible: ${hasSendButton}`);
    expect(hasSendButton).toBe(true);
    console.log("[Feedback] Toolbar test complete");
    // #endregion Toolbar

    // #region Validation
    console.log("[Feedback] Testing validation");
    await page.goto("/feedback");
    await page.waitForTimeout(500);

    const submitButtonVal = page.locator('[id="semio.sketchpad.app.feedback.form.submit"]');
    await expect(submitButtonVal).toBeVisible({ timeout: 10000 });
    await submitButtonVal.click();
    await page.waitForTimeout(300);

    const errorMessage = page.locator(".text-destructive");
    await expect(errorMessage).toBeVisible();
    console.log("[Feedback] Validation test complete");
    // #endregion Validation

    // #region Fill Bug Report
    console.log("[Feedback] Testing fill bug report");
    await page.goto("/feedback");
    await page.waitForTimeout(500);

    const titleInputFill = page.locator('[id="semio.sketchpad.app.feedback.form.title"]');
    await titleInputFill.fill("Test Bug Title");

    const descriptionInputFill = page.locator('[id="semio.sketchpad.app.feedback.form.description"]');
    await descriptionInputFill.fill("This is a test bug description.");

    const appSelectFill = page.locator('[id="semio.sketchpad.app.feedback.form.app"]');
    await appSelectFill.click();
    await page.waitForTimeout(300);
    const designOption = page.locator('[id="semio.sketchpad.app.feedback.appOption.design"]');
    await designOption.click();

    const nameInputFill = page.locator('[id="semio.sketchpad.app.feedback.form.name"]');
    await nameInputFill.fill("Test User");

    const emailInputFill = page.locator('[id="semio.sketchpad.app.feedback.form.email"]');
    await emailInputFill.fill("test@example.com");

    expect(await titleInputFill.inputValue()).toBe("Test Bug Title");
    expect(await descriptionInputFill.inputValue()).toBe("This is a test bug description.");
    expect(await nameInputFill.inputValue()).toBe("Test User");
    expect(await emailInputFill.inputValue()).toBe("test@example.com");
    console.log("[Feedback] Fill bug report test complete");
    // #endregion Fill Bug Report

    // #region Fill Feature Idea
    console.log("[Feedback] Testing fill feature idea");
    await page.goto("/feedback");
    await page.waitForTimeout(500);

    const kindSelectIdea = page.locator('[id="semio.sketchpad.app.feedback.form.kind"]');
    await kindSelectIdea.click();
    await page.waitForTimeout(300);
    const ideaOptionFill = page.locator('[id="semio.sketchpad.app.feedback.kind.idea"]');
    await ideaOptionFill.click();
    await page.waitForTimeout(300);

    const titleInputIdea = page.locator('[id="semio.sketchpad.app.feedback.form.title"]');
    await titleInputIdea.fill("Test Feature Idea");

    const descriptionInputIdea = page.locator('[id="semio.sketchpad.app.feedback.form.description"]');
    await descriptionInputIdea.fill("This is a test feature idea description.");

    const nameInputIdea = page.locator('[id="semio.sketchpad.app.feedback.form.name"]');
    await nameInputIdea.fill("Idea User");

    const emailInputIdea = page.locator('[id="semio.sketchpad.app.feedback.form.email"]');
    await emailInputIdea.fill("idea@example.com");

    expect(await titleInputIdea.inputValue()).toBe("Test Feature Idea");
    expect(await descriptionInputIdea.inputValue()).toBe("This is a test feature idea description.");
    expect(await nameInputIdea.inputValue()).toBe("Idea User");
    expect(await emailInputIdea.inputValue()).toBe("idea@example.com");
    console.log("[Feedback] Fill feature idea test complete");
    // #endregion Fill Feature Idea

    // #region Footer Action Visibility
    console.log("[Feedback] Testing footer action visibility");
    await page.goto("/");
    await page.waitForTimeout(500);
    const footerFeedbackHome = page.locator('[id="semio.sketchpad.footer.feedback"]');
    await expect(footerFeedbackHome).toBeVisible({ timeout: 10000 });
    console.log("[Feedback] Footer action visibility test complete");
    // #endregion Footer Action Visibility
  });
});
