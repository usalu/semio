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
        const diagramCenter = { x: diagramBox!.x + diagramBox!.width / 2, y: diagramBox!.y + diagramBox!.height / 2 };
        const sceneCenter = { x: sceneBox!.x + sceneBox!.width / 2, y: sceneBox!.y + sceneBox!.height / 2 };
        const countPieces = async () => {
          const reactFlowNodes = diagramDropZone.locator(".react-flow__node");
          return await reactFlowNodes.count();
        };
        const initialPieceCount = await countPieces();
        expect(initialPieceCount).toBe(0);
        await page.mouse.move(sourceCenter.x, sourceCenter.y);
        await page.mouse.down();
        await page.waitForTimeout(100);
        await page.mouse.move(diagramCenter.x, diagramCenter.y, { steps: 10 });
        await page.waitForTimeout(100);
        await page.mouse.up();
        await page.waitForTimeout(500);
        const diagramPieceCount = await countPieces();
        expect(diagramPieceCount).toBe(1);
        const diagramPieceNode = diagramDropZone.locator(".react-flow__node").first();
        const pieceBox = await diagramPieceNode.boundingBox();
        expect(pieceBox).not.toBeNull();
        const pieceCenter = { x: pieceBox!.x + pieceBox!.width / 2, y: pieceBox!.y + pieceBox!.height / 2 };
        const distanceToDrop = Math.sqrt((pieceCenter.x - diagramCenter.x) ** 2 + (pieceCenter.y - diagramCenter.y) ** 2);
        expect(distanceToDrop).toBeLessThan(50);
        await page.mouse.move(sourceCenter.x, sourceCenter.y);
        await page.mouse.down();
        await page.waitForTimeout(100);
        await page.mouse.move(sceneCenter.x, sceneCenter.y, { steps: 10 });
        await page.waitForTimeout(100);
        await page.mouse.up();
        await page.waitForTimeout(1000);
        const scenePieceCount = await countPieces();
        expect(scenePieceCount).toBe(2);
      });
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
