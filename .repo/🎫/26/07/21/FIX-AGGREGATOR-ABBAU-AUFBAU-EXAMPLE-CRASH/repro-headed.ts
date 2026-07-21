import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: false,
  args: ["--disable-dev-shm-usage"],
});
const context = await browser.newContext();
const page = await context.newPage();
const logs: string[] = [];
page.on("console", (msg) => {
  const text = `[${msg.type()}] ${msg.text()}`;
  logs.push(text);
  if (/error|Error|crash|OOM|memory|wasm|WebGL|THREE|failed|DEBUG/i.test(text)) console.log(text);
});
page.on("pageerror", (err) => console.log(`[pageerror] ${err}`));
page.on("crash", () => console.log("[CRASH]"));

await page.addInitScript(() => {
  (window as unknown as { __semioDebug?: unknown }).__semioDebug = true;
});

await page.goto(url, { waitUntil: "networkidle", timeout: 180_000 });
await page.waitForTimeout(8000);

const probe = await page.evaluate(async () => {
  // Count mesh-ish three.js objects if exposed; else canvas pixel non-black sample
  const canvas = document.querySelector("canvas") as HTMLCanvasElement | null;
  let nonBlack = 0;
  if (canvas) {
    try {
      const gl = canvas.getContext("webgl2") || canvas.getContext("webgl");
      // can't read from existing context easily; draw to 2d copy via toDataURL may fail for webgl
      const url = canvas.toDataURL("image/png");
      return { hasCanvas: true, dataUrlLen: url.length, exampleText: document.body.innerText.includes("Abbau Aufbau") };
    } catch (e) {
      return { hasCanvas: true, readError: String(e), exampleText: document.body.innerText.includes("Abbau Aufbau") };
    }
  }
  return { hasCanvas: false, nonBlack };
});
console.log(`[DEBUG] probe=${JSON.stringify(probe)}`);

// Force setActiveExample via clicking the example dropdown more carefully
await page.getByRole("button", { name: "Überspringen" }).click().catch(() => {});
await page.waitForTimeout(500);

// Find the example select trigger near "Beispiel"
const trigger = page.locator("text=Abbau Aufbau").first();
await trigger.click({ force: true }).catch((e) => console.log(`[DEBUG] trigger ${e}`));
await page.waitForTimeout(500);
const nakagin = page.locator("text=/Nakagin/i").first();
if (await nakagin.count()) {
  console.log("[DEBUG] switching to Nakagin");
  await nakagin.click();
  await page.waitForTimeout(6000);
  console.log(`[DEBUG] after nakagin closed=${page.isClosed()}`);
}
await page.locator("text=Beispiel").first().click().catch(() => {});
await page.waitForTimeout(300);
const abbauOpt = page.locator("[role='option'], div, span, li").filter({ hasText: /^Abbau Aufbau$/ }).first();
if (await abbauOpt.count()) {
  console.log("[DEBUG] switching back to Abbau Aufbau");
  await abbauOpt.click();
  await page.waitForTimeout(10000);
  console.log(`[DEBUG] after abbau closed=${page.isClosed()}`);
} else {
  console.log("[DEBUG] no Abbau option found for reselect");
}

console.log(`[DEBUG] finalClosed=${page.isClosed()}`);
console.log(`[DEBUG] errorLogs=${JSON.stringify(logs.filter((l) => /error|Error|CRASH|pageerror|OOM|wasm/i.test(l)).slice(-40), null, 2)}`);
await browser.close();
