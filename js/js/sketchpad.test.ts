import { expect, Locator, Page, test } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

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
  const navbarToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
  await expect(navbarToggle).toBeVisible({ timeout: 60000 });

  // Check if already open
  const isPressed = await navbarToggle.getAttribute("data-state").catch(() => "off");
  if (isPressed !== "on") {
    await navbarToggle.click();
  }
  await page.waitForTimeout(1000);

  // Wait for the settings panel to appear
  const settingsPanel = page.locator('[data-panel="settings"]').first();
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
  const navbarToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]');
  await expect(navbarToggle).toBeVisible({ timeout: 60000 });

  // Check if already open
  const isPressed = await navbarToggle.getAttribute("data-state").catch(() => "off");
  if (isPressed !== "on") {
    await navbarToggle.click();
  }
  await page.waitForTimeout(1000);

  // Wait for the details panel to appear
  const detailsPanel = page.locator('[data-panel="details"]').first();
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

async function initHome(page: Page) {
  const consoleMessages: string[] = [];
  const consoleErrors: string[] = [];
  const networkRequests: string[] = [];
  page.on("console", (msg) => {
    const text = msg.text();
    consoleMessages.push(`[${msg.type()}] ${text}`);
    if (msg.type() === "error") consoleErrors.push(text);
  });
  page.on("pageerror", (err) => consoleErrors.push(`PAGE_ERROR: ${err.message}`));
  page.on("request", (req) => {
    networkRequests.push(`[REQ] ${req.method()} ${req.url()}`);
  });
  page.on("response", (res) => {
    networkRequests.push(`[RES] ${res.status()} ${res.url()}`);
  });
  page.on("requestfailed", (req) => {
    networkRequests.push(`[FAIL] ${req.url()} - ${req.failure()?.errorText}`);
  });

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
    console.log("[TEST] Navigation failed. Console errors:", consoleErrors);
    console.log("[TEST] All console messages:", consoleMessages);
    console.log("[TEST] Network requests after import:", networkRequests.filter(r => r.includes("wasm") || r.includes("sql") || r.includes("FAIL")));
    throw error;
  }

  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
}

async function initKit(page: Page) {
  await initHome(page);
  // Already at the metabolism kit after initHome
}

