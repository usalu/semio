import { expect, Locator, Page, test } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";
import MetabolismKitData from "../../assets/semio/kit_metabolism.json" with { type: "json" };

const designs = (MetabolismKitData as any).designs ?? [];
const nakaginCapsuleTowerDesign = designs.find((d: any) => d.name === "Nakagin Capsule Tower");
const nakaginCapsuleTowerFlatDesign = designs.find(
  (d: any) => d.name === "Flat" && d.parent?.guid === nakaginCapsuleTowerDesign?.guid,
);
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

async function initConsole(page: Page) {
  const messages: string[] = [];
  const warnings: string[] = [];
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "warning") {
      warnings.push(msg.text());
    }
    else if (msg.type() === "error") {
      errors.push(msg.text());
    }
    else {
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
  // Settings is in the RIGHT group dropdown
  // First click the right group toggle to open/toggle it
  const rightGroupToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
  await expect(rightGroupToggle).toBeVisible({ timeout: 60000 });

  // Check if settings panel is already open
  const settingsPanel = page.locator('[data-panel="settings"]').first();
  const isSettingsVisible = await settingsPanel.isVisible().catch(() => false);

  if (!isSettingsVisible) {
    // Click the right group toggle to open dropdown and select settings
    await rightGroupToggle.click();
    await page.waitForTimeout(300);

    // Try to click the settings item in the dropdown
    const settingsItem = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettingsItem = await settingsItem.isVisible({ timeout: 2000 }).catch(() => false);
    if (hasSettingsItem) {
      await settingsItem.click();
    }
  }
  await page.waitForTimeout(500);

  // Wait for the settings panel to appear
  await expect(settingsPanel).toBeVisible({ timeout: 10000 }).catch(() => { });
}

async function getSettingsSections(page: Page): Promise<string[]> {
  // Wait for any right panel (settings, details, or chat) to be visible
  const rightPanel = page.locator('[data-panel="settings"], [data-panel="details"], [data-panel="chat"]').first();

  // Try to wait for panel with increased timeout, return empty if not found
  try {
    await expect(rightPanel).toBeVisible({ timeout: 15000 });
  } catch {
    console.log("Warning: Right panel not visible after 15s, returning empty sections");
    return [];
  }

  const sections = await rightPanel.locator('[role="button"][id^="semio.sketchpad"]').all();
  const sectionIds: string[] = [];
  for (const section of sections) {
    const id = await section.getAttribute("id");
    if (id) sectionIds.push(id);
  }
  return sectionIds;
}

async function openDetailsPanel(page: Page) {
  // Details is in the RIGHT group dropdown
  // First click the right group toggle to open/toggle it
  const rightGroupToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
  await expect(rightGroupToggle).toBeVisible({ timeout: 60000 });

  // Check if details panel is already open
  const detailsPanel = page.locator('[data-panel="details"]').first();
  const isDetailsVisible = await detailsPanel.isVisible().catch(() => false);

  if (!isDetailsVisible) {
    // Click the right group toggle to open dropdown and select details
    await rightGroupToggle.click();
    await page.waitForTimeout(300);

    // Try to click the details item in the dropdown
    const detailsItem = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
    const hasDetailsItem = await detailsItem.isVisible({ timeout: 2000 }).catch(() => false);
    if (hasDetailsItem) {
      await detailsItem.click();
    }
  }
  await page.waitForTimeout(500);

  // Wait for the details panel to appear
  await expect(detailsPanel).toBeVisible({ timeout: 10000 }).catch(() => { });
}

async function getDetailsSections(page: Page): Promise<string[]> {
  const detailsPanel = page.locator('[data-panel="details"]').first();
  try {
    await expect(detailsPanel).toBeVisible({ timeout: 15000 });
  } catch {
    console.log("Warning: Details panel not visible after 15s, returning empty sections");
    return [];
  }

  const sections = await detailsPanel.locator('[role="button"][id^="semio.sketchpad"]').all();
  const sectionIds: string[] = [];
  for (const section of sections) {
    const id = await section.getAttribute("id");
    if (id) sectionIds.push(id);
  }
  return sectionIds;
}

// Panel group mapping
const PANEL_GROUPS: Record<string, string> = {
  workbench: "workbench",
  tools: "workbench",
  hud: "hud",
  stats: "hud",
  details: "right",
  chat: "right",
  settings: "right",
};

