import { chromium } from "playwright";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const proofHtml = resolve(ticketDir, "celebrate-foreground-proof.html");
const screenshotPath = resolve(ticketDir, "celebrate-foreground-proof.png");

const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(`file://${proofHtml}`, { waitUntil: "domcontentloaded" });
const icon = page.locator("[data-icon]");
const paint = await icon.evaluate((el) => {
  const style = getComputedStyle(el);
  const before = getComputedStyle(el, "::before");
  return {
    maskImage: style.maskImage,
    backgroundImage: style.backgroundImage,
    beforeBackgroundImage: before.backgroundImage,
  };
});
console.log("[DEBUG] celebrate foreground proof paint", paint);
await page.locator("[data-slot=\"panel-tab-button\"]").screenshot({ path: screenshotPath });
await browser.close();

if (!paint.maskImage.includes("data:image/svg+xml")) {
  console.error("maskImage missing svg data uri");
  process.exit(1);
}
if (!paint.backgroundImage.includes("conic-gradient")) {
  console.error("backgroundImage missing conic-gradient");
  process.exit(1);
}
if (paint.beforeBackgroundImage && paint.beforeBackgroundImage !== "none") {
  console.error("::before still paints a background fill");
  process.exit(1);
}
console.log(`[DEBUG] screenshot written to ${screenshotPath}`);
