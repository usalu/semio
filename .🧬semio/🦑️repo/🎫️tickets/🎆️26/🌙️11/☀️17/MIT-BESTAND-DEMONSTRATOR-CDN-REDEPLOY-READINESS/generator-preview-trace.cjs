/** 🧭️ Opens the generator pane headless and records the full console (consecutive duplicates collapsed) plus the final pane text. Usage: node generator-preview-trace.cjs <origin> <out-file> [seconds] */
const { createRequire } = require("node:module");
const { resolve, join } = require("node:path");
const fs = require("node:fs");
const root = resolve(__dirname, "../../../../../../..");
const appRequire = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = appRequire("@playwright/test");
const [origin = "http://cdn.localhost:8775", out = join(__dirname, "🗑️generated", "preview-trace", "console.txt"), seconds = "90"] = process.argv.slice(2);
(async () => {
  fs.mkdirSync(resolve(out, ".."), { recursive: true });
  const browser = await chromium.launch({ args: [process.platform === "darwin" ? "--use-angle=metal" : "--use-angle=gl", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"] });
  const page = await (await browser.newContext({ locale: "en-US", viewport: { width: 1440, height: 900 } })).newPage();
  const lines = []; let last = "", repeat = 0; const start = Date.now();
  const flush = () => { if (repeat > 1) lines.push(`   (x${repeat})`); };
  const push = text => { if (text === last) { repeat++; return; } flush(); last = text; repeat = 1; lines.push(`${((Date.now() - start) / 1000).toFixed(1)}s ${text}`); };
  page.on("console", message => push(`[${message.type()}] ${message.text().slice(0, 1500)}`));
  page.on("pageerror", error => push(`[pageerror] ${String(error).slice(0, 1500)}`));
  await page.goto(`${origin}/#generator`).catch(error => push(`[goto] ${error.message}`));
  await page.waitForTimeout(Number(seconds) * 1000);
  const text = await Promise.race([page.evaluate(() => document.body.innerText.slice(0, 2500)), new Promise(r => setTimeout(() => r("unresponsive"), 15000))]).catch(e => String(e));
  flush(); lines.push("--- PANE TEXT ---", text);
  fs.writeFileSync(out, lines.join("\n"));
  await page.screenshot({ path: out.replace(/\.txt$/, ".png"), timeout: 20000 }).catch(() => {});
  console.log(`lines=${lines.length}`);
  await Promise.race([browser.close(), new Promise(r => setTimeout(r, 5000))]);
  process.exit(0);
})();
