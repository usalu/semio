import { test, expect } from '@playwright/test';

test('toolbar selection dropdown positioning and items', async ({ page }) => {

  await page.goto('/');

  // Click Create group on Home Toolbar
  await page.locator('#semio\\.sketchpad\\.toolbar\\.group\\.create').click();
  // Create temporary kit to reach Design App
  await page.locator('#semio\\.sketchpad\\.app\\.home\\.toolbar\\.createTemporary').click();
  
  // Wait for toolbar
  const toolbar = page.locator('#semio\\.sketchpad\\.toolbar');
  await expect(toolbar).toBeVisible();

  // Find the selection group toggle (Root ID)
  const selectionGroup = page.locator('#semio\\.sketchpad\\.toolbar\\.group\\.selection');
  await expect(selectionGroup).toBeVisible();

  // Find the chevron action button inside the selection group
  // It's the div inside the item that acts as the action trigger
  const actionButton = selectionGroup.locator('[data-slot="toggle-group-item"] > div').last(); 
  
  await actionButton.click();

  // Check if popover is open. Radix renders content in a portal, usually appended to body.
  // We look for text "Hand" which should be in the dropdown.
  // Or look for role="dialog" or similar which Radix Popover might use (it uses role="dialog" usually for non-modal).
  // Actually, just searching for text of known items is good.
  
  // The subtools registered are "select" and "hand".
  // Assuming i18n falls back or keys contain these words.
  // In Design.tsx: subToolId: "hand", subToolLabelId: "...hand"
  // Even if i18n missing, likely key or fallback used? No, I18next usually returns key.
  // So "semio.sketchpad.toolbar.subtool.hand"
  
  // Or maybe "Select"? Let's search loosely.
  const selectOption = page.getByRole('button').filter({ hasText: /select/i }).first();
  await expect(selectOption).toBeVisible();

  // Positioning check
  const buttonBox = await actionButton.boundingBox();
  const popoverBox = await selectOption.locator('xpath=../../..').boundingBox(); // Up to PopoverContent?
  
  // Radix Content wrapper
  const contentWrapper = page.locator('[data-side="top"]'); // Radix adds data-side attribute
  await expect(contentWrapper).toBeVisible();
  
  if (buttonBox) {
      const wrapperBox = await contentWrapper.boundingBox();
      if (wrapperBox) {
          // Verify it is ABOVE the button (since side="top")
          // Y increases downwards. So Popover Y should be < Button Y
          expect(wrapperBox.y + wrapperBox.height).toBeLessThanOrEqual(buttonBox.y + 10); // Allow small overlap/margin
          
          // Verify horizontal centering (approximate)
          const buttonCenter = buttonBox.x + buttonBox.width / 2;
          const popoverCenter = wrapperBox.x + wrapperBox.width / 2;
          expect(Math.abs(popoverCenter - buttonCenter)).toBeLessThan(50);
      }
  }
});