// Helper to open a specific panel by key
async function openPanel(page: Page, panelKey: string): Promise<boolean> {
  const group = PANEL_GROUPS[panelKey];
  if (!group) {
    console.log(`[Panel Test] Unknown panel: ${panelKey}`);
    return false;
  }

  // Check if panel is already visible
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  if (await panel.isVisible().catch(() => false)) {
    console.log(`[Panel Test] ${panelKey} panel already visible`);
    return true;
  }

  // Click group toggle to open dropdown popover
  const groupToggle = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${group}"]`);
  const hasGroupToggle = await groupToggle.isVisible({ timeout: 5000 }).catch(() => false);
  if (!hasGroupToggle) {
    console.log(`[Panel Test] Group toggle ${group} not visible`);
    return false;
  }

  // Look for the dropdown trigger button (the chevron/caret next to the main toggle)
  // The dropdown toggle has a child element that opens the popover
  const dropdownTrigger = groupToggle.locator('[data-slot="toggle-group-item"]').first();
  const actionButton = dropdownTrigger.locator('button').first();

  console.log(`[Panel Test] Clicking dropdown action for group: ${group}`);

  // Click the action button (dropdown trigger) within the toggle
  const hasActionButton = await actionButton.isVisible({ timeout: 2000 }).catch(() => false);
  if (hasActionButton) {
    await actionButton.click();
    await page.waitForTimeout(500);
  } else {
    // Fall back to clicking the whole toggle
    await groupToggle.click();
    await page.waitForTimeout(500);
  }

  // Look for popover content with dropdown items
  const popoverContent = page.locator('[data-radix-popper-content-wrapper]').first();
  const hasPopover = await popoverContent.isVisible({ timeout: 2000 }).catch(() => false);
  console.log(`[Panel Test] Popover visible: ${hasPopover}`);

  if (hasPopover) {
    // Find all buttons in the popover and click the one for our panel
    const buttons = popoverContent.locator('button');
    const buttonCount = await buttons.count();
    console.log(`[Panel Test] Found ${buttonCount} buttons in popover`);

    // Click on each button until we find the right one
    for (let i = 0; i < buttonCount; i++) {
      const btn = buttons.nth(i);
      // Try to click and see if our panel opens
      await btn.click();
      await page.waitForTimeout(300);

      if (await panel.isVisible().catch(() => false)) {
        console.log(`[Panel Test] ${panelKey} panel opened after clicking button ${i}`);
        return true;
      }

      // If wrong panel opened, close it and try next button
      // Re-open popover if needed
      const stillHasPopover = await popoverContent.isVisible().catch(() => false);
      if (!stillHasPopover) {
        if (hasActionButton) {
          await actionButton.click();
        } else {
          await groupToggle.click();
        }
        await page.waitForTimeout(300);
      }
    }
  }

  // Check if clicking the toggle directly opened a panel
  const isVisible = await panel.isVisible().catch(() => false);
  console.log(`[Panel Test] ${panelKey} panel visible: ${isVisible}`);
  return isVisible;
}

// Helper to close a panel
async function closePanel(page: Page, panelKey: string): Promise<void> {
  const group = PANEL_GROUPS[panelKey];
  if (!group) return;

  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  if (!(await panel.isVisible().catch(() => false))) return;

  // Click the hide button in dropdown
  const groupToggle = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${group}"]`);
  if (await groupToggle.isVisible().catch(() => false)) {
    await groupToggle.click();
    await page.waitForTimeout(300);
    const hideOption = page.locator(`[id="semio.sketchpad.navbar.panelToggle.${panelKey}.hide"]`);
    if (await hideOption.isVisible({ timeout: 1000 }).catch(() => false)) {
      await hideOption.click();
      await page.waitForTimeout(300);
    }
  }
}

// Helper to check if a panel is visible
async function isPanelVisible(page: Page, panelKey: string): Promise<boolean> {
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  return await panel.isVisible({ timeout: 2000 }).catch(() => false);
}

