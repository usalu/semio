import { expect, test } from '@playwright/test';

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
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.navbar.back"]').click();
        await page.waitForTimeout(500);
        await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
        await page.waitForTimeout(500);
      });
      test('Windows visible', async ({ page }) => {
        const diagramWindow = page.locator('text=diagram').first();
        await expect(diagramWindow).toBeVisible();
        const sceneWindow = page.locator('text=scene').first();
        await expect(sceneWindow).toBeVisible();
      });
      // test('Drag and Drop Pieces', async ({ page }) => {
      //   await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]').click();
      //   await page.waitForTimeout(500);
      //   const workbenchDialog = page.locator('[role="dialog"]');
      //   await expect(workbenchDialog).toBeVisible();
      //   const draggableItem = workbenchDialog.locator('[draggable="true"]').first();
      //   const typeItems = workbenchDialog.locator('button, [role="listitem"]').first();
      //   const canvas = page.locator('application').first();
      //   await expect(canvas).toBeVisible();
      //   const sourceElement = (await draggableItem.count() > 0) ? draggableItem : typeItems;
      //   const targetElement = canvas;
      //   await sourceElement.dragTo(targetElement, {
      //     sourcePosition: { x: 10, y: 10 },
      //     targetPosition: { x: 200, y: 200 }
      //   });
      //   await page.waitForTimeout(1000);
      //   const canvasContent = page.locator('application').first();
      //   await expect(canvasContent).toBeVisible();
      // });
    });
  });
});
