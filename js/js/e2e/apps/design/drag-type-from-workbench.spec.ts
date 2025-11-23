import { test, expect } from '@playwright/test';

test.describe('Design App - Drag and Drop', () => {
  test('Drag Type from Workbench to Diagram', async ({ page }) => {
    // Navigate to Storybook
    await page.goto('http://localhost:6006');

    // Wait for Storybook to load
    await page.waitForLoadState('networkidle');

    // TODO: Navigate to the Kit app in Storybook
    // You may need to click on the sidebar to select the right story

    // Access the preview iframe
    const preview = page.frameLocator('iframe#storybook-preview-iframe');

    // SEED: Create a temporary kit
    // TODO: Implement kit creation based on your UI
    // Example: await preview.locator('[aria-label="Create Kit"]').click();

    // SEED: Create a type
    // TODO: Click button to create a new type
    // Example: await preview.locator('[aria-label="Add Type"]').click();
    const typeName = `TestType_${Date.now()}`;

    // SEED: Navigate back to kit
    // TODO: Navigate back using breadcrumb or back button

    // SEED: Create a design
    // TODO: Click button to create a new design
    // Example: await preview.locator('[aria-label="Add Design"]').click();
    const designName = `TestDesign_${Date.now()}`;

    // SEED: Navigate back to kit
    // TODO: Navigate back

    // SEED: Open the design
    // TODO: Double-click or click on the design to open it

    // TEST: Toggle workbench panel
    // Based on code analysis: the panel is toggled via a button
    await preview.locator('[data-panel="workbench"], button:has-text("Workbench")').click();

    // Wait for workbench panel to be visible
    await page.waitForTimeout(500); // Allow animation to complete

    // Verify type is visible in workbench
    // Based on code: types are rendered in the workbench tree with draggable IDs like "type-{guid}"
    const typeInWorkbench = preview.locator('[id^="type-"]').first();
    await expect(typeInWorkbench).toBeVisible();

    // Get the diagram canvas
    // Based on code: diagram has id="diagram" and data-diagram-id
    const diagram = preview.locator('#diagram, [data-diagram-id]').first();
    await expect(diagram).toBeVisible();

    // Perform drag and drop
    const sourceBox = await typeInWorkbench.boundingBox();
    const targetBox = await diagram.boundingBox();

    if (!sourceBox || !targetBox) {
      throw new Error('Could not get bounding boxes for drag operation');
    }

    // Start drag from type
    await page.mouse.move(
      sourceBox.x + sourceBox.width / 2,
      sourceBox.y + sourceBox.height / 2
    );
    await page.mouse.down();

    // Move to center of diagram
    const dropX = targetBox.x + targetBox.width / 2;
    const dropY = targetBox.y + targetBox.height / 2;
    await page.mouse.move(dropX, dropY, { steps: 10 });

    // Drop
    await page.mouse.up();

    // Wait for piece to be created
    await page.waitForTimeout(500);

    // Verify piece appears in diagram
    // Based on code: React Flow nodes are created with class react-flow__node
    const piece = preview.locator('.react-flow__node').first();
    await expect(piece).toBeVisible({ timeout: 5000 });

    // Verify the piece is positioned approximately where we dropped it
    const pieceBox = await piece.boundingBox();
    if (pieceBox) {
      const pieceCenterX = pieceBox.x + pieceBox.width / 2;
      const pieceCenterY = pieceBox.y + pieceBox.height / 2;

      // Allow 200px tolerance for positioning
      expect(Math.abs(pieceCenterX - dropX)).toBeLessThan(200);
      expect(Math.abs(pieceCenterY - dropY)).toBeLessThan(200);
    }
  });
});
