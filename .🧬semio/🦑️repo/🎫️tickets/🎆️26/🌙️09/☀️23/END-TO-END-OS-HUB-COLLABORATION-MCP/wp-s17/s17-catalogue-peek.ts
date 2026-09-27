/** 🔎️ S17 one-off: open a parent program in the served `s` shell, raise its Catalogue panel and dump the panel text — which
 * operator sections the palette offers after the contributions push. usage: bun s17-catalogue-peek.ts <baseUrl> <pluginId> <appId> <out.png> */
import type { Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { awaitBeacon, dismissIntroduction } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const [baseUrl, pluginId, appId, png] = process.argv.slice(2) as [string, string, string, string];
ensureParityPlaywrightBrowsersPath();
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page: Page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15_000 });
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId);
await page.waitForTimeout(1_200);
for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
  if ((await item.count()) > 0) { await item.click(); break; }
}
await page.waitForTimeout(15_000);
await page.getByRole("button", { name: /^Catalogue/u }).first().click().catch(() => undefined);
await page.waitForTimeout(4_000);
await page.getByText(/^(EXTENSIONS|Extensions|ERWEITERUNGEN|Erweiterungen)$/u).first().click().catch(() => undefined);
await page.waitForTimeout(3_000);
const text = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel-body"], [role="tabpanel"], [data-slot="side-panel"]')].map((element) => (element as HTMLElement).innerText.replace(/\s+/gu, " ").slice(0, 1500)).join("\n---\n"));
console.log(text);
await page.screenshot({ path: png });
await browser.close();
