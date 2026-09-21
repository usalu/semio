/** 🩺️ S10 — captures the first pageerror's full stack off a cold `s` boot, so a host crash that
 * swallows the command palette is attributed to a file and a line instead of a message. */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
const SHELL = process.argv[2] ?? "http://127.0.0.1:6071/";
const WAIT = Number(process.argv[3] ?? 120_000);
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const errors = [];
page.on("pageerror", (error) => errors.push(`${String(error.message)}\n${String(error.stack ?? "").split("\n").slice(0, 14).join("\n")}`));
page.on("console", (m) => { if (/error|refused|fault/i.test(m.type() + m.text())) errors.push(`${m.type()}: ${m.text().slice(0, 300)}`); });
await page.goto(SHELL, { waitUntil: "commit", timeout: 300_000 });
if (process.env.S10_SIGN_IN === "1") {
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill("user1@semio.dev");
  await workspace.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
  await workspace.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => undefined);
}
await page.waitForTimeout(WAIT);
if (process.env.S10_RELOAD === "1") {
  console.log("BEFORE-RELOAD", await page.evaluate(() => document.body.innerText.replace(/\s+/gu, " ").slice(0, 260)));
  await page.reload({ waitUntil: "commit", timeout: 300_000 });
  await page.waitForTimeout(Number(process.env.S10_RELOAD_WAIT ?? 90_000));
}
console.log("READY", await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")));
console.log("PROBE", await page.evaluate(() => (window.__semioOsCatalogProbe ? "present" : "absent")));
console.log("BODY", await page.evaluate(() => document.body.innerText.replace(/\s+/gu, " ").slice(0, 300)));
console.log("ERRORS");
for (const line of [...new Set(errors)].slice(0, 6)) console.log("---\n" + line);
await page.screenshot({ path: `${OUT}s10-boot-diagnose.png` }).catch(() => undefined);
await browser.close();
