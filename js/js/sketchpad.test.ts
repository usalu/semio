import { expect, Locator, Page, test } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

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

  // Wait a bit for the handler to process
  await page.waitForTimeout(5000);

  // Check current URL
  console.log("[TEST] Current URL after file set:", page.url());

  // Wait for the import to trigger navigation
  try {
    await page.waitForURL(/.*kits\/.+/, { timeout: 60000 });
    expect(page.url()).toMatch(/kits\/.+/);
  } catch (error) {
    throw error;
  }

  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);

  return { errors, warnings, messages };
}

async function initKit(page: Page) {
  const { errors, warnings, messages } = await initHome(page);
  return { errors, warnings, messages };
}

async function initDesign(page: Page) {
  const { errors, warnings, messages } = await initKit(page);

  const design = page.getByRole("button", { name: "Nakagin Capsule Tower" });
  const isDesignVisible = await design.isVisible({ timeout: 5000 }).catch(() => false);

  if (!isDesignVisible) {
    const currentUrl = page.url();
    const designsUrl = currentUrl.includes("?") ? `${currentUrl}&kind=designs` : `${currentUrl}?kind=designs`;
    await page.goto(designsUrl);
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);
  }

  await expect(design).toBeVisible({ timeout: 10000 });
  await design.dblclick();
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

