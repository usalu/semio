import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:16067/";
const errors = [];
const logs = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("requestfailed", (request) => {
  errors.push(`request failed: ${request.url()} ${request.failure()?.errorText ?? ""}`);
});
page.on("console", (msg) => {
  const text = msg.text();
  logs.push(`[${msg.type()}] ${text}`);
  if (msg.type() === "error") errors.push(text);
});
page.on("pageerror", (error) => errors.push(`${error.message}\n${error.stack ?? ""}`));

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
try {
  await page.waitForFunction(
    () => {
      const root = document.getElementById("root");
      const styled = document.documentElement.dataset.semioStyled === "ready";
      return styled && Boolean(root && root.childElementCount > 0);
    },
    { timeout: 180_000 },
  );
} catch (error) {
  errors.push(`root mount timeout: ${error}`);
}
await page.waitForTimeout(3000);

const childProcessError = errors.find((e) => e.includes("node:child_process") || e.includes("child_process"));
const bootFailed = errors.find((e) => e.includes("os-dev boot failed"));
const root = await page
  .locator("#root")
  .innerHTML()
  .catch(() => "");
const diagnostics = await page.evaluate(() => ({
  styled: document.documentElement.dataset.semioStyled ?? "",
  rootChildren: document.getElementById("root")?.childElementCount ?? 0,
  rootHtmlLength: document.getElementById("root")?.innerHTML.length ?? 0,
  pathname: window.location.pathname,
}));

console.log("[DEBUG] url", url);
console.log("[DEBUG] diagnostics", diagnostics);
console.log("[DEBUG] child_process error", childProcessError ?? "none");
console.log("[DEBUG] boot failed", bootFailed ?? "none");
console.log("[DEBUG] error count", errors.length);
if (errors.length) console.log("[DEBUG] errors", errors.slice(0, 10));
const debugLogs = logs.filter((l) => l.includes("[DEBUG]"));
if (debugLogs.length) console.log("[DEBUG] console logs", debugLogs.slice(0, 20));

await browser.close();
if (childProcessError || bootFailed) process.exit(1);
if (diagnostics.rootChildren < 1 && diagnostics.rootHtmlLength < 10) {
  console.error("[DEBUG] root appears empty");
  process.exit(1);
}
console.log("[DEBUG] os-dev browser boot verify passed");
