import { test } from '@playwright/test';

test.describe('sketchpad', () => {
  test('seed', async ({ page }) => {
    await page.goto('http://localhost:5173');
    await page.waitForLoadState('networkidle');
  });
});
