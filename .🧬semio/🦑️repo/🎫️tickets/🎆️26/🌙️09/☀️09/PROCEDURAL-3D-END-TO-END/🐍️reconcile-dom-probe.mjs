import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🧬️ Names the ONE DOM node that is inserted per style recalculation during the console-silent
// windows (`Performance.getMetrics`: `RecalcStyleDuration` == the whole silence, `Nodes` delta ==
// `RecalcStyleCount`). A MutationObserver on the whole document records what is added and by whom.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 130);
const outDir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.env.SEMIO_PROBE_OUT ?? "dom");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
await context.addInitScript(() => { try { window.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await context.addInitScript(() => {
  const t0 = performance.now();
  const seen = new Map();
  const note = (key) => { seen.set(key, (seen.get(key) ?? 0) + 1); };
  const describe = (node) => {
    if (node.nodeType === 3) return `#text(${(node.nodeValue ?? "").trim().slice(0, 24)})`;
    if (node.nodeType !== 1) return `#node${node.nodeType}`;
    const el = node;
    return `<${el.tagName.toLowerCase()}${el.id ? "#" + el.id : ""}${el.className && typeof el.className === "string" ? "." + el.className.split(/\s+/).slice(0, 2).join(".") : ""}>`;
  };
  const observer = new MutationObserver((records) => {
    for (const r of records) {
      for (const n of r.addedNodes) note(`ADD ${describe(n)} parent=${describe(r.target)}`);
      for (const n of r.removedNodes) note(`REMOVE ${describe(n)} parent=${describe(r.target)}`);
      if (r.type === "attributes") note(`ATTR ${r.attributeName} on ${describe(r.target)}`);
    }
  });
  const start = () => observer.observe(document.documentElement, { childList: true, subtree: true, attributes: true, attributeFilter: ["style", "class"] });
  if (document.documentElement) start(); else document.addEventListener("DOMContentLoaded", start);
  setInterval(() => {
    const top = [...seen.entries()].sort((a, b) => b[1] - a[1]).slice(0, 8).map(([k, v]) => `${v}x ${k}`).join(" | ");
    console.log(`[DEBUG] dommut at=${(performance.now() - t0).toFixed(0)} styleTags=${document.querySelectorAll("style").length} sheets=${document.styleSheets.length} rules=${[...document.styleSheets].reduce((a, s) => { try { return a + s.cssRules.length; } catch { return a; } }, 0)} elements=${document.getElementsByTagName("*").length} top=${top}`);
    seen.clear();
  }, 1000);
});
const page = await context.newPage();
const t0 = performance.now();
page.on("console", (msg) => lines.push(`${(performance.now() - t0).toFixed(2)} ${msg.type()} ${msg.text().slice(0, 2000)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE lines", lines.length);
await browser.close();
