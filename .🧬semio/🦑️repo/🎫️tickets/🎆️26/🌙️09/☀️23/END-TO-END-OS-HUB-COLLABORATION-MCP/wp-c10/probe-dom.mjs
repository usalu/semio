/** 🔎️ C10 DOM probe: loads a shell URL and prints elements whose text matches a pattern, with id/role/data attributes. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2], pattern = process.argv[3], waitMs = Number(process.argv[4] ?? 15000);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(waitMs);
if (process.env.PROBE_EMAIL) {
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30000 });
  await form.locator('input[type="email"]').fill(process.env.PROBE_EMAIL);
  await form.locator('input[type="password"]').fill(process.env.PROBE_PASSWORD);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.waitForTimeout(Number(process.env.PROBE_AFTER_MS ?? 8000));
  await page.screenshot({ path: "/Users/ueli/Documents/semio/.tmp-ticket/wp-c10/generated/probe-dom-signed-in.png" });
}
const rows = await page.evaluate((source) => {
  const re = new RegExp(source, "i");
  return [...document.querySelectorAll("*")].filter((el) => el.children.length < 4 && re.test(el.textContent ?? "") && (el.textContent ?? "").length < 120).map((el) => ({ tag: el.tagName, id: el.id, role: el.getAttribute("role"), slot: el.getAttribute("data-slot"), rowId: el.getAttribute("data-row-id"), text: (el.textContent ?? "").slice(0, 80), attrs: [...el.attributes].map((a) => `${a.name}=${a.value.slice(0, 40)}`).join(" ").slice(0, 300) }));
}, pattern);
for (const row of rows.slice(0, 40)) console.log(JSON.stringify(row));
await browser.close();
