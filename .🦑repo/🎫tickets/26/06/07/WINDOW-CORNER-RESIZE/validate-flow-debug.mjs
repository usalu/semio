/** @emoji 🧪 Temporary flow dev runtime probe — captures console + page errors. */
import { chromium } from "@playwright/test";
import { writeFileSync } from "node:fs";

const baseUrl = process.env.FLOW_PLAY_URL ?? "http://127.0.0.1:6016/";
const logs = [];
const errors = [];

const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan"],
});
const page = await browser.newPage();
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => errors.push(String(err)));
await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 60_000 });
await page.waitForTimeout(5000);
const html = await page.content();
const bodyText = await page
  .locator("body")
  .innerText()
  .catch(() => "");
await browser.close();

writeFileSync(".repo/🎫/26/06/07/WINDOW-CORNER-RESIZE/validate-flow-page.html", html);
writeFileSync(".repo/🎫/26/06/07/WINDOW-CORNER-RESIZE/validate-flow-console.txt", [...logs, ...errors.map((e) => `[pageerror] ${e}`)].join("\n"));

console.log("[validate-flow] body:", bodyText.slice(0, 500));
console.log("[validate-flow] errors:", errors);
console.log(
  "[validate-flow] debug logs:",
  logs.filter((l) => l.includes("DEBUG")),
);
