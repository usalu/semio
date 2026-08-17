import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import path from "node:path";

const TICKET = import.meta.dirname;
const PANE_IDS = ["generator", "koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen"] as const;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1920, height: 1080 } });
const consoleLines: string[] = [];
page.on("console", (msg) => consoleLines.push(`[${msg.type()}] ${msg.text()}`));

await page.goto("http://127.0.0.1:6029/", { waitUntil: "domcontentloaded", timeout: 180000 });

for (let i = 0; i < 6; i++) {
  const skip = page.getByRole("button", { name: /überspringen|skip/i }).first();
  if (await skip.isVisible().catch(() => false)) {
    await skip.click({ force: true });
    await page.waitForTimeout(400);
  } else break;
}

const deadline = Date.now() + 120_000;
let paneSnaps: Record<string, unknown> = {};
while (Date.now() < deadline) {
  paneSnaps = await page.evaluate((ids) => {
    const out: Record<string, { bodyText: string; hasError: boolean; loading: boolean; canvas: number }> = {};
    for (const id of ids) {
      const shell = document.querySelector(`[data-shell-id="${id}"]`);
      const text = shell?.textContent ?? "";
      out[id] = {
        bodyText: text.slice(0, 400).replace(/\s+/g, " ").trim(),
        hasError: /Keine Plugins geladen/i.test(text),
        loading: /Plugins werden geladen/i.test(text),
        canvas: shell?.querySelectorAll("canvas").length ?? 0,
      };
    }
    return out;
  }, PANE_IDS);
  const booted = PANE_IDS.filter((id) => {
    const s = paneSnaps[id] as { hasError: boolean; loading: boolean; bodyText: string };
    return !s.loading && !s.hasError && s.bodyText.length > 80;
  }).length;
  console.log(`[DEBUG] booted=${booted}/6`, PANE_IDS.map((id) => `${id}:${(paneSnaps[id] as { hasError: boolean }).hasError ? "err" : "ok"}`).join(" "));
  if (booted >= 6) break;
  await page.waitForTimeout(2000);
}

await page.screenshot({ path: path.join(TICKET, "probe-six-panes-overview.png"), fullPage: true });

const aussuchen = paneSnaps.aussuchen as { bodyText: string; hasError: boolean };
const result = {
  paneSnaps,
  aussuchenOk: !aussuchen.hasError && /Pool|Glulam|Kuratieren|sourcing/i.test(aussuchen.bodyText),
  consoleTail: consoleLines.slice(-80),
};

writeFileSync(path.join(TICKET, "probe-six-panes.json"), JSON.stringify(result, null, 2));
writeFileSync(path.join(TICKET, "probe-six-panes-console.txt"), consoleLines.join("\n"));
console.log(JSON.stringify({ aussuchenOk: result.aussuchenOk, aussuchen: paneSnaps.aussuchen }, null, 2));

await browser.close();
process.exit(result.aussuchenOk ? 0 : 1);
