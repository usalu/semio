/** 📜️ Dumps every console line of a fresh FEM editor in headless Chromium, then screenshots. Usage: node console-dump-probe.mjs <url> [seconds] */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/", seconds = Number(process.argv[3] ?? 120);
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  const started = Date.now();
  page.on("console", message => console.log(`${((Date.now() - started) / 1000).toFixed(1)}s [${message.type()}] ${message.text().slice(0, 400).replace(/[^\x20-\x7e]/g, "")}`));
  page.on("pageerror", error => console.log(`pageerror ${error.message.slice(0, 400)}`));
  await page.addInitScript(() => {
    const context = CanvasRenderingContext2D.prototype, fillText = context.fillText;
    window.femLabels = [];
    setInterval(() => { const host = document.querySelector('[data-surface-window-id="fem3d-results"]'); const text = host ? host.innerText.split(String.fromCharCode(10))[0].slice(0, 80) : "<none>"; if (window.lastCaption !== text) { window.lastCaption = text; console.log(`[DEBUG] results caption ${text}`); } }, 200);
    context.fillText = function (...args) { if (/· .* \/ /.test(String(args[0]))) window.femLabels.push(`${(performance.now() / 1000).toFixed(1)}s ${args[0]}`); return fillText.apply(this, args); };
  });
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForTimeout(seconds * 1000);
  console.log(`labels ${JSON.stringify(await page.evaluate(() => [...new Set(window.femLabels)].slice(-8)))}`);
  console.log(`caption-owner ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("*")].filter(node => node.childElementCount === 0 && /Preparing|Analysis queued|Dead Load|Load case|Case/.test(node.textContent ?? "")).slice(0, 6).map(node => `${node.textContent} @ ${(() => { const chain = []; for (let at = node; at && chain.length < 9; at = at.parentElement) chain.push(`${at.tagName}${[...at.attributes].filter(attribute => attribute.name.startsWith("data-")).map(attribute => `[${attribute.name}=${attribute.value.slice(0, 40)}]`).join("")}`); return chain.join(" < "); })()}`)))}`);
  console.log(`caption ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("*")].map(node => node.childElementCount === 0 ? node.textContent : "").filter(text => /Converged|Pending|Failed|Preparing/.test(text ?? "")).slice(0, 10)))}`);
  await page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/console-dump-${new URL(url).port}.png`, import.meta.url)) });
} finally {
  await browser.close();
}