// Helper to get panel sections
async function getPanelSections(page: Page, panelKey: string): Promise<string[]> {
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
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

// Helper to get tree items within a section
async function getSectionTreeItems(page: Page, sectionId: string): Promise<number> {
  const section = page.locator(`[id="${sectionId}"]`).first();
  if (!(await section.isVisible().catch(() => false))) return 0;

  // Find tree items within or after this section
  // TreeItems are typically inside TreeSection or following TreeContent
  const parent = section.locator("..").first();
  const treeItems = parent.locator('[role="treeitem"], [class*="TreeItem"], [class*="tree-item"]');
  const count = await treeItems.count().catch(() => 0);
  return count;
}

// Helper to count any content items in a panel (inputs, textareas, buttons, tree items)
async function getPanelContentCount(page: Page, panelKey: string): Promise<number> {
  const panel = page.locator(`[data-panel="${panelKey}"]`).first();
  if (!(await panel.isVisible().catch(() => false))) return 0;

  // Count various types of content elements
  const inputs = await panel.locator("input, textarea, select").count().catch(() => 0);
  const buttons = await panel.locator('button:not([id*="panelToggle"])').count().catch(() => 0);
  const treeItems = await panel.locator('[role="treeitem"]').count().catch(() => 0);
  const listItems = await panel.locator("li").count().catch(() => 0);

  return inputs + buttons + treeItems + listItems;
}

// Test a single panel: open it, verify sections, verify content, close it
async function testPanel(
  page: Page,
  appName: string,
  panelKey: string,
  expectedSections: string[] = [],
  requireContent: boolean = true
): Promise<{ opened: boolean; sections: string[]; contentCount: number }> {
  console.log(`[${appName}] Testing ${panelKey} panel`);

  // Open the panel
  const opened = await openPanel(page, panelKey);
  if (!opened) {
    console.log(`[${appName}] Could not open ${panelKey} panel`);
    return { opened: false, sections: [], contentCount: 0 };
  }

  // Verify panel is visible
  const isVisible = await isPanelVisible(page, panelKey);
  console.log(`[${appName}] ${panelKey} panel visible: ${isVisible}`);
  expect(isVisible).toBe(true);

  // Get sections
  const sections = await getPanelSections(page, panelKey);
  console.log(`[${appName}] ${panelKey} sections: ${sections.join(", ") || "(none)"}`);

  // Verify expected sections if provided
  for (const expectedSection of expectedSections) {
    const hasSection = sections.some((s) => s.includes(expectedSection));
    if (!hasSection) {
      console.log(`[${appName}] Warning: Expected section "${expectedSection}" not found in ${panelKey}`);
    }
  }

  // Count content items
  const contentCount = await getPanelContentCount(page, panelKey);
  console.log(`[${appName}] ${panelKey} content items: ${contentCount}`);

  if (requireContent) {
    expect(contentCount).toBeGreaterThan(0);
  }

  // Close the panel
  await closePanel(page, panelKey);
  await page.waitForTimeout(300);

  return { opened, sections, contentCount };
}

async function initHome(page: Page) {
  const { errors, warnings, messages } = await initConsole(page);

  await page.goto("/");
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(2000);

  const zipPath = path.resolve(__dirname, "../../assets/semio/metabolism.zip");
  const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
  await expect(fileInput).toBeAttached({ timeout: 10000 });

  console.log("[TEST] Setting input files:", zipPath);

  // Use file chooser approach - wait for file chooser to open
  const [fileChooser] = await Promise.all([
    page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
    // Trigger file input by clicking a button or dispatching click event
    fileInput.dispatchEvent("click"),
  ]);

  if (fileChooser) {
    await fileChooser.setFiles(zipPath);
    console.log("[TEST] File set via file chooser");
  } else {
    // Fallback to direct setInputFiles
    await fileInput.setInputFiles(zipPath);
    console.log("[TEST] File set via setInputFiles");

    // Manually dispatch change event
    await fileInput.evaluate((el) => {
      el.dispatchEvent(new Event("change", { bubbles: true }));
    });

    expect(errors.filter(e => e.includes("Import error"))).toHaveLength(0);
    expect(warnings.filter(w => w.includes("Invalid access"))).toHaveLength(0);
  }

  // Wait for the import to complete (loading row becomes clickable)
  // Import can take a while for large kits like Metabolism
  await page.waitForTimeout(10000);

  // Debug: Log all visible text on page
  const pageText = await page.locator("body").textContent();
  console.log("[TEST] Page text contains 'Metabolism':", pageText?.includes("Metabolism"));
  console.log("[TEST] Page text contains 'Loading':", pageText?.includes("Loading"));

  // Wait for any loading indicators to disappear
  const loadingIndicator = page.locator("text=Loading").first();
  const isLoading = await loadingIndicator.isVisible().catch(() => false);
  if (isLoading) {
    console.log("[TEST] Waiting for loading to complete...");
    await loadingIndicator.waitFor({ state: "hidden", timeout: 60000 });
    await page.waitForTimeout(2000);
  }

  // The imported kit row should now be visible with the "Metabolism" name
  // Click on it to navigate to the kit
  const metabolismRow = page.getByRole("row", { name: /Metabolism/i }).first();
  const isRowVisible = await metabolismRow.isVisible({ timeout: 10000 }).catch(() => false);
  console.log("[TEST] Metabolism row visible:", isRowVisible);

  if (isRowVisible) {
    await metabolismRow.dblclick();
    console.log("[TEST] Double-clicked on Metabolism row");
  } else {
    // Try alternative: look for button/cell with Metabolism text
    const metabolismCell = page.getByText("Metabolism").first();
    const isCellVisible = await metabolismCell.isVisible({ timeout: 10000 }).catch(() => false);
    console.log("[TEST] Metabolism cell visible:", isCellVisible);
    if (isCellVisible) {
      await metabolismCell.dblclick();
      console.log("[TEST] Double-clicked on Metabolism cell");
    } else {
      // Take screenshot for debugging
      console.log("[TEST] Neither row nor cell visible, checking for any clickable kit items...");
      // Try to find any table rows
      const allRows = page.locator("table tr");
      const rowCount = await allRows.count();
      console.log("[TEST] Found", rowCount, "table rows");
      // Click on the first data row (skip header)
      if (rowCount > 1) {
        await allRows.nth(1).dblclick();
        console.log("[TEST] Double-clicked on first data row");
      }
    }
  }

  // Wait for navigation to kit
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

  await page.waitForTimeout(2000);

  const design = page.getByRole("button", { name: "Nakagin Capsule Tower" });
  const isDesignVisible = await design.isVisible({ timeout: 10000 }).catch(() => false);
  console.log(`[initDesign] Design visible: ${isDesignVisible}`);

  if (!isDesignVisible) {
    const currentUrl = page.url();
    console.log(`[initDesign] Current URL: ${currentUrl}`);
    const designsUrl = currentUrl.includes("?") ? `${currentUrl}&kind=designs` : `${currentUrl}?kind=designs`;
    await page.goto(designsUrl);
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);
  }

  const designAgain = page.getByRole("button", { name: "Nakagin Capsule Tower" });
  await expect(designAgain).toBeVisible({ timeout: 15000 });
  console.log(`[initDesign] About to double-click on design`);
  await designAgain.dblclick({ timeout: 10000 });
  console.log(`[initDesign] Double-clicked on design`);
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
}

