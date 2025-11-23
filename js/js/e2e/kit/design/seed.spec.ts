import { test, expect } from '@playwright/test';

test.describe('design', () => {
  test('seed', async ({ page }) => {
    // Requires kit seed to run first

    // 1. Navigate to http://localhost:5173
    await page.goto('http://localhost:5173');
    await page.waitForLoadState('networkidle');

    // 2. Create a kit if not exists
    const createKitButton = page.getByRole('button').nth(5);
    await createKitButton.click();
    await page.waitForTimeout(1000);

    // 3. Create a design
    const createButtons = page.locator('.group\\/toggle-group.flex.w-fit.items-center.border button');
    await createButtons.first().click();
    await page.waitForTimeout(1000);

    // 4. Verify the design app loaded with diagram canvas
    await expect(page.getByText('diagram')).toBeVisible();
    await expect(page.getByText('New Design')).toBeVisible();

    // Verify we're in the design app (application elements visible)
    const canvas = page.locator('application').first();
    await expect(canvas).toBeVisible();
  });
});
