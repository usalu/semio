// SPDX-License-Identifier: AGPL-3.0-only
import { test, expect } from "@playwright/test";

test.describe("Kit Diagram Node-Edge Alignment", () => {
  test("measure node avatar size vs edge connection points", async ({ page }) => {
    await page.goto("http://localhost:5173");

    await page.locator('[id="compose.sketchpad.navbar.home"]').click();

    await page.locator('[id="compose.sketchpad.app.home.createTemporary"]').click();
    await page.waitForTimeout(500);

    await page.locator('[id="compose.sketchpad.app.kit.createType"]').click();
    await page.waitForTimeout(500);
    await page.locator('[id="compose.sketchpad.navbar.back"]').click();
    await page.waitForTimeout(500);

    await page.locator('[id="compose.sketchpad.app.kit.createType"]').click();
    await page.waitForTimeout(500);
    await page.locator('[id="compose.sketchpad.navbar.back"]').click();
    await page.waitForTimeout(500);

    const diagramToggle = page.locator('[id="compose.sketchpad.app.kit.windowKind.diagram"]');
    await expect(diagramToggle).toBeVisible();
    await diagramToggle.click();
    await page.waitForTimeout(1000);

    const nodes = page.locator("[data-id]").filter({ has: page.locator('[data-slot="avatar"]') });
    const nodeCount = await nodes.count();
    console.log(`Found ${nodeCount} nodes in diagram`);

    if (nodeCount > 0) {
      const firstNode = nodes.first();
      await expect(firstNode).toBeVisible();

      const avatar = firstNode.locator('[data-slot="avatar"]').first();
      await expect(avatar).toBeVisible();

      const avatarBox = await avatar.boundingBox();
      console.log("Avatar bounding box:", avatarBox);

      const avatarWidth = avatarBox?.width ?? 0;
      const avatarHeight = avatarBox?.height ?? 0;
      const avatarRadius = Math.min(avatarWidth, avatarHeight) / 2;
      console.log(`Avatar dimensions: ${avatarWidth}x${avatarHeight}, radius: ${avatarRadius}`);

      const computedStyle = await avatar.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return {
          width: style.width,
          height: style.height,
          border: style.borderWidth,
          padding: style.padding,
        };
      });
      console.log("Avatar computed style:", computedStyle);

      const spacing = await page.evaluate(() => {
        const root = document.documentElement;
        const spacingValue = getComputedStyle(root).getPropertyValue("--spacing");
        const spacingSingle = getComputedStyle(root).getPropertyValue("--spacing-single");
        const sizeSmall = getComputedStyle(root).getPropertyValue("--size-small");
        return { spacing: spacingValue, spacingSingle, sizeSmall };
      });
      console.log("CSS variables:", spacing);

      const edges = page.locator('path[class*="react-flow__edge-path"]');
      const edgeCount = await edges.count();
      console.log(`Found ${edgeCount} edges in diagram`);

      if (edgeCount > 0) {
        const firstEdge = edges.first();
        const edgePath = await firstEdge.getAttribute("d");
        console.log("Edge path data:", edgePath);

        const pathParts = edgePath?.match(/M\s*([\d.]+)\s+([\d.]+)/);
        if (pathParts) {
          const edgeStartX = parseFloat(pathParts[1]);
          const edgeStartY = parseFloat(pathParts[2]);
          console.log(`Edge start point: (${edgeStartX}, ${edgeStartY})`);

          const nodeBox = await firstNode.boundingBox();
          if (nodeBox && avatarBox) {
            const nodeCenterX = nodeBox.x + nodeBox.width / 2;
            const nodeCenterY = nodeBox.y + nodeBox.height / 2;
            console.log(`Node center: (${nodeCenterX}, ${nodeCenterY})`);

            const diagramContainer = page.locator(".react-flow").first();
            const containerBox = await diagramContainer.boundingBox();
            console.log("Diagram container:", containerBox);

            const avatarCenterX = avatarBox.x + avatarBox.width / 2;
            const avatarCenterY = avatarBox.y + avatarBox.height / 2;
            console.log(`Avatar center: (${avatarCenterX}, ${avatarCenterY})`);
          }
        }
      }

      const ICON_WIDTH = 50;
      const NODE_SCALE = 2;
      const expectedNodeWidth = ICON_WIDTH * NODE_SCALE;
      const expectedNodeHeight = ICON_WIDTH * NODE_SCALE;
      const expectedNodeRadius = Math.min(expectedNodeWidth, expectedNodeHeight) / 2;

      console.log("Expected node dimensions from code:");
      console.log(`  NODE_WIDTH: ${expectedNodeWidth}`);
      console.log(`  NODE_HEIGHT: ${expectedNodeHeight}`);
      console.log(`  NODE_RADIUS: ${expectedNodeRadius}`);

      const sizeSmallPixels = parseFloat(spacing.sizeSmall);
      console.log(`CSS --size-small in pixels: ${sizeSmallPixels}`);
      console.log(`Avatar radius vs expected node radius: ${avatarRadius} vs ${expectedNodeRadius}`);
      console.log(`Difference: ${Math.abs(avatarRadius - expectedNodeRadius)}px`);

      const tolerance = 2;
      const isAligned = Math.abs(avatarRadius - expectedNodeRadius) <= tolerance;
      console.log(`Alignment check (tolerance ${tolerance}px): ${isAligned ? "PASS" : "FAIL"}`);

      if (!isAligned) {
        console.log("PROBLEM: Avatar radius does not match expected node radius for edge calculations!");
      }
    }
  });

  test("verify consistent node dimensions across diagram", async ({ page }) => {
    await page.goto("http://localhost:5173");

    await page.locator('[id="compose.sketchpad.navbar.home"]').click();
    await page.locator('[id="compose.sketchpad.app.home.createTemporary"]').click();
    await page.waitForTimeout(500);

    for (let i = 0; i < 3; i++) {
      await page.locator('[id="compose.sketchpad.app.kit.createType"]').click();
      await page.waitForTimeout(300);
      await page.locator('[id="compose.sketchpad.navbar.back"]').click();
      await page.waitForTimeout(300);
    }

    const diagramToggle = page.locator('[id="compose.sketchpad.app.kit.windowKind.diagram"]');
    await diagramToggle.click();
    await page.waitForTimeout(1000);

    const nodes = page.locator("[data-id]").filter({ has: page.locator('[data-slot="avatar"]') });
    const nodeCount = await nodes.count();
    console.log(`Checking ${nodeCount} nodes for consistent dimensions`);

    const dimensions: { width: number; height: number; radius: number }[] = [];

    for (let i = 0; i < Math.min(nodeCount, 5); i++) {
      const node = nodes.nth(i);
      const avatar = node.locator('[data-slot="avatar"]').first();
      const box = await avatar.boundingBox();

      if (box) {
        const radius = Math.min(box.width, box.height) / 2;
        dimensions.push({ width: box.width, height: box.height, radius });
        console.log(`Node ${i}: ${box.width}x${box.height}, radius: ${radius}`);
      }
    }

    if (dimensions.length > 1) {
      const firstRadius = dimensions[0].radius;
      const allSame = dimensions.every((d) => Math.abs(d.radius - firstRadius) < 1);
      console.log(`All nodes have consistent radius: ${allSame ? "PASS" : "FAIL"}`);

      expect(allSame).toBe(true);
    }
  });
});
