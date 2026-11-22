import assert from "node:assert/strict";
import { chromium } from "playwright";

const url = new URL(process.argv[2] ?? "http://127.0.0.1:6083/?example=demo");
assert.equal(url.hostname, "127.0.0.1");
const browser = await chromium.launch({ channel: "chrome", headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 } });
  await page.goto(url.toString(), { waitUntil: "domcontentloaded", timeout: 60_000 });
  const shell = page.locator("[data-semio-os-ready][data-history-json]");
  const viewport = page.locator(".semio-world-3d-host");
  await viewport.locator('[data-slot="terrain-feature-list"] li').first().waitFor({ state: "attached", timeout: 60_000 });
  const window = shell.locator('[data-slot="window-body"]').filter({ has: viewport }).first();
  const overlay = window.locator('[data-slot="window-engagement-overlay"]');
  if (await overlay.getAttribute("data-folded") === "true") await overlay.locator('button[id$=".engagement.toggle"]').click();
  await overlay.locator('[data-slot="tree-item-row"][id="action.editElevation"]').click();
  await page.getByText("Column", { exact: true }).first().waitFor({ state: "attached", timeout: 20_000 }).catch(async (error) => {
    console.log("[DEBUG] Terrain staged action text", (await page.locator("body").innerText()).slice(-4000));
    throw error;
  });
  const fields = await page.locator('input, button, select').evaluateAll((controls) =>
    controls.map((control) => ({
      id: control.id,
      name: control.getAttribute("aria-label") ?? control.name,
      tag: control.tagName,
      type: control.type,
      text: control.textContent?.trim(),
      value: control.value,
    })).filter((control) => control.name || control.text || control.id));
  console.log("[DEBUG] Terrain staged action controls", JSON.stringify(fields));
} finally {
  await browser.close();
}
