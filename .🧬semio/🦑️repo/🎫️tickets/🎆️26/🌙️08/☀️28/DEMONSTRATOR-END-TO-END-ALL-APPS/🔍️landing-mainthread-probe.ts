/** 🔍️ Landing page, one load: after each pane beacon change, samples main-thread latency (setTimeout(0) delay, rAF gap) and console line rates,
 * to tell a starved/wedged page from a boot that is merely waiting. Read-only. */
import { chromium } from "playwright";
const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const BUDGET_MS = Number(process.env.PROBE_BUDGET_MS ?? 200_000);
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now(); let total = 0; const byKey = new Map<string, number>();
page.on("console", (m) => { total++; const t = m.text(); for (const k of ["setContributions", "contributions", "program load", "lease", "timeout", "stall", "worker", "boot"]) if (t.includes(k)) byKey.set(k, (byKey.get(k) ?? 0) + 1); });
await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(8_000);
const skip = page.getByText("Überspringen", { exact: true }); if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});
const seen = new Map<string, string>(); let lastTotal = 0; let lastKeys = new Map<string, number>();
const sample = async (label: string) => {
  const lat = await page.evaluate(() => new Promise<{ timeoutMs: number; rafMs: number; longTasks: number }>((resolve) => {
    const a = performance.now(); let longTasks = 0; try { new PerformanceObserver((l) => { longTasks += l.getEntries().length; }).observe({ type: "longtask", buffered: true }); } catch {}
    setTimeout(() => { const timeoutMs = performance.now() - a; const b = performance.now(); requestAnimationFrame(() => resolve({ timeoutMs: Math.round(timeoutMs), rafMs: Math.round(performance.now() - b), longTasks })); }, 0);
  })).catch(() => null);
  const delta = total - lastTotal; lastTotal = total;
  const keys = [...byKey].map(([k, v]) => `${k}:+${v - (lastKeys.get(k) ?? 0)}`).join(" "); lastKeys = new Map(byKey);
  console.log(`${((Date.now() - t0) / 1000).toFixed(0)}s ${label} | latency ${JSON.stringify(lat)} | console +${delta} (${keys})`);
};
while (Date.now() - t0 < BUDGET_MS) {
  const shells = await page.$$eval("[data-shell-id]", (els) => els.map((e) => [e.getAttribute("data-shell-id"), e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error" : "booting"] as const)).catch(() => []);
  for (const [id, s] of shells) if (id && seen.get(id) !== s) { seen.set(id, s); console.log(`${((Date.now() - t0) / 1000).toFixed(0)}s [beacon] ${id}: ${s}`); }
  await sample("tick");
  await page.waitForTimeout(10_000);
}
console.log("final:", JSON.stringify(Object.fromEntries(seen)));
await browser.close();
