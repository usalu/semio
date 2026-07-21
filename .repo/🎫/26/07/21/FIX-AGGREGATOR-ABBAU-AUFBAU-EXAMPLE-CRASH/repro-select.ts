import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
const logs: string[] = [];
const failed: string[] = [];
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => logs.push(`[pageerror] ${err.message}\n${err.stack}`));
page.on("crash", () => logs.push("[CRASH] page crashed"));
page.on("response", (res) => {
  if (res.status() >= 400) failed.push(`${res.status()} ${res.url()}`);
});
page.on("requestfailed", (req) => failed.push(`FAILED ${req.url()} ${req.failure()?.errorText}`));

await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(3000);

// Dismiss tour
await page.getByRole("button", { name: "Überspringen" }).click({ timeout: 5000 }).catch(() => {});
await page.waitForTimeout(1000);

// Open example control — look for "Beispiel" then Abbau / Nakagin
const beispiel = page.getByText("Beispiel", { exact: false }).first();
await beispiel.click({ timeout: 5000 }).catch(() => {});
await page.waitForTimeout(500);

const options = await page.evaluate(() =>
  [...document.querySelectorAll("[role='option'], [data-value], li, button, div")]
    .map((el) => (el.textContent || "").trim())
    .filter((t) => /Abbau|Nakagin|concrete|Beispiel|Aufbau/i.test(t))
    .slice(0, 40),
);
console.log(`[DEBUG] optionTexts=${JSON.stringify(options, null, 2)}`);

// Try selecting Nakagin then back to Abbau Aufbau to force reload
const nakagin = page.getByText("Nakagin", { exact: false }).first();
if (await nakagin.count()) {
  await nakagin.click();
  await page.waitForTimeout(5000);
  console.log(`[DEBUG] after nakagin alive=${!(await page.isClosed())}`);
}

await page.getByText("Beispiel", { exact: false }).first().click().catch(() => {});
await page.waitForTimeout(300);
const abbau = page.getByText("Abbau Aufbau", { exact: false }).first();
await abbau.click({ timeout: 5000 }).catch((e) => console.log(`[DEBUG] abbau click failed ${e}`));
await page.waitForTimeout(10000);

console.log(`[DEBUG] pageClosed=${page.isClosed()}`);
console.log(`[DEBUG] failed=${JSON.stringify(failed, null, 2)}`);
console.log(`[DEBUG] logsTail=${JSON.stringify(logs.slice(-60), null, 2)}`);

if (!page.isClosed()) {
  const scene = await page.evaluate(() => ({
    canvas: [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height })),
    webglError: (() => {
      try {
        const c = document.createElement("canvas");
        const gl = c.getContext("webgl2") || c.getContext("webgl");
        return gl ? null : "no-webgl";
      } catch (e) {
        return String(e);
      }
    })(),
  }));
  console.log(`[DEBUG] scene=${JSON.stringify(scene)}`);
}

await browser.close();
