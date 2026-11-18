import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright-core/index.mjs";
import { writeFileSync, mkdirSync } from "node:fs";
const T = new URL(".", import.meta.url).pathname;
mkdirSync(T + "🗑️generated", { recursive: true });
const parts = ["engagementFull", "engagementOptions", "search", "window", "treeFiles", "overview", "popover", "select", "toggle", "fields", "bands"];
const browser = await chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", headless: true, args: ["--no-sandbox"], env: { ...process.env, HOME: "/private/tmp/claude-501/r3fw-home" } });
const out = [];
for (const [w, h] of [[1440, 900], [375, 812]]) {
  const ctx = await browser.newContext({ viewport: { width: w, height: h } });
  const page = await ctx.newPage();
  page.on("pageerror", e => out.push("PAGEERROR " + e.message));
  await page.goto("http://127.0.0.1:5462/");
  await page.waitForSelector("body[data-ready]", { timeout: 60000 });
  const runUrl = "/@fs" + encodeURI(T + "r3fw-run.js");
  for (const part of parts) {
    for (const state of part === "fields" ? ["idle", "focus"] : ["idle"]) {
      let r;
      try {
        r = await page.evaluate(async ([u, p, s]) => {
          const m = await import(u + "?t=" + Date.now());
          const text = await m.run(p, { mark: true });
          if (s === "focus") { document.querySelector("#r3fw-live input")?.focus(); await new Promise(r => setTimeout(r, 500)); }
          return text;
        }, [runUrl, part, state]);
      } catch (e) { r = "ERR " + e.message.slice(0, 300); }
      out.push(`[${w}] ${state} ${r}`);
      await page.screenshot({ path: `${T}🗑️generated/r3fw-${w}-${part}-${state}.png` });
    }
  }
  await ctx.close();
}
await browser.close();
writeFileSync(T + "🗑️generated/r3fw-results.txt", out.join("\n"));
console.log(out.join("\n"));
