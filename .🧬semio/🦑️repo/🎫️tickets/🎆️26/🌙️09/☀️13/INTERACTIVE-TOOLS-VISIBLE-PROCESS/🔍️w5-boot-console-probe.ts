/** 🩺️ Boot console dump for :6013 — prints errors/warnings and whether the main window mounted within 90 s. */
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const lines: string[] = [];
const t0 = Date.now();
page.on("console", (msg) => { if (msg.type() !== "debug") lines.push(`+${Date.now() - t0}ms ${msg.type()} ${msg.text().slice(0, 300)}`); });
page.on("pageerror", (error) => lines.push(`+${Date.now() - t0}ms pageerror ${String(error).slice(0, 400)}`));
await page.goto("http://127.0.0.1:6013/?plugin=puzzle3d");
const mounted = await page.waitForFunction(() => document.querySelectorAll('[data-surface-id="window:puzzle3d-main-perspective"]').length >= 1, undefined, { timeout: 90000 }).then(() => true, () => false);
console.log(`mounted=${mounted} records=${await page.evaluate(() => document.querySelectorAll("[data-tool-run-records]").length)}`);
console.log(lines.filter((line) => / (error|warning|pageerror) /.test(line)).slice(0, 40).join("\n"));
console.log(lines.slice(-15).join("\n"));
await browser.close();
