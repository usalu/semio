import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const seenKey = "ui.introduction.seen.entwerfen-mit-bestand:puzzle3d-play";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
const debugLogs: string[] = [];
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});

await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.evaluate((key) => localStorage.setItem(key, "true"), seenKey);
await page.reload({ waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(3000);

const tourVisible = await page.locator("text=Willkommen beim Aggregator").first().isVisible().catch(() => false);
const replayLog = debugLogs.find((line) => line.includes("replaying introduction on load"));

console.log(`[DEBUG] capturedLogs=${JSON.stringify(debugLogs)}`);
console.log(`[DEBUG] tourVisible=${tourVisible}`);
console.log(`[DEBUG] replayLog=${replayLog ?? "(missing)"}`);

await browser.close();

if (!tourVisible || !replayLog) {
  console.error("[DEBUG] runtime verification failed");
  process.exit(1);
}
console.log("[DEBUG] runtime verification passed — tour shown after refresh with seen flag set");
