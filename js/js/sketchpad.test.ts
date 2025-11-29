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

test.describe("sketchpad", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await page.waitForLoadState("networkidle");
  });

  test.describe("Kit", () => {
    test.beforeEach(async ({ page }) => {
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.waitForTimeout(1000);
    });
    test.describe("Design", () => {
      test.beforeEach(async ({ page }) => {
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
        await page.waitForTimeout(500);
      });
      test("Windows", async ({ page }) => {
        const diagramWindow = page.locator("text=diagram").first();
        const sceneWindow = page.locator("text=scene").first();
        await expect(diagramWindow).toBeVisible();
        await expect(sceneWindow).toBeVisible();
        await expect(diagramWindow).toBeInViewport();
        await expect(sceneWindow).toBeInViewport();
        await expectFullyInViewport(diagramWindow, page, [0, 100], [0, 100]);
        await expectFullyInViewport(sceneWindow, page, [400, 800], [0, 100]);
      });
      test("Drag and Drop Pieces", async ({ page }) => {
        test.setTimeout(60000);
        await page.locator('[id="semio.sketchpad.navbar.back"]').click();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.navbar.back"]').click();
        await page.waitForTimeout(500);
        await page.getByRole("button", { name: "Design" }).dblclick();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]').click();
        await page.waitForTimeout(500);
        const typesSection = page.locator('[id="semio.sketchpad.app.kit.types"]').first();
        await expect(typesSection).toBeVisible();
        const draggableTypeAvatar = page.locator('[data-slot="avatar"][title="New Type"]').first();
        const diagramDropZone = page.locator('[data-drop-zone="diagram"]').first();
        const sceneDropZone = page.locator('[data-drop-zone="scene"]').first();
        await expect(draggableTypeAvatar).toBeVisible();
        await expect(diagramDropZone).toBeVisible();
        await expect(sceneDropZone).toBeVisible();
        const sourceBox = await draggableTypeAvatar.boundingBox();
        const diagramBox = await diagramDropZone.boundingBox();
        const sceneBox = await sceneDropZone.boundingBox();
        expect(sourceBox).not.toBeNull();
        expect(diagramBox).not.toBeNull();
        expect(sceneBox).not.toBeNull();
        const sourceCenter = { x: sourceBox!.x + sourceBox!.width / 2, y: sourceBox!.y + sourceBox!.height / 2 };
        const sceneCenter = { x: sceneBox!.x + sceneBox!.width / 2, y: sceneBox!.y + sceneBox!.height / 2 };
        const countPieces = async () => {
          const reactFlowNodes = diagramDropZone.locator(".react-flow__node");
          return await reactFlowNodes.count();
        };
        const dragAndDrop = async (from: { x: number; y: number }, to: { x: number; y: number }) => {
          await page.mouse.move(from.x, from.y);
          await page.mouse.down();
          await page.waitForTimeout(100);
          await page.mouse.move(to.x, to.y, { steps: 10 });
          await page.waitForTimeout(100);
          await page.mouse.up();
          await page.waitForTimeout(500);
        };
        const verifyPiecePlacement = async (pieceIndex: number, dropPoint: { x: number; y: number }) => {
          const pieceNode = diagramDropZone.locator(".react-flow__node").nth(pieceIndex);
          const pieceBox = await pieceNode.boundingBox();
          expect(pieceBox).not.toBeNull();
          const pieceCenter = { x: pieceBox!.x + pieceBox!.width / 2, y: pieceBox!.y + pieceBox!.height / 2 };
          const distance = Math.sqrt((pieceCenter.x - dropPoint.x) ** 2 + (pieceCenter.y - dropPoint.y) ** 2);
          expect(distance).toBeLessThan(50);
          await page.mouse.move(pieceCenter.x, pieceCenter.y);
          await page.waitForTimeout(300);
          const pieceAvatar = pieceNode.locator('[data-slot="avatar"]');
          const avatarClasses = await pieceAvatar.evaluate((el) => el.className);
          expect(avatarClasses).toContain("ring");
        };
        const initialPieceCount = await countPieces();
        expect(initialPieceCount).toBe(0);
        const margin = 50;
        const panelWidth = 230;
        const visibleLeft = diagramBox!.x + panelWidth + margin;
        const visibleRight = diagramBox!.x + diagramBox!.width - margin;
        const visibleTop = diagramBox!.y + margin;
        const visibleBottom = diagramBox!.y + diagramBox!.height - margin;
        const visibleCenterX = (visibleLeft + visibleRight) / 2;
        const visibleCenterY = (visibleTop + visibleBottom) / 2;
        const cornerDropPoints = [
          { name: "top-left", x: visibleLeft + 50, y: visibleTop + 50 },
          { name: "top-right", x: visibleRight - 50, y: visibleTop + 50 },
          { name: "bottom-left", x: visibleLeft + 50, y: visibleBottom - 50 },
          { name: "bottom-right", x: visibleRight - 50, y: visibleBottom - 50 },
          { name: "center", x: visibleCenterX, y: visibleCenterY },
        ];
        for (let i = 0; i < cornerDropPoints.length; i++) {
          await dragAndDrop(sourceCenter, cornerDropPoints[i]);
          const pieceCount = await countPieces();
          expect(pieceCount).toBe(i + 1);
          await verifyPiecePlacement(i, cornerDropPoints[i]);
        }
        const panStart = { x: visibleRight - 100, y: visibleBottom - 100 };
        await page.mouse.move(panStart.x, panStart.y);
        await page.mouse.down();
        await page.mouse.move(panStart.x - 80, panStart.y - 80, { steps: 5 });
        await page.mouse.up();
        await page.waitForTimeout(300);
        await page.mouse.move(visibleCenterX, visibleCenterY);
        await page.mouse.wheel(0, -150);
        await page.waitForTimeout(300);
        const afterPanZoomDropPoint = { x: visibleCenterX, y: visibleCenterY };
        await dragAndDrop(sourceCenter, afterPanZoomDropPoint);
        const pieceCountAfterPanZoom = await countPieces();
        expect(pieceCountAfterPanZoom).toBe(6);
        await verifyPiecePlacement(5, afterPanZoomDropPoint);
        const sceneMargin = 80;
        const sceneCornerDropPoints = [
          { name: "scene-top-left", x: sceneBox!.x + sceneMargin, y: sceneBox!.y + sceneMargin },
          { name: "scene-top-right", x: sceneBox!.x + sceneBox!.width - sceneMargin, y: sceneBox!.y + sceneMargin },
          { name: "scene-bottom-left", x: sceneBox!.x + sceneMargin, y: sceneBox!.y + sceneBox!.height - sceneMargin },
          { name: "scene-bottom-right", x: sceneBox!.x + sceneBox!.width - sceneMargin, y: sceneBox!.y + sceneBox!.height - sceneMargin },
        ];
        for (let i = 0; i < sceneCornerDropPoints.length; i++) {
          await dragAndDrop(sourceCenter, sceneCornerDropPoints[i]);
          await page.waitForTimeout(200);
          const pieceCount = await countPieces();
          expect(pieceCount).toBe(7 + i);
          const pieceNode = diagramDropZone.locator(".react-flow__node").nth(6 + i);
          const pieceBox = await pieceNode.boundingBox();
          expect(pieceBox).not.toBeNull();
        }
        const finalPieceCount = await countPieces();
        expect(finalPieceCount).toBe(10);
      });
    });
    test.describe("Type", () => {
      test.beforeEach(async ({ page }) => {
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();
        await page.waitForTimeout(1000);
      });
      test("Toolbar is visible with selection and port tools", async ({ page }) => {
        // Requires kit seed to run first
        // Wait for the toolbar to render
        await page.waitForTimeout(500);

        // Toolbar container should be visible - use a more flexible selector
        const toolbar = page.locator("div.flex.items-stretch.border.overflow-hidden.h-large").first();
        await expect(toolbar).toBeVisible({ timeout: 10000 });

        // Selection tool toggle should be visible (dropdown with multiple modes)
        const selectionTool = page.locator('[id="semio.sketchpad.tool.selection"]');
        await expect(selectionTool).toBeVisible({ timeout: 5000 });

        // Port tool toggle should be visible
        const portTool = page.locator('[id="semio.sketchpad.tool.port"]');
        await expect(portTool).toBeVisible({ timeout: 5000 });
      });
      test("Port tool can be selected", async ({ page }) => {
        // Requires kit seed to run first
        await page.waitForTimeout(500);

        // Click on port tool to activate it
        const portTool = page.locator('[id="semio.sketchpad.tool.port"]');
        await expect(portTool).toBeVisible({ timeout: 10000 });
        await portTool.click();
        await page.waitForTimeout(200);

        // Port tool should now be pressed (active)
        await expect(portTool).toHaveAttribute("data-state", "on");
      });
      test("Selection tool is active by default", async ({ page }) => {
        // Requires kit seed to run first
        await page.waitForTimeout(500);

        // Selection tool should be pressed by default
        const selectionTool = page.locator('[id="semio.sketchpad.tool.selection"]');
        await expect(selectionTool).toBeVisible({ timeout: 10000 });
        await expect(selectionTool).toHaveAttribute("data-state", "on");
      });
    });
  });

  test.describe("Kit Import Drag and Drop", () => {
    test("Drop metabolism.zip creates temporary kit and navigates", async ({ page }) => {
      test.setTimeout(60000);
      const consoleErrors: string[] = [];
      page.on("console", (msg) => { if (msg.type() === "error") consoleErrors.push(msg.text()); });
      page.on("pageerror", (err) => consoleErrors.push(`PAGE_ERROR: ${err.message}`));
      await page.waitForTimeout(1000);

      // Import kit via file input
      const zipPath = path.resolve(__dirname, "../../assets/semio/metabolism.zip");
      const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
      await expect(fileInput).toBeAttached({ timeout: 10000 });
      await fileInput.setInputFiles(zipPath);

      // Wait for navigation to kit page
      await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });

      // Verify navigation completed and page is responsive
      const url = page.url();
      expect(url).toMatch(/kits\/.+/);

      // Verify no import errors
      expect(consoleErrors.filter(e => e.includes("Import error"))).toHaveLength(0);

      // Wait for page to be interactive (not hung)
      await page.waitForTimeout(2000);
      const isResponsive = await Promise.race([
        page.evaluate(() => true),
        new Promise<boolean>((resolve) => setTimeout(() => resolve(false), 5000))
      ]);
      expect(isResponsive).toBe(true);
    });
  });

  test.describe("Docs", () => {
    test.beforeEach(async ({ page }) => {
      await page.goto("/docs/index");
      await page.waitForLoadState("networkidle");
    });

    test("Content Loads", async ({ page }) => {
      const pageTitle = page.getByRole("heading", { name: "Welcome to Semio", level: 1 });
      await expect(pageTitle).toBeVisible();
      const pageDescription = page.getByText("Design Information Modeling for Architecture");
      await expect(pageDescription).toBeVisible();
      const cardHeading = page.getByRole("heading", { name: /Just want to toy around/ });
      await expect(cardHeading).toBeVisible();
      const researchCard = page.getByRole("heading", { name: /More into research/ });
      await expect(researchCard).toBeVisible();
    });

    test("Workbench Panel Shows All Sections", async ({ page }) => {
      await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]').click();
      await page.waitForTimeout(500);
      const gettingStartedSection = page.getByText("Getting Started").first();
      await expect(gettingStartedSection).toBeVisible();
      const tutorialsSection = page.getByText("Tutorials").first();
      await expect(tutorialsSection).toBeVisible();
      const integrationsSection = page.getByText("Integrations").first();
      await expect(integrationsSection).toBeVisible();
      const manualsSection = page.getByText("Manuals").first();
      await expect(manualsSection).toBeVisible();
      const theorySection = page.getByText("Theory").first();
      await expect(theorySection).toBeVisible();
      const showcasesSection = page.getByText("Showcases").first();
      await expect(showcasesSection).toBeVisible();
    });

    test("Workbench Panel Shows Pages In Sections", async ({ page }) => {
      await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench.show"]').click();
      await page.waitForTimeout(500);
      const installationPage = page.getByText("Installation").first();
      await expect(installationPage).toBeVisible();
      const starterPage = page.getByText("Starter").first();
      await expect(starterPage).toBeVisible();
      const rhinoPage = page.getByText("Rhino").first();
      await expect(rhinoPage).toBeVisible();
      const sketchpadPage = page.getByText("sketchpad").first();
      await expect(sketchpadPage).toBeVisible();
    });

    test("Details Panel Shows Page Section", async ({ page }) => {
      await page.goto("/docs/manuals/sketchpad");
      await page.waitForLoadState("networkidle");
      await page.waitForTimeout(500);
      const appsHeading = page.getByRole("heading", { name: "Apps", level: 1 });
      await expect(appsHeading).toBeVisible();
      const homeHeading = page.getByRole("heading", { name: "Home", level: 2 });
      await expect(homeHeading).toBeVisible();
      const kitHeading = page.getByRole("heading", { name: "Kit", level: 2 });
      await expect(kitHeading).toBeVisible();
      const designHeading = page.getByRole("heading", { name: "Design", level: 2 });
      await expect(designHeading).toBeVisible();
      await page.locator('[id="semio.sketchpad.navbar.panelToggle.details.show"]').click();
      await page.waitForTimeout(500);
      const pageSectionButton = page.locator('[id="semio.sketchpad.app.docs.page"]');
      await expect(pageSectionButton).toBeVisible();
    });

    test("Navigation Works Between Pages", async ({ page }) => {
      const nextButton = page.getByRole("button", { name: /Intro/i });
      await expect(nextButton).toBeVisible();
      await nextButton.click();
      await page.waitForLoadState("networkidle");
      await page.waitForTimeout(500);
      await expect(page).toHaveURL(/.*docs\/getting-started\/intro/);
      const introTitle = page.getByRole("heading", { level: 1 }).first();
      await expect(introTitle).toBeVisible();
    });
  });

  test.describe("Settings Panel Hierarchy", () => {
    /**
     * App hierarchy: Sketchpad -> Home -> Kit -> Design | Type
     * Panel sections should be ordered from most specific (top) to least specific (bottom)
     *
     * Expected settings sections:
     * - Home: Home section (specificity 20), Sketchpad section (specificity 0)
     * - Kit: Kit section (specificity 10), Sketchpad section (specificity 0)
     * - Design: Design section (specificity 30), Kit section (specificity 10), Sketchpad section (specificity 0)
     * - Type: Type section (specificity 30), Kit section (specificity 10), Sketchpad section (specificity 0)
     */

    const openSettingsPanel = async (page: Page) => {
      // Wait for the navbar toggle button to be visible
      const navbarToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.settings.show"]');
      await expect(navbarToggle).toBeVisible({ timeout: 60000 });
      await navbarToggle.click();
      await page.waitForTimeout(500);
    };

    const getSettingsSections = async (page: Page): Promise<string[]> => {
      const settingsPanel = page.locator('[data-panel="settings"]').first();
      await expect(settingsPanel).toBeVisible();

      // Get all section buttons within the settings panel
      const sections = await settingsPanel.locator('[role="button"][id^="semio.sketchpad"]').all();
      const sectionIds: string[] = [];

      for (const section of sections) {
        const id = await section.getAttribute('id');
        if (id) {
          sectionIds.push(id);
        }
      }

      return sectionIds;
    };

    test("Home app shows correct settings sections in order", async ({ page }) => {
      await page.goto("/");
      await page.waitForLoadState("networkidle");
      await openSettingsPanel(page);

      const sections = await getSettingsSections(page);

      // Verify sections exist
      expect(sections).toContain("semio.sketchpad.app.home.settings");
      expect(sections).toContain("semio.sketchpad.settings");

      // Verify order: Home (most specific) before Sketchpad (least specific)
      const homeIndex = sections.indexOf("semio.sketchpad.app.home.settings");
      const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");

      expect(homeIndex).toBeGreaterThanOrEqual(0);
      expect(sketchpadIndex).toBeGreaterThanOrEqual(0);
      expect(homeIndex).toBeLessThan(sketchpadIndex);
    });

    test("Kit app shows correct settings sections in order", async ({ page }) => {
      // Navigate to home and create a kit
      await page.goto("/");
      await page.waitForLoadState("networkidle");
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.waitForTimeout(1000);

      await openSettingsPanel(page);

      const sections = await getSettingsSections(page);

      // Verify sections exist
      expect(sections).toContain("semio.sketchpad.app.kit.settings");
      expect(sections).toContain("semio.sketchpad.settings");

      // Verify order: Kit (most specific) before Sketchpad (least specific)
      const kitIndex = sections.indexOf("semio.sketchpad.app.kit.settings");
      const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");

      expect(kitIndex).toBeGreaterThanOrEqual(0);
      expect(sketchpadIndex).toBeGreaterThanOrEqual(0);
      expect(kitIndex).toBeLessThan(sketchpadIndex);
    });

    test("Design app shows correct settings sections in order", async ({ page }) => {
      // Navigate to home, create a kit, then create a design
      await page.goto("/");
      await page.waitForLoadState("networkidle");
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.waitForTimeout(1000);
      await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
      await page.waitForTimeout(1000);

      await openSettingsPanel(page);

      const sections = await getSettingsSections(page);

      // Verify sections exist
      expect(sections).toContain("semio.sketchpad.app.design.settings");
      expect(sections).toContain("semio.sketchpad.app.kit.settings");
      expect(sections).toContain("semio.sketchpad.settings");

      // Verify order: Design (most specific) > Kit (middle) > Sketchpad (least specific)
      const designIndex = sections.indexOf("semio.sketchpad.app.design.settings");
      const kitIndex = sections.indexOf("semio.sketchpad.app.kit.settings");
      const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");

      expect(designIndex).toBeGreaterThanOrEqual(0);
      expect(kitIndex).toBeGreaterThanOrEqual(0);
      expect(sketchpadIndex).toBeGreaterThanOrEqual(0);
      expect(designIndex).toBeLessThan(kitIndex);
      expect(kitIndex).toBeLessThan(sketchpadIndex);
    });

    test("Type app shows correct settings sections in order", async ({ page }) => {
      // Navigate to home, create a kit, then create a type
      await page.goto("/");
      await page.waitForLoadState("networkidle");
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.waitForTimeout(1000);
      await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();
      await page.waitForTimeout(1000);

      await openSettingsPanel(page);

      const sections = await getSettingsSections(page);

      // Verify sections exist
      expect(sections).toContain("semio.sketchpad.app.type.settings");
      expect(sections).toContain("semio.sketchpad.app.kit.settings");
      expect(sections).toContain("semio.sketchpad.settings");

      // Verify order: Type (most specific) > Kit (middle) > Sketchpad (least specific)
      const typeIndex = sections.indexOf("semio.sketchpad.app.type.settings");
      const kitIndex = sections.indexOf("semio.sketchpad.app.kit.settings");
      const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");

      expect(typeIndex).toBeGreaterThanOrEqual(0);
      expect(kitIndex).toBeGreaterThanOrEqual(0);
      expect(sketchpadIndex).toBeGreaterThanOrEqual(0);
      expect(typeIndex).toBeLessThan(kitIndex);
      expect(kitIndex).toBeLessThan(sketchpadIndex);
    });

    test("All apps have global Sketchpad settings available", async ({ page }) => {
      const apps = [
        { name: "Home", setup: async () => { await page.goto("/"); } },
        {
          name: "Kit",
          setup: async () => {
            await page.goto("/");
            await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
            await page.waitForTimeout(1000);
          }
        },
        {
          name: "Design",
          setup: async () => {
            await page.goto("/");
            await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
            await page.waitForTimeout(1000);
            await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
            await page.waitForTimeout(1000);
          }
        },
        {
          name: "Type",
          setup: async () => {
            await page.goto("/");
            await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
            await page.waitForTimeout(1000);
            await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();
            await page.waitForTimeout(1000);
          }
        },
      ];

      for (const app of apps) {
        await page.goto("/");
        await page.waitForLoadState("networkidle");
        await app.setup();
        await openSettingsPanel(page);

        const sections = await getSettingsSections(page);

        // All apps should have Sketchpad settings
        expect(sections, `${app.name} should have Sketchpad settings`).toContain("semio.sketchpad.settings");
      }
    });
  });
});
