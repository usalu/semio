import { chromium } from "playwright";

const browser = await chromium.launch({
  headless: true,
  channel: process.argv.includes("--full") ? "chromium" : undefined,
  args: ["--no-sandbox", "--disable-crashpad-for-testing"],
});
try {
  const context = await browser.newContext({ permissions: ["clipboard-read", "clipboard-write"] });
  const page = await context.newPage();
  await page.goto(new URL("./clipboard-browser-smoke.html", import.meta.url).href);
  await page.locator("#source").click();
  await page.keyboard.press("ControlOrMeta+a");
  await page.keyboard.press("ControlOrMeta+c");
  const copied = await page.evaluate(() => navigator.clipboard.readText());
  await page.locator("#destination").click();
  await page.keyboard.press("ControlOrMeta+v");
  const pasted = await page.locator("#destination").inputValue();
  if (copied !== "puzzle clipboard" || pasted !== copied) throw new Error(JSON.stringify({ copied, pasted }));
  console.log(JSON.stringify({ copied, pasted }));
} finally {
  await browser.close();
}
