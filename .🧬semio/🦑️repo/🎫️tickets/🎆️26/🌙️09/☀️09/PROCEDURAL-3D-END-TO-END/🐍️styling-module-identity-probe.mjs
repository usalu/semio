// 🧩️ Lists every request whose url mentions the styling theme module, to detect duplicate instances.
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, colorScheme: "dark" });
const urls = new Set();
page.on("request", (request) => {
  const url = decodeURIComponent(request.url());
  if (url.includes("🌓️theme") || url.includes("ui-styling") || url.includes("🎠️kernel")) urls.add(url.split("?")[0] + (url.includes("?") ? "?" + url.split("?")[1].slice(0, 40) : ""));
});
await page.goto(process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6021/?plugin=generation3d", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 60) * 1000);
console.log([...urls].sort().join("\n"));
await browser.close();
