/** @emoji 🧪 Temporary flow dev runtime probe — captures [DEBUG] console logs. */
import { chromium } from "@playwright/test";

const baseUrl = process.env.FLOW_PLAY_URL ?? "http://127.0.0.1:6016/";
const debugLogs = [];

const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan"],
});
const page = await browser.newPage();
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});
await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 60_000 });
await page.waitForTimeout(3000);
const previewText = await page
  .locator("strong.tabular-nums")
  .first()
  .textContent()
  .catch(() => null);
await browser.close();

console.log("[validate-flow] debug logs:", debugLogs);
console.log("[validate-flow] preview text:", previewText);

if (!debugLogs.some((l) => l.includes("flow evaluate preview"))) {
  console.error("[validate-flow] missing evaluate debug log");
  process.exit(1);
}
if (previewText == null || previewText === "—") {
  console.error("[validate-flow] preview not computed");
  process.exit(1);
}
console.log("[validate-flow] ok");
