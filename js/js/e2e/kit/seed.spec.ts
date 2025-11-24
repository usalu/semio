import { test, expect } from '@playwright/test';

test.describe('kit', () => {
  test('seed', async ({ page }) => {
    // 1. Navigate to http://localhost:5173
    await page.goto('http://localhost:5173');
    await page.waitForLoadState('networkidle');

    // 2. Create a temporary kit by clicking the create kit button
    await page.locator('#semio\\.sketchpad\\.app\\.home\\.createKit').click();
    await page.waitForTimeout(1000);

    // Verify we're in the kit view
    await expect(page.getByText('New Kit')).toBeVisible();

    // 3. Create a type
    await page.locator('#semio\\.sketchpad\\.app\\.kit\\.kitApp\\.createType').click();
    await page.waitForTimeout(500);

    // 4. Navigate back to kit view
    await page.goBack();
    await page.waitForLoadState('networkidle');

    // 5. Create a design
    await page.locator('#semio\\.sketchpad\\.app\\.kit\\.kitApp\\.createDesign').click();
    await page.waitForTimeout(1000);

    // Verify design was created
    await expect(page.getByText('New Design')).toBeVisible();

    // 6. Navigate back to kit view
    await page.goBack();
    await page.waitForLoadState('networkidle');

    // Verify we're back in the kit with the created artifacts
    await expect(page.getByText('Default')).toBeVisible();
  });
});
