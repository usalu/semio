/** 🔍️ Slice C1c — what the signed-in Home shell actually renders, so the two-user scenario probe is
 * written against the DOM that exists rather than the one the harness assumes. Signs one human in and
 * prints every element id, every `data-row-id`, every toolbar/table host and the presence container.
 *
 * Usage: bun 🐍️c1c-home-census.mjs <uiOrigin>
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6108";
const USER1 = { email: "user1@semio.dev", password: "collab e2e first human phrase" };

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
const started = Date.now();
const since = () => `${((Date.now() - started) / 1000).toFixed(1)}s`;
page.on("console", (message) => {
  const text = message.text();
  console.log(`[${since()}] console:${message.type()} ${text}`.slice(0, 500));
});
page.on("pageerror", (error) => console.log(`[${since()}] pageerror ${error.message}`.slice(0, 500)));
page.on("response", (response) => {
  if (response.status() >= 400) console.log(`[${since()}] http ${response.status()} ${response.url()}`.slice(0, 300));
});
try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForTimeout(4_000);
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(USER1.email);
  await form.locator('input[type="password"]').fill(USER1.password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator("[data-semio-hub-workspace] button[aria-label]").first().click();
  await page.waitForTimeout(6_000);

  const readCensus = () => page.evaluate(() => {
    const ids = [...document.querySelectorAll("[id]")].map((element) => element.id).filter((id) => id.length > 0);
    const rows = [...document.querySelectorAll("[data-row-id]")].map((element) => element.getAttribute("data-row-id"));
    const tabs = [...document.querySelectorAll("[data-tab-id]")].map((element) => element.getAttribute("data-tab-id"));
    const tableHosts = document.querySelectorAll(".semio-table-host").length;
    const buttons = [...document.querySelectorAll("button")].map((element) => element.getAttribute("aria-label") ?? element.textContent?.trim() ?? "").filter((label) => label.length > 0);
    const semioAttributes = new Set();
    for (const element of document.querySelectorAll("*")) for (const attribute of element.getAttributeNames()) if (attribute.startsWith("data-semio")) semioAttributes.add(attribute);
    return { url: location.href, ids, rows, tabs, tableHosts, buttons, semioAttributes: [...semioAttributes], bodyText: document.body.innerText.slice(0, 1500) };
  });
  console.log("=== after sign-in ===");
  console.log(JSON.stringify(await readCensus(), null, 2));
  await page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/c1c-home-census.png", import.meta.url)) });
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.waitForTimeout(15_000);
  console.log("=== after a reload with the session restored ===");
  console.log(JSON.stringify(await readCensus(), null, 2));
  await page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/c1c-home-census-reloaded.png", import.meta.url)) });
} finally {
  await browser.close();
}
