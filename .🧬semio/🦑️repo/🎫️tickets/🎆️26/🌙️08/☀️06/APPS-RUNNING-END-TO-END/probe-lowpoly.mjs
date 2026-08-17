import { chromium } from "playwright";
import { join } from "path";
const ticketDir = import.meta.dirname;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const pageErrors = [];
const consoleMsgs = [];
page.on("pageerror", (err) => pageErrors.push({ message: String(err.message || err).slice(0, 600), stack: String(err.stack || "").slice(0, 1200) }));
page.on("console", (msg) => {
  consoleMsgs.push({ type: msg.type(), text: msg.text().slice(0, 800) });
});
await page.goto("http://127.0.0.1:6078/", { waitUntil: "networkidle", timeout: 120000 });
await page.waitForTimeout(8000);
const title = await page.title();
const bodyText = await page.locator("body").innerText().catch(() => "");
const html = await page.content();
const canvasCount = await page.locator("canvas").count();
const btnTexts = await page.locator("button").allTextContents().catch(() => []);
const roleStructure = await page.evaluate(() => {
  const root = document.body;
  const summary = {
    classes: [...new Set([...root.querySelectorAll("[class]")].slice(0, 80).map((el) => el.className?.toString?.().slice(0, 80)))].slice(0, 40),
    dataAttrs: [...root.querySelectorAll("*")].flatMap((el) => [...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => `${a.name}=${a.value}`)).slice(0, 60),
    textNodes: (root.innerText || "").split("\n").map((s) => s.trim()).filter(Boolean).slice(0, 80),
  };
  return summary;
});
await page.screenshot({ path: join(ticketDir, "🧪lowpoly-e2e-screenshot.png"), fullPage: true }).catch(() => {});
// wait another few seconds to catch "after a second" flip
await page.waitForTimeout(4000);
const afterText = await page.locator("body").innerText().catch(() => "");
await page.screenshot({ path: join(ticketDir, "🧪lowpoly-e2e-screenshot-after.png"), fullPage: true }).catch(() => {});
const out = {
  title,
  canvasCount,
  btnTexts: btnTexts.slice(0, 40),
  bodyTextSample: bodyText.slice(0, 1200),
  afterTextSample: afterText.slice(0, 1200),
  roleStructure,
  pageErrors,
  consoleErrors: consoleMsgs.filter((m) => m.type === "error").slice(0, 40),
  consoleWarns: consoleMsgs.filter((m) => m.type === "warning").slice(0, 20),
  consoleDebug: consoleMsgs.filter((m) => /\[DEBUG\]|lowpoly|plugin|wasm|failed/i.test(m.text)).slice(0, 40),
  htmlLen: html.length,
};
await Bun.write(join(ticketDir, "🧪lowpoly-e2e-probe.json"), JSON.stringify(out, null, 2));
console.log(JSON.stringify({
  title: out.title,
  canvasCount: out.canvasCount,
  btnTexts: out.btnTexts.slice(0, 20),
  pageErrorCount: pageErrors.length,
  pageErrors: pageErrors.slice(0, 8),
  consoleErrors: out.consoleErrors.slice(0, 15),
  text: out.roleStructure.textNodes.slice(0, 40),
  afterText: out.afterTextSample.slice(0, 400),
  debug: out.consoleDebug.slice(0, 20),
}, null, 2));
await browser.close();
process.exit(pageErrors.length === 0 && canvasCount > 0 ? 0 : 2);
