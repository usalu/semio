// 🔎️ Recon: after opening the command bar, walks the ancestor chain of a visible command row and of
// the node-graph canvas, reporting each ancestor's slot/level/appearance-relevant paint.
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6021/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "popover-contrast/recon");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, colorScheme: "dark" });
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(28000);

await page.evaluate(() => document.getElementById("framework.category.command")?.click());
await page.waitForTimeout(1500);
await page.screenshot({ path: join(outDir, "command.png"), type: "png" });

const report = await page.evaluate(() => {
  const chain = (node) => {
    const out = [];
    for (let walk = node; walk; walk = walk.parentElement) {
      const style = getComputedStyle(walk);
      out.push({
        tag: walk.tagName,
        id: walk.id || undefined,
        slot: walk.getAttribute("data-slot") || undefined,
        role: walk.getAttribute("role") || undefined,
        level: walk.getAttribute("data-level") || undefined,
        cls: (typeof walk.className === "string" ? walk.className : "").slice(0, 110),
        color: style.color,
        background: style.backgroundColor,
      });
      if (walk.classList.contains("semio-scope") || walk === document.body) break;
    }
    return out;
  };
  const byText = (needle) => [...document.querySelectorAll("body *")].filter((node) => node.children.length === 0 && (node.textContent ?? "").trim() === needle);
  const rows = ["Set Driver…", "Reset panels", "Set Layout…", "Fullscreen"].flatMap((needle) => byText(needle).map((node) => ({ needle, chain: chain(node) })));
  const canvases = [...document.querySelectorAll("canvas")].map((node) => ({ cls: (typeof node.className === "string" ? node.className : "").slice(0, 110), rect: node.getBoundingClientRect().toJSON(), chainTop: chain(node).slice(0, 4) }));
  return { rows, canvases };
});

writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2).slice(0, 14000));
await browser.close();
