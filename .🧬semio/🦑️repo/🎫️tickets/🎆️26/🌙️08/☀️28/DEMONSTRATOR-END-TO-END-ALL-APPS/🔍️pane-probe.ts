/** 🔍️ Loads one demonstrator pane, decodes any `{"tag":"fault","val":{…}}` byte-map into readable text,
 * and reports the fatal boot fault distinctly from non-fatal program-load warnings. Read-only. */
import { chromium } from "playwright";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const PANE = process.env.PROBE_PANE ?? "generator";

function decodeFaults(text: string): string {
  return text.replace(/\{"tag":"fault","val":\{[^}]*\}\}/g, (blob) => {
    try {
      const val = JSON.parse(blob).val as Record<string, number>;
      const bytes = Object.keys(val).sort((a, b) => Number(a) - Number(b)).map((k) => val[k]!);
      return new TextDecoder().decode(new Uint8Array(bytes));
    } catch { return blob; }
  });
}

const browser = await chromium.launch();
const page = await browser.newPage();
const errors: string[] = [];
page.on("console", (m) => { if (m.type() === "error") errors.push(decodeFaults(m.text())); });
page.on("pageerror", (e) => errors.push("PAGEERROR " + decodeFaults(String(e))));
await page.goto(`${BASE}#${PANE}`, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(30_000);
const fatal = errors.filter((e) => /boot failed|boot fault|first-step|trapped/.test(e));
const loadFails = [...new Set(errors.filter((e) => /program load failed/.test(e)).map((e) => (e.match(/program load failed (\S+)/) ?? [])[1]))];
console.log("PANE:", PANE);
console.log("non-fatal program-load failures:", loadFails.join(", ") || "(none)");
console.log("FATAL (" + fatal.length + "):");
for (const f of [...new Set(fatal)].slice(0, 4)) console.log("  -", f.slice(0, 500));
await browser.close();
