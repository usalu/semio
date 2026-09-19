import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
const [base, out, ...variants] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const browser = await chromium.launch({ args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"] });
for (const variant of variants) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const lines = [];
  page.on("console", m => lines.push(`[${m.type()}] ${m.text()}`));
  page.on("pageerror", e => lines.push(`[pageerror] ${e.message}`));
  await page.goto(`${base}/#${variant}`);
  await page.waitForFunction(id => { const el = document.querySelector(`[data-shell-id="${id}"]`); return el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined); }, variant, { timeout: 120000 }).catch(() => lines.push("[probe] shell outcome timeout"));
  await page.waitForTimeout(6000);
  writeFileSync(`${out}/${variant}.txt`, lines.join("\n"));
  console.log(variant, lines.filter(l => l.startsWith("[error]") || /refused/.test(l)).length);
  await page.close();
}
await browser.close();
