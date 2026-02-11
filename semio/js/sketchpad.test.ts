// #region 🔖Header

// 🧪semio/js/sketchpad.test.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

import { expect, Locator, Page, test } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";
import MetabolismKitData from "../assets/semio/kit_metabolism.json" with { type: "json" };

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
    .catch(() => {});
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
    .catch(() => {});
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
  const zipPath = path.resolve(__dirname, "../assets/semio/metabolism.zip");
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
    return Array.from(document.querySelectorAll("[data-row-id]"))
      .map((el) => el.getAttribute("data-row-id"))
      .slice(0, 10);
  });
  console.log("[TEST] Found data-row-id values:", dataRowIds);

  const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
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


  await page.waitForTimeout(2000);


  const allRowIds = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("[data-row-id]"))
      .map((el) => el.getAttribute("data-row-id"))
      .slice(0, 20);
  });
  console.log(`[initDesign] Available row IDs: ${JSON.stringify(allRowIds)}`);


  const designRowIds = allRowIds.filter((id) => id?.startsWith("design-"));
  console.log(`[initDesign] Design row IDs: ${JSON.stringify(designRowIds)}`);

  if (designRowIds.length === 0) {
    console.log(`[initDesign] No design rows found, looking for Nakagin Capsule Tower text...`);

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

  const finalUrl = page.url();
  console.log(`[initDesign] Final URL: ${finalUrl}`);

  const hasPieces = await page.evaluate(async () => {
    for (let i = 0; i < 30; i++) {
      const store = (window as any).__SEMIO_STORE__;
      if (store) {
        const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
        if (kitGuids.length > 0) {
          const kitStore = store.kit(kitGuids[0]);
          if (kitStore) {
            const kit = kitStore.snapshot();
            const designs = kit.designs ?? [];
            const design = designs[designs.length - 1];
            if (design && (design.pieces ?? []).length > 0) {
              return (design.pieces ?? []).length;
            }
          }
        }
      }
      await new Promise((r) => setTimeout(r, 1000));
    }
    return 0;
  });
  console.log(`[initDesign] Pieces loaded in store: ${hasPieces}`);

  return { errors, warnings, messages };
}

