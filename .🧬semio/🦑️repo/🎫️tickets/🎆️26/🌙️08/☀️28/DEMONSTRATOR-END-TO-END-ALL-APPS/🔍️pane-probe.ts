/** 🔍️ Loads one demonstrator pane and dumps every console error plus the shell's own state attribute,
 * so a boot failure can be attributed to a specific plugin instead of guessing. Read-only. */
import { chromium } from "playwright";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const PANE = process.env.PROBE_PANE ?? "generator";

const browser = await chromium.launch();
const page = await browser.newPage();
const errors: string[] = [];
page.on("console", (m) => { if (m.type() === "error") errors.push(m.text().slice(0, 400)); });
page.on("pageerror", (e) => errors.push("PAGEERROR " + String(e).slice(0, 400)));
await page.goto(`${BASE}#${PANE}`, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(30_000);
const state = await page.evaluate(() => {
  const el = document.querySelector("[data-pane-shell-state],[data-shell-state],[data-demonstrator-pane]");
  return el ? Object.fromEntries([...el.attributes].map((a) => [a.name, a.value])) : null;
});
console.log("PANE:", PANE);
console.log("SHELL ATTRS:", JSON.stringify(state));
console.log("ERRORS (" + errors.length + "):");
for (const e of [...new Set(errors)].slice(0, 8)) console.log("  -", e);
await browser.close();
