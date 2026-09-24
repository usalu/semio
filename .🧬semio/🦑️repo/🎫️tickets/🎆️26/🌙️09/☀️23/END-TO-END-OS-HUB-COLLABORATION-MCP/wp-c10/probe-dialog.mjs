/** 🔎️ C10 dialog probe: signs in, opens Home's Create Space by keyboard, dumps the dialog DOM (slots, ids, roles). */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(10000);
await page.locator('[data-semio-hub-sign-in=""]').first().click();
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 30000 });
await form.locator('input[type="email"]').fill("user1@semio.dev");
await form.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
await form.locator('button[type="submit"][aria-label="Sign in"]').click();
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60000 });
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
await page.waitForTimeout(5000);
const button = page.locator('[data-ui-node-key="s-home-create-space"]').first();
await button.focus();
await button.press("Enter");
await page.waitForTimeout(3000);
const dump = await page.evaluate(() => {
  const dialog = document.querySelector('[role="dialog"], [role="alertdialog"]');
  const root = dialog ?? document.body;
  return { dialogTag: dialog?.tagName, dialogAttrs: dialog ? [...dialog.attributes].map((a) => `${a.name}=${a.value.slice(0, 50)}`) : null, inner: [...root.querySelectorAll("[id],[data-slot],input,button,[role]")].slice(0, 60).map((el) => `${el.tagName} id=${el.id} slot=${el.getAttribute("data-slot")} role=${el.getAttribute("role")} name=${el.getAttribute("name")} aria=${el.getAttribute("aria-label")} text=${(el.textContent ?? "").trim().slice(0, 30)}`) };
});
console.log(JSON.stringify(dump, null, 1));
const toast = await page.evaluate(() => [...document.querySelectorAll("*")].filter((el) => /Updating directory/.test(el.textContent ?? "") && el.children.length < 3).map((el) => `${el.tagName} slot=${el.getAttribute("data-slot")} id=${el.id} ${el.textContent.slice(0, 80)}`).slice(0, 3));
console.log(JSON.stringify(toast));
await browser.close();
