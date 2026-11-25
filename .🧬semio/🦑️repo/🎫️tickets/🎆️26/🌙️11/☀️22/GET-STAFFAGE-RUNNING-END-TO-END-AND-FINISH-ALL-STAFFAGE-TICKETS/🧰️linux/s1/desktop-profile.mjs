/** 🔬️ Profiles the main thread of a page of a built site after a reload and lists where its time goes: self time per function and long tasks. Usage: node desktop-profile.mjs <dist-directory-or-url> <output-directory> [route] [seconds] [width] [height] */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "playwright";
import { serve } from "../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs";

const [release, out, route = "", seconds = "45", width = "1440", height = "900"] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const local = /^https?:/u.test(release) ? { url: release, server: { close: () => undefined } } : await serve(release);
const browser = await chromium.launch({ channel: "chromium", args: ["--disable-crash-reporter", "--disable-crashpad", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
browser.on("disconnected", () => console.log("[DEBUG] browser disconnected"));
try {
  const context = await browser.newContext({ viewport: { width: Number(width), height: Number(height) }, locale: "fr-FR" });
  await context.addInitScript(() => {
    window.__long = [];
    new PerformanceObserver((list) => { for (const entry of list.getEntries()) window.__long.push([Math.round(entry.startTime), Math.round(entry.duration)]); }).observe({ type: "longtask", buffered: true });
  });
  const page = await context.newPage();
  page.on("crash", () => console.log("[DEBUG] page crashed"));
  page.on("pageerror", (error) => console.log("[DEBUG] pageerror", error.message));
  await page.goto(local.url, { waitUntil: "domcontentloaded" });
  await page.getByRole("button", { name: "English", exact: true }).click();
  if (route) await page.goto(new URL(route, local.url).href);
  const session = await context.newCDPSession(page);
  await session.send("Profiler.enable");
  await session.send("Profiler.setSamplingInterval", { interval: 2000 });
  await session.send("Profiler.start");
  const started = Date.now();
  await page.reload({ waitUntil: "domcontentloaded" });
  await new Promise((done) => setTimeout(done, Number(seconds) * 1000));
  const { profile } = await session.send("Profiler.stop");
  console.log("[DEBUG] profile returned after", ((Date.now() - started) / 1000).toFixed(1), "s");
  const self = new Map();
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const interval = (profile.endTime - profile.startTime) / Math.max(1, profile.samples.length);
  for (const id of profile.samples) {
    const { callFrame } = byId.get(id);
    const key = (callFrame.functionName || "(anonymous)") + " " + decodeURIComponent(callFrame.url.split("/").pop() ?? "") + ":" + callFrame.lineNumber;
    self.set(key, (self.get(key) ?? 0) + interval / 1000);
  }
  const top = [...self].sort((a, b) => b[1] - a[1]).slice(0, 40).map(([name, ms]) => ms.toFixed(0).padStart(8) + " ms  " + name);
  const long = await page.evaluate(() => window.__long);
  const state = await page.evaluate(() => ({ pages: [...document.querySelectorAll("[data-page]")].map((node) => node.getAttribute("data-page")), cards: document.querySelectorAll("[data-card]").length, busy: document.querySelectorAll('[aria-busy="true"]').length, figures: document.querySelectorAll("[data-staffage-figure] svg").length, heapMB: Math.round((performance.memory?.usedJSHeapSize ?? 0) / 1048576) }));
  writeFileSync(join(out, "profile.cpuprofile"), JSON.stringify(profile));
  console.log("[DEBUG] state", JSON.stringify(state));
  console.log("[DEBUG] long tasks [start ms, duration ms] over 200 ms:", JSON.stringify(long.filter(([, duration]) => duration > 200)), "sum", long.reduce((sum, [, duration]) => sum + duration, 0), "ms of", long.length);
  console.log("[DEBUG] self time over", ((profile.endTime - profile.startTime) / 1e6).toFixed(1), "s:\n" + top.join("\n"));
  await page.screenshot({ path: join(out, "profile-end.png"), timeout: 30000 }).catch(() => undefined);
} finally {
  await browser.close().catch(() => undefined);
  local.server.close();
}
