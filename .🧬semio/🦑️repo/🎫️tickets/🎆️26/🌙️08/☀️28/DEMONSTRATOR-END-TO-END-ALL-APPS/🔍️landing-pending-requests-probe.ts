/** 🔍️ Landing page, one load: tracks in-flight HTTP requests and reports the long-lived ones once the 3rd pane has been "booting" for a while — connection-pool exhaustion diagnosis. Read-only. */
import { chromium } from "playwright";
const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const BUDGET_MS = Number(process.env.PROBE_BUDGET_MS ?? 150_000);
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now(); const pending = new Map<any, { url: string; at: number }>();
page.on("request", (r) => pending.set(r, { url: r.url(), at: Date.now() }));
page.on("requestfinished", (r) => pending.delete(r)); page.on("requestfailed", (r) => pending.delete(r));
await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(8_000);
const skip = page.getByText("Überspringen", { exact: true }); if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});
const seen = new Map<string, string>();
const report = (label: string) => {
  const now = Date.now(); const rows = [...pending.values()].map((p) => ({ age: Math.round((now - p.at) / 1000), url: p.url.replace(/^http:\/\/127\.0\.0\.1:6029/, "").replace(/%F0%9F[^/]*\//g, "…/").slice(0, 110) })).sort((a, b) => b.age - a.age);
  console.log(`--- ${label} @ ${((now - t0) / 1000).toFixed(0)}s: ${rows.length} in-flight (${rows.filter((r) => r.age > 20).length} older than 20s)`);
  for (const r of rows.slice(0, 14)) console.log(`   ${String(r.age).padStart(4)}s ${r.url}`);
};
while (Date.now() - t0 < BUDGET_MS) {
  const shells = await page.$$eval("[data-shell-id]", (els) => els.map((e) => [e.getAttribute("data-shell-id"), e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error" : "booting"] as const));
  for (const [id, s] of shells) if (id && seen.get(id) !== s) { seen.set(id, s); console.log(`${((Date.now() - t0) / 1000).toFixed(0)}s [beacon] ${id}: ${s}`); if (s === "ready") report(`after ${id} ready`); }
  await page.waitForTimeout(4_000);
}
report("final"); console.log("final:", JSON.stringify(Object.fromEntries(seen)));
await browser.close();
