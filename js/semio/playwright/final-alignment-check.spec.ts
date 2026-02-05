// SPDX-License-Identifier: AGPL-3.0-only
import { test, expect } from "@playwright/test";

test("verify kit diagram node-avatar alignment", async ({ page }) => {
  test.setTimeout(90000);
  
  console.log("Navigating to home...");
  await page.goto("http://localhost:5173/home");
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(2000);
  
  console.log("Looking for create button...");
  const createBtn = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
  const isVisible = await createBtn.isVisible();
  console.log(`Create button visible: ${isVisible}`);
  
  if (!isVisible) {
    console.log("Taking screenshot of home page...");
    await page.screenshot({ path: "/tmp/home-page-debug.png", fullPage: true });
    console.log("Screenshot saved to /tmp/home-page-debug.png");
    
    const homeElements = await page.locator('[id*="home"]').count();
    console.log(`Found ${homeElements} elements with "home" in ID`);
    
    for (let i = 0; i < Math.min(homeElements, 10); i++) {
      const el = page.locator('[id*="home"]').nth(i);
      const id = await el.getAttribute("id");
      const visible = await el.isVisible();
      console.log(`  Element ${i}: id="${id}" visible=${visible}`);
    }
    throw new Error("Create button not found");
  }
  
  console.log("Clicking create temporary kit...");
  await createBtn.click();
  await page.waitForTimeout(1500);
  
  console.log("Creating types...");
  for (let i = 0; i < 3; i++) {
    const createTypeBtn = page.locator('[id="semio.sketchpad.app.kit.createType"]');
    await createTypeBtn.waitFor({ state: "visible", timeout: 5000 });
    await createTypeBtn.click();
    await page.waitForTimeout(400);
    
    const backBtn = page.locator('button[id="semio.sketchpad.navbar.back"]');
    await backBtn.waitFor({ state: "visible", timeout: 5000 });
    await backBtn.click();
    await page.waitForTimeout(400);
  }
  
  console.log("Switching to diagram view...");
  await page.waitForTimeout(1000);
  
  const diagramButtons = page.locator('button[id*="diagram"]');
  const diagramBtnCount = await diagramButtons.count();
  console.log(`Found ${diagramBtnCount} diagram buttons`);
  
  for (let i = 0; i < diagramBtnCount; i++) {
    const btn = diagramButtons.nth(i);
    const id = await btn.getAttribute("id");
    const visible = await btn.isVisible();
    console.log(`  Diagram button ${i}: id="${id}" visible=${visible}`);
  }
  
  const diagramBtn = page.locator('button[id*="windowKind"][id*="diagram"]').first();
  await diagramBtn.waitFor({ state: "visible", timeout: 5000 });
  await diagramBtn.click();
  await page.waitForTimeout(3000);
  
  console.log("Taking screenshot of diagram...");
  await page.screenshot({ path: "/tmp/diagram-view.png", fullPage: true });
  
  console.log("Analyzing node dimensions...");
  const nodes = page.locator('[data-kit-node]');
  const nodeCount = await nodes.count();
  console.log(`Found ${nodeCount} kit diagram nodes`);
  
  expect(nodeCount).toBeGreaterThan(0);
  
  const firstNode = nodes.first();
  const avatar = firstNode.locator('[data-slot="avatar"]');
  
  await expect(firstNode).toBeVisible();
  await expect(avatar).toBeVisible();
  
  const nodeBox = await firstNode.boundingBox();
  const avatarBox = await avatar.boundingBox();
  
  console.log("\n=== DIMENSION ANALYSIS ===");
  console.log("Node container:", nodeBox);
  console.log("Avatar:", avatarBox);
  
  if (nodeBox && avatarBox) {
    const nodeRadius = Math.min(nodeBox.width, nodeBox.height) / 2;
    const avatarRadius = Math.min(avatarBox.width, avatarBox.height) / 2;
    
    console.log(`\nNode radius: ${nodeRadius}px`);
    console.log(`Avatar radius: ${avatarRadius}px`);
    console.log(`Difference: ${Math.abs(nodeRadius - avatarRadius)}px`);
    console.log(`Expected radius: 50px`);
    
    const nodeCorrect = Math.abs(nodeBox.width - 100) < 2 && Math.abs(nodeBox.height - 100) < 2;
    const avatarCorrect = Math.abs(avatarBox.width - 100) < 5 && Math.abs(avatarBox.height - 100) < 5;
    
    console.log(`\nNode size correct (100x100): ${nodeCorrect ? "✅" : "❌"}`);
    console.log(`Avatar size correct (100x100): ${avatarCorrect ? "✅" : "❌"}`);
    
    const aligned = Math.abs(avatarRadius - nodeRadius) < 5;
    console.log(`Alignment check: ${aligned ? "✅ PASS" : "❌ FAIL"}`);
    
    expect(nodeBox.width).toBeCloseTo(100, 0);
    expect(nodeBox.height).toBeCloseTo(100, 0);
    expect(avatarBox.width).toBeCloseTo(100, 0);
    expect(avatarBox.height).toBeCloseTo(100, 0);
  }
});
