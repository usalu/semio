/** 🔎️ Opens one asset page of a built site in German on a desktop viewport and lists every element that says "Nicht verfügbar", with its section. Usage: node unavailable-probe.mjs <dist-directory> <output-directory> <category/asset> */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "playwright";
import { serve } from "../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs";

const [release, out, asset] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const local = await serve(release);
const browser = await chromium.launch({ channel: "chromium", args: ["--disable-crash-reporter", "--disable-crashpad", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, reducedMotion: "reduce", locale: "fr-FR" })).newPage();
  await page.goto(local.url);
  await page.getByRole("button", { name: "Deutsch", exact: true }).click();
  await page.goto(new URL("#/" + asset, local.url).href);
  await page.reload();
  const detail = page.locator('[data-page="asset:' + asset.split("/")[1] + '"]:visible');
  await detail.locator("[data-viewer] [data-stage] svg").first().waitFor({ timeout: 90000 });
  await page.waitForTimeout(3000);
  console.log("[DEBUG]", JSON.stringify(await detail.evaluate((root) => [...root.querySelectorAll("*")].filter((node) => [...node.childNodes].some((child) => child.nodeType === 3 && /Nicht verfügbar/u.test(child.textContent))).map((node) => ({ text: node.textContent.slice(0, 160), section: node.closest("[data-section],[data-format],[data-viewpoint-selector],[data-viewer]")?.getAttributeNames().filter((name) => name.startsWith("data-")).map((name) => name + "=" + node.closest("[data-section],[data-format],[data-viewpoint-selector],[data-viewer]").getAttribute(name)).join(" "), notice: node.closest("[data-notice]")?.textContent.slice(0, 300) }))), null, 1));
  await page.screenshot({ path: join(out, "unavailable-" + asset.replace("/", "-") + ".png") });
} finally {
  await browser.close();
  local.server.close();
}