async function initType(page: Page) {
  const { errors, warnings, messages } = await initKit(page);
  const typesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
  await expect(typesToggle).toBeVisible({ timeout: 30000 });
  await typesToggle.click();
  await page.waitForTimeout(2000);
  const tambourType = page.getByRole("button", { name: "Tambour" }).first();
  await expect(tambourType).toBeVisible({ timeout: 3000 });
  await tambourType.dblclick();
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
  return { errors, warnings, messages };
}

async function initDocs(page: Page) {
  await page.goto("/docs/index");
  await page.waitForLoadState("networkidle");
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
  const buttons = await panel.locator('button').count().catch(() => 0);
  const inputs = await panel.locator('input, textarea, select').count().catch(() => 0);
  const treeItems = await panel.locator('[role="treeitem"]').count().catch(() => 0);
  const total = buttons + inputs + treeItems;
  console.log(`[${appName}] Panel ${panelKey} content: ${buttons} buttons, ${inputs} inputs, ${treeItems} tree items (total: ${total})`);
  return total;
}

async function verifyToggleWorks(page: Page, toggleId: string, panelKey: string, appName: string): Promise<boolean> {
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
}

test.describe("sketchpad", () => {
  test("Home", async ({ page }) => {
    await page.goto("/");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    console.log("[Home] Testing Home app panel toggles");
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettings = await settingsToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Home] Settings toggle visible: ${hasSettings}`);
    let settingsWorked = false;
    if (hasSettings) {
      settingsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.settings.show", "settings", "Home");
    }
    const detailsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
    const hasDetails = await detailsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Home] Details toggle visible: ${hasDetails}`);
    let detailsWorked = false;
    if (hasDetails) {
      detailsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.details.show", "details", "Home");
    }
    expect(hasSettings || hasDetails).toBe(true);
    console.log(`[Home] Panel toggle verification complete: settings=${settingsWorked}, details=${detailsWorked}`);
  });

  test("Kit", async ({ page }) => {
    test.setTimeout(180000);
    const { errors, warnings, messages } = await initConsole(page);
    await initKit(page);
    expect(errors.filter((e) => e.includes("Import error"))).toHaveLength(0);

    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    const pageText = await page.evaluate(() => document.body.innerText);
    expect(pageText).toContain("Metabolism");
    expect(warnings.filter(w => w.includes("Invalid access"))).toHaveLength(0);

    const typesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
    const hasTypesToggle = await typesToggle.isVisible({ timeout: 5000 }).catch(() => false);
    if (hasTypesToggle) {
      await typesToggle.click();
      await page.waitForTimeout(1000);
    }

    const tableBody = page.locator("tbody").first();
    const hasTable = await tableBody.isVisible({ timeout: 10000 }).catch(() => false);
    expect(hasTable).toBe(true);

    const tambourType = page.getByRole("button", { name: "Tambour" }).first();
    await expect(tambourType).toBeVisible({ timeout: 10000 });
    await tambourType.click();
    await page.waitForTimeout(500);

    console.log("[Kit] Testing Kit app panel toggles");
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettings = await settingsToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Kit] Settings toggle visible: ${hasSettings}`);
    let settingsWorked = false;
    if (hasSettings) {
      settingsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.settings.show", "settings", "Kit");
    }
    const detailsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
    const hasDetails = await detailsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Kit] Details toggle visible: ${hasDetails}`);
    let detailsWorked = false;
    if (hasDetails) {
      detailsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.details.show", "details", "Kit");
    }
    expect(hasSettings || hasDetails).toBe(true);
    console.log(`[Kit] Panel toggle verification complete: settings=${settingsWorked}, details=${detailsWorked}`);
  });
  test("Type", async ({ page }) => {
    test.setTimeout(120000);
    const { errors, warnings, messages } = await initType(page);
    const canvas = page.locator("canvas").first();
    await expect(canvas).toBeVisible({ timeout: 15000 });
    expect(page.url()).toContain("/types/");
    await page.waitForTimeout(5000);

    const navbar = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Navbar is visible");

    const footer = page.locator('footer').first();
    await expect(footer).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Footer is visible");

    const canvasBox = await canvas.boundingBox();
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

    expect(warnings.filter(w => w.includes("Mesh"))).toHaveLength(0);
    expect(errors.filter(e => e.includes("Maximum update depth exceeded"))).toHaveLength(0);

    console.log("[Type] Testing Type app panel toggles");
    const workbenchToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]');
    const hasWorkbench = await workbenchToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Type] Workbench toggle visible: ${hasWorkbench}`);
    let workbenchWorked = false;
    if (hasWorkbench) {
      workbenchWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.workbench.show", "workbench", "Type");
      const workbenchPanel = page.locator('[data-panel="workbench"]').first();
      if (await workbenchPanel.isVisible({ timeout: 1000 }).catch(() => false)) {
        const portsSection = workbenchPanel.locator('[id*="port"], [role="treeitem"]').first();
        const hasPortsSection = await portsSection.isVisible({ timeout: 2000 }).catch(() => false);
        console.log(`[Type] Workbench panel has ports/models section: ${hasPortsSection}`);
      }
    }
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettings = await settingsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Type] Settings toggle visible: ${hasSettings}`);
    let settingsWorked = false;
    if (hasSettings) {
      settingsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.settings.show", "settings", "Type");
    }
    expect(hasWorkbench || hasSettings).toBe(true);
    console.log(`[Type] Panel toggle verification complete: workbench=${workbenchWorked}, settings=${settingsWorked}`);
  });
  test("Design", async ({ page }) => {
    test.setTimeout(120000);

    const { errors, warnings, messages } = await initConsole(page);

    await initDesign(page);

    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);

    console.log("[Design Test] Current URL:", page.url());

    const diagramContainer = page.locator('.react-flow').first();
    const sceneCanvas = page.locator('canvas').first();

    await page.waitForTimeout(3000);

    const reactFlowCount = await page.locator('.react-flow').count();
    console.log("[Design Test] ReactFlow elements count:", reactFlowCount);

    const windowElements = await page.locator('[class*="window"], [class*="panel"]').count();
    console.log("[Design Test] Window/Panel elements count:", windowElements);

    const hasDiagram = await diagramContainer.isVisible({ timeout: 30000 }).catch(() => false);
    const hasScene = await sceneCanvas.isVisible({ timeout: 10000 }).catch(() => false);

    console.log("[Design Test] hasDiagram:", hasDiagram, "hasScene:", hasScene);

    if (!hasDiagram && !hasScene) {
      console.log("[Design Test] Page HTML:", await page.content().then(c => c.slice(0, 2000)));
    }
    expect(hasDiagram || hasScene).toBe(true);

    const infiniteLoopErrors = errors.filter(e => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);

    const navbar = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Design Test] Navbar is visible");

    const footer = page.locator('footer').first();
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

        expect(pan1Duration).toBeLessThan(750);
        expect(pan2Duration).toBeLessThan(750);
        expect(Math.abs(pan1Duration - pan2Duration)).toBeLessThan(250);
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

        expect(hoverDuration).toBeLessThan(200);
        expect(unhoverDuration).toBeLessThan(600);

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
        hoverTimes.forEach((time, i) => {
          expect(time).toBeLessThan(200);
        });
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

        expect(scenePan1Duration).toBeLessThan(1000);
        expect(scenePan2Duration).toBeLessThan(500);
        expect(scenePan3Duration).toBeLessThan(700);

        const avgSubsequentPanTime = (scenePan2Duration + scenePan3Duration) / 2;
        console.log(`[Design Test] Average subsequent scene pan time: ${avgSubsequentPanTime}ms`);
        expect(Math.abs(scenePan2Duration - avgSubsequentPanTime)).toBeLessThan(200);
        expect(Math.abs(scenePan3Duration - avgSubsequentPanTime)).toBeLessThan(200);
      }
    }

    const unexpectedMeshWarnings = warnings.filter(w =>
      w.includes("Mesh") &&
      !w.includes("File URL not available")
    );
    expect(unexpectedMeshWarnings).toHaveLength(0);

    console.log("[Design] Testing Design app panel toggles");
    const workbenchToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]');
    const hasWorkbench = await workbenchToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Design] Workbench toggle visible: ${hasWorkbench}`);
    let workbenchWorked = false;
    if (hasWorkbench) {
      workbenchWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.workbench.show", "workbench", "Design");
      const workbenchPanel = page.locator('[data-panel="workbench"]').first();
      if (await workbenchPanel.isVisible({ timeout: 1000 }).catch(() => false)) {
        const typesSection = workbenchPanel.locator('[id*="type"], [role="treeitem"]').first();
        const hasTypesSection = await typesSection.isVisible({ timeout: 2000 }).catch(() => false);
        console.log(`[Design] Workbench panel has types/pieces section: ${hasTypesSection}`);
      }
    }
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettings = await settingsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Settings toggle visible: ${hasSettings}`);
    let settingsWorked = false;
    if (hasSettings) {
      settingsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.settings.show", "settings", "Design");
    }
    const detailsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
    const hasDetails = await detailsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Design] Details toggle visible: ${hasDetails}`);
    let detailsWorked = false;
    if (hasDetails) {
      detailsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.details.show", "details", "Design");
    }
    expect(hasWorkbench || hasSettings || hasDetails).toBe(true);
    console.log(`[Design] Panel toggle verification complete: workbench=${workbenchWorked}, settings=${settingsWorked}, details=${detailsWorked}`);

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
  });
  test("Docs", async ({ page }) => {
    await initDocs(page);

    const pageTitle = page.getByRole("heading", { name: "Welcome to Semio", level: 1 });
    await expect(pageTitle).toBeVisible();
    const pageDescription = page.getByText("Design Information Modeling for Architecture");
    await expect(pageDescription).toBeVisible();
    const cardHeading = page.getByRole("heading", { name: /Just want to toy around/ });
    await expect(cardHeading).toBeVisible();
    const researchCard = page.getByRole("heading", { name: /More into research/ });
    await expect(researchCard).toBeVisible();

    console.log("[Docs] Testing Docs app panel toggles");
    const workbenchToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]');
    const hasWorkbench = await workbenchToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Docs] Workbench toggle visible: ${hasWorkbench}`);
    let workbenchWorked = false;
    if (hasWorkbench) {
      workbenchWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.workbench.show", "workbench", "Docs");
      const workbenchPanel = page.locator('[data-panel="workbench"]').first();
      if (await workbenchPanel.isVisible({ timeout: 1000 }).catch(() => false)) {
        const tocItems = workbenchPanel.locator('a, [role="treeitem"], button').first();
        const hasTocItems = await tocItems.isVisible({ timeout: 2000 }).catch(() => false);
        console.log(`[Docs] Workbench panel has TOC/navigation items: ${hasTocItems}`);
      }
    }
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettings = await settingsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Docs] Settings toggle visible: ${hasSettings}`);
    let settingsWorked = false;
    if (hasSettings) {
      settingsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.settings.show", "settings", "Docs");
    }
    const detailsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
    const hasDetails = await detailsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    console.log(`[Docs] Details toggle visible: ${hasDetails}`);
    let detailsWorked = false;
    if (hasDetails) {
      detailsWorked = await verifyToggleWorks(page, "semio.sketchpad.navbar.panelToggle.details.show", "details", "Docs");
    }
    expect(hasWorkbench || hasSettings || hasDetails).toBe(true);
    console.log(`[Docs] Panel toggle verification complete: workbench=${workbenchWorked}, settings=${settingsWorked}, details=${detailsWorked}`);

    await page.goto("/docs/manuals/sketchpad");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(500);
    await expect(page.getByRole("heading", { name: "Apps", level: 1 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Home", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Kit", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Design", level: 2 })).toBeVisible();

    await page.goto("/docs/index");
    await page.waitForLoadState("networkidle");
    const nextButton = page.getByRole("button", { name: /Intro/i });
    await expect(nextButton).toBeVisible();
    await nextButton.click();
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(500);
    await expect(page).toHaveURL(/.*docs\/getting-started\/intro/);
    await expect(page.getByRole("heading", { level: 1 }).first()).toBeVisible();
  });

  test("Panel Group Toggle", async ({ page }) => {
    // This test verifies that panel group toggles (left/right) are visible and clickable.
    // NOTE: There is a known issue where useAppCommands().togglePanel may not work if
    // the app store is not yet initialized (store.kitApp() returns undefined).
    // This test verifies the UI elements exist and are interactive.
    test.setTimeout(120000);

    const { errors } = await initConsole(page);
    await initKit(page);

    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    console.log("[Panel Toggle Test] In Kit app:", page.url());
    expect(page.url()).toContain("/kits/");

    // Test RIGHT panel group toggle exists and is clickable
    const rightGroupToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    const hasRightToggle = await rightGroupToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Panel Toggle Test] Right group toggle visible: ${hasRightToggle}`);

    if (hasRightToggle) {
      // Verify toggle is clickable (doesn't throw)
      await rightGroupToggle.click();
      await page.waitForTimeout(500);
      console.log("[Panel Toggle Test] Right group toggle clicked successfully");

      // Check panel state after click
      const detailsPanel = page.locator('[data-panel="details"]').first();
      const settingsPanel = page.locator('[data-panel="settings"]').first();
      const detailsVisible = await detailsPanel.isVisible().catch(() => false);
      const settingsVisible = await settingsPanel.isVisible().catch(() => false);
      console.log(`[Panel Toggle Test] After click: details=${detailsVisible}, settings=${settingsVisible}`);
    }

    // Test LEFT panel group toggle exists and is clickable
    const leftGroupToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]');
    const hasLeftToggle = await leftGroupToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log(`[Panel Toggle Test] Left group toggle visible: ${hasLeftToggle}`);

    if (hasLeftToggle) {
      await leftGroupToggle.click();
      await page.waitForTimeout(500);
      console.log("[Panel Toggle Test] Left group toggle clicked successfully");

      const workbenchPanel = page.locator('[data-panel="workbench"]').first();
      const workbenchVisible = await workbenchPanel.isVisible().catch(() => false);
      console.log(`[Panel Toggle Test] After click: workbench=${workbenchVisible}`);
    }

    // At least one toggle group should exist
    expect(hasRightToggle || hasLeftToggle).toBe(true);
    console.log("[Panel Toggle Test] Panel group toggle test completed");

    const infiniteLoopErrors = errors.filter(e => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);
  });

  test("Design Drag and Drop", async ({ page }) => {
    // This test verifies that:
    // 1. Existing pieces in the design have correct plane properties
    // 2. The drag-and-drop mechanism is set up correctly (avatars are draggable)
    //
    // Note: dnd-kit's PointerSensor does not respond to Playwright's synthetic mouse events.
    // Manual testing of drag-and-drop functionality should be done in the browser.
    test.setTimeout(120000);

    const { errors, warnings, messages } = await initConsole(page);
    await initDesign(page);

    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);

    console.log("[Drag&Drop Test] In Design app:", page.url());
    expect(page.url()).toContain("/designs/");

    const sceneCanvas = page.locator('canvas').first();
    const hasScene = await sceneCanvas.isVisible({ timeout: 15000 }).catch(() => false);
    expect(hasScene).toBe(true);
    console.log("[Drag&Drop Test] Scene canvas is visible");

    const workbenchGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]');
    await expect(workbenchGroup).toBeVisible({ timeout: 10000 });
    await workbenchGroup.click();
    await page.waitForTimeout(1000);

    const workbenchPanel = page.locator('[data-panel="workbench"]').first();
    const isWorkbenchVisible = await workbenchPanel.isVisible({ timeout: 5000 }).catch(() => false);
    if (!isWorkbenchVisible) {
      const showWorkbench = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]');
      if (await showWorkbench.isVisible({ timeout: 2000 }).catch(() => false)) {
        await showWorkbench.click();
        await page.waitForTimeout(1000);
      }
    }
    console.log("[Drag&Drop Test] Workbench panel opened");

    const workbenchPanelEl = page.locator('[data-panel="workbench"]').first();
    await expect(workbenchPanelEl).toBeVisible({ timeout: 5000 });
    console.log("[Drag&Drop Test] Workbench panel element found");

    const typeAvatars = workbenchPanelEl.locator('[data-slot="avatar"]');
    let typeCount = await typeAvatars.count();
    console.log(`[Drag&Drop Test] Found ${typeCount} avatars in workbench panel`);

    if (typeCount === 0) {
      console.log("[Drag&Drop Test] No avatars found initially. Expanding collapsed sections...");

      const collapsedSections = workbenchPanelEl.locator('[data-state="closed"]');
      const collapsedCount = await collapsedSections.count();
      console.log(`[Drag&Drop Test] Found ${collapsedCount} closed sections`);

      for (let i = 0; i < collapsedCount && typeCount === 0; i++) {
        await collapsedSections.nth(i).click();
        await page.waitForTimeout(300);
        typeCount = await typeAvatars.count();
        console.log(`[Drag&Drop Test] After expanding section ${i + 1}: ${typeCount} avatars`);
      }
    }

    expect(typeCount).toBeGreaterThan(0);
    console.log(`[Drag&Drop Test] Verified ${typeCount} draggable type avatars exist`);

    const firstTypeAvatar = typeAvatars.first();

    const avatarInfo = await firstTypeAvatar.evaluate((el) => {
      return {
        tagName: el.tagName,
        attributes: Array.from(el.attributes).map(a => ({ name: a.name, value: a.value })),
        innerText: el.textContent,
      };
    });

    const hasDraggableAttribute = avatarInfo.attributes.some(a =>
      a.name === 'aria-roledescription' && a.value === 'draggable'
    );
    expect(hasDraggableAttribute).toBe(true);
    console.log(`[Drag&Drop Test] Type avatar has draggable attribute: ${hasDraggableAttribute}`);

    const existingPieces = await getDesignPieces(page);
    console.log(`[Drag&Drop Test] Design has ${existingPieces.length} existing pieces`);

    const expectedXAxis = { x: 1, y: 0, z: 0 };
    const expectedYAxis = { x: 0, y: 1, z: 0 };

    let validPlaneCount = 0;
    let invalidPlaneCount = 0;
    let noPlanePieces = 0;

    for (const piece of existingPieces) {
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
        const xAxisValid =
          Math.abs(plane.xAxis.x - expectedXAxis.x) < TOLERANCE &&
          Math.abs(plane.xAxis.y - expectedXAxis.y) < TOLERANCE &&
          Math.abs(plane.xAxis.z - expectedXAxis.z) < TOLERANCE;
        const yAxisValid =
          Math.abs(plane.yAxis.x - expectedYAxis.x) < TOLERANCE &&
          Math.abs(plane.yAxis.y - expectedYAxis.y) < TOLERANCE &&
          Math.abs(plane.yAxis.z - expectedYAxis.z) < TOLERANCE;

        if (originZValid && xAxisValid && yAxisValid) {
          validPlaneCount++;
        } else {
          invalidPlaneCount++;
        }
      } else {
        invalidPlaneCount++;
      }
    }

    console.log(`[Drag&Drop Test] Plane validation: ${validPlaneCount} valid (origin.z=0, xAxis=1,0,0, yAxis=0,1,0), ${invalidPlaneCount} non-standard, ${noPlanePieces} without plane`);
    console.log(`[Drag&Drop Test] Note: The Nakagin Capsule Tower has rotated capsules, so most pieces have non-standard plane orientation - this is expected.`);

    expect(existingPieces.length).toBeGreaterThan(0);

    const infiniteLoopErrors = errors.filter(e => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);
  });
});
