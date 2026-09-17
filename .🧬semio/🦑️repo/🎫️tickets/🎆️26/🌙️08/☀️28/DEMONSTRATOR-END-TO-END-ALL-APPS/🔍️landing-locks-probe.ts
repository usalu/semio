/** 🔍️ Landing page, one load: once the 3rd pane has been "booting" for 40 s, dumps navigator.locks.query(), IndexedDB databases and per-shell DOM state. Read-only. */
import { chromium } from "playwright";
const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(8_000);
const skip = page.getByText("Überspringen", { exact: true }); if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});
const seen = new Map<string, [string, number]>();
while (Date.now() - t0 < 200_000) {
  const shells = await page.$$eval("[data-shell-id]", (els) => els.map((e) => [e.getAttribute("data-shell-id"), e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error" : "booting"] as const)).catch(() => []);
  for (const [id, s] of shells) if (id && seen.get(id)?.[0] !== s) { seen.set(id, [s, Date.now()]); console.log(`${((Date.now() - t0) / 1000).toFixed(0)}s [beacon] ${id}: ${s}`); }
  const stuck = [...seen].filter(([, [s, at]]) => s === "booting" && Date.now() - at > 40_000);
  if (stuck.length) { console.log("stuck:", stuck.map(([id]) => id).join(", ")); break; }
  await page.waitForTimeout(4_000);
}
const dump = await page.evaluate(async () => {
  const locks = await (navigator as any).locks?.query?.().catch((e: any) => ({ error: String(e) }));
  const dbs = await (indexedDB as any).databases?.().catch((e: any) => [{ error: String(e) }]);
  const shells = [...document.querySelectorAll("[data-shell-id]")].map((e) => ({ id: e.getAttribute("data-shell-id"), attrs: [...e.attributes].filter((a) => a.name.startsWith("data-")).map((a) => `${a.name}=${a.value.slice(0, 40)}`), text: (e as HTMLElement).innerText.slice(0, 80).replace(/\n/g, " | ") }));
  const workers = (performance.getEntriesByType("resource") as PerformanceResourceTiming[]).filter((r) => /\.wasm|bridge\.js|descriptor|🔣️/.test(decodeURIComponent(r.name))).map((r) => ({ n: decodeURIComponent(r.name).replace(/^.*plugin-modules\//, "").slice(0, 60), ms: Math.round(r.duration), size: r.transferSize }));
  return { locks, dbs, shells, workers };
});
console.log("LOCKS", JSON.stringify(dump.locks)); console.log("DBS", JSON.stringify(dump.dbs)); console.log("WORKERS", JSON.stringify(dump.workers)); console.log("SHELLS", JSON.stringify(dump.shells.map((s: any) => [s.id, s.attrs.filter((a: string) => a.startsWith("data-shell")), s.text])));
await browser.close();
