// SPDX-License-Identifier: AGPL-3.0-only
import { test } from "@playwright/test";

test("analyze kit diagram node dimensions", async ({ page }) => {
  await page.goto("http://localhost:5173/home");
  await page.waitForTimeout(1500);
  
  const createBtn = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
  await createBtn.waitFor({ state: "visible", timeout: 10000 });
  await createBtn.click();
  await page.waitForTimeout(1000);
  
  await page.locator('[id="semio.sketchpad.app.kit.createType"]').click();
  await page.waitForTimeout(500);
  await page.locator('button[id="semio.sketchpad.navbar.back"]').click();
  await page.waitForTimeout(500);
  
  await page.locator('[id="semio.sketchpad.app.kit.createType"]').click();
  await page.waitForTimeout(500);
  await page.locator('button[id="semio.sketchpad.navbar.back"]').click();
  await page.waitForTimeout(1000);
  
  const diagramToggle = page.locator('button').filter({ hasText: /diagram/i }).first();
  if (await diagramToggle.isVisible()) {
    await diagramToggle.click();
    await page.waitForTimeout(2000);
  }
  
  const kitNodes = page.locator('[data-kit-node]');
  const kitNodeCount = await kitNodes.count();
  console.log(`\n=== KIT DIAGRAM NODE ANALYSIS ===`);
  console.log(`Found ${kitNodeCount} kit diagram nodes`);
  
  if (kitNodeCount > 0) {
    const firstNode = kitNodes.first();
    const nodeBox = await firstNode.boundingBox();
    console.log(`\nNode container bounding box:`, nodeBox);
    
    const nodeStyle = await firstNode.evaluate((el) => {
      const computed = window.getComputedStyle(el);
      return {
        width: computed.width,
        height: computed.height,
        display: computed.display,
        padding: computed.padding,
        margin: computed.margin,
        border: computed.border,
      };
    });
    console.log(`Node computed style:`, nodeStyle);
    
    const avatar = firstNode.locator('[data-slot="avatar"]').first();
    const avatarBox = await avatar.boundingBox();
    console.log(`\nAvatar bounding box:`, avatarBox);
    
    if (avatarBox) {
      const avatarRadius = Math.min(avatarBox.width, avatarBox.height) / 2;
      console.log(`Avatar radius: ${avatarRadius}px`);
    }
    
    const avatarStyle = await avatar.evaluate((el) => {
      const computed = window.getComputedStyle(el);
      return {
        width: computed.width,
        height: computed.height,
        border: computed.borderWidth,
        padding: computed.padding,
      };
    });
    console.log(`Avatar computed style:`, avatarStyle);
    
    console.log(`\n=== EXPECTED VALUES ===`);
    console.log(`ICON_WIDTH: 50px`);
    console.log(`NODE_SCALE: 2`);
    console.log(`NODE_WIDTH: 100px`);
    console.log(`NODE_HEIGHT: 100px`);
    console.log(`NODE_RADIUS (for edges): 50px`);
    console.log(`\n=== ANALYSIS ===`);
    console.log(`Node container size: ${nodeBox?.width}px x ${nodeBox?.height}px`);
    console.log(`Avatar size: ${avatarBox?.width}px x ${avatarBox?.height}px`);
    const expectedRadius = 50;
    const actualRadius = avatarBox ? Math.min(avatarBox.width, avatarBox.height) / 2 : 0;
    const diff = Math.abs(actualRadius - expectedRadius);
    console.log(`Radius difference: ${diff}px`);
    console.log(`Status: ${diff < 2 ? "✅ ALIGNED" : "❌ MISALIGNED"}`);
  }
  
  await page.screenshot({ path: "/tmp/kit-diagram-debug.png", fullPage: true });
  console.log(`\nScreenshot saved to /tmp/kit-diagram-debug.png`);
});