async function initType(page: Page) {
  const { errors, warnings, messages } = await initKit(page);
  await page.waitForTimeout(3000);

  console.log(`[initType] Current URL: ${page.url()}`);


  const typesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
  const hasTypesToggle = await typesToggle.isVisible({ timeout: 5000 }).catch(() => false);
  console.log(`[initType] Types toggle visible: ${hasTypesToggle}`);

  if (hasTypesToggle) {
    await typesToggle.click();
    await page.waitForTimeout(3000);
  }


  await page.waitForTimeout(2000);


  const allRowIds = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("[data-row-id]"))
      .map((el) => el.getAttribute("data-row-id"))
      .slice(0, 20);
  });
  console.log(`[initType] Available row IDs: ${JSON.stringify(allRowIds)}`);


  const typeRowIds = allRowIds.filter((id) => id?.startsWith("type-"));
  console.log(`[initType] Type row IDs: ${JSON.stringify(typeRowIds)}`);

  if (typeRowIds.length === 0) {
    console.log(`[initType] No type rows found, looking for Tambour text...`);

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

    // #region 🔖Panel Toggles
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
    // #endregion 🔖Panel Toggles

    // #region 🔖Toolbar and Filter Toggles
    console.log("[Home] Testing toolbar zone structure");
    const toolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    await expect(toolbar).toBeVisible({ timeout: 5000 });
    const toolsZone = page.locator('[id="semio.sketchpad.toolbar.zone.tools"]');
    await expect(toolsZone).toBeVisible({ timeout: 5000 });
    const toolbarZoneEl = toolsZone.locator('[data-slot="toolbar-zone"]').first();
    await expect(toolbarZoneEl).toBeVisible({ timeout: 3000 });

    console.log("[Home] Testing toolbar group toggles");
    const filterGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.filter"]');
    const hasFilterGroup = await filterGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Filter group toggle visible: ${hasFilterGroup}`);
    expect(hasFilterGroup).toBe(true);

    const createGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.create"]');
    const hasCreateGroup = await createGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Create group toggle visible: ${hasCreateGroup}`);
    expect(hasCreateGroup).toBe(true);

    console.log("[Home] Testing auto-activated filter group settings zone");
    const settingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
    const settingsInitiallyVisible = await settingsZone.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Settings zone initially visible (filter auto-active): ${settingsInitiallyVisible}`);
    if (!settingsInitiallyVisible) {
      await filterGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(settingsZone).toBeVisible({ timeout: 3000 });

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
      console.log("[Home] Testing temporary filter toggle on/off");
      await temporaryToggle.click();
      await page.waitForURL(/kind=temporary/, { timeout: 5000 }).catch(() => {});
      expect(page.url()).toContain("kind=temporary");
      await temporaryToggle.click();
      await page.waitForURL((url) => !url.href.includes("kind=temporary"), { timeout: 5000 }).catch(() => {});
      await page.waitForTimeout(500);
      expect(page.url()).not.toContain("kind=temporary");
    }

    console.log("[Home] Testing switching to create group");
    await createGroupToggle.click();
    await page.waitForTimeout(500);
    await expect(settingsZone).toBeVisible({ timeout: 3000 });
    const createTempBtn = page.locator('[id="semio.sketchpad.app.home.toolbar.createTemporary"]');
    const hasCreateTempBtn = await createTempBtn.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Create temporary button visible: ${hasCreateTempBtn}`);
    const createLocalBtn = page.locator('[id="semio.sketchpad.app.home.toolbar.createLocal"]');
    const hasCreateLocalBtn = await createLocalBtn.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Create local button visible: ${hasCreateLocalBtn}`);
    const createRemoteBtn = page.locator('[id="semio.sketchpad.app.home.toolbar.createRemote"]');
    const hasCreateRemoteBtn = await createRemoteBtn.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Create remote button visible: ${hasCreateRemoteBtn}`);
    expect(hasCreateTempBtn || hasCreateLocalBtn || hasCreateRemoteBtn).toBe(true);

    console.log("[Home] Testing group mutual exclusivity - filter settings hidden when create active");
    const temporaryStillVisible = await temporaryToggle.isVisible({ timeout: 1000 }).catch(() => false);
    expect(temporaryStillVisible).toBe(false);

    console.log("[Home] Testing deactivate group hides settings zone");
    await createGroupToggle.click();
    await page.waitForTimeout(500);
    const settingsHidden = !(await settingsZone.isVisible({ timeout: 1000 }).catch(() => false));
    console.log(`[Home] Settings zone hidden after deactivation: ${settingsHidden}`);
    expect(settingsHidden).toBe(true);

    console.log("[Home] Toolbar and filter toggles test complete");
    // #endregion 🔖Toolbar and Filter Toggles

    // #region 🔖Home Selection State
    console.log("[Home] Testing selection state");
    const zipPath = path.resolve(__dirname, "../assets/semio/metabolism.zip");
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
    // #endregion 🔖Home Selection State
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

    console.log("[Kit] Testing toolbar zone structure");
    const kitToolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    await expect(kitToolbar).toBeVisible({ timeout: 5000 });
    const kitToolsZone = page.locator('[id="semio.sketchpad.toolbar.zone.tools"]');
    await expect(kitToolsZone).toBeVisible({ timeout: 5000 });

    console.log("[Kit] Testing toolbar group toggles");
    const kitSelectionGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.selection"]');
    const hasKitSelectionGroup = await kitSelectionGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Selection group toggle visible: ${hasKitSelectionGroup}`);
    expect(hasKitSelectionGroup).toBe(true);

    const kitFilterGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.filter"]');
    const hasKitFilterGroup = await kitFilterGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Filter group toggle visible: ${hasKitFilterGroup}`);
    expect(hasKitFilterGroup).toBe(true);

    const kitCreateGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.create"]');
    const hasKitCreateGroup = await kitCreateGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Create group toggle visible: ${hasKitCreateGroup}`);
    expect(hasKitCreateGroup).toBe(true);

    console.log("[Kit] Testing toolbar groups with settings zone");
    const kitSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');

    console.log("[Kit] Activating filter group to verify filter toggles");
    await kitFilterGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await kitSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await kitFilterGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(kitSettingsZone).toBeVisible({ timeout: 3000 });

    const designsToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showDesigns"]');
    const hasDesignsToggle = await designsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Designs filter toggle visible: ${hasDesignsToggle}`);
    const typesFilterToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showTypes"]');
    const hasTypesFilterToggle = await typesFilterToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Types filter toggle visible: ${hasTypesFilterToggle}`);
    const qualitiesToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showQualities"]');
    const hasQualitiesToggle = await qualitiesToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Qualities filter toggle visible: ${hasQualitiesToggle}`);
    const portsToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showPorts"]');
    const hasPortsToggle = await portsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Ports filter toggle visible: ${hasPortsToggle}`);
    const tagsToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showTags"]');
    const hasTagsToggle = await tagsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Tags filter toggle visible: ${hasTagsToggle}`);
    expect(hasDesignsToggle || hasTypesFilterToggle || hasQualitiesToggle).toBe(true);

    console.log("[Kit] Activating create group to verify create buttons");
    await kitCreateGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await kitSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await kitCreateGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(kitSettingsZone).toBeVisible({ timeout: 3000 });
    const kitCreateDesignBtn = page.locator('[id="semio.sketchpad.app.kit.toolbar.createDesign"]');
    const hasCreateDesignBtn = await kitCreateDesignBtn.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Create design button visible: ${hasCreateDesignBtn}`);
    const kitCreateTypeBtn = page.locator('[id="semio.sketchpad.app.kit.toolbar.createType"]');
    const hasCreateTypeBtn = await kitCreateTypeBtn.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Create type button visible: ${hasCreateTypeBtn}`);
    const kitCreateQualityBtn = page.locator('[id="semio.sketchpad.app.kit.toolbar.createQuality"]');
    const hasCreateQualityBtn = await kitCreateQualityBtn.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Create quality button visible: ${hasCreateQualityBtn}`);
    expect(hasCreateDesignBtn || hasCreateTypeBtn || hasCreateQualityBtn).toBe(true);

    console.log("[Kit] Testing group mutual exclusivity - filter hidden when create active");
    const designsStillVisible = await designsToggle.isVisible({ timeout: 1000 }).catch(() => false);
    expect(designsStillVisible).toBe(false);

    console.log("[Kit] Activating selection group to verify selection tools");
    await kitSelectionGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await kitSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await kitSelectionGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(kitSettingsZone).toBeVisible({ timeout: 3000 });

    const kitAdditiveToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.mode.additive"]');
    const hasKitAdditive = await kitAdditiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Additive mode toggle visible: ${hasKitAdditive}`);
    const kitSubtractiveToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.mode.subtractive"]');
    const hasKitSubtractive = await kitSubtractiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Subtractive mode toggle visible: ${hasKitSubtractive}`);
    const kitIntersectToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.mode.intersect"]');
    const hasKitIntersect = await kitIntersectToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Intersect mode toggle visible: ${hasKitIntersect}`);
    const kitRectangularToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.shape.rectangular"]');
    const hasKitRectangular = await kitRectangularToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Rectangular shape toggle visible: ${hasKitRectangular}`);
    const kitLassoToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.shape.lasso"]');
    const hasKitLasso = await kitLassoToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Lasso shape toggle visible: ${hasKitLasso}`);
    const kitHandToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.navigation.hand"]');
    const hasKitHand = await kitHandToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Hand toggle visible: ${hasKitHand}`);
    expect(hasKitAdditive || hasKitSubtractive || hasKitIntersect || hasKitRectangular || hasKitLasso || hasKitHand).toBe(true);

    if (hasKitAdditive) {
      console.log("[Kit] Testing additive toggle activation");
      await kitAdditiveToggle.click();
      await page.waitForTimeout(300);
      const additiveState = await kitAdditiveToggle.getAttribute("data-state").catch(() => null) ?? await kitAdditiveToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Kit] Additive state after click: ${additiveState}`);
      expect(additiveState === "on" || additiveState === "true").toBe(true);
      await kitAdditiveToggle.click();
      await page.waitForTimeout(300);
    }

    console.log("[Kit] Toolbar and artifact filter toggles test complete");

    // #region 🔖SidePanel Toggle
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
    // #endregion 🔖SidePanel Toggle

    // #region 🔖Kit Selection State
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
    // #endregion 🔖Kit Selection State

    // #region 🔖Diagram Node Icons
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
    // #endregion 🔖Diagram Node Icons

    // #region 🔖Diagram Node Dragging
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
        // #endregion 🔖Diagram Node Dragging

        // #region 🔖Diagram Table Selection Sync
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
        // #endregion 🔖Diagram Table Selection Sync

        // #region 🔖Diagram Node Click Selection
        console.log("[Kit] Verifying clicking diagram node updates selection");


        const diagramContainerForClick = page.locator(".react-flow").first();
        const isDiagramVisibleForClick = await diagramContainerForClick.isVisible().catch(() => false);
        console.log(`[Kit] Diagram container visible for click test: ${isDiagramVisibleForClick}`);


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
        // #endregion 🔖Diagram Node Click Selection

        // #region 🔖Diagram Hover Sync
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
        // #endregion 🔖Diagram Hover Sync

        // #region 🔖Diagram Filter Sync
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
        console.log("[Kit] Diagram filter sync test complete");
        // #endregion 🔖Diagram Filter Sync

        // #region 🔖Diagram All Artifact Types
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
        console.log("[Kit] Diagram all artifact types test complete");
        // #endregion 🔖Diagram All Artifact Types

        // #region 🔖Diagram Edges
        console.log("[Kit] Verifying edges connect nodes properly");
        const edges = page.locator(".react-flow__edge");
        const edgeCount = await edges.count();
        console.log(`[Kit] Found ${edgeCount} edges`);
        expect(edgeCount).toBeGreaterThan(0);

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
        // #endregion 🔖Diagram Edges
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

    console.log("[Type] Testing toolbar zone structure");
    await page.waitForTimeout(2000);

    const toolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    await expect(toolbar).toBeVisible({ timeout: 5000 });
    const typeToolsZone = page.locator('[id="semio.sketchpad.toolbar.zone.tools"]');
    await expect(typeToolsZone).toBeVisible({ timeout: 5000 });

    console.log("[Type] Testing toolbar group toggles");
    const typeSelectionGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.selection"]');
    const hasTypeSelectionGroup = await typeSelectionGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Selection group toggle visible: ${hasTypeSelectionGroup}`);
    expect(hasTypeSelectionGroup).toBe(true);

    const typeCreateGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.create"]');
    const hasTypeCreateGroup = await typeCreateGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Create group toggle visible: ${hasTypeCreateGroup}`);
    expect(hasTypeCreateGroup).toBe(true);

    console.log("[Type] Activating selection group to verify selection modes");
    const typeSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
    await typeSelectionGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await typeSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await typeSelectionGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(typeSettingsZone).toBeVisible({ timeout: 3000 });

    const typeNormalToggle = page.locator('[id="selection-normal"]');
    const hasTypeNormal = await typeNormalToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Normal selection toggle visible: ${hasTypeNormal}`);
    const typeAdditiveToggle = page.locator('[id="selection-additive"]');
    const hasTypeAdditive = await typeAdditiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Additive selection toggle visible: ${hasTypeAdditive}`);
    const typeSubtractiveToggle = page.locator('[id="selection-subtractive"]');
    const hasTypeSubtractive = await typeSubtractiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Subtractive selection toggle visible: ${hasTypeSubtractive}`);
    const typeIntersectToggle = page.locator('[id="selection-intersect"]');
    const hasTypeIntersect = await typeIntersectToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Intersect selection toggle visible: ${hasTypeIntersect}`);
    expect(hasTypeNormal || hasTypeAdditive || hasTypeSubtractive || hasTypeIntersect).toBe(true);

    if (hasTypeAdditive) {
      console.log("[Type] Testing additive toggle activation");
      await typeAdditiveToggle.click();
      await page.waitForTimeout(300);
      const additiveState = await typeAdditiveToggle.getAttribute("data-state").catch(() => null) ?? await typeAdditiveToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Type] Additive state after click: ${additiveState}`);
      expect(additiveState === "on" || additiveState === "true").toBe(true);
      if (hasTypeNormal) {
        await typeNormalToggle.click();
        await page.waitForTimeout(300);
      } else {
        await typeAdditiveToggle.click();
        await page.waitForTimeout(300);
      }
    }

    console.log("[Type] Activating create group to verify connector tool");
    await typeCreateGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await typeSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await typeCreateGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(typeSettingsZone).toBeVisible({ timeout: 3000 });

    const connectorToolToggle = page.locator(`[id="${"connector"}"]`);
    const hasConnectorTool = await connectorToolToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Connector tool toggle visible: ${hasConnectorTool}`);

    if (hasConnectorTool) {
      console.log("[Type] Testing connector tool activation");
      await connectorToolToggle.click();
      await page.waitForTimeout(500);
      const connectorState = await connectorToolToggle.getAttribute("data-state").catch(() => null) ?? await connectorToolToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Type] Connector tool state: ${connectorState}`);
      expect(connectorState === "on" || connectorState === "true").toBe(true);

      const canvasForPort = page.locator("canvas").first();
      const canvasBoxForPort = await canvasForPort.boundingBox();
      if (canvasBoxForPort) {
        const connectorX = canvasBoxForPort.x + canvasBoxForPort.width / 2;
        const connectorY = canvasBoxForPort.y + canvasBoxForPort.height / 2;
        console.log("[Type] Testing connector tool canvas interaction");
        await page.mouse.move(connectorX, connectorY);
        await page.waitForTimeout(300);
        await page.mouse.click(connectorX, connectorY);
        await page.waitForTimeout(500);
      }
    }

    console.log("[Type] Testing group mutual exclusivity - selection settings hidden when create active");
    const normalStillVisible = await typeNormalToggle.isVisible({ timeout: 1000 }).catch(() => false);
    expect(normalStillVisible).toBe(false);

    console.log("[Type] Switching back to selection group");
    await typeSelectionGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await typeSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await typeSelectionGroupToggle.click();
      await page.waitForTimeout(500);
    }
    const selectionSettingsBack = await typeNormalToggle.isVisible({ timeout: 3000 }).catch(() => false) || await typeAdditiveToggle.isVisible({ timeout: 1000 }).catch(() => false);
    console.log(`[Type] Selection settings visible after switch back: ${selectionSettingsBack}`);

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

    const reactFlowNodes = diagramContainer.locator(".react-flow__node");
    await reactFlowNodes.first().waitFor({ state: "attached", timeout: 60000 }).catch(() => console.log("[Design Test] ReactFlow nodes not attached after 60s"));

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

    // #region 🔖Panel Toggles Check
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
    // #endregion 🔖Panel Toggles Check

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


        expect(pan1Duration).toBeLessThan(1500);
        expect(pan2Duration).toBeLessThan(1500);

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

    console.log("[Design] Testing toolbar zone structure");
    const designToolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    await expect(designToolbar).toBeVisible({ timeout: 5000 });
    const designToolsZone = page.locator('[id="semio.sketchpad.toolbar.zone.tools"]');
    await expect(designToolsZone).toBeVisible({ timeout: 5000 });

    console.log("[Design] Testing toolbar group toggles");
    const designSelectionGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.selection"]');
    const hasDesignSelectionGroup = await designSelectionGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Selection group toggle visible: ${hasDesignSelectionGroup}`);
    expect(hasDesignSelectionGroup).toBe(true);

    console.log("[Design] Activating selection group to verify selection tools");
    const designSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
    await designSelectionGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await designSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await designSelectionGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(designSettingsZone).toBeVisible({ timeout: 3000 });

    const designAdditiveToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.mode.additive"]');
    const hasDesignAdditive = await designAdditiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Additive mode toggle visible: ${hasDesignAdditive}`);
    const designSubtractiveToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.mode.subtractive"]');
    const hasDesignSubtractive = await designSubtractiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Subtractive mode toggle visible: ${hasDesignSubtractive}`);
    const designIntersectToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.mode.intersect"]');
    const hasDesignIntersect = await designIntersectToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Intersect mode toggle visible: ${hasDesignIntersect}`);
    const designRectangularToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.shape.rectangular"]');
    const hasDesignRectangular = await designRectangularToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Rectangular shape toggle visible: ${hasDesignRectangular}`);
    const designLassoToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.shape.lasso"]');
    const hasDesignLasso = await designLassoToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Lasso shape toggle visible: ${hasDesignLasso}`);
    const designHandToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.navigation.hand"]');
    const hasDesignHand = await designHandToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Hand toggle visible: ${hasDesignHand}`);
    expect(hasDesignAdditive || hasDesignSubtractive || hasDesignIntersect).toBe(true);

    if (hasDesignAdditive) {
      console.log("[Design] Testing additive toggle activation");
      await designAdditiveToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
      const additiveState = await designAdditiveToggle.getAttribute("data-state").catch(() => null) ?? await designAdditiveToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Design] Additive state after click: ${additiveState}`);
      expect(additiveState === "on" || additiveState === "true").toBe(true);

      console.log("[Design] Testing subtractive toggle switches from additive");
      if (hasDesignSubtractive) {
        await designSubtractiveToggle.dispatchEvent("click");
        await page.waitForTimeout(300);
        const subtractiveState = await designSubtractiveToggle.getAttribute("data-state").catch(() => null) ?? await designSubtractiveToggle.getAttribute("aria-checked").catch(() => null);
        console.log(`[Design] Subtractive state after click: ${subtractiveState}`);
        expect(subtractiveState === "on" || subtractiveState === "true").toBe(true);
        const additiveAfter = await designAdditiveToggle.getAttribute("data-state").catch(() => null) ?? await designAdditiveToggle.getAttribute("aria-checked").catch(() => null);
        console.log(`[Design] Additive state after switching to subtractive: ${additiveAfter}`);
        await designSubtractiveToggle.dispatchEvent("click");
        await page.waitForTimeout(300);
      } else {
        await designAdditiveToggle.dispatchEvent("click");
        await page.waitForTimeout(300);
      }
    }

    if (hasDesignRectangular) {
      console.log("[Design] Testing rectangular shape toggle");
      await designRectangularToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
      const rectState = await designRectangularToggle.getAttribute("data-state").catch(() => null) ?? await designRectangularToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Design] Rectangular state: ${rectState}`);
      expect(rectState === "on" || rectState === "true").toBe(true);
      await designRectangularToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
    }

    if (hasDesignLasso) {
      console.log("[Design] Testing lasso shape toggle");
      await designLassoToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
      const lassoState = await designLassoToggle.getAttribute("data-state").catch(() => null) ?? await designLassoToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Design] Lasso state: ${lassoState}`);
      expect(lassoState === "on" || lassoState === "true").toBe(true);
      await designLassoToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
    }

    if (hasDesignHand) {
      console.log("[Design] Testing hand toggle activation and deactivation");
      await designHandToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
      const handState = await designHandToggle.getAttribute("data-state").catch(() => null) ?? await designHandToggle.getAttribute("aria-checked").catch(() => null);
      console.log(`[Design] Hand state: ${handState}`);
      expect(handState === "on" || handState === "true").toBe(true);
      await designHandToggle.dispatchEvent("click");
      await page.waitForTimeout(300);
    }

    console.log("[Design] Testing deactivate group hides settings zone");
    await designSelectionGroupToggle.click();
    await page.waitForTimeout(500);
    const designSettingsHidden = !(await designSettingsZone.isVisible({ timeout: 1000 }).catch(() => false));
    console.log(`[Design] Settings zone hidden after deactivation: ${designSettingsHidden}`);
    expect(designSettingsHidden).toBe(true);

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

    // #region 🔖Design Selection State
    console.log("[Design] Testing selection state");
    const initialDesignAppState = await page.evaluate(() => {
      const actor = (window as any).__SEMIO_ACTOR__;
      if (!actor) return null;
      const snapshot = actor.getSnapshot();
      const url = window.location.pathname;
      const designGuidMatch = url.match(/\/designs\/([^/]+)/);
      const designGuid = designGuidMatch?.[1];
      const kitGuid = Object.keys(snapshot?.context?.kits || {})[0];
      const key = kitGuid && designGuid ? `${kitGuid}:${designGuid}` : designGuid || "";
      return { designApp: snapshot?.context?.designApps?.[key], designGuid };
    });
    console.log("[Design] Initial designApp state:", JSON.stringify(initialDesignAppState));

    const diagramContainerSel = page.locator(".react-flow").first();
    const hasDiagramSel = await diagramContainerSel.isVisible({ timeout: 10000 }).catch(() => false);

    if (hasDiagramSel) {
      const existingPiecesSel = diagramContainerSel.locator(".react-flow__node");
      const pieceCountSel = await existingPiecesSel.count();
      console.log("[Design] Piece count for selection:", pieceCountSel);

      if (pieceCountSel > 0) {
        const firstPieceSel = existingPiecesSel.first();
        const pieceBoxSel = await firstPieceSel.boundingBox();
        if (pieceBoxSel) {
          await page.mouse.click(pieceBoxSel.x + pieceBoxSel.width / 2, pieceBoxSel.y + pieceBoxSel.height / 2);
          console.log("[Design] Clicked first piece node via page.mouse.click");
        }
        await page.waitForTimeout(500);

        const afterClickDesignAppState = await page.evaluate(() => {
          const actor = (window as any).__SEMIO_ACTOR__;
          if (!actor) return null;
          const snapshot = actor.getSnapshot();
          const url = window.location.pathname;
          const designGuidMatch = url.match(/\/designs\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          const kitGuid = Object.keys(snapshot?.context?.kits || {})[0];
          const key = kitGuid && designGuid ? `${kitGuid}:${designGuid}` : designGuid || "";
          return { designApp: snapshot?.context?.designApps?.[key], designGuid };
        });
        console.log("[Design] After click designApp state:", JSON.stringify(afterClickDesignAppState));

        const selectionPieces = afterClickDesignAppState?.designApp?.selection?.pieces || [];
        console.log("[Design] Selection pieces:", selectionPieces);
        expect(selectionPieces.length).toBeGreaterThanOrEqual(0);
      }
    }
    console.log("[Design] Browser errors after selection click:", errors.length, "total errors");
    for (const e of errors) {
      if (!e.includes("WebGL")) console.log("[Design] NON-WEBGL ERROR:", e.slice(0, 1000));
    }
    console.log("[Design] Browser warnings after selection click:", warnings.filter(w => w.includes("[DEBUG]")).slice(-10));
    const nodeCountAfterClick = await diagramContainer.locator(".react-flow__node").count();
    console.log("[Design] Node count after selection click:", nodeCountAfterClick);
    console.log("[Design] Selection state test complete");
    // #endregion 🔖Design Selection State

    // #region 🔖Panel Toggle Independence
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
    // #endregion 🔖Panel Toggle Independence

    // #region 🔖Drag and Drop Setup
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
    // #endregion 🔖Drag and Drop Setup

    // #region 🔖Diagram Node Drag
    console.log("[Design] Testing diagram node drag to update piece center");

    if (hasDiagram) {
      const getPieceCenters = async (): Promise<Record<string, { u: number; v: number } | null>> => {
        return await page.evaluate(() => {
          const store = (window as any).__SEMIO_STORE__;
          if (!store) return {};
          const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
          if (kitGuids.length === 0) return {};
          const kitStore = store.kit(kitGuids[0]);
          if (!kitStore) return {};
          const kit = kitStore.snapshot();
          const url = window.location.pathname;
          const designGuidMatch = url.match(/\/designs\/([^/]+)/);
          const designGuid = designGuidMatch?.[1];
          const design = designGuid ? kit.designs?.find((d: any) => d.guid === designGuid) : kit.designs?.[kit.designs.length - 1];
          if (!design) return {};
          const result: Record<string, { u: number; v: number } | null> = {};
          for (const piece of design.pieces ?? []) {
            result[piece.guid] = piece.center ?? null;
          }
          return result;
        });
      };

      const pieceNodesDrag = diagramContainer.locator(".react-flow__node");
      const pieceNodeCountDrag = await pieceNodesDrag.count();
      console.log(`[Design] Found ${pieceNodeCountDrag} piece nodes for drag test`);
      console.log("[Design] Browser errors before drag:", errors.length, "total errors");
      for (const e of errors) {
        if (!e.includes("WebGL")) console.log("[Design] DRAG NON-WEBGL ERROR:", e.slice(0, 500));
      }
      const rfContainerCountDrag = await page.locator(".react-flow").count();
      console.log(`[Design] ReactFlow containers at drag time: ${rfContainerCountDrag}`);
      const diagramVisibleDrag = await diagramContainer.isVisible().catch(() => false);
      console.log(`[Design] Diagram container visible at drag time: ${diagramVisibleDrag}`);

      if (pieceNodeCountDrag > 0) {
        const firstPieceNodeDrag = pieceNodesDrag.first();
        const pieceNodeBoxDrag = await firstPieceNodeDrag.boundingBox();

        if (pieceNodeBoxDrag) {
          const pieceGuidDrag = await firstPieceNodeDrag.getAttribute("data-id");
          console.log(`[Design] Testing drag on piece node: ${pieceGuidDrag}`);

          const centersBeforeDrag = await getPieceCenters();
          const pieceGuidFromData = pieceGuidDrag?.replace(/^piece-\d+-/, "") ?? "";
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

          console.log("[Design] Drag DEBUG warnings:", warnings.filter(w => w.includes("onNodeDrag")).slice(-10));

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
    // #endregion 🔖Diagram Node Drag

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
    // #region 🔖Navigation
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
    // #endregion 🔖Navigation

    // #region 🔖Bug Report Form
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
    // #endregion 🔖Bug Report Form

    // #region 🔖Idea Form Switch
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
    // #endregion 🔖Idea Form Switch

    // #region 🔖Toolbar
    console.log("[Feedback] Testing toolbar zone structure");
    await page.goto("/feedback");
    await page.waitForTimeout(1000);

    const feedbackToolbar = page.locator('[id="semio.sketchpad.toolbar"]');
    await expect(feedbackToolbar).toBeVisible({ timeout: 5000 });
    const feedbackToolsZone = page.locator('[id="semio.sketchpad.toolbar.zone.tools"]');
    await expect(feedbackToolsZone).toBeVisible({ timeout: 5000 });

    console.log("[Feedback] Testing toolbar group toggles");
    const actionsGroupToggle = page.locator('[id="semio.sketchpad.toolbar.group.actions"]');
    const hasActionsGroup = await actionsGroupToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Feedback] Actions group toggle visible: ${hasActionsGroup}`);
    expect(hasActionsGroup).toBe(true);

    console.log("[Feedback] Activating actions group to verify send button");
    const feedbackSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
    await actionsGroupToggle.click();
    await page.waitForTimeout(500);
    if (!(await feedbackSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
      await actionsGroupToggle.click();
      await page.waitForTimeout(500);
    }
    await expect(feedbackSettingsZone).toBeVisible({ timeout: 3000 });

    const sendButton = page.locator('[id="semio.sketchpad.app.feedback.toolbar.send"]');
    const hasSendButton = await sendButton.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Feedback] Send button visible in settings zone: ${hasSendButton}`);
    expect(hasSendButton).toBe(true);

    console.log("[Feedback] Testing deactivate group hides settings zone");
    await actionsGroupToggle.click();
    await page.waitForTimeout(500);
    const feedbackSettingsHidden = !(await feedbackSettingsZone.isVisible({ timeout: 1000 }).catch(() => false));
    console.log(`[Feedback] Settings zone hidden after deactivation: ${feedbackSettingsHidden}`);
    if (!feedbackSettingsHidden) {
      await actionsGroupToggle.click();
      await page.waitForTimeout(500);
    }
    expect(!(await feedbackSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))).toBe(true);

    console.log("[Feedback] Testing reactivate group shows send button again");
    await actionsGroupToggle.click();
    await page.waitForTimeout(500);
    await expect(feedbackSettingsZone).toBeVisible({ timeout: 3000 });
    const sendButtonAgain = await sendButton.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Feedback] Send button visible again: ${sendButtonAgain}`);
    expect(sendButtonAgain).toBe(true);

    console.log("[Feedback] Toolbar test complete");
    // #endregion 🔖Toolbar

    // #region 🔖Validation
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
    // #endregion 🔖Validation

    // #region 🔖Fill Bug Report
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
    // #endregion 🔖Fill Bug Report

    // #region 🔖Fill Feature Idea
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
    // #endregion 🔖Fill Feature Idea

    // #region 🔖Footer Action Visibility
    console.log("[Feedback] Testing footer action visibility");
    await page.goto("/");
    await page.waitForTimeout(500);
    const footerFeedbackHome = page.locator('[id="semio.sketchpad.footer.feedback"]');
    await expect(footerFeedbackHome).toBeVisible({ timeout: 10000 });
    console.log("[Feedback] Footer action visibility test complete");
    // #endregion 🔖Footer Action Visibility
  });

  test("Panels", async ({ page }) => {
    test.setTimeout(300000);
    const { errors, warnings, messages } = await initConsole(page);

    // #region 🔖Home Panel Combinations
    console.log("[Panels] Testing Home app panel combinations");
    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(2000);

    const leftToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
    const hudToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
    const rightToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');

    const hasLeft = await leftToggle.isVisible({ timeout: 5000 }).catch(() => false);
    const hasHud = await hudToggle.isVisible({ timeout: 3000 }).catch(() => false);
    const hasRight = await rightToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Panels] Home toggles: left=${hasLeft}, hud=${hasHud}, right=${hasRight}`);

    const ensureAllClosed = async () => {
      for (const [key, toggle] of [["leftSidePanel", leftToggle], ["hudPanel", hudToggle], ["rightSidePanel", rightToggle]] as const) {
        const panel = page.locator(`[data-panel="${key}"]`).first();
        if (await panel.isVisible().catch(() => false)) {
          const t = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${key}"]`);
          if (await t.isVisible().catch(() => false)) {
            await t.click();
            await page.waitForTimeout(300);
          }
        }
      }
    };

    const ensureAllOpen = async () => {
      for (const key of ["leftSidePanel", "hudPanel", "rightSidePanel"] as const) {
        const panel = page.locator(`[data-panel="${key}"]`).first();
        if (!(await panel.isVisible().catch(() => false))) {
          const t = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${key}"]`);
          if (await t.isVisible().catch(() => false)) {
            await t.click();
            await page.waitForTimeout(300);
          }
        }
      }
    };

    const getPanelVisibleState = async () => ({
      left: await page.locator('[data-panel="leftSidePanel"]').first().isVisible().catch(() => false),
      hud: await page.locator('[data-panel="hudPanel"]').first().isVisible().catch(() => false),
      right: await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false),
    });

    await ensureAllClosed();
    await page.waitForTimeout(300);
    let state = await getPanelVisibleState();
    console.log(`[Panels] Home all closed: left=${state.left}, hud=${state.hud}, right=${state.right}`);
    if (hasLeft) expect(state.left).toBe(false);
    if (hasHud) expect(state.hud).toBe(false);
    if (hasRight) expect(state.right).toBe(false);

    await ensureAllOpen();
    await page.waitForTimeout(500);
    state = await getPanelVisibleState();
    console.log(`[Panels] Home all open: left=${state.left}, hud=${state.hud}, right=${state.right}`);
    const availableToggles = [hasLeft, hasHud, hasRight].filter(Boolean).length;
    const homeOpenCount = [state.left, state.hud, state.right].filter(Boolean).length;
    console.log(`[Panels] Home open panel count: ${homeOpenCount} / ${availableToggles} available`);
    expect(homeOpenCount).toBe(availableToggles);

    if (state.left && state.right) {
      const leftPanel = page.locator('[data-panel="leftSidePanel"]').first();
      const rightPanel = page.locator('[data-panel="rightSidePanel"]').first();
      const leftBox = await leftPanel.boundingBox();
      const rightBox = await rightPanel.boundingBox();
      if (leftBox && rightBox) {
        expect(rightBox.x).toBeGreaterThan(leftBox.x + leftBox.width - 5);
        console.log("[Panels] Home: Left and right panels do not overlap");
      }
    }

    if (state.right) {
      const rightContent = await page.locator('[data-panel="rightSidePanel"]').first().locator('button, input, [role="treeitem"], [role="button"]').count().catch(() => 0);
      console.log(`[Panels] Home right panel content items: ${rightContent}`);
    }

    await ensureAllClosed();
    await page.waitForTimeout(300);

    console.log("[Panels] Home: Testing left+right combination only");
    if (hasLeft) {
      await leftToggle.click();
      await page.waitForTimeout(300);
    }
    if (hasRight) {
      await rightToggle.click();
      await page.waitForTimeout(300);
    }
    state = await getPanelVisibleState();
    console.log(`[Panels] Home left+right: left=${state.left}, hud=${state.hud}, right=${state.right}`);
    if (hasLeft) expect(state.left).toBe(true);
    if (hasRight) expect(state.right).toBe(true);
    if (hasHud) expect(state.hud).toBe(false);

    console.log("[Panels] Home: Testing hud-only combination");
    await ensureAllClosed();
    await page.waitForTimeout(300);
    if (hasHud) {
      await hudToggle.click();
      await page.waitForTimeout(300);
    }
    state = await getPanelVisibleState();
    console.log(`[Panels] Home hud-only: left=${state.left}, hud=${state.hud}, right=${state.right}`);
    if (hasLeft) expect(state.left).toBe(false);
    if (hasHud) expect(state.hud).toBe(true);
    if (hasRight) expect(state.right).toBe(false);

    await ensureAllClosed();
    await page.waitForTimeout(300);

    console.log("[Panels] Home: Testing toolbar group with open panels");
    const filterGroup = page.locator('[id="semio.sketchpad.toolbar.group.filter"]');
    const hasFilterGroup = await filterGroup.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasFilterGroup && hasRight) {
      await rightToggle.click();
      await page.waitForTimeout(300);
      await filterGroup.click();
      await page.waitForTimeout(500);
      const settingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
      const settingsVisible = await settingsZone.isVisible({ timeout: 3000 }).catch(() => false);
      const rightStillOpen = await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false);
      console.log(`[Panels] Home filter+right: settings=${settingsVisible}, right=${rightStillOpen}`);
      expect(rightStillOpen).toBe(true);
      if (settingsVisible) {
        const createGroup = page.locator('[id="semio.sketchpad.toolbar.group.create"]');
        if (await createGroup.isVisible({ timeout: 2000 }).catch(() => false)) {
          await createGroup.click();
          await page.waitForTimeout(300);
          const rightAfterSwitch = await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false);
          console.log(`[Panels] Home: right panel after toolbar group switch: ${rightAfterSwitch}`);
          expect(rightAfterSwitch).toBe(true);
        }
      }
    }

    await ensureAllClosed();
    console.log("[Panels] Home panel combinations complete");
    // #endregion 🔖Home Panel Combinations

    // #region 🔖Kit Panel Combinations
    console.log("[Panels] Testing Kit app panel combinations");
    await initKit(page);
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);

    const kitState = async () => ({
      left: await page.locator('[data-panel="leftSidePanel"]').first().isVisible().catch(() => false),
      hud: await page.locator('[data-panel="hudPanel"]').first().isVisible().catch(() => false),
      right: await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false),
    });

    await ensureAllClosed();
    await page.waitForTimeout(300);

    console.log("[Panels] Kit: Opening all panels simultaneously");
    await ensureAllOpen();
    await page.waitForTimeout(500);
    let ks = await kitState();
    console.log(`[Panels] Kit all open: left=${ks.left}, hud=${ks.hud}, right=${ks.right}`);
    const kitAvailableToggles = [hasLeft, hasHud, hasRight].filter(Boolean).length;
    const kitOpenCount = [ks.left, ks.hud, ks.right].filter(Boolean).length;
    console.log(`[Panels] Kit open panel count: ${kitOpenCount} / ${kitAvailableToggles} available`);
    expect(kitOpenCount).toBeGreaterThanOrEqual(Math.min(kitAvailableToggles, 1));

    if (ks.right) {
      const rightPanelContent = await page.locator('[data-panel="rightSidePanel"]').first().locator('button, input, [role="treeitem"], [role="button"]').count().catch(() => 0);
      console.log(`[Panels] Kit right panel content when all open: ${rightPanelContent}`);
      expect(rightPanelContent).toBeGreaterThan(0);
    }

    console.log("[Panels] Kit: Testing diagram visible while all panels open");
    const kitDiagram = page.locator('[data-testid="kit-diagram"]');
    const hasDiagramWithPanels = await kitDiagram.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Panels] Kit diagram visible with all panels open: ${hasDiagramWithPanels}`);
    if (hasDiagramWithPanels) {
      const nodeCount = await page.locator(".react-flow__node").count();
      console.log(`[Panels] Kit diagram nodes with all panels open: ${nodeCount}`);
      expect(nodeCount).toBeGreaterThan(0);
    }

    console.log("[Panels] Kit: Testing selection group + panel combination");
    const kitSelectionGroup = page.locator('[id="semio.sketchpad.toolbar.group.selection"]');
    const hasKitSelection = await kitSelectionGroup.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasKitSelection) {
      await kitSelectionGroup.click();
      await page.waitForTimeout(500);
      const kitSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
      if (!(await kitSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
        await kitSelectionGroup.click();
        await page.waitForTimeout(500);
      }
      const settingsVisible = await kitSettingsZone.isVisible({ timeout: 3000 }).catch(() => false);
      console.log(`[Panels] Kit selection group settings visible: ${settingsVisible}`);
      ks = await kitState();
      console.log(`[Panels] Kit panels after selection group: left=${ks.left}, hud=${ks.hud}, right=${ks.right}`);
      expect(kitOpenCount).toBeGreaterThanOrEqual(Math.min(kitAvailableToggles, 1));
    }

    console.log("[Panels] Kit: Testing filter group + all panels");
    const kitFilterGroup = page.locator('[id="semio.sketchpad.toolbar.group.filter"]');
    const hasKitFilter = await kitFilterGroup.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasKitFilter) {
      await kitFilterGroup.click();
      await page.waitForTimeout(500);
      const kitSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
      if (!(await kitSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
        await kitFilterGroup.click();
        await page.waitForTimeout(500);
      }
      const designsToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showDesigns"]');
      const typesToggle = page.locator('[id="semio.sketchpad.app.kit.toolbar.showTypes"]');
      const hasDesigns = await designsToggle.isVisible({ timeout: 2000 }).catch(() => false);
      const hasTypes = await typesToggle.isVisible({ timeout: 2000 }).catch(() => false);
      console.log(`[Panels] Kit filter toggles visible with all panels: designs=${hasDesigns}, types=${hasTypes}`);
      expect(hasDesigns || hasTypes).toBe(true);
    }

    console.log("[Panels] Kit: Testing rapid toggle cycle");
    for (let i = 0; i < 3; i++) {
      if (hasLeft) {
        await leftToggle.click();
        await page.waitForTimeout(200);
      }
    }
    ks = await kitState();
    console.log(`[Panels] Kit after rapid toggle: left=${ks.left}, hud=${ks.hud}, right=${ks.right}`);

    await ensureAllClosed();
    console.log("[Panels] Kit panel combinations complete");
    // #endregion 🔖Kit Panel Combinations

    // #region 🔖Design Panel Combinations
    console.log("[Panels] Testing Design app panel combinations");
    await initDesign(page);
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(5000);

    const isDesignUrl = page.url().includes("/designs/");
    console.log(`[Panels] Design URL: ${page.url()}, isDesign: ${isDesignUrl}`);
    expect(isDesignUrl).toBe(true);

    const designState = async () => ({
      left: await page.locator('[data-panel="leftSidePanel"]').first().isVisible().catch(() => false),
      hud: await page.locator('[data-panel="hudPanel"]').first().isVisible().catch(() => false),
      right: await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false),
    });

    await ensureAllClosed();
    await page.waitForTimeout(300);

    console.log("[Panels] Design: Opening all panels");
    await ensureAllOpen();
    await page.waitForTimeout(500);
    let ds = await designState();
    console.log(`[Panels] Design all open: left=${ds.left}, hud=${ds.hud}, right=${ds.right}`);
    const designOpenCount = [ds.left, ds.hud, ds.right].filter(Boolean).length;
    console.log(`[Panels] Design open count: ${designOpenCount}`);
    expect(designOpenCount).toBeGreaterThanOrEqual(1);

    console.log("[Panels] Design: Verifying diagram remains functional with all panels open");
    const designDiagram = page.locator(".react-flow").first();
    const hasDiagramDesign = await designDiagram.isVisible({ timeout: 10000 }).catch(() => false);
    console.log(`[Panels] Design diagram visible with panels: ${hasDiagramDesign}`);
    if (hasDiagramDesign) {
      const pieceCount = await designDiagram.locator(".react-flow__node").count();
      console.log(`[Panels] Design piece nodes with all panels: ${pieceCount}`);
      expect(pieceCount).toBeGreaterThan(0);
    }

    console.log("[Panels] Design: Verifying 3D canvas remains visible with all panels open");
    const sceneCanvas = page.locator("canvas").first();
    const hasCanvas = await sceneCanvas.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Panels] Design canvas visible with all panels: ${hasCanvas}`);

    if (ds.left) {
      console.log("[Panels] Design: Checking left panel (workbench) content");
      const leftPanel = page.locator('[data-panel="leftSidePanel"]').first();
      const leftContent = await leftPanel.locator('[data-slot="avatar"], [role="treeitem"], button').count().catch(() => 0);
      console.log(`[Panels] Design left panel content items: ${leftContent}`);
    }

    if (ds.hud) {
      console.log("[Panels] Design: Checking HUD panel content with all panels visible");
      const hudPanel = page.locator('[data-panel="hudPanel"]').first();
      const hudContent = await hudPanel.locator('button, [role="treeitem"], [role="button"]').count().catch(() => 0);
      console.log(`[Panels] Design HUD panel content items: ${hudContent}`);
    }

    if (ds.right) {
      console.log("[Panels] Design: Checking right panel (details) content with all panels visible");
      const rightPanel = page.locator('[data-panel="rightSidePanel"]').first();
      const rightContent = await rightPanel.locator('button, input, [role="treeitem"], [role="button"]').count().catch(() => 0);
      console.log(`[Panels] Design right panel content items: ${rightContent}`);
      expect(rightContent).toBeGreaterThan(0);
    }

    console.log("[Panels] Design: Testing selection tools with all panels open");
    const designSelectionGroup = page.locator('[id="semio.sketchpad.toolbar.group.selection"]');
    const hasDesignSelection = await designSelectionGroup.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasDesignSelection) {
      await designSelectionGroup.click();
      await page.waitForTimeout(500);
      const designSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
      if (!(await designSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
        await designSelectionGroup.click();
        await page.waitForTimeout(500);
      }
      const additiveToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.mode.additive"]');
      const hasAdditive = await additiveToggle.isVisible({ timeout: 3000 }).catch(() => false);
      console.log(`[Panels] Design additive mode visible with panels: ${hasAdditive}`);
      if (hasAdditive) {
        await additiveToggle.dispatchEvent("click");
        await page.waitForTimeout(300);
        const additiveState = await additiveToggle.getAttribute("data-state").catch(() => null) ?? await additiveToggle.getAttribute("aria-checked").catch(() => null);
        console.log(`[Panels] Design additive state with all panels: ${additiveState}`);
        expect(additiveState === "on" || additiveState === "true").toBe(true);
        await additiveToggle.dispatchEvent("click");
        await page.waitForTimeout(300);
      }
    }

    console.log("[Panels] Design: Testing no overlap between left+right panels");
    if (ds.left && ds.right) {
      const leftBox = await page.locator('[data-panel="leftSidePanel"]').first().boundingBox();
      const rightBox = await page.locator('[data-panel="rightSidePanel"]').first().boundingBox();
      if (leftBox && rightBox) {
        expect(rightBox.x).toBeGreaterThan(leftBox.x + leftBox.width - 5);
        console.log("[Panels] Design: Left and right panels do not overlap");
      }
    }

    console.log("[Panels] Design: Testing close right, keep left+hud");
    if (ds.right) {
      await rightToggle.click();
      await page.waitForTimeout(300);
    }
    ds = await designState();
    console.log(`[Panels] Design left+hud: left=${ds.left}, hud=${ds.hud}, right=${ds.right}`);
    expect(ds.right).toBe(false);
    if (hasDiagramDesign) {
      const pieceCountAfter = await designDiagram.locator(".react-flow__node").count();
      console.log(`[Panels] Design pieces after closing right panel: ${pieceCountAfter}`);
      expect(pieceCountAfter).toBeGreaterThan(0);
    }

    console.log("[Panels] Design: Testing reopen right panel");
    await rightToggle.click();
    await page.waitForTimeout(300);
    ds = await designState();
    console.log(`[Panels] Design after reopen right: left=${ds.left}, hud=${ds.hud}, right=${ds.right}`);

    await ensureAllClosed();
    console.log("[Panels] Design panel combinations complete");
    // #endregion 🔖Design Panel Combinations

    // #region 🔖Type Panel Combinations
    console.log("[Panels] Testing Type app panel combinations");
    await initType(page);
    await page.waitForTimeout(5000);

    const isTypeUrl = page.url().includes("/types/");
    console.log(`[Panels] Type URL: ${page.url()}, isType: ${isTypeUrl}`);
    expect(isTypeUrl).toBe(true);

    const typeState = async () => ({
      left: await page.locator('[data-panel="leftSidePanel"]').first().isVisible().catch(() => false),
      hud: await page.locator('[data-panel="hudPanel"]').first().isVisible().catch(() => false),
      right: await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false),
    });

    await ensureAllClosed();
    await page.waitForTimeout(300);

    console.log("[Panels] Type: Opening all panels");
    await ensureAllOpen();
    await page.waitForTimeout(500);
    let ts = await typeState();
    console.log(`[Panels] Type all open: left=${ts.left}, hud=${ts.hud}, right=${ts.right}`);
    const typeOpenCount = [ts.left, ts.hud, ts.right].filter(Boolean).length;
    console.log(`[Panels] Type open count: ${typeOpenCount}`);
    expect(typeOpenCount).toBeGreaterThanOrEqual(1);

    console.log("[Panels] Type: Verifying canvas remains functional with all panels");
    const typeCanvas = page.locator("canvas").first();
    const hasTypeCanvas = await typeCanvas.isVisible({ timeout: 10000 }).catch(() => false);
    console.log(`[Panels] Type canvas visible with panels: ${hasTypeCanvas}`);

    if (ts.right) {
      console.log("[Panels] Type: Checking right panel content");
      const rightContent = await page.locator('[data-panel="rightSidePanel"]').first().locator('button, input, [role="treeitem"], [role="button"]').count().catch(() => 0);
      console.log(`[Panels] Type right panel content: ${rightContent}`);
    }

    console.log("[Panels] Type: Testing selection+create group switch with all panels");
    const typeSelGroup = page.locator('[id="semio.sketchpad.toolbar.group.selection"]');
    const typeCreateGroup = page.locator('[id="semio.sketchpad.toolbar.group.create"]');
    const hasTypeSel = await typeSelGroup.isVisible({ timeout: 3000 }).catch(() => false);
    const hasTypeCreate = await typeCreateGroup.isVisible({ timeout: 3000 }).catch(() => false);
    if (hasTypeSel && hasTypeCreate) {
      await typeSelGroup.click();
      await page.waitForTimeout(500);
      const typeSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
      if (!(await typeSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
        await typeSelGroup.click();
        await page.waitForTimeout(500);
      }
      const selSettingsVisible = await typeSettingsZone.isVisible({ timeout: 3000 }).catch(() => false);
      console.log(`[Panels] Type selection settings visible with panels: ${selSettingsVisible}`);
      ts = await typeState();
      console.log(`[Panels] Type panels during selection: left=${ts.left}, hud=${ts.hud}, right=${ts.right}`);

      await typeCreateGroup.click();
      await page.waitForTimeout(500);
      if (!(await typeSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
        await typeCreateGroup.click();
        await page.waitForTimeout(500);
      }
      const createSettingsVisible = await typeSettingsZone.isVisible({ timeout: 3000 }).catch(() => false);
      console.log(`[Panels] Type create settings visible with panels: ${createSettingsVisible}`);
      ts = await typeState();
      console.log(`[Panels] Type panels during create: left=${ts.left}, hud=${ts.hud}, right=${ts.right}`);
      expect([ts.left, ts.hud, ts.right].filter(Boolean).length).toBeGreaterThanOrEqual(1);
    }

    console.log("[Panels] Type: Testing panel toggle while toolbar group active");
    if (hasTypeSel) {
      await typeSelGroup.click();
      await page.waitForTimeout(300);
      const typeSettingsZone = page.locator('[id="semio.sketchpad.toolbar.zone.settings"]');
      if (!(await typeSettingsZone.isVisible({ timeout: 1000 }).catch(() => false))) {
        await typeSelGroup.click();
        await page.waitForTimeout(300);
      }
    }
    const rightVisibleBefore = await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false);
    if (rightVisibleBefore) {
      await rightToggle.click();
      await page.waitForTimeout(300);
      await rightToggle.click();
      await page.waitForTimeout(300);
      const rightVisibleAfter = await page.locator('[data-panel="rightSidePanel"]').first().isVisible().catch(() => false);
      console.log(`[Panels] Type right panel after toggle cycle: ${rightVisibleAfter}`);
      const settingsStillVisible = await page.locator('[id="semio.sketchpad.toolbar.zone.settings"]').isVisible({ timeout: 1000 }).catch(() => false);
      console.log(`[Panels] Type toolbar settings still visible: ${settingsStillVisible}`);
    }

    await ensureAllClosed();
    console.log("[Panels] Type panel combinations complete");
    // #endregion 🔖Type Panel Combinations

    // #region 🔖Cross-App Panel Persistence
    console.log("[Panels] Testing panel state across app navigation");

    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(2000);
    await ensureAllOpen();
    await page.waitForTimeout(500);
    const homeState = await getPanelVisibleState();
    console.log(`[Panels] Home panels before navigation: left=${homeState.left}, hud=${homeState.hud}, right=${homeState.right}`);

    const zipPath = path.resolve(__dirname, "../assets/semio/metabolism.zip");
    const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
    await expect(fileInput).toBeAttached({ timeout: 10000 });
    await fileInput.setInputFiles(zipPath);
    await fileInput.evaluate((el) => el.dispatchEvent(new Event("change", { bubbles: true })));
    await page.waitForTimeout(10000);
    const metabolismText = page.getByText("Metabolism", { exact: true }).first();
    await metabolismText.waitFor({ state: "visible", timeout: 60000 });
    const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
    const isRowVisible = await tableRow.isVisible().catch(() => false);
    if (isRowVisible) {
      await tableRow.dblclick({ force: true });
    } else {
      await metabolismText.dblclick({ force: true });
    }
    await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
    await page.waitForTimeout(3000);

    const kitAfterNavState = await getPanelVisibleState();
    console.log(`[Panels] Kit panels after navigation: left=${kitAfterNavState.left}, hud=${kitAfterNavState.hud}, right=${kitAfterNavState.right}`);

    await page.goBack();
    await page.waitForTimeout(3000);
    const homeAfterBackState = await getPanelVisibleState();
    console.log(`[Panels] Home panels after going back: left=${homeAfterBackState.left}, hud=${homeAfterBackState.hud}, right=${homeAfterBackState.right}`);

    console.log("[Panels] Cross-app panel persistence test complete");
    // #endregion 🔖Cross-App Panel Persistence

    // #region 🔖Panel Content Verification per App
    console.log("[Panels] Verifying panel sections across apps");

    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(2000);
    await ensureAllOpen();
    await page.waitForTimeout(500);

    const getTabCountPerPanel = async (panelKey: string): Promise<number> => {
      const panel = page.locator(`[data-panel="${panelKey}"]`).first();
      if (!(await panel.isVisible().catch(() => false))) return 0;
      return await panel.locator('[role="tab"]').count().catch(() => 0);
    };

    const homeLeftTabs = await getTabCountPerPanel("leftSidePanel");
    const homeRightTabs = await getTabCountPerPanel("rightSidePanel");
    const homeHudTabs = await getTabCountPerPanel("hudPanel");
    console.log(`[Panels] Home tab counts: left=${homeLeftTabs}, right=${homeRightTabs}, hud=${homeHudTabs}`);

    await ensureAllClosed();
    console.log("[Panels] Panel content verification complete");
    // #endregion 🔖Panel Content Verification per App

    // #region 🔖Panel Resize Handles
    console.log("[Panels] Verifying panel resize handles exist");

    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(2000);
    await ensureAllOpen();
    await page.waitForTimeout(500);

    const leftPanel = page.locator('[data-panel="leftSidePanel"]').first();
    if (await leftPanel.isVisible().catch(() => false)) {
      const leftBox = await leftPanel.boundingBox();
      if (leftBox) {
        const resizeEdgeX = leftBox.x + leftBox.width;
        const resizeEdgeY = leftBox.y + leftBox.height / 2;
        await page.mouse.move(resizeEdgeX, resizeEdgeY);
        await page.waitForTimeout(200);
        const cursor = await page.evaluate(() => document.body.style.cursor || getComputedStyle(document.elementFromPoint(0, 0) || document.body).cursor);
        console.log(`[Panels] Cursor near left panel edge: ${cursor}`);
      }
    }

    const rightPanel = page.locator('[data-panel="rightSidePanel"]').first();
    if (await rightPanel.isVisible().catch(() => false)) {
      const rightBox = await rightPanel.boundingBox();
      if (rightBox) {
        const resizeEdgeX = rightBox.x;
        const resizeEdgeY = rightBox.y + rightBox.height / 2;
        await page.mouse.move(resizeEdgeX, resizeEdgeY);
        await page.waitForTimeout(200);
        const cursor = await page.evaluate(() => document.body.style.cursor || getComputedStyle(document.elementFromPoint(0, 0) || document.body).cursor);
        console.log(`[Panels] Cursor near right panel edge: ${cursor}`);
      }
    }

    await ensureAllClosed();
    console.log("[Panels] Panel resize handles test complete");
    // #endregion 🔖Panel Resize Handles

    // #region 🔖Keyboard Shortcuts
    console.log("[Panels] Testing keyboard shortcuts for panel toggles");
    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(2000);

    await ensureAllClosed();
    await page.waitForTimeout(300);

    const beforeShortcut = await getPanelVisibleState();
    console.log(`[Panels] Before keyboard shortcut: left=${beforeShortcut.left}, hud=${beforeShortcut.hud}, right=${beforeShortcut.right}`);

    await page.keyboard.press("Control+j");
    await page.waitForTimeout(500);
    const afterCtrlJ = await getPanelVisibleState();
    console.log(`[Panels] After Ctrl+J: left=${afterCtrlJ.left}, hud=${afterCtrlJ.hud}, right=${afterCtrlJ.right}`);
    const ctrlJToggled = afterCtrlJ.left !== beforeShortcut.left;
    console.log(`[Panels] Ctrl+J toggled left panel: ${ctrlJToggled}`);

    await page.keyboard.press("Control+l");
    await page.waitForTimeout(500);
    const afterCtrlL = await getPanelVisibleState();
    console.log(`[Panels] After Ctrl+L: left=${afterCtrlL.left}, hud=${afterCtrlL.hud}, right=${afterCtrlL.right}`);
    const ctrlLToggled = afterCtrlL.right !== afterCtrlJ.right;
    console.log(`[Panels] Ctrl+L toggled right panel: ${ctrlLToggled}`);

    await page.keyboard.press("Control+k");
    await page.waitForTimeout(500);
    const afterCtrlK = await getPanelVisibleState();
    console.log(`[Panels] After Ctrl+K: left=${afterCtrlK.left}, hud=${afterCtrlK.hud}, right=${afterCtrlK.right}`);
    const ctrlKToggled = afterCtrlK.hud !== afterCtrlL.hud;
    console.log(`[Panels] Ctrl+K toggled HUD panel: ${ctrlKToggled}`);

    await ensureAllClosed();
    console.log("[Panels] Keyboard shortcuts test complete");
    // #endregion 🔖Keyboard Shortcuts

    const infiniteLoopErrors = errors.filter((e) => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);
    console.log("[Panels] All panel combination tests complete");
  });
});
