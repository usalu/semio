/** 🔍️ Does selecting a sourcing-pool row make the aussuchen preview window render a real World3d
 * scene? Read-only apart from the click it performs. */
import { chromium } from "playwright";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";

const browser = await chromium.launch({ args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(`${BASE}#aussuchen`, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForFunction(() => (document.querySelector('[data-shell-id="aussuchen"]') as HTMLElement | null)?.dataset.shellReady !== undefined, undefined, { timeout: 120_000 });
await page.waitForTimeout(15_000);

const preview = page.locator('[data-shell-id="aussuchen"] [id="framework.window.sourcingPreview"]');
const dump = async (stage: string): Promise<void> => {
  const state = await preview.evaluate((el) => ({
    text: (el as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 120),
    host: !!el.querySelector(".semio-world-3d-host"),
    meshes: el.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json")?.length ?? 0,
    instances: el.querySelector(".semio-world-3d-host")?.getAttribute("data-instances-json")?.length ?? 0,
  }));
  console.log(stage, JSON.stringify(state));
};
await dump("before");

const row = page.locator('[data-shell-id="aussuchen"] [id="framework.window.sourcingPool"] [data-row-id]').first();
console.log("pool rows:", await page.locator('[data-shell-id="aussuchen"] [id="framework.window.sourcingPool"] [data-row-id]').count());
await row.click({ timeout: 15_000 });
await page.waitForTimeout(8_000);
await dump("after-click");

await browser.close();
