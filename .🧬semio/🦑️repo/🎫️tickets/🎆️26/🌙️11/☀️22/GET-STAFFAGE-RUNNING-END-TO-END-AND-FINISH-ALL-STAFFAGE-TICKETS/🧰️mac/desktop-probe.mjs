/** 🔎️ Opens the desktop overview of a built site, chooses English and reports what the page holds every few seconds: pages, links, cards, script heap and the browser's lifecycle events. Usage: node desktop-probe.mjs <dist-directory-or-url> <output-directory> [route] [seconds] */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "playwright";
import { serve } from "../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs";

const [release, out, route = "", seconds = "40"] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const local = /^https?:/u.test(release) ? { url: release, server: { close: () => undefined } } : await serve(release);
const browser = await chromium.launch({ channel: "chromium", args: ["--disable-crash-reporter", "--disable-crashpad", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
browser.on("disconnected", () => console.log("[DEBUG] browser disconnected"));
try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "fr-FR" });
  const page = await context.newPage();
  page.on("crash", () => console.log("[DEBUG] page crashed"));
  page.on("close", () => console.log("[DEBUG] page closed"));
  page.on("pageerror", (error) => console.log("[DEBUG] pageerror", error.message));
  page.on("console", (message) => message.type() === "error" && console.log("[DEBUG] console", message.text()));
  const session = await context.newCDPSession(page);
  await page.goto(local.url, { waitUntil: "domcontentloaded" });
  await page.getByRole("button", { name: "English", exact: true }).click();
  if (route) await page.goto(new URL(route, local.url).href);
  await page.reload({ waitUntil: "domcontentloaded" });
  const started = Date.now();
  for (let step = 0; (Date.now() - started) / 1000 < Number(seconds); step += 1) {
    await page.waitForTimeout(4000);
    const state = await Promise.race([
      page.evaluate(() => ({
        pages: [...document.querySelectorAll("[data-page]")].map((node) => node.getAttribute("data-page") + (node.checkVisibility() ? "" : " (hidden)")),
        links: document.querySelectorAll('a[href^="#/"]').length,
        cards: document.querySelectorAll("[data-card]").length,
        busy: document.querySelectorAll('[aria-busy="true"]').length,
        figures: document.querySelectorAll("[data-staffage-figure] svg").length,
        heapMB: Math.round((performance.memory?.usedJSHeapSize ?? 0) / 1048576),
        elements: document.querySelectorAll("*").length,
        assetModules: performance.getEntriesByType("resource").filter((entry) => decodeURIComponent(entry.name).includes("/assets/🔣️")).length,
        assetModulesMB: Math.round(performance.getEntriesByType("resource").filter((entry) => decodeURIComponent(entry.name).includes("/assets/🔣️")).reduce((sum, entry) => sum + entry.decodedBodySize, 0) / 1048576),
        lang: document.documentElement.lang,
        title: document.title,
      })),
      new Promise((done) => setTimeout(() => done("page did not answer in 20 s"), 20000)),
    ]);
    const within = (promise) => Promise.race([promise, new Promise((done) => setTimeout(() => done(null), 5000))]);
    const heap = await within(session.send("Runtime.getHeapUsage"));
    const dom = await within(session.send("Memory.getDOMCounters"));
    const workers = await within(Promise.all(page.workers().map((worker) => worker.evaluate(() => Math.round((performance.memory?.usedJSHeapSize ?? 0) / 1048576)).then((megabytes) => decodeURIComponent(worker.url().split("/").pop().split("?")[0]) + " " + megabytes + " MB", () => "gone"))));
    console.log("[DEBUG]", ((Date.now() - started) / 1000).toFixed(0) + " s", JSON.stringify(state), heap === null ? "" : JSON.stringify({ usedMB: Math.round(heap.usedSize / 1048576), totalMB: Math.round(heap.totalSize / 1048576), embedderMB: Math.round((heap.embedderHeapUsedSize ?? 0) / 1048576), backingMB: Math.round((heap.backingStorageSize ?? 0) / 1048576) }), dom === null ? "" : JSON.stringify(dom), JSON.stringify(workers));
    if (typeof state !== "string") await page.screenshot({ path: join(out, "desktop-probe-" + step + ".png"), timeout: 20000 }).catch((error) => console.log("[DEBUG] screenshot", error.message.split("\n")[0]));
  }
} finally {
  await browser.close().catch(() => undefined);
  local.server.close();
}
