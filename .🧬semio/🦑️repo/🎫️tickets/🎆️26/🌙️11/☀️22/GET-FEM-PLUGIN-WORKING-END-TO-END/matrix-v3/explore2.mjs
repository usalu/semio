import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const [url, wait = "40", ...steps] = process.argv.slice(2);
const out = fileURLToPath(new URL("../🗑️generated/matrix-v3/", import.meta.url));
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(Number(wait) * 1000);
  for (const s of steps) {
    const [kind, arg] = [s.slice(0, s.indexOf(":")), s.slice(s.indexOf(":") + 1)];
    if (kind === "click") await page.getByRole("button", { name: arg, exact: true }).first().click({ timeout: 8000 }).catch(e => console.log("click fail", arg, String(e).slice(0, 80)));
    if (kind === "text") await page.getByText(arg, { exact: true }).first().click({ timeout: 8000 }).catch(e => console.log("text fail", arg));
    if (kind === "wait") await page.waitForTimeout(Number(arg));
    if (kind === "dump") { writeFileSync(`${out}x-${arg}.aria.txt`, await page.locator("body").ariaSnapshot()); writeFileSync(`${out}x-${arg}.text.txt`, await page.locator("body").innerText()); await page.screenshot({ path: `${out}x-${arg}.png` }); }
  }
} finally { await browser.close(); }
