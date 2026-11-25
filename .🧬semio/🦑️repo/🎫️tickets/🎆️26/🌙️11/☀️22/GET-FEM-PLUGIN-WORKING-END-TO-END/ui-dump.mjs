/** 🗺️ Boots a FEM editor headless and dumps its accessibility tree, interactive controls and a screenshot. Usage: node ui-dump.mjs <url> <waitText> [seconds] */
import { writeFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/", wait = process.argv[3] ?? "Converged", seconds = Number(process.argv[4] ?? 180);
const out = fileURLToPath(new URL("./🗑️generated/", import.meta.url)), port = new URL(url).port;
mkdirSync(out, { recursive: true });
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  page.on("pageerror", error => console.log(`pageerror ${error.message.slice(0, 300)}`));
  page.on("console", m => { if (m.type() === "error") console.log(`console.error ${m.text().slice(0, 300)}`); });
  const started = Date.now();
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.getByText(wait).first().waitFor({ timeout: seconds * 1000 }).catch(() => console.log(`no "${wait}" after ${seconds}s`));
  console.log(`ready ${((Date.now() - started) / 1000).toFixed(1)}s`);
  writeFileSync(`${out}ui-${port}.aria.yaml`, await page.locator("body").ariaSnapshot());
  const controls = await page.$$eval("button,[role=button],input,select,textarea,[role=tab],[role=menuitem],[role=checkbox],[role=radio],[role=slider],[role=combobox],canvas,[tabindex]", nodes => nodes.map(n => {
    const r = n.getBoundingClientRect();
    return `${n.tagName.toLowerCase()} role=${n.getAttribute("role") ?? ""} name=${JSON.stringify(n.getAttribute("aria-label") ?? n.getAttribute("title") ?? n.textContent?.trim().slice(0, 60) ?? "")} value=${JSON.stringify(n.value ?? "")} disabled=${n.disabled ?? n.getAttribute("aria-disabled") ?? ""} box=${Math.round(r.x)},${Math.round(r.y)},${Math.round(r.width)}x${Math.round(r.height)}`;
  }));
  writeFileSync(`${out}ui-${port}.controls.txt`, controls.join("\n"));
  await page.screenshot({ path: `${out}ui-${port}.png` });
  console.log(`controls ${controls.length}`);
} finally {
  await browser.close();
}
