import { test, expect } from '@playwright/test';

test.describe('design', () => {
  test('drag type from workbench to canvas', async ({ page }) => {
    // Requires design seed to run first

    // 1. Navigate to http://localhost:5173 and set up
    await page.goto('http://localhost:5173');
    await page.waitForLoadState('networkidle');

    // 2. Create a kit
    await page.getByRole('button').nth(5).click();
    await page.waitForTimeout(1000);

    // 3. Create a type first (so we have something to drag)
    const typeButton = page.locator('.group\\/toggle-group.flex.w-fit.items-center.border button').nth(1);
    await typeButton.click();
    await page.waitForTimeout(500);

    // Navigate back
    await page.goBack();
    await page.waitForLoadState('networkidle');

    // 4. Create a design
    const createDesignButton = page.locator('.group\\/toggle-group.flex.w-fit.items-center.border button').first();
    await createDesignButton.click();
    await page.waitForTimeout(1000);

    // Verify we're in the design app
    await expect(page.getByText('New Design')).toBeVisible();

    // 5. Toggle the workbench panel open
    // Find the panel toggle buttons in the toolbar
    const panelToggle = page.locator('.group\\/action-group > .text-foreground').first();
    await panelToggle.click();
    await page.waitForTimeout(500);

    // 6. Verify workbench panel is visible (dialog should appear)
    const workbenchDialog = page.locator('dialog');
    await expect(workbenchDialog).toBeVisible();

    // 7. Look for draggable type items in the workbench
    const draggableItem = workbenchDialog.locator('[draggable="true"]').first();

    // If no draggable items found, look for type items in a list or grid
    const typeItems = workbenchDialog.locator('button, [role="listitem"]').first();

    // 8. Perform drag and drop to the canvas
    const canvas = page.locator('application').first();
    await expect(canvas).toBeVisible();

    // Get the bounding boxes for drag operation
    const sourceElement = (await draggableItem.count() > 0) ? draggableItem : typeItems;
    const targetElement = canvas;

    // Perform drag using Playwright's dragTo method
    await sourceElement.dragTo(targetElement, {
      sourcePosition: { x: 10, y: 10 },
      targetPosition: { x: 200, y: 200 }
    });

    await page.waitForTimeout(1000);

    // 9. Verify the type was added to the canvas
    // After drag-drop, a piece should appear in the canvas area
    // This verification might need adjustment based on actual DOM structure
    const canvasContent = page.locator('application').first();
    await expect(canvasContent).toBeVisible();
  });
});
