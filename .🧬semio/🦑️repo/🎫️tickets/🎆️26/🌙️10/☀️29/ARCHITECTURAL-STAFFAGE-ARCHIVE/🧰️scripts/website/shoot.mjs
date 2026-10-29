import { chromium } from "playwright";
import { readFileSync, mkdirSync } from "node:fs";

const [specPath, outDir] = process.argv.slice(2);
const spec = JSON.parse(readFileSync(specPath, "utf8"));
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ args: ["--force-prefers-reduced-motion=0"] });
const base = spec.base ?? "http://localhost:6071/";
for (const shot of spec.shots) {
  const context = await browser.newContext({ viewport: { width: shot.width ?? 1440, height: shot.height ?? 900 }, locale: shot.locale ?? "en-US", colorScheme: shot.scheme ?? "light", deviceScaleFactor: 1 });
  const page = await context.newPage();
  const logs = [];
  page.on("console", (message) => ["error", "warning"].includes(message.type()) && logs.push(`${message.type()}: ${message.text().slice(0, 300)}`));
  page.on("pageerror", (error) => logs.push(`pageerror: ${String(error).slice(0, 300)}`));
  await page.goto(base + (shot.hash ?? "#/"), { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(shot.wait ?? 4000);
  for (const step of shot.steps ?? []) {
    if (step.click) await page.locator(step.click).first().click({ timeout: 5000 }).catch((e) => logs.push(`click failed ${step.click}: ${String(e).slice(0, 120)}`));
    if (step.press) await page.keyboard.press(step.press);
    if (step.hover) await page.locator(step.hover).first().hover({ timeout: 5000 }).catch((e) => logs.push(`hover failed ${step.hover}`));
    if (step.fill) await page.locator(step.fill[0]).first().fill(step.fill[1]);
    if (step.scroll) await page.evaluate(([sel, y]) => { const e = document.querySelector(sel); if (e) e.scrollTop = y; }, step.scroll);
    if (step.wait) await page.waitForTimeout(step.wait);
  }
  await page.screenshot({ path: `${outDir}/${shot.name}.png`, fullPage: shot.full ?? false });
  console.log(`== ${shot.name}`, logs.slice(0, 8).join("\n   "));
  await context.close();
}
await browser.close();
