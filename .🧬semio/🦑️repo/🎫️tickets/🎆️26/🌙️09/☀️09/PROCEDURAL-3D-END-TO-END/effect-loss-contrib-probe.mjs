import { chromium } from "playwright";

/** 🧭️ Capped probe: lists the installed contribution rows (plugin, topic, size) 20 s after boot. One browser, 60 s cap. */
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 60000);
const page = await (await browser.newContext({ viewport: { width: 1200, height: 800 } })).newPage();
await page.goto(process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d", { waitUntil: "load" });
await page.waitForTimeout(20000);
console.log(await page.evaluate(() => { const rows = JSON.parse(window.__semioOsInstalledContributions(1)); return rows.map((r) => `${r.pluginId} topic=${r.topicContribution?.topic} size=${JSON.stringify(r).length} keys=${Object.keys(r.topicContribution?.payload ?? {}).join(",")}`).join("\n"); }));
clearTimeout(cap); await browser.close();
