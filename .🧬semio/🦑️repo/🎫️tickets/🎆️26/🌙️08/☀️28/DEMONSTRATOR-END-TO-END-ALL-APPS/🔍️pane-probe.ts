/** 🔍️ Loads each demonstrator pane and reports the first boot fault, so a stale component can be
 * attributed to ONE plugin instead of rebuilding all nine. Read-only: navigates and reads console. */
import { chromium } from "playwright";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const PANES = ["generator", "koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen"];

const browser = await chromium.launch();
for (const pane of PANES) {
  const page = await browser.newPage();
  const faults: string[] = [];
  page.on("console", (message) => {
    const text = message.text();
    if (/boot fault text|unknown app|descriptor-invalid|worker fault/.test(text)) faults.push(text.slice(0, 300));
  });
  try {
    await page.goto(`${BASE}#${pane}`, { waitUntil: "domcontentloaded", timeout: 60_000 });
    await page.waitForTimeout(25_000);
  } catch (error) {
    faults.push(`NAV ERROR ${String(error).slice(0, 120)}`);
  }
  const unknown = [...new Set(faults.flatMap((f) => f.match(/unknown app: [^"]+/g) ?? []))];
  console.log(`\n=== ${pane} ===`);
  console.log(unknown.length ? unknown.join("\n") : (faults[0] ?? "no fault captured"));
  await page.close();
}
await browser.close();
