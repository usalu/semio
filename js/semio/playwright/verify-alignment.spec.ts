// SPDX-License-Identifier: AGPL-3.0-only
import { test, expect } from "@playwright/test";

test("kit diagram node-edge alignment verification", async ({ page }) => {
  test.setTimeout(60000);

  await page.goto("http://localhost:5173/home");
  await page.waitForLoadState("networkidle");

  const createBtn = page.locator('[id="semio.sketchpad.app.home.createTemporary"]');
  await createBtn.waitFor({ state: "visible", timeout: 15000 });
  await createBtn.click();
  await page.waitForTimeout(1000);

  for (let i = 0; i < 3; i++) {
    await page.locator('[id="semio.sketchpad.app.kit.createType"]').click();
    await page.waitForTimeout(300);
    await page.locator('button[id="semio.sketchpad.navbar.back"]').click();
    await page.waitForTimeout(300);
  }

  const diagramButton = page.locator('[id*="windowKind"][id*="diagram"]').first();
  if (await diagramButton.isVisible()) {
    await diagramButton.click();
    await page.waitForTimeout(2000);

    const nodes = page.locator("[data-kit-node]");
    const count = await nodes.count();

    if (count > 0) {
      const firstNode = nodes.first();
      const avatar = firstNode.locator('[data-slot="avatar"]').first();

      const nodeBox = await firstNode.boundingBox();
      const avatarBox = await avatar.boundingBox();

      console.log("Node dimensions:", nodeBox);
      console.log("Avatar dimensions:", avatarBox);

      if (nodeBox && avatarBox) {
        const nodeRadius = Math.min(nodeBox.width, nodeBox.height) / 2;
        const avatarRadius = Math.min(avatarBox.width, avatarBox.height) / 2;

        console.log(`Node radius: ${nodeRadius}px`);
        console.log(`Avatar radius: ${avatarRadius}px`);
        console.log(`Difference: ${Math.abs(nodeRadius - avatarRadius)}px`);

        expect(Math.abs(avatarRadius - 50)).toBeLessThan(5);
      }
    }
  }
});
