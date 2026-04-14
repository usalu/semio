import { test, expect } from "@playwright/test";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test.use({
  baseURL: process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4181",
});

test.describe("Drag", () => {
  test("seed + drag: children and descendants move with parent", async ({ page }) => {
    test.setTimeout(240000);

    const KIT_GUID = "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc";
    const DESIGN_GUID = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

    // Step 1: Navigate to home and import kit
    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(3000);

    const zipPath = path.resolve(__dirname, "semio/assets/semio/metabolism.zip");
    const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
    await expect(fileInput).toBeAttached({ timeout: 30000 });
    await fileInput.setInputFiles(zipPath);
    await fileInput.evaluate((el) => {
      el.dispatchEvent(new Event("change", { bubbles: true }));
    });

    // Wait for Metabolism kit to appear
    const metabolismText = page.getByText("Metabolism", { exact: true }).first();
    await metabolismText.waitFor({ state: "visible", timeout: 60000 });
    await page.waitForTimeout(2000);

    // Step 2: Navigate to design page via client-side navigation (not page.goto which loses in-memory state)
    await page.evaluate((url: string) => {
      window.history.pushState({}, "", url);
      window.dispatchEvent(new PopStateEvent("popstate"));
    }, `/kits/${KIT_GUID}/designs/${DESIGN_GUID}`);
    await page.waitForTimeout(3000);

    // Wait for ReactFlow nodes to appear
    const rfContainer = page.locator(".react-flow").first();
    await rfContainer.waitFor({ state: "visible", timeout: 30000 });
    await page.waitForTimeout(5000);

    const anyNode = page.locator(".react-flow__node").first();
    await anyNode.waitFor({ state: "visible", timeout: 30000 });
    await page.waitForTimeout(3000);

    // Step 3: Get pre-drag metadata
    const metadataBefore = await page.evaluate(() => (window as any).__SEMIO_PIECES_METADATA__);
    const metaKeys = metadataBefore ? Object.keys(metadataBefore).length : 0;
    expect(metaKeys).toBeGreaterThan(0);

    // Find root (no parentPieceId) and child
    const entries = Object.entries(metadataBefore) as [string, any][];
    const rootEntry = entries.find(([, m]) => !m.parentPieceId);
    expect(rootEntry).toBeDefined();
    const rootGuid = rootEntry![0];
    const childEntry = entries.find(([, m]) => m.parentPieceId === rootGuid);
    expect(childEntry).toBeDefined();
    const childGuid = childEntry![0];

    const rootCenterBefore = rootEntry![1].center;
    const childCenterBefore = childEntry![1].center;

    // Step 4: Find the root piece node and get its bounding box
    const rootNode = page.locator(`.react-flow__node[data-id*="${rootGuid}"]`).first();
    await expect(rootNode).toBeVisible({ timeout: 10000 });
    const rootBox = await rootNode.boundingBox();
    expect(rootBox).toBeTruthy();

    // Step 5: Perform drag — 200px right, 100px down
    const startX = rootBox!.x + rootBox!.width / 2;
    const startY = rootBox!.y + rootBox!.height / 2;
    await page.mouse.move(startX, startY);
    await page.mouse.down();
    for (let i = 1; i <= 20; i++) {
      await page.mouse.move(startX + (200 * i) / 20, startY + (100 * i) / 20);
      await page.waitForTimeout(30);
    }
    await page.waitForTimeout(200);
    await page.mouse.up();

    // Wait for reactive chain to propagate
    await page.waitForTimeout(3000);

    // Step 6: Get post-drag metadata
    const metadataAfter = await page.evaluate(() => (window as any).__SEMIO_PIECES_METADATA__);
    expect(metadataAfter).toBeTruthy();
    const rootCenterAfter = metadataAfter[rootGuid]?.center;
    const childCenterAfter = metadataAfter[childGuid]?.center;

    // Step 7: Get post-drag DOM positions
    const rootBoxAfter = await rootNode.boundingBox();

    // Step 8: Verify root moved in metadata
    expect(rootCenterAfter).toBeTruthy();
    const rootDeltaU = rootCenterAfter.u - rootCenterBefore.u;
    const rootDeltaV = rootCenterAfter.v - rootCenterBefore.v;
    expect(Math.abs(rootDeltaU) > 0.5 || Math.abs(rootDeltaV) > 0.5).toBe(true);

    // Verify child also moved in metadata
    expect(childCenterAfter).toBeTruthy();
    const childDeltaU = childCenterAfter.u - childCenterBefore.u;
    const childDeltaV = childCenterAfter.v - childCenterBefore.v;
    expect(Math.abs(childDeltaU) > 0.5 || Math.abs(childDeltaV) > 0.5).toBe(true);

    // Verify DOM positions changed after drag
    expect(rootBoxAfter).toBeTruthy();
    expect(Math.abs(rootBoxAfter!.x - rootBox!.x) > 10 || Math.abs(rootBoxAfter!.y - rootBox!.y) > 10).toBe(true);
  });

  test("seed + drag leaf: leaf node offsets through parent connection", async ({ page }) => {
    test.setTimeout(240000);

    const KIT_GUID = "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc";
    const DESIGN_GUID = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

    // Step 1: Navigate to home and import kit
    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(3000);

    const zipPath = path.resolve(__dirname, "semio/assets/semio/metabolism.zip");
    const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
    await expect(fileInput).toBeAttached({ timeout: 30000 });
    await fileInput.setInputFiles(zipPath);
    await fileInput.evaluate((el) => {
      el.dispatchEvent(new Event("change", { bubbles: true }));
    });

    const metabolismText = page.getByText("Metabolism", { exact: true }).first();
    await metabolismText.waitFor({ state: "visible", timeout: 60000 });
    await page.waitForTimeout(2000);

    // Step 2: Navigate to design
    await page.evaluate((url: string) => {
      window.history.pushState({}, "", url);
      window.dispatchEvent(new PopStateEvent("popstate"));
    }, `/kits/${KIT_GUID}/designs/${DESIGN_GUID}`);
    await page.waitForTimeout(3000);

    const rfContainer = page.locator(".react-flow").first();
    await rfContainer.waitFor({ state: "visible", timeout: 30000 });
    await page.waitForTimeout(5000);

    const anyNode = page.locator(".react-flow__node").first();
    await anyNode.waitFor({ state: "visible", timeout: 30000 });
    await page.waitForTimeout(3000);

    // Step 3: Get metadata and find a LEAF node (has parent, no children)
    const metadataBefore = await page.evaluate(() => (window as any).__SEMIO_PIECES_METADATA__);
    expect(metadataBefore).toBeTruthy();
    const entries = Object.entries(metadataBefore) as [string, any][];
    const childrenMap = new Map<string, string[]>();
    for (const [guid, meta] of entries) {
      if (meta.parentPieceId) {
        const siblings = childrenMap.get(meta.parentPieceId);
        if (siblings) siblings.push(guid);
        else childrenMap.set(meta.parentPieceId, [guid]);
      }
    }
    const leafEntry = entries.find(([guid, meta]) => meta.parentPieceId && !childrenMap.has(guid));
    expect(leafEntry).toBeDefined();
    const leafGuid = leafEntry![0];
    const leafCenterBefore = leafEntry![1].center;

    // Step 4: Find and drag the leaf node
    const leafNode = page.locator(`.react-flow__node[data-id*="${leafGuid}"]`).first();
    await expect(leafNode).toBeVisible({ timeout: 10000 });
    const leafBox = await leafNode.boundingBox();
    expect(leafBox).toBeTruthy();

    const startX = leafBox!.x + leafBox!.width / 2;
    const startY = leafBox!.y + leafBox!.height / 2;
    await page.mouse.move(startX, startY);
    await page.mouse.down();
    for (let i = 1; i <= 20; i++) {
      await page.mouse.move(startX + (150 * i) / 20, startY + (75 * i) / 20);
      await page.waitForTimeout(30);
    }
    await page.waitForTimeout(200);
    await page.mouse.up();

    // Wait for reactive chain
    await page.waitForTimeout(3000);

    // Step 5: Get post-drag metadata
    const metadataAfter = await page.evaluate(() => (window as any).__SEMIO_PIECES_METADATA__);
    expect(metadataAfter).toBeTruthy();
    const leafCenterAfter = metadataAfter[leafGuid]?.center;

    // Step 6: Verify leaf moved in metadata (connection u/v should have changed)
    expect(leafCenterAfter).toBeTruthy();
    const leafDeltaU = leafCenterAfter.u - leafCenterBefore.u;
    const leafDeltaV = leafCenterAfter.v - leafCenterBefore.v;
    expect(Math.abs(leafDeltaU) > 0.5 || Math.abs(leafDeltaV) > 0.5).toBe(true);

    // Step 7: Verify DOM position changed
    const leafBoxAfter = await leafNode.boundingBox();
    expect(leafBoxAfter).toBeTruthy();
    expect(Math.abs(leafBoxAfter!.x - leafBox!.x) > 10 || Math.abs(leafBoxAfter!.y - leafBox!.y) > 10).toBe(true);

    // Step 8: Get console logs to verify debug output
    const logs = await page.evaluate(() => (window as any).__SEMIO_DEBUG_LOGS__ ?? []);
    console.log("[DEBUG] Browser console logs:", JSON.stringify(logs));
  });
});
