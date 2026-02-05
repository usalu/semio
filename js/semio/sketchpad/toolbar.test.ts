
import { test, expect } from '@playwright/test';

test('toolbar visibility', async ({ page }) => {
  // Go to the home page
  await page.goto("http://localhost:3000/");

  // Wait for the app to load
  await expect(page.locator("#semio\\.sketchpad\\.navbar\\.navigationButtons")).toBeVisible();

  // The toolbar should be visible
  const toolbar = page.locator("#semio\\.sketchpad\\.toolbar");
  await expect(toolbar).toBeVisible();

  // Check if it has dimensions
  const box = await toolbar.boundingBox();
  expect(box).not.toBeNull();
  expect(box!.width).toBeGreaterThan(0);
  expect(box!.height).toBeGreaterThan(0);

  // Check the inner container visibility
  const innerContainer = toolbar.locator("div.flex.gap-single.items-center.pointer-events-auto");
  await expect(innerContainer).toBeVisible();
  
  // Check that the inner container has width (this is likely where the issue is with w-fit/calc)
  const innerBox = await innerContainer.boundingBox();
  expect(innerBox).not.toBeNull();
  expect(innerBox!.width).toBeGreaterThan(0);

  // In Home app, we expect 'filter' and 'create' to be present usually
  // Let's check for any group toggle
  const anyGroup = toolbar.locator("button[id^='semio\\.sketchpad\\.toolbar\\.group\\.']").first();
  await expect(anyGroup).toBeVisible();
});
