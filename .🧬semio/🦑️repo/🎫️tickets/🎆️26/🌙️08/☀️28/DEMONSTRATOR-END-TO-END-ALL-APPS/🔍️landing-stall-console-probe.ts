/** 🔍️ Landing page, one load: records the shell beacon timeline and every console line matching boot/lease/timeout keywords once the 3rd pane starts. Read-only. */
import { chromium } from "playwright";
const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const BUDGET_MS = Number(process.env.PROBE_BUDGET_MS ?? 200_000);
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now(); const lines: string[] = [];
const keep = /aggregator|puzzle|lease|acquire|timeout|stall|queue|program load|boot|worker|shard|descriptor|READY|ready|fault|refused|admission|setContributions/i;
page.on("console", (m) => { const t = m.text(); if (keep.test(t)) lines.push(`${((Date.now() - t0) / 1000).toFixed(0)}s [${m.type()}] ${t.slice(0, 260)}`); });
page.on("pageerror", (e) => lines.push(`${((Date.now() - t0) / 1000).toFixed(0)}s [pageerror] ${String(e).slice(0, 260)}`));
await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(8_000);
const skip = page.getByText("Überspringen", { exact: true }); if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});
const seen = new Map<string, string>();
while (Date.now() - t0 < BUDGET_MS) {
  const shells = await page.$$eval("[data-shell-id]", (els) => els.map((e) => [e.getAttribute("data-shell-id"), e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error" : "booting"] as const));
  for (const [id, s] of shells) if (id && seen.get(id) !== s) { seen.set(id, s); lines.push(`${((Date.now() - t0) / 1000).toFixed(0)}s [beacon] ${id}: ${s}`); }
  await page.waitForTimeout(4_000);
}
const cut = lines.findIndex((l) => l.includes("[beacon] aggregator"));
for (const l of lines.slice(Math.max(0, cut - 3))) console.log(l);
console.log("final:", JSON.stringify(Object.fromEntries(seen)));
await browser.close();
