// 🖼️ Reports which element paints the flow window's body and whether every <canvas> resolved the
// shell's appearance — the non-DOM half of the appearance-scope contract.
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const outDir = join(import.meta.dir, "🗑️generated", "popover-contrast/canvas");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, colorScheme: "dark" });
await page.goto(process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6021/?plugin=generation3d", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 50) * 1000);
const out = await page.evaluate(() => {
  const at = document.elementFromPoint(800, 300);
  const chain = [];
  for (let walk = at; walk; walk = walk.parentElement) {
    const style = getComputedStyle(walk);
    chain.push({ tag: walk.tagName, slot: walk.getAttribute("data-slot"), cls: (typeof walk.className === "string" ? walk.className : "").slice(0, 80), background: style.backgroundColor });
    if (walk.classList.contains("semio-scope")) break;
  }
  const canvases = [...document.querySelectorAll("canvas")].map((node) => {
    const rect = node.getBoundingClientRect();
    return { cls: (typeof node.className === "string" ? node.className : "").slice(0, 60), bitmap: [node.width, node.height], rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)], contextKind: (() => { try { return node.getContext("2d") ? "2d" : "other"; } catch { return "lost"; } })() };
  });
  return { elementAtGraphCentre: chain, canvases };
});
await page.screenshot({ path: join(outDir, "canvas.png"), type: "png" });
writeFileSync(join(outDir, "canvas.json"), JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 1).slice(0, 4000));
await browser.close();