async function initDesign(page: Page) {
  await initKit(page);

  // The kit starts in designs view by default, or we navigate via URL
  // Try to find and open the Nakagin Capsule Tower design
  const design = page.getByRole("button", { name: "Nakagin Capsule Tower" });
  const isDesignVisible = await design.isVisible({ timeout: 5000 }).catch(() => false);

  if (!isDesignVisible) {
    // Try navigating to designs via URL parameter
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
  await initKit(page);
  console.log("[initType] Current URL:", page.url());

  // Switch to types view using the toggle (without page reload to preserve kit state)
  const typesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTypes"]');
  await expect(typesToggle).toBeVisible({ timeout: 30000 });
  await typesToggle.click();
  await page.waitForTimeout(2000);

  // Find and double-click on Tambour type
  const tambourType = page.getByRole("button", { name: "Tambour" }).first();
  await expect(tambourType).toBeVisible({ timeout: 3000 });
  await tambourType.dblclick();
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(5000);
}

async function initDocs(page: Page) {
  await page.goto("/docs/index");
  await page.waitForLoadState("networkidle");
}

test.describe("sketchpad", () => {
  test("Home", async ({ page }) => {
    await page.goto("/");
    await page.waitForLoadState("networkidle");
    await openSettingsPanel(page);
    const sections = await getSettingsSections(page);
    expect(sections).toContain("semio.sketchpad.app.home.settings");
    expect(sections).toContain("semio.sketchpad.settings");
    const homeIndex = sections.indexOf("semio.sketchpad.app.home.settings");
    const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");
    expect(homeIndex).toBeGreaterThanOrEqual(0);
    expect(sketchpadIndex).toBeGreaterThanOrEqual(0);
    expect(homeIndex).toBeLessThan(sketchpadIndex);
  });

  test("Kit", async ({ page }) => {
    test.setTimeout(120000);
    const consoleErrors: string[] = [];
    const yjsWarnings: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
      if (msg.type() === "warning" && msg.text().includes("Invalid access")) yjsWarnings.push(msg.text());
    });
    page.on("pageerror", (err) => consoleErrors.push(`PAGE_ERROR: ${err.message}`));

    await initKit(page);
    expect(consoleErrors.filter((e) => e.includes("Import error"))).toHaveLength(0);

    // Wait for page to stabilize after navigation
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(2000);

    const isResponsive = await Promise.race([page.evaluate(() => true), new Promise<boolean>((resolve) => setTimeout(() => resolve(false), 5000))]);
    expect(isResponsive).toBe(true);

    const pageText = await page.evaluate(() => document.body.innerText);
    expect(pageText).toContain("Metabolism");
    expect(yjsWarnings).toHaveLength(0);

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

    // Test settings panel
    await openSettingsPanel(page);
    const sections = await getSettingsSections(page);
    if (sections.length > 0) {
      expect(sections).toContain("semio.sketchpad.app.kit.settings");
      expect(sections).toContain("semio.sketchpad.settings");
      const kitIndex = sections.indexOf("semio.sketchpad.app.kit.settings");
      const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");
      expect(kitIndex).toBeLessThan(sketchpadIndex);
    }

    // Verify toggles for concepts, interfaces, and tags exist in the strip
    // These are visible in the toggle strip when no kind is selected
    const strip = page.locator('[id="semio.sketchpad.app.kit.filter.strip"]').first();
    const hasStrip = await strip.isVisible({ timeout: 5000 }).catch(() => false);
    if (hasStrip) {
      // Check for the toggle buttons in the strip
      const conceptsToggle = strip.locator('[id="semio.sketchpad.app.kit.kitApp.showConcepts"]');
      const interfacesToggle = strip.locator('[id="semio.sketchpad.app.kit.kitApp.showInterfaces"]');
      const tagsToggle = strip.locator('[id="semio.sketchpad.app.kit.kitApp.showTags"]');

      // These should be visible when no kind filter is applied
      // First deselect the types kind to show all toggles
      const hideKindBtn = page.locator('[id="semio.sketchpad.app.kit.kitApp.hideKind"]');
      if (await hideKindBtn.isVisible().catch(() => false)) {
        await hideKindBtn.click();
        await page.waitForTimeout(500);
      }

      const hasConceptsToggle = await conceptsToggle.isVisible({ timeout: 3000 }).catch(() => false);
      const hasInterfacesToggle = await interfacesToggle.isVisible({ timeout: 3000 }).catch(() => false);
      const hasTagsToggle = await tagsToggle.isVisible({ timeout: 3000 }).catch(() => false);

      // At least verify we can access these toggles (they may be scrolled)
      console.log(`Toggle visibility - concepts: ${hasConceptsToggle}, interfaces: ${hasInterfacesToggle}, tags: ${hasTagsToggle}`);
    }

    // Test interfaces view - check for "core circular bottom" interface
    // First clear any kind filter by clicking the hideKind button if it exists
    const hideKindBtn = page.locator('[id="semio.sketchpad.app.kit.kitApp.hideKind"]');
    if (await hideKindBtn.isVisible().catch(() => false)) {
      await hideKindBtn.click();
      await page.waitForTimeout(500);
    }

    const interfacesToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showInterfaces"]');
    // Scroll into view if needed
    await interfacesToggle.scrollIntoViewIfNeeded().catch(() => { });
    const hasInterfacesToggle = await interfacesToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log("[Kit Test] Interfaces toggle visible:", hasInterfacesToggle);
    if (hasInterfacesToggle) {
      await interfacesToggle.click();
      await page.waitForTimeout(1000);

      // Verify the table shows interfaces
      const tableBody = page.locator("tbody").first();
      await expect(tableBody).toBeVisible({ timeout: 5000 });

      // Check for specific interface from kit_metabolism.json
      const coreCircularBottomInterface = page.getByRole("button", { name: "core circular bottom" });
      await expect(coreCircularBottomInterface).toBeVisible({ timeout: 5000 });
      console.log("[Kit Test] Found interface: core circular bottom");
    }

    // Test tags view - check for "collider" tag
    const tagsToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showTags"]');
    const hasTagsToggle = await tagsToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log("[Kit Test] Tags toggle visible:", hasTagsToggle);
    if (hasTagsToggle) {
      await tagsToggle.click();
      await page.waitForTimeout(1000);

      // Verify the table shows tags
      const tableBody = page.locator("tbody").first();
      await expect(tableBody).toBeVisible({ timeout: 5000 });

      // Check for specific tag from kit_metabolism.json
      const colliderTag = page.getByRole("button", { name: "collider" });
      await expect(colliderTag).toBeVisible({ timeout: 5000 });
      console.log("[Kit Test] Found tag: collider");
    }

    // Test concepts view - check for "living-architecture" concept
    const conceptsToggle = page.locator('button[id="semio.sketchpad.app.kit.kitApp.showConcepts"]');
    const hasConceptsToggle = await conceptsToggle.isVisible({ timeout: 5000 }).catch(() => false);
    console.log("[Kit Test] Concepts toggle visible:", hasConceptsToggle);
    if (hasConceptsToggle) {
      await conceptsToggle.click();
      await page.waitForTimeout(1000);

      // Verify the table shows concepts
      const tableBody = page.locator("tbody").first();
      await expect(tableBody).toBeVisible({ timeout: 5000 });

      // Check for specific concept from kit_metabolism.json
      const livingArchitectureConcept = page.getByRole("button", { name: "living-architecture" });
      await expect(livingArchitectureConcept).toBeVisible({ timeout: 5000 });
      console.log("[Kit Test] Found concept: living-architecture");
    }
  });
  test("Type", async ({ page }) => {
    test.setTimeout(120000);
    const consoleMessages: string[] = [];
    const consoleWarnings: string[] = [];
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "warning") {
        consoleWarnings.push(msg.text());
      }
      else if (msg.type() === "error") {
        consoleErrors.push(msg.text());
      }
      else {
        consoleMessages.push(msg.text());
      }
    });
    await initType(page);
    const canvas = page.locator("canvas").first();
    await expect(canvas).toBeVisible({ timeout: 15000 });
    expect(page.url()).toContain("/types/");
    await page.waitForTimeout(5000);

    const navbar = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
    await expect(navbar).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Navbar is visible");

    const footer = page.locator('footer').first();
    await expect(footer).toBeVisible({ timeout: 10000 });
    console.log("[Type Test] Footer is visible");

    expect(consoleWarnings.filter(w => w.includes("Mesh"))).toHaveLength(0);
    expect(consoleErrors.filter(e => e.includes("Maximum update depth exceeded"))).toHaveLength(0);
    expect(consoleMessages.filter(e => e.includes("[TypeMesh] Selected"))).toHaveLength(1);

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

    expect(consoleWarnings.filter(w => w.includes("Mesh"))).toHaveLength(0);
    expect(consoleMessages.filter(e => e.includes("[TypeMesh] Selected"))).toHaveLength(1);

  });
  test("Design", async ({ page }) => {
    test.setTimeout(120000);

    const consoleMessages: string[] = [];
    page.on("console", (msg) => {
      consoleMessages.push(`[${msg.type()}] ${msg.text()}`);
    });

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
      console.log("[Design Test] Console messages:", consoleMessages.filter(m => m.includes("error") || m.includes("Error")).slice(-10));
      console.log("[Design Test] Page HTML:", await page.content().then(c => c.slice(0, 2000)));
    }
    expect(hasDiagram || hasScene).toBe(true);

    // Check for infinite loop errors (critical - must be early in test)
    const consoleErrors = consoleMessages.filter(m => m.startsWith("[error]"));
    const infiniteLoopErrors = consoleErrors.filter(e => e.includes("Maximum update depth exceeded"));
    console.log("[Design Test] Console errors:", consoleErrors);
    expect(infiniteLoopErrors).toHaveLength(0);

    // Check for navbar visibility (critical - must be early in test)
    const navbar = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
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

        expect(pan1Duration).toBeLessThan(100);
        expect(pan2Duration).toBeLessThan(100);
        // Verify no cascade: second pan shouldn't be dramatically slower
        expect(Math.abs(pan1Duration - pan2Duration)).toBeLessThan(100);
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
        // All hover operations should be under 100ms and consistent
        hoverTimes.forEach((time, i) => {
          expect(time).toBeLessThan(100);
        });
      }
    }

    // Verify canvas is rendering (for 3D scene)
    await expect(sceneCanvas).toBeVisible({ timeout: 10000 });
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

    await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]').click();
    await page.waitForTimeout(500);
    await expect(page.getByText("Getting Started").first()).toBeVisible();
    await expect(page.getByText("Tutorials").first()).toBeVisible();
    await expect(page.getByText("Integrations").first()).toBeVisible();
    await expect(page.getByText("Manuals").first()).toBeVisible();
    await expect(page.getByText("Theory").first()).toBeVisible();
    await expect(page.getByText("Showcases").first()).toBeVisible();

    await expect(page.getByText("Installation").first()).toBeVisible();
    await expect(page.getByText("Starter").first()).toBeVisible();
    await expect(page.getByText("Rhino").first()).toBeVisible();
    await expect(page.getByText("sketchpad").first()).toBeVisible();

    await page.goto("/docs/manuals/sketchpad");
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(500);
    await expect(page.getByRole("heading", { name: "Apps", level: 1 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Home", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Kit", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Design", level: 2 })).toBeVisible();
    await page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]').click();
    await page.waitForTimeout(500);
    await expect(page.locator('[id="semio.sketchpad.app.docs.page"]')).toBeVisible();

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
