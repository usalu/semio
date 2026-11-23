import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🔥️ Boots the 6118 page once and records console, page errors and a screenshot into 🗑️generated/wgpu-native-smoke. */
const out = join(import.meta.dir, "🗑️generated/wgpu-native-smoke"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
page.on("console", m => lines.push(`${m.type()} ${m.text()}`));
page.on("pageerror", e => lines.push(`pageerror ${e.message}`));
page.on("requestfailed", r => lines.push(`requestfailed ${r.url()} ${r.failure()?.errorText}`));
page.on("response", r => { if (r.status() >= 400) lines.push(`http ${r.status()} ${r.url()}`); });
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(Number(process.env.SMOKE_MS ?? 45000));
await page.screenshot({ path: join(out, "shot.png") });
writeFileSync(join(out, "console.txt"), lines.join("\n"));
console.log(lines.length, "console lines;", lines.filter(l => /pageerror|panicked|http 4|http 5|requestfailed/.test(l)).slice(0, 15).join("\n"));
await browser.close();
