/** 🔍️ Landing page, one load: logs every EventSource/long-lived request's open → connected → closed lifecycle next to the shell beacons,
 * to test the HTTP/1.1 six-connections-per-origin hypothesis for the 3rd-pane boot stall. Read-only. */
import { chromium } from "playwright";
const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now(); const ts = () => `${((Date.now() - t0) / 1000).toFixed(0)}s`;
const short = (u: string) => decodeURIComponent(u).replace(/^http:\/\/127\.0\.0\.1:6029/, "").replace(/\/[^/]*?([a-z0-9-]+)\/(?=.*watch)/g, "/$1/").slice(0, 90);
const open = new Map<any, { url: string; at: number; connected: boolean }>();
page.on("request", (r) => { if (r.resourceType() === "eventsource" || /watch/.test(r.url())) { open.set(r, { url: r.url(), at: Date.now(), connected: false }); console.log(`${ts()} SSE open      ${short(r.url())}`); } });
page.on("response", (res) => { const r = res.request(); const o = open.get(r); if (o) { o.connected = true; console.log(`${ts()} SSE connected ${short(o.url)} (${res.status()})`); } });
page.on("requestfinished", (r) => { const o = open.get(r); if (o) { open.delete(r); console.log(`${ts()} SSE closed    ${short(o.url)} after ${Math.round((Date.now() - o.at) / 1000)}s`); } });
page.on("requestfailed", (r) => { const o = open.get(r); if (o) { open.delete(r); console.log(`${ts()} SSE failed    ${short(o.url)} ${r.failure()?.errorText}`); } });
await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(8_000);
const skip = page.getByText("Überspringen", { exact: true }); if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});
const seen = new Map<string, string>();
while (Date.now() - t0 < 200_000) {
  const shells = await page.$$eval("[data-shell-id]", (els) => els.map((e) => [e.getAttribute("data-shell-id"), e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error" : "booting"] as const)).catch(() => []);
  for (const [id, s] of shells) if (id && seen.get(id) !== s) { seen.set(id, s); console.log(`${ts()} [beacon] ${id}: ${s}  | open streams: ${open.size} (unconnected ${[...open.values()].filter((o) => !o.connected).length})`); }
  await page.waitForTimeout(3_000);
}
console.log("final open streams:", [...open.values()].map((o) => `${short(o.url)}${o.connected ? "" : " [UNCONNECTED]"}`).join(" ; "));
console.log("final:", JSON.stringify(Object.fromEntries(seen)));
await browser.close();
