import { chromium } from "playwright";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const proofHtml = resolve(ticketDir, "celebrate-label-proof.html");
const screenshotPath = resolve(ticketDir, "celebrate-label-proof.png");

const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(`file://${proofHtml}`, { waitUntil: "domcontentloaded" });
const paint = await page.evaluate(() => {
  const label = document.querySelector('[data-slot="inline-label"]');
  const icon = document.querySelector("[data-icon]");
  if (!label || !icon) return null;
  const labelStyle = getComputedStyle(label);
  const iconStyle = getComputedStyle(icon);
  return {
    labelBackgroundImage: labelStyle.backgroundImage,
    labelColor: labelStyle.color,
    labelWebkitTextFillColor: labelStyle.webkitTextFillColor,
    labelBackgroundClip: labelStyle.backgroundClip,
    iconMaskImage: iconStyle.maskImage,
    iconBackgroundImage: iconStyle.backgroundImage,
  };
});
console.log("[DEBUG] celebrate label proof paint", paint);
await page.locator('[data-slot="panel-tab-button"]').screenshot({ path: screenshotPath });
await browser.close();

if (!paint) {
  console.error("missing label or icon");
  process.exit(1);
}
if (!paint.labelBackgroundImage.includes("conic-gradient")) {
  console.error("label backgroundImage missing conic-gradient");
  process.exit(1);
}
if (paint.labelBackgroundClip !== "text") {
  console.error(`label backgroundClip expected text, got ${paint.labelBackgroundClip}`);
  process.exit(1);
}
if (paint.labelWebkitTextFillColor !== "rgba(0, 0, 0, 0)" && paint.labelColor !== "rgba(0, 0, 0, 0)") {
  console.error(`label fill not transparent: color=${paint.labelColor} webkit=${paint.labelWebkitTextFillColor}`);
  process.exit(1);
}
if (!paint.iconMaskImage.includes("data:image/svg+xml")) {
  console.error("icon maskImage missing svg data uri");
  process.exit(1);
}
console.log(`[DEBUG] screenshot written to ${screenshotPath}`);
