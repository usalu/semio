import { build } from "esbuild";
import { compile } from "@tailwindcss/node";
import { Scanner } from "@tailwindcss/oxide";
import { chromium } from "playwright";
import { createServer } from "node:http";
import { fileURLToPath } from "node:url";
import { resolve, dirname } from "node:path";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { browserOptions } from "./browser-env.mjs";

const folder = fileURLToPath(new URL(".", import.meta.url));
const generated = resolve(folder, "🗑️generated");
await mkdir(generated, { recursive: true });
console.log("[DEBUG] Building isolated browser fixture");
await build({ entryPoints: [resolve(folder, "browser.tsx")], outfile: resolve(generated, "browser.js"), bundle: true, format: "esm", platform: "browser", jsx: "automatic", loader: { ".css": "empty" }, plugins: [{ name: "repository-paths", setup(plugin) { plugin.onResolve({ filter: /^\/🧰/ }, args => ({ path: resolve(process.cwd(), args.path.slice(1)) })); } }] });
const cssPath = resolve("🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css");
const compiler = await compile(await readFile(cssPath, "utf8"), { base: dirname(cssPath), onDependency() {} });
const scanner = new Scanner({ sources: [{ base: resolve("🧰️framework/🔨️modules/🖱️ui"), pattern: "**/*.{ts,tsx}", negated: false }] });
await writeFile(resolve(generated, "browser.css"), compiler.build(scanner.scan()));
console.log("[DEBUG] Browser fixture built");
const server = createServer(async (request, response) => {
  try {
    if (new URL(request.url, "http://localhost").pathname === "/") { response.setHeader("Content-Type", "text/html"); response.end('<!doctype html><html lang="en"><head><link rel="stylesheet" href="/browser.css"></head><body><div id="root"></div><script type="module" src="/browser.js"></script></body></html>'); return; }
    const name = request.url === "/browser.css" ? "browser.css" : "browser.js";
    response.setHeader("Content-Type", name.endsWith("css") ? "text/css" : "text/javascript");
    response.end(await readFile(resolve(generated, name)));
  } catch (error) { response.statusCode = 500; response.end(String(error)); }
});
let browser;
try {
  await new Promise(done => server.listen(0, "127.0.0.1", done));
  browser = await chromium.launch(browserOptions());
  const page = await browser.newPage({ viewport: { width: 1000, height: 700 } });
  page.setDefaultTimeout(15000);
  const errors = [];
  page.on("pageerror", error => { errors.push(error.message); console.log("[DEBUG] Page error", error.message); });
  await page.goto(`http://127.0.0.1:${server.address().port}/`, { timeout: 20000, waitUntil: "domcontentloaded" });
  console.log("[DEBUG] Page loaded");
  await page.locator("[data-window-silhouette]").first().waitFor();
  await page.locator("[data-shared-border-segment]").first().waitFor({ state: "attached" });
  await page.mouse.move(450, 200);
  await page.waitForFunction(() => [...document.querySelectorAll("[data-shared-border-segment]")].some(element => element.getAttribute("data-shared-border-state") === "hover"));
  const result = await page.evaluate(() => ({
    windows: document.querySelectorAll("[data-window-silhouette]").length,
    segments: document.querySelectorAll("[data-shared-border-segment]").length,
    states: [...document.querySelectorAll("[data-shared-border-segment]")].map(element => element.getAttribute("data-shared-border-state")),
    attached: [...document.querySelectorAll("[data-shared-border-attached]")].map(element => ({ id: element.id, sides: element.getAttribute("data-shared-border-attached") })),
  }));
  if (errors.length) throw new Error(errors.join("\n"));
  if (result.windows !== 2 || !result.states.includes("hover")) throw new Error("Shared boundary hover did not render");
  const overlaySizes = await page.locator('#Left-select').evaluate(element => {
    const overlay = element.querySelector('[data-shared-border-overlay]');
    const frame = element.getBoundingClientRect();
    const paint = overlay?.getBoundingClientRect();
    return { frameWidth: frame.width, paintWidth: paint?.width, frameHeight: frame.height, paintHeight: paint?.height };
  });
  if (Math.abs(overlaySizes.frameWidth - (overlaySizes.paintWidth ?? 0)) > 1 || Math.abs(overlaySizes.frameHeight - (overlaySizes.paintHeight ?? 0)) > 1) throw new Error(`Icon sizing shrank the frame overlay: ${JSON.stringify(overlaySizes)}`);
  await page.locator('#Left-menu').click();
  const popup = page.locator('[data-probe="Left-popup"]');
  await popup.waitFor();
  const menu = await popup.evaluate(element => {
    const rect = element.getBoundingClientRect();
    const target = document.elementFromPoint(rect.right - 8, rect.top + rect.height / 2);
    return { portal: !!element.closest('#portal-layer'), reachable: element.contains(target), width: rect.width, height: rect.height };
  });
  if (!menu.portal || !menu.reachable) throw new Error(`Menu is clipped or behind its neighboring window: ${JSON.stringify(menu)}`);
  await page.keyboard.press('Escape');
  await popup.waitFor({ state: 'detached' });
  await page.locator('#Left-select').click();
  const listbox = page.getByRole('listbox');
  await listbox.waitFor();
  if (!await listbox.evaluate(element => !!element.closest('#portal-layer'))) throw new Error('Select escaped its shell portal');
  await page.keyboard.press('Escape');
  await page.mouse.move(450, 200);
  await page.screenshot({ path: resolve(generated, "shared-borders.png") });
  await page.emulateMedia({ forcedColors: 'active', reducedMotion: 'reduce' });
  await page.waitForFunction(() => matchMedia('(forced-colors: active)').matches);
  const forcedHover = await page.waitForFunction(() => {
    const segment = document.querySelector('[data-shared-border-segment][data-shared-border-state="hover"]');
    if (!segment?.parentElement) return false;
    const probe = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    probe.style.setProperty('stroke', 'Highlight', 'important'); segment.parentElement.append(probe);
    const highlight = getComputedStyle(probe).stroke; probe.remove();
    const stroke = getComputedStyle(segment).stroke;
    return stroke === highlight ? { stroke, highlight } : false;
  }).then(handle => handle.jsonValue());
  if (forcedHover.stroke !== forcedHover.highlight) throw new Error(`Forced-colors hover lost system Highlight: ${JSON.stringify(forcedHover)}`);
  await page.screenshot({ path: resolve(generated, "forced-colors.png") });
  await page.emulateMedia({ forcedColors: 'none', reducedMotion: 'no-preference' });
  await page.goto(`http://127.0.0.1:${server.address().port}/?markers-only`, { timeout: 20000, waitUntil: 'domcontentloaded' });
  await page.locator('#standalone-left[data-shared-border-attached]').waitFor();
  await page.mouse.move(250, 95);
  await page.waitForFunction(() => !!document.querySelector('#standalone-right [data-shared-border-state="hover"]'));
  const standalone = await page.evaluate(() => ({ windows: document.querySelectorAll('[data-window-silhouette]').length, attached: document.querySelectorAll('[data-shared-border-attached]').length }));
  if (standalone.windows !== 0 || standalone.attached < 2) throw new Error(`Marker-only root did not activate: ${JSON.stringify(standalone)}`);
  if (errors.length) throw new Error(errors.join("\n"));
  console.log(JSON.stringify({ ...result, overlaySizes, menu, forcedHover, standalone }, null, 2));
} catch (error) {
  console.error("[DEBUG] Browser validation failed", error);
  throw error;
} finally {
  await browser?.close();
  await new Promise(done => server.close(done));
}
