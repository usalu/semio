/** 🔬 Probes WebGPU availability in the pinned chromium on a secure origin; extra launch flags may be passed as arguments. */
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
process.env.PLAYWRIGHT_BROWSERS_PATH = repoToolCacheEnv(join(ticket, "../../../../../../..")).PLAYWRIGHT_BROWSERS_PATH;
const { chromium } = await import("playwright");
const extra = process.argv.slice(2);
const browser = await chromium.launch({ channel: "chromium", headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--enable-gpu", ...(process.platform === "darwin" ? ["--use-angle=metal"] : []), ...extra] });
try {
  const page = await browser.newPage();
  await page.route("https://probe.test/", route => route.fulfill({ contentType: "text/html", body: "<html></html>" }));
  await page.goto("https://probe.test/");
  const result = await page.evaluate(async () => {
    if (!navigator.gpu) return { gpu: false };
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) return { gpu: true, adapter: null };
    let device;
    try { device = !!(await adapter.requestDevice()); } catch (error) { device = String(error); }
    return { gpu: true, adapter: { vendor: adapter.info?.vendor, architecture: adapter.info?.architecture, description: adapter.info?.description, fallback: adapter.isFallbackAdapter }, device };
  });
  console.log(JSON.stringify({ browsers: process.env.PLAYWRIGHT_BROWSERS_PATH, executable: chromium.executablePath(), version: browser.version(), extra, result }));
} finally { await browser.close(); }
