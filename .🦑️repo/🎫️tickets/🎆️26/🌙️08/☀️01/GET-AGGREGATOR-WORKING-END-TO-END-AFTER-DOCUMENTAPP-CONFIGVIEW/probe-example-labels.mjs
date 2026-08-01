import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => {
  if (m.type() === "error" || m.text().includes("[DEBUG]") || m.text().includes("[os]")) console.log("LOG", m.type(), m.text().slice(0, 250));
});
await page.goto("http://127.0.0.1:6023/", { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForSelector("canvas", { timeout: 120000 });
for (let i = 0; i < 6; i++) {
  const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
  if (await skip.count()) {
    await skip.first().click().catch(() => {});
    await page.waitForTimeout(400);
  } else break;
}
await page.waitForTimeout(2000);
// try open Beispiel combobox / select
const beispiel = page.getByText("Beispiel", { exact: true }).first();
console.log("beispiel count", await page.getByText("Beispiel", { exact: true }).count());
// click near Concrete Forest
const cf = page.getByText("Concrete Forest").first();
if (await cf.count()) {
  await cf.click();
  await page.waitForTimeout(500);
}
const text = await page.evaluate(() => document.body?.innerText || "");
console.log("has Abbau", text.includes("Abbau Aufbau"));
console.log("has Betonwald", text.includes("Betonwald"));
console.log("has Concrete", text.includes("Concrete Forest"));
// dump select options if any
const options = await page.evaluate(() =>
  [...document.querySelectorAll("[role=option], option, [data-radix-collection-item]")].map((el) => el.textContent?.trim()).filter(Boolean),
);
console.log("options", options);
await page.screenshot({ path: new URL("./probe-example.png", import.meta.url).pathname });
await browser.close();