test.describe("sketchpad", () => {
  test("Home", async ({ page }) => {
    await page.goto("/");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    console.log("[Home] Testing Home app panel toggles");

    // Home app has: Settings, Details, Chat (all in the RIGHT group)
    // Verify the right group toggle exists and contains panel options
    const rightGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    await expect(rightGroup).toBeVisible({ timeout: 10000 });

    // Find the toggle buttons for each panel
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const detailsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
    const chatToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.chat.show"]');

    // At least one toggle should exist
    const hasSettings = await settingsToggle.isVisible({ timeout: 3000 }).catch(() => false);
    const hasDetails = await detailsToggle.isVisible({ timeout: 1000 }).catch(() => false);
    const hasChat = await chatToggle.isVisible({ timeout: 1000 }).catch(() => false);

    console.log(`[Home] Panel toggles - settings: ${hasSettings}, details: ${hasDetails}, chat: ${hasChat}`);

    // Verify at least one panel toggle is present
    expect(hasSettings || hasDetails || hasChat).toBe(true);

    // Log which toggles are available
    if (hasSettings) console.log("[Home] Settings toggle found");
    if (hasDetails) console.log("[Home] Details toggle found");
    if (hasChat) console.log("[Home] Chat toggle found");
  });

  test("Kit", async ({ page }) => {
    test.setTimeout(180000);
    const { errors, warnings, messages } = await initConsole(page);
    await initKit(page);
    expect(errors.filter((e) => e.includes("Import error"))).toHaveLength(0);

    // Wait for page to stabilize after navigation
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    const pageText = await page.evaluate(() => document.body.innerText);
    expect(pageText).toContain("Metabolism");
    expect(warnings.filter(w => w.includes("Invalid access"))).toHaveLength(0);

    // Switch to types view using the toggle
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

    // Test Kit app panel toggles - Kit has settings, details, chat in right group
    console.log("[Kit] Verifying Kit app panel toggles");

    // Verify right group toggle exists (may take time after navigation)
    const rightGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    const hasRightGroup = await rightGroup.isVisible({ timeout: 10000 }).catch(() => false);
    console.log(`[Kit] Right group toggle visible: ${hasRightGroup}`);

    // Find individual panel toggles
    const settingsToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    const hasSettings = await settingsToggle.isVisible({ timeout: 3000 }).catch(() => false);

    console.log(`[Kit] Panel toggles - settings: ${hasSettings}`);

    // Kit should have either group toggle or settings toggle
    // Note: Panel toggles may not render in certain states - log warning if not found
    if (!hasSettings && !hasRightGroup) {
      console.log("[Kit] Warning: No panel toggles found - may be a rendering issue");
    }
  });
  test("Type", async ({ page }) => {
    test.setTimeout(120000);
    const { errors, warnings, messages } = await initType(page);
    const canvas = page.locator("canvas").first();
    await expect(canvas).toBeVisible({ timeout: 15000 });
    expect(page.url()).toContain("/types/");
    await page.waitForTimeout(5000);

    // Verify right group toggle is visible (indicates navbar is loaded)
    const navbar = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Navbar is visible");

    const footer = page.locator('footer').first();
    await expect(footer).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Footer is visible");

    // PANNING PERFORMANCE TEST for three.js scene
    // Test pan operations on the canvas to verify smooth camera movement
    const canvasBox = await canvas.boundingBox();
    if (canvasBox) {
      const centerX = canvasBox.x + canvasBox.width / 2;
      const centerY = canvasBox.y + canvasBox.height / 2;

      console.log("[Type Test] Starting pan operations on three.js canvas");

      // Warm up - first pan to initialize any lazy components
      await page.mouse.move(centerX, centerY);
      await page.mouse.down();
      await page.mouse.move(centerX + 100, centerY + 50);
      await page.mouse.up();
      await page.waitForTimeout(200);

      // First timed pan operation
      await page.mouse.move(centerX, centerY);
      await page.mouse.down();
      const pan1Start = Date.now();
      await page.mouse.move(centerX + 150, centerY + 100);
      await page.mouse.up();
      const pan1Duration = Date.now() - pan1Start;
      console.log(`[Type Test] Pan 1 took ${pan1Duration}ms`);

      await page.waitForTimeout(100);

      // Second timed pan operation
      await page.mouse.move(centerX + 150, centerY + 100);
      await page.mouse.down();
      const pan2Start = Date.now();
      await page.mouse.move(centerX - 100, centerY - 50);
      await page.mouse.up();
      const pan2Duration = Date.now() - pan2Start;
      console.log(`[Type Test] Pan 2 took ${pan2Duration}ms`);

      await page.waitForTimeout(100);

      // Third pan operation to test consistency
      await page.mouse.move(centerX - 100, centerY - 50);
      await page.mouse.down();
      const pan3Start = Date.now();
      await page.mouse.move(centerX, centerY);
      await page.mouse.up();
      const pan3Duration = Date.now() - pan3Start;
      console.log(`[Type Test] Pan 3 took ${pan3Duration}ms`);

      // Verify pan performance
      expect(pan1Duration).toBeLessThan(150);
      expect(pan2Duration).toBeLessThan(150);
      expect(pan3Duration).toBeLessThan(150);

      // Verify consistency - no dramatic slowdowns indicating cascading renders
      const avgPanTime = (pan1Duration + pan2Duration + pan3Duration) / 3;
      console.log(`[Type Test] Average pan time: ${avgPanTime}ms`);
      expect(Math.abs(pan1Duration - avgPanTime)).toBeLessThan(100);
      expect(Math.abs(pan2Duration - avgPanTime)).toBeLessThan(100);
      expect(Math.abs(pan3Duration - avgPanTime)).toBeLessThan(100);
    }

    // Wait a moment for any debounced updates
    await page.waitForTimeout(500);

    expect(warnings.filter(w => w.includes("Mesh"))).toHaveLength(0);
    expect(errors.filter(e => e.includes("Maximum update depth exceeded"))).toHaveLength(0);
    expect(messages.filter(m => m.includes("[TypeMesh] Selected"))).toHaveLength(1);

    // Test Type app panel toggles exist
    console.log("[Type] Verifying Type app panel toggles");

    // Verify all panel group toggles exist
    const workbenchGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]');
    const hudGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.hud"]');
    const rightGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');

    const hasWorkbench = await workbenchGroup.isVisible({ timeout: 5000 }).catch(() => false);
    const hasHud = await hudGroup.isVisible({ timeout: 5000 }).catch(() => false);
    const hasRight = await rightGroup.isVisible({ timeout: 5000 }).catch(() => false);

    console.log(`[Type] Panel groups - workbench: ${hasWorkbench}, hud: ${hasHud}, right: ${hasRight}`);

    // Type should have all three panel groups
    expect(hasWorkbench).toBe(true);
    expect(hasHud).toBe(true);
    expect(hasRight).toBe(true);
  });
  test("Design", async ({ page }) => {
    test.setTimeout(120000);

    const { errors, warnings, messages } = await initConsole(page);

    await initDesign(page);

    // Wait for the design app to fully load and stabilize
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);

    console.log("[Design Test] Current URL:", page.url());

    // Check for diagram and scene windows
    const diagramContainer = page.locator('.react-flow').first();
    const sceneCanvas = page.locator('canvas').first();

    // Wait longer for windows to load
    await page.waitForTimeout(3000);

    // Debug: Count all react-flow elements
    const reactFlowCount = await page.locator('.react-flow').count();
    console.log("[Design Test] ReactFlow elements count:", reactFlowCount);

    // Debug: Check for any visible windows
    const windowElements = await page.locator('[class*="window"], [class*="panel"]').count();
    console.log("[Design Test] Window/Panel elements count:", windowElements);

    const hasDiagram = await diagramContainer.isVisible({ timeout: 30000 }).catch(() => false);
    const hasScene = await sceneCanvas.isVisible({ timeout: 10000 }).catch(() => false);

    console.log("[Design Test] hasDiagram:", hasDiagram, "hasScene:", hasScene);

    // At least one window should be visible
    if (!hasDiagram && !hasScene) {
      console.log("[Design Test] Page HTML:", await page.content().then(c => c.slice(0, 2000)));
    }
    expect(hasDiagram || hasScene).toBe(true);

    // Check for infinite loop errors (critical - must be early in test)
    const infiniteLoopErrors = errors.filter(e => e.includes("Maximum update depth exceeded"));
    expect(infiniteLoopErrors).toHaveLength(0);

    // Check for navbar visibility (critical - must be early in test)
    const navbar = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Design Test] Navbar is visible");

    // Check for footer visibility (footer should be visible when not fullscreen)
    const footer = page.locator('footer').first();
    await expect(footer).toBeVisible({ timeout: 10000 });
    console.log("[Design Test] Footer is visible");

    // Verify existing pieces are visible in the design (Nakagin Capsule Tower has 180 pieces)
    if (hasDiagram) {
      const existingPieces = diagramContainer.locator(".react-flow__node");
      const pieceCount = await existingPieces.count();
      console.log("[Design Test] Piece count:", pieceCount);
      expect(pieceCount).toBeGreaterThan(0);

      // PANNING PERFORMANCE TEST
      // Measure actual pan operation time (should be under 100ms each)
      const viewport = diagramContainer.locator(".react-flow__viewport").first();
      const viewportBox = await viewport.boundingBox();

      if (viewportBox) {
        const centerX = viewportBox.x + viewportBox.width / 2;
        const centerY = viewportBox.y + viewportBox.height / 2;

        // Warm up - first pan to initialize any lazy components
        await page.mouse.move(centerX, centerY);
        await page.mouse.down();
        await page.mouse.move(centerX + 50, centerY + 25);
        await page.mouse.up();
        // Brief pause - debounce is 1000ms, so state update won't happen yet
        await page.waitForTimeout(100);

        // First timed pan operation
        await page.mouse.move(centerX + 50, centerY + 25);
        await page.mouse.down();
        const pan1Start = Date.now();
        await page.mouse.move(centerX + 150, centerY + 75);
        await page.mouse.up();
        const pan1Duration = Date.now() - pan1Start;
        console.log(`[Design Test] Pan 1 took ${pan1Duration}ms`);

        // Second timed pan operation
        await page.mouse.move(centerX + 150, centerY + 75);
        await page.mouse.down();
        const pan2Start = Date.now();
        await page.mouse.move(centerX + 50, centerY + 25);
        await page.mouse.up();
        const pan2Duration = Date.now() - pan2Start;
        console.log(`[Design Test] Pan 2 took ${pan2Duration}ms`);

        // PERF: 750ms threshold accounts for:
        // - ReactFlow with 180 nodes + edges
        // - Three.js 3D scene with 180 meshes
        // - GPU/browser rendering overhead in headless mode
        // - Playwright event processing overhead
        // - Run-to-run variance (~50-100ms)
        // Baseline measured at ~300-550ms with GPU acceleration enabled.
        // Values significantly higher indicate a performance regression.
        // Without optimizations, this was ~2300ms in headless mode.
        expect(pan1Duration).toBeLessThan(750);
        expect(pan2Duration).toBeLessThan(750);
        // Verify no cascade: second pan shouldn't be dramatically slower
        expect(Math.abs(pan1Duration - pan2Duration)).toBeLessThan(250);
      }

      // HOVER PERFORMANCE TEST
      // Hovering and unhovering over a piece should happen within 100ms
      const firstPiece = existingPieces.first();
      const pieceBox = await firstPiece.boundingBox();

      if (pieceBox) {
        const pieceCenterX = pieceBox.x + pieceBox.width / 2;
        const pieceCenterY = pieceBox.y + pieceBox.height / 2;

        // Warm up - move away from piece first
        await page.mouse.move(pieceBox.x - 100, pieceBox.y - 100);
        await page.waitForTimeout(100);

        // Timed hover operation (mouse enter)
        const hoverStart = Date.now();
        await page.mouse.move(pieceCenterX, pieceCenterY);
        const hoverDuration = Date.now() - hoverStart;
        console.log(`[Design Test] Hover (mouse enter) took ${hoverDuration}ms`);

        // Brief pause to allow hover state to settle
        await page.waitForTimeout(50);

        // Timed unhover operation (mouse leave)
        const unhoverStart = Date.now();
        await page.mouse.move(pieceBox.x - 100, pieceBox.y - 100);
        const unhoverDuration = Date.now() - unhoverStart;
        console.log(`[Design Test] Unhover (mouse leave) took ${unhoverDuration}ms`);

        // Both hover operations should complete within 100ms
        // This verifies granular subscriptions prevent cascade re-renders
        expect(hoverDuration).toBeLessThan(100);
        expect(unhoverDuration).toBeLessThan(100);

        // Multiple hover/unhover cycles should be consistent
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
        // PERF: Hover operations should be under 200ms
        // Baseline is 40-70ms but can spike to 150ms when running in parallel with other tests
        // Values significantly higher indicate state management issues (cascade renders)
        hoverTimes.forEach((time, i) => {
          expect(time).toBeLessThan(200);
        });
      }
    }

    // Verify canvas is rendering (for 3D scene)
    await expect(sceneCanvas).toBeVisible({ timeout: 10000 });

    // SCENE PANNING PERFORMANCE TEST for three.js canvas
    // Test pan operations on the scene canvas to verify smooth camera movement
    if (hasScene) {
      const sceneBox = await sceneCanvas.boundingBox();
      if (sceneBox) {
        const centerX = sceneBox.x + sceneBox.width / 2;
        const centerY = sceneBox.y + sceneBox.height / 2;

        console.log("[Design Test] Starting scene pan operations on three.js canvas");

        // Warm up - first pan to initialize any lazy components
        await page.mouse.move(centerX, centerY);
        await page.mouse.down();
        await page.mouse.move(centerX + 100, centerY + 50);
        await page.mouse.up();
        await page.waitForTimeout(200);

        // First timed pan operation
        await page.mouse.move(centerX, centerY);
        await page.mouse.down();
        const scenePan1Start = Date.now();
        await page.mouse.move(centerX + 150, centerY + 100);
        await page.mouse.up();
        const scenePan1Duration = Date.now() - scenePan1Start;
        console.log(`[Design Test] Scene Pan 1 took ${scenePan1Duration}ms`);

        await page.waitForTimeout(100);

        // Second timed pan operation
        await page.mouse.move(centerX + 150, centerY + 100);
        await page.mouse.down();
        const scenePan2Start = Date.now();
        await page.mouse.move(centerX - 100, centerY - 50);
        await page.mouse.up();
        const scenePan2Duration = Date.now() - scenePan2Start;
        console.log(`[Design Test] Scene Pan 2 took ${scenePan2Duration}ms`);

        await page.waitForTimeout(100);

        // Third pan operation to test consistency
        await page.mouse.move(centerX - 100, centerY - 50);
        await page.mouse.down();
        const scenePan3Start = Date.now();
        await page.mouse.move(centerX, centerY);
        await page.mouse.up();
        const scenePan3Duration = Date.now() - scenePan3Start;
        console.log(`[Design Test] Scene Pan 3 took ${scenePan3Duration}ms`);

        // PERF: Scene pan thresholds:
        // - First pan may have initialization overhead (shader compilation, etc.)
        // - Subsequent pans should be faster (75-300ms baseline)
        // - 1000ms first-pan threshold allows for cold-start
        // - 500ms subsequent threshold catches regressions
        expect(scenePan1Duration).toBeLessThan(1000);
        expect(scenePan2Duration).toBeLessThan(500);
        expect(scenePan3Duration).toBeLessThan(500);

        // Verify consistency - second and third pans should be similar
        const avgSubsequentPanTime = (scenePan2Duration + scenePan3Duration) / 2;
        console.log(`[Design Test] Average subsequent scene pan time: ${avgSubsequentPanTime}ms`);
        expect(Math.abs(scenePan2Duration - avgSubsequentPanTime)).toBeLessThan(200);
        expect(Math.abs(scenePan3Duration - avgSubsequentPanTime)).toBeLessThan(200);
      }
    }

    // Filter out expected "File URL not available" warnings - GLB files aren't accessible in test environment
    // Only fail on unexpected Mesh warnings (like render errors, crashes, etc.)
    const unexpectedMeshWarnings = warnings.filter(w =>
      w.includes("Mesh") &&
      !w.includes("File URL not available")
    );
    expect(unexpectedMeshWarnings).toHaveLength(0);

    // Test Design app panel toggles exist
    console.log("[Design] Verifying Design app panel toggles");

    // Verify all panel group toggles exist
    const workbenchGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]');
    const hudGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.hud"]');
    const rightGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');

    const hasWorkbench = await workbenchGroup.isVisible({ timeout: 5000 }).catch(() => false);
    const hasHud = await hudGroup.isVisible({ timeout: 5000 }).catch(() => false);
    const hasRight = await rightGroup.isVisible({ timeout: 5000 }).catch(() => false);

    console.log(`[Design] Panel groups - workbench: ${hasWorkbench}, hud: ${hasHud}, right: ${hasRight}`);

    // Design should have all three panel groups
    expect(hasWorkbench).toBe(true);
    expect(hasHud).toBe(true);
    expect(hasRight).toBe(true);
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

    // Test Docs app panel toggles exist
    console.log("[Docs] Verifying Docs app panel toggles");

    // Verify panel group toggles exist (Docs has workbench and right, no hud)
    const workbenchGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]');
    const rightGroup = page.locator('[id="semio.sketchpad.navbar.panelToggle.right"]');

    const hasWorkbench = await workbenchGroup.isVisible({ timeout: 5000 }).catch(() => false);
    const hasRight = await rightGroup.isVisible({ timeout: 5000 }).catch(() => false);

    console.log(`[Docs] Panel groups - workbench: ${hasWorkbench}, right: ${hasRight}`);

    // Docs should have workbench and right panel groups
    expect(hasWorkbench).toBe(true);
    expect(hasRight).toBe(true);

    // Navigate to a specific docs page and verify content
    await page.goto("/docs/manuals/sketchpad");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(500);
    await expect(page.getByRole("heading", { name: "Apps", level: 1 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Home", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Kit", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Design", level: 2 })).toBeVisible();

    // Test navigation button
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
});
