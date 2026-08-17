import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const url = process.env.PROBE_URL ?? "http://127.0.0.1:6018/";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleErrors: string[] = [];
const pageErrors: string[] = [];
const debugLogs: string[] = [];
page.on("pageerror", (err) => pageErrors.push(String(err)));
page.on("console", (msg) => {
  const t = msg.text();
  if (msg.type() === "error") consoleErrors.push(t);
  if (t.includes("[DEBUG]") || /fault|Render error|is not defined|WebGPU|wgpu/i.test(t)) debugLogs.push(t.slice(0, 500));
});
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240_000 });

async function dump(label: string) {
  const d = await page.evaluate(() => {
    const faults = [...document.querySelectorAll("[data-shell-fault-boundary]")].map((el) => ({
      id: el.getAttribute("data-shell-fault-boundary"),
      text: (el as HTMLElement).innerText.slice(0, 500),
    }));
    return {
      title: document.title,
      hasRenderError: /Render error|is not defined/i.test(document.body.innerText),
      faultBoundaries: faults,
      bodySample: document.body.innerText
        .split(/\n/)
        .map((s) => s.trim())
        .filter(Boolean)
        .slice(0, 80),
      previewText: [...document.querySelectorAll("*")]
        .map((el) => (el as HTMLElement).innerText)
        .find((t) => t && t.includes("procedural-preview"))
        ?.slice(0, 300),
    };
  });
  await page.screenshot({ path: path.join(ticketDir, `🧪fault-${label}.png`) });
  console.log("DUMP", label, JSON.stringify(d, null, 2));
  return d;
}

await page.waitForTimeout(10_000);
const d1 = await dump("t10s");
await page.waitForTimeout(25_000);
const d2 = await dump("t35s");
await page.mouse.click(700, 450, { button: "right" });
await page.waitForTimeout(2_000);
const d3 = await dump("after-contextmenu");
const result = {
  d1,
  d2,
  d3,
  consoleErrors: consoleErrors.slice(0, 50),
  pageErrors: pageErrors.slice(0, 50),
  debugLogs: debugLogs.slice(0, 80),
};
await writeFile(path.join(ticketDir, "🧪fault-probe.json"), JSON.stringify(result, null, 2));
console.log("FINAL", JSON.stringify({ consoleErrors: result.consoleErrors, pageErrors: result.pageErrors, faults: [d1, d2, d3].map((d) => d.faultBoundaries) }, null, 2));
await browser.close();
