/** 🔎️ The s1 idle probe of the English desktop overview with the memory of the browser added: every 4 s the pages, figures, live previews, DOM nodes and script heap of the page, and the resident memory of the Chromium renderer processes and of all Chromium processes read from `/proc`. Writes one JSON line per sample to `<output-directory>/s7-samples.jsonl`. Usage: node idle-probe.mjs <dist-directory-or-url> <output-directory> [route] [seconds] */
import { appendFileSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "playwright";
import { serve } from "../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs";

const [release, out, route = "", seconds = "160"] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const samples = join(out, "s7-samples.jsonl");

function chromiumMemory() {
  const table = new Map();
  for (const name of readdirSync("/proc")) {
    if (!/^\d+$/u.test(name)) continue;
    try {
      const stat = readFileSync(`/proc/${name}/stat`, "utf8");
      const parent = Number(stat.slice(stat.lastIndexOf(")") + 2).split(" ")[1]);
      const status = readFileSync(`/proc/${name}/status`, "utf8");
      const rss = Number(/VmRSS:\s+(\d+)/u.exec(status)?.[1] ?? 0);
      const swap = Number(/VmSwap:\s+(\d+)/u.exec(status)?.[1] ?? 0);
      table.set(Number(name), { parent, rss, swap, command: readFileSync(`/proc/${name}/cmdline`, "utf8") });
    } catch {}
  }
  const mine = (pid) => { for (let at = pid, hops = 0; at > 1 && hops < 12; hops += 1) { if (at === process.pid) return true; at = table.get(at)?.parent ?? 0; } return false; };
  let rendererKB = 0, rendererSwapKB = 0, allKB = 0, largestRendererKB = 0, renderers = 0;
  for (const [pid, entry] of table) {
    if (pid === process.pid || !mine(pid) || !/chrom/iu.test(entry.command)) continue;
    allKB += entry.rss;
    if (!entry.command.includes("--type=renderer")) continue;
    renderers += 1;
    rendererKB += entry.rss;
    rendererSwapKB += entry.swap;
    largestRendererKB = Math.max(largestRendererKB, entry.rss);
  }
  const megabytes = (kilobytes) => Math.round(kilobytes / 1024);
  return { renderers, rendererMB: megabytes(rendererKB), largestRendererMB: megabytes(largestRendererKB), rendererSwapMB: megabytes(rendererSwapKB), chromiumMB: megabytes(allKB) };
}

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
        cards: document.querySelectorAll("[data-card]").length,
        busy: document.querySelectorAll('[aria-busy="true"]').length,
        figures: document.querySelectorAll("[data-staffage-figure] svg, [data-placed] svg").length,
        live: document.querySelectorAll('[data-preview-state="live"]').length,
        still: document.querySelectorAll('[data-preview-state="still"]').length,
        svgNodes: document.querySelectorAll("svg *").length,
        heapMB: Math.round((performance.memory?.usedJSHeapSize ?? 0) / 1048576),
        elements: document.querySelectorAll("*").length,
        lang: document.documentElement.lang,
      })),
      new Promise((done) => setTimeout(() => done("page did not answer in 20 s"), 20000)),
    ]);
    const within = (promise) => Promise.race([promise, new Promise((done) => setTimeout(() => done(null), 5000))]);
    const heap = await within(session.send("Runtime.getHeapUsage"));
    const dom = await within(session.send("Memory.getDOMCounters"));
    const workers = await within(Promise.all(page.workers().map((worker) => worker.evaluate(() => Math.round((performance.memory?.usedJSHeapSize ?? 0) / 1048576)).then((megabytes) => decodeURIComponent(worker.url().split("/").pop().split("?")[0]) + " " + megabytes + " MB", () => "gone"))));
    const sample = { s: Math.round((Date.now() - started) / 1000), ...chromiumMemory(), state, heap: heap === null ? null : { usedMB: Math.round(heap.usedSize / 1048576), totalMB: Math.round(heap.totalSize / 1048576), embedderMB: Math.round((heap.embedderHeapUsedSize ?? 0) / 1048576) }, dom, workers };
    appendFileSync(samples, JSON.stringify(sample) + "\n");
    console.log("[DEBUG]", JSON.stringify(sample));
    if (typeof state !== "string") await page.screenshot({ path: join(out, "s7-probe-" + step + ".png"), timeout: 20000 }).catch((error) => console.log("[DEBUG] screenshot", error.message.split("\n")[0]));
  }
} finally {
  await browser.close().catch(() => undefined);
  local.server.close();
}
