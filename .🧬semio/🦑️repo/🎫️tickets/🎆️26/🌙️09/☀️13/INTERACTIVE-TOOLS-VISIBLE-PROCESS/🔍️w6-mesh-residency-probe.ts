/** 🩺️ [DEBUG] temp: sample the guest's published meshResidency (process-wide mesh store installs) and the registerBrushMesh
 * traffic on :6013 over the first minute after boot. Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. */
import { chromium } from "@playwright/test";
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const t0 = Date.now();
page.on("console", (msg) => { if (/brush ?mesh|registerBrushMesh|register-mesh|residency|notify|toast/i.test(msg.text())) console.log(`+${Date.now() - t0} console ${msg.text().slice(0, 3000)}`); });
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`);
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 600000 });
await page.waitForTimeout(8000);
await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 3000 }).catch(() => {});
const utility = () => page.evaluate(() => (JSON.parse(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-interaction-json") ?? "{}") as { activeUtility?: string }).activeUtility);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Tool", exact: true }).first().click();
await page.waitForTimeout(3000);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Fill", exact: true }).first().click();
console.log("utility", await utility());
for (let i = 0; i < 1; i += 1) {
  const state = await page.evaluate(() => {
    const node = document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]');
    const interaction = JSON.parse(node?.getAttribute("data-interaction-json") ?? "{}");
    return { meshResidency: interaction.meshResidency, keys: Object.keys(interaction).filter((k) => /mesh/i.test(k)) };
  });
  console.log(`+${Date.now() - t0}ms ${JSON.stringify(state)}`);
  await page.waitForTimeout(5000);
}
await browser.close();
