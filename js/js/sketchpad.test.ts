import { expect, Locator, Page, test } from '@playwright/test';

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

test.describe('sketchpad', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Kit', () => {
    test.beforeEach(async ({ page }) => {
      await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
      await page.waitForTimeout(1000);
    });
    test.describe('Design', () => {
      test.beforeEach(async ({ page }) => {
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
        await page.waitForTimeout(500);
      });
      test('Windows', async ({ page }) => {
        const diagramWindow = page.locator('text=diagram').first();
        const sceneWindow = page.locator('text=scene').first();
        await expect(diagramWindow).toBeVisible();
        await expect(sceneWindow).toBeVisible();
        await expect(diagramWindow).toBeInViewport();
        await expect(sceneWindow).toBeInViewport();
        await expectFullyInViewport(diagramWindow, page, [0, 100], [0, 100]);
        await expectFullyInViewport(sceneWindow, page, [400, 800], [0, 100]);
      });
      test('Drag and Drop Pieces', async ({ page }) => {
        await page.locator('[id="semio.sketchpad.navbar.back"]').click();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.navbar.back"]').click();
        await page.waitForTimeout(500);
        await page.getByRole('button', { name: 'Design' }).dblclick();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]').click();
        await page.waitForTimeout(500);
        const draggableTypeAvatar = page.locator('[id="semio.sketchpad.workbench.draggableTypeAvatar"]').first();
        await draggableTypeAvatar.dragTo(page.locator('[id="semio.sketchpad.workbench.canvas"]').first(), {
          sourcePosition: { x: 10, y: 10 },
          targetPosition: { x: 200, y: 200 }
        });
      });
    });
  });
});
