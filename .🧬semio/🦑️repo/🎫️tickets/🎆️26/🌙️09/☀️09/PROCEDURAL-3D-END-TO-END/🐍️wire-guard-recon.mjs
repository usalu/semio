/** 🔎️ Recon: does the guest receive the operator catalogue (and therefore the port types) on React? */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const outDir = join(import.meta.dir, "🗑️generated", "wire-guard");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const lines = [];
const t0 = Date.now();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));
await page.goto("http://127.0.0.1:6024/?plugin=generation3d", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(45000);
writeFileSync(join(outDir, "recon-console.txt"), lines.join("\n"));
const interesting = lines.filter((l) => /catalogue|KindInfo|kind info|contributes|exceeds|extension|operator/i.test(l));
console.log("total", lines.length, "interesting", interesting.length);
console.log(interesting.slice(0, 40).join("\n").slice(0, 4000));
await browser.close();
