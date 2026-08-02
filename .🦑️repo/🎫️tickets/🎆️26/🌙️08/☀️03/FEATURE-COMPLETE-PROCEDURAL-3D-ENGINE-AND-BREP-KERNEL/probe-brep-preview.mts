import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const outDir = dirname(fileURLToPath(import.meta.url));
const baseUrl = "http://127.0.0.1:6018/";
const examples: { id: string; label: RegExp }[] = [
  { id: "hexagonal-mushroom-column", label: /Hexagonal Mushroom/i },
  { id: "rectangle-extrude-volume", label: /Rectangle Extrude/i },
  { id: "sphere-cut-with-torus", label: /Sphere Cut/i },
  { id: "box-fillet-preview", label: /Box Fillet/i },
  { id: "sphere-box-fuse", label: /Sphere Box Fuse/i },
  { id: "face-sweep-extrude", label: /Face Sweep/i },
  { id: "rectangle-wire-preview", label: /Rectangle Wire/i },
  { id: "box-shell-preview", label: /Box Shell/i },
];

const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const debugLogs: string[] = [];
const errors: string[] = [];
page.on("console", (m) => {
  const text = m.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text.slice(0, 500));
  if (m.type() === "error") errors.push(text.slice(0, 500));
});
page.on("pageerror", (e) => errors.push(`pageerror: ${e.message.slice(0, 500)}`));

const results: Record<string, unknown> = {};

for (const example of examples) {
  await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120000 });
  await page.waitForTimeout(8000);
  const btn = page.locator("button").filter({ hasText: example.label }).first();
  if (await btn.count()) {
    await btn.click();
    await page.waitForTimeout(12000);
  }
  const report = await page.evaluate(() => {
    const canvases = [...document.querySelectorAll("canvas")].map((c) => ({
      w: c.width,
      h: c.height,
    }));
    const worldCanvas = canvases.find((c) => c.w > 200 && c.h > 200) ?? canvases[canvases.length - 1];
    return {
      canvasCount: canvases.length,
      worldCanvas,
      bodyText: document.body.innerText.slice(0, 400),
    };
  });
  await page.screenshot({ path: `${outDir}/probe-${example.id}.png`, fullPage: false });
  results[example.id] = report;
  console.log(`[DEBUG] probe ${example.id} canvas`, JSON.stringify(report.worldCanvas));
}

const full = {
  results,
  debugLogs: debugLogs.slice(0, 40),
  errors: errors.filter((e) => !e.includes("favicon")).slice(0, 20),
  ok:
    Object.values(results).every((r) => {
      const wc = (r as { worldCanvas?: { w: number; h: number } }).worldCanvas;
      return wc && wc.w > 100 && wc.h > 100;
    }) && errors.filter((e) => e.startsWith("pageerror:")).length === 0,
};
writeFileSync(`${outDir}/probe-brep-preview.json`, JSON.stringify(full, null, 2));
writeFileSync(`${outDir}/probe-brep-preview-out.txt`, JSON.stringify(full, null, 2));
console.log(JSON.stringify({ ok: full.ok, exampleCount: examples.length }, null, 2));
await browser.close();
process.exit(full.ok ? 0 : 1);
