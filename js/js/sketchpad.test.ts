import { expect, Locator, Page, test } from "@playwright/test";

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
    test("Import metabolism.zip creates temporary kit with all types, designs, ports, pieces, and files", async ({ page }) => {
      test.setTimeout(120000);
      const dataTransfer = await page.evaluateHandle(async () => {
        const response = await fetch(`/assets/semio/metabolism.zip`);
        if (!response.ok) throw new Error(`Failed to fetch metabolism.zip: ${response.status}`);
        const blob = await response.blob();
        const file = new File([blob], "metabolism.zip", { type: "application/zip" });
        const dt = new DataTransfer();
        dt.items.add(file);
        return dt;
      });
      const canvas = page.locator("body");
      await canvas.dispatchEvent("dragover", { dataTransfer });
      await page.waitForTimeout(500);
      await canvas.dispatchEvent("drop", { dataTransfer });
      await page.waitForTimeout(5000);
      await expect(page).toHaveURL(/.*kit\/.+/);
      const kitState = await page.evaluate(() => {
        const store = (window as any).__SKETCHPAD_STORE__;
        if (!store) return null;
        const state = store.getCompleteState?.() || store.snapshot?.();
        if (!state) return null;
        const kits = state.kits || [];
        if (kits.length === 0) return null;
        const lastKit = kits[kits.length - 1];
        return {
          kit: lastKit.kit,
          local: lastKit.local,
          remote: lastKit.remote,
        };
      });
      expect(kitState).not.toBeNull();
      expect(kitState!.local).toBe(false);
      expect(kitState!.remote).toBe(false);
      const kit = kitState!.kit;
      expect(kit).toBeDefined();
      expect(kit.name).toBe("Metabolism");
      const expectedTypeNames = ["Capsule", "Ellipsoid", "Trapezoid", "Balcony", "Base", "Blob", "Bridge", "Capital", "Cylindric Capital", "Tambour", "Cylindric Tambour"];
      const typeNames = kit.types?.map((t: any) => t.name) || [];
      for (const expectedName of expectedTypeNames) {
        expect(typeNames).toContain(expectedName);
      }
      const expectedDesignNames = ["Nakagin Capsule Tower", "Capsule Dream"];
      const protoDesigns = kit.designs?.filter((d: any) => !d.parent) || [];
      const protoDesignNames = protoDesigns.map((d: any) => d.name);
      for (const expectedName of expectedDesignNames) {
        expect(protoDesignNames).toContain(expectedName);
      }
      const tambour = kit.types?.find((t: any) => t.name === "Tambour");
      expect(tambour).toBeDefined();
      expect(tambour.ports).toBeDefined();
      expect(tambour.ports.length).toBe(10);
      const expectedPorts = [
        { name: "b", point: { x: 0, y: 0, z: 0.9166667 }, direction: { x: 0, y: 0, z: -1 }, t: 0.5 },
        { name: "t", point: { x: 0, y: 0, z: 3.6666667 }, direction: { x: 0, y: 0, z: 1 }, t: 0 },
        { name: "sl0_d0", point: { x: 2.75, y: 0.9, z: 0.2 }, direction: { x: 1, y: 0, z: 0 }, t: 0.2 },
        { name: "sl0_d1", point: { x: 0.9, y: 2.75, z: 0.2 }, direction: { x: 0, y: 1, z: 0 }, t: 0.05 },
        { name: "sl0_d2", point: { x: -0.9, y: 2.75, z: 0.2 }, direction: { x: 0, y: 1, z: 0 }, t: 0.95 },
        { name: "sl0_d3", point: { x: -2.75, y: 0.9, z: 0.2 }, direction: { x: -1, y: 0, z: 0 }, t: 0.8 },
        { name: "sl1_d0", point: { x: -2.75, y: -0.9, z: 1.1166667 }, direction: { x: -1, y: 0, z: 0 }, t: 0.7 },
        { name: "sl1_d1", point: { x: -0.9, y: -2.75, z: 1.1166667 }, direction: { x: 0, y: -1, z: 0 }, t: 0.55 },
        { name: "sl2_d0", point: { x: 0.9, y: -2.75, z: 2.0333333 }, direction: { x: 0, y: -1, z: 0 }, t: 0.45 },
        { name: "sl2_d1", point: { x: 2.75, y: -0.9, z: 2.0333333 }, direction: { x: 1, y: 0, z: 0 }, t: 0.3 },
      ];
      const tolerance = 0.001;
      for (const expected of expectedPorts) {
        const port = tambour.ports.find((p: any) => p.name === expected.name);
        expect(port).toBeDefined();
        expect(Math.abs(port.point.x - expected.point.x)).toBeLessThan(tolerance);
        expect(Math.abs(port.point.y - expected.point.y)).toBeLessThan(tolerance);
        expect(Math.abs(port.point.z - expected.point.z)).toBeLessThan(tolerance);
        expect(Math.abs(port.direction.x - expected.direction.x)).toBeLessThan(tolerance);
        expect(Math.abs(port.direction.y - expected.direction.y)).toBeLessThan(tolerance);
        expect(Math.abs(port.direction.z - expected.direction.z)).toBeLessThan(tolerance);
        expect(Math.abs(port.t - expected.t)).toBeLessThan(tolerance);
      }
      const nakagin = kit.designs?.find((d: any) => d.name === "Nakagin Capsule Tower" && !d.parent);
      expect(nakagin).toBeDefined();
      expect(nakagin.pieces).toBeDefined();
      expect(nakagin.pieces.length).toBe(180);
      const files = await page.evaluate(() => {
        const store = (window as any).__SKETCHPAD_STORE__;
        if (!store) return [];
        const state = store.getCompleteState?.() || store.snapshot?.();
        if (!state) return [];
        const kits = state.kits || [];
        if (kits.length === 0) return [];
        const lastKit = kits[kits.length - 1];
        return lastKit.kit.files?.map((f: any) => f.path) || [];
      });
      const hasSemioFolder = files.some((f: string) => f.startsWith(".semio/") || f.startsWith(".semio\\"));
      expect(hasSemioFolder).toBe(false);
      const hasRepresentations = files.some((f: string) => f.includes("representations"));
      const hasIcons = files.some((f: string) => f.includes("icons"));
      expect(hasRepresentations).toBe(true);
      expect(hasIcons).toBe(true);
      const representationFiles = files.filter((f: string) => f.includes("representations"));
      const iconFiles = files.filter((f: string) => f.includes("icons"));
      expect(representationFiles.length).toBeGreaterThan(100);
      expect(iconFiles.length).toBeGreaterThan(30);
      const expectedRepFiles = ["base.glb", "tambour.glb", "capsule_backslash.glb", "capital.glb", "bridge.glb"];
      for (const expected of expectedRepFiles) {
        const found = representationFiles.some((f: string) => f.includes(expected));
        expect(found).toBe(true);
      }
      const expectedIconFiles = ["base.svg", "tambour.svg", "metabolism.svg", "capital.svg"];
      for (const expected of expectedIconFiles) {
        const found = iconFiles.some((f: string) => f.includes(expected));
        expect(found).toBe(true);
      }
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
});
