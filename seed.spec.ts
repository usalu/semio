import { test, expect } from '@playwright/test';
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test.use({
  baseURL: process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:5173",
});

test.describe('Test group', () => {
  test('seed', async ({ page }) => {
    test.setTimeout(120000);

    // Navigate to home
    await page.goto("/");
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(2000);

    // 📥Import kit via zip upload
    const zipPath = path.resolve(__dirname, "semio/assets/semio/metabolism.zip");
    const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
    await expect(fileInput).toBeAttached({ timeout: 10000 });
    await fileInput.setInputFiles(zipPath);
    await fileInput.evaluate((el) => {
      el.dispatchEvent(new Event("change", { bubbles: true }));
    });

    // 📝Wait for Metabolism kit to appear
    const metabolismText = page.getByText("Metabolism", { exact: true }).first();
    await metabolismText.waitFor({ state: "visible", timeout: 60000 });
    await page.waitForTimeout(500);

    // 📊Navigate to kit
    const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
    const isTableRowVisible = await tableRow.isVisible().catch(() => false);
    if (isTableRowVisible) {
      await tableRow.dblclick({ force: true });
    } else {
      await page.getByText("Metabolism").first().dblclick({ force: true });
    }
    await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(3000);

    // 🔷Find and navigate to design
    const allRowIds = await page.evaluate(() => {
      return Array.from(document.querySelectorAll("[data-row-id]"))
        .map((el) => el.getAttribute("data-row-id"));
    });
    const designRowIds = allRowIds.filter((id) => id?.startsWith("design-"));

    if (designRowIds.length > 0) {
      const nakaginRowId = designRowIds.find((id) => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
      await page.evaluate((rowId) => {
        const row = document.querySelector(`[data-row-id="${rowId}"]`);
        if (row) {
          row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
        }
      }, nakaginRowId);
    } else {
      const designElement = page.getByText("Nakagin Capsule Tower", { exact: true }).first();
      if (await designElement.isVisible({ timeout: 5000 }).catch(() => false)) {
        await designElement.dblclick({ force: true });
      }
    }

    // Wait for design page
    await page.waitForURL(/\/designs\//, { timeout: 30000 });
    await page.waitForLoadState("networkidle");
    await page.waitForTimeout(8000);
  });
});
