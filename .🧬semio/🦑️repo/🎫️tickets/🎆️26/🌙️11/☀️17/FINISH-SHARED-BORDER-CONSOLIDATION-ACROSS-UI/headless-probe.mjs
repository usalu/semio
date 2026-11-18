/** 🛰️ Headless border-gap probe through system Chrome (fallback when the browser pane is unreachable).
 * Usage: node headless-probe.mjs <url> [--sizes 1440x900,375x812] [--wait ms] [--steps steps.json] [--shot dir] [--base /admin]
 * steps.json: [{ "click": "css" } | { "eval": "js" } | { "hash": "#x" } | { "hover": "css" } | { "focus": "css" } | { "press": "Tab" } | { "wait": 300 } | { "probe": "label" }]
 * Prints one JSON line per probe: { size, label, count, pending, v[] }.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright-core/index.mjs";
import { readFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const args = process.argv.slice(2);
const url = args[0];
const opt = k => { const i = args.indexOf(k); return i < 0 ? undefined : args[i + 1]; };
const sizes = (opt("--sizes") ?? "1440x900,375x812").split(",").map(s => s.split("x").map(Number));
const wait = Number(opt("--wait") ?? 1500);
const steps = opt("--steps") ? JSON.parse(readFileSync(opt("--steps"), "utf8")) : [{ probe: "initial" }];
const shot = opt("--shot");
const base = opt("--base") ?? "";
const probePath = join(dirname(fileURLToPath(import.meta.url)), "border-gap-probe.js");
const browser = await chromium.launch({ channel: "chrome", headless: true });
const run = async page => {
  for (let i = 0; i < 20; i++) {
    const r = await page.evaluate(async src => {
      const p = await import(src + "?t=" + Date.now());
      const r = p.probe({ mark: true });
      return { count: r.count, pending: !!r.pending, v: r.violations.map(v => `${v.distance} ${v.a} || ${v.b}`) };
    }, `${base}/@fs${encodeURI(probePath)}`);
    if (!r.pending) return r;
    await page.waitForTimeout(250);
  }
  return { count: -1, pending: true, v: [] };
};
try {
  for (const [w, h] of sizes) {
    const page = await browser.newPage({ viewport: { width: w, height: h }, deviceScaleFactor: 2, isMobile: w < 768, hasTouch: w < 768 });
    page.on("pageerror", e => console.error(`[DEBUG] pageerror ${e.message}`));
    await page.goto(url, { waitUntil: "load" });
    await page.waitForTimeout(wait);
    for (const s of steps) {
      if (s.click) await page.click(s.click, { timeout: 5000 }).catch(e => console.error(`[DEBUG] click ${s.click} ${e.message.split("\n")[0]}`));
      if (s.hover) await page.hover(s.hover, { timeout: 5000 }).catch(() => {});
      if (s.focus) await page.focus(s.focus, { timeout: 5000 }).catch(() => {});
      if (s.press) await page.keyboard.press(s.press);
      if (s.hash) await page.evaluate(x => { location.hash = x; }, s.hash);
      if (s.eval) await page.evaluate(s.eval);
      if (s.mouse) await page.mouse.move(...s.mouse);
      if (s.wait) await page.waitForTimeout(s.wait);
      if (s.probe) {
        await page.mouse.move(1, h - 1);
        await page.waitForTimeout(300);
        const r = await run(page);
        console.log(JSON.stringify({ size: `${w}x${h}`, label: s.probe, ...r }));
        if (shot) { mkdirSync(shot, { recursive: true }); await page.screenshot({ path: join(shot, `${s.probe.replace(/\W+/g, "_")}-${w}.png`) }); }
      }
    }
    await page.close();
  }
} finally { await browser.close(); }
