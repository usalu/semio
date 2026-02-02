import { test, expect } from '@playwright/test';

test('debug kit toolbar visibility', async ({ page }) => {
  await page.goto('http://localhost:5173');
  await page.waitForTimeout(2000);
  
  // Create a temporary kit
  const createBtn = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
  if (await createBtn.isVisible()) {
    await createBtn.click();
    await page.waitForTimeout(2000);
  }
  
  // Take screenshot of the page
  await page.screenshot({ path: '/tmp/kit-page.png', fullPage: true });
  
  // Check toolbar
  const toolbar = page.locator('.bg-panel').filter({ hasText: /filter|selection/i }).first();
  const toolbarExists = await toolbar.count() > 0;
  console.log('Toolbar exists:', toolbarExists);
  
  if (toolbarExists) {
    const toolbarHTML = await toolbar.innerHTML();
    console.log('Toolbar HTML:', toolbarHTML);
  }
  
  // Check for specific toolbar elements
  const selectionTools = page.locator('[id*="selection"]');
  const selectionToolsCount = await selectionTools.count();
  console.log('Selection tool elements found:', selectionToolsCount);
  
  const filterToggles = page.locator('[id*="showDesigns"], [id*="showTypes"]');
  const filterTogglesCount = await filterToggles.count();
  console.log('Filter toggle elements found:', filterTogglesCount);
  
  // Get all elements with toolbar-related IDs
  const allToolbarElements = page.locator('[id*="toolbar"], [id*="filter"], [id*="selection"]');
  const allCount = await allToolbarElements.count();
  console.log('All toolbar-related elements:', allCount);
  
  for (let i = 0; i < Math.min(allCount, 10); i++) {
    const el = allToolbarElements.nth(i);
    const id = await el.getAttribute('id');
    const isVisible = await el.isVisible();
    console.log(`Element ${i}: id="${id}" visible=${isVisible}`);
  }
});
