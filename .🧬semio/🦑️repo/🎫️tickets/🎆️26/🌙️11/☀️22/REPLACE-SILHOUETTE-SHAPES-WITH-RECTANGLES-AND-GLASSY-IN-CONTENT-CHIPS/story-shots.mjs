/** 📷️ Screenshots and frame facts of a statically built Storybook (argument 1) in Chromium. Usage: node story-shots.mjs <static dir> <out dir> <story id>… */
import { createServer } from "node:http";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { extname, join, resolve } from "node:path";
import { chromium } from "playwright";
const [dist, out, ...ids] = process.argv.slice(2);
const variant = (process.env.SHOT_VARIANT ?? "").split(",").filter(Boolean);
const width = variant.includes("mobile") ? 375 : 1280, height = variant.includes("mobile") ? 812 : 720;
const suffix = (variant.length ? "." + variant.join("-") : "") + (process.env.SHOT_CLIP ? ".zoom" : "");
const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml", ".png": "image/png", ".woff2": "font/woff2" };
const server = createServer((request, response) => {
  const file = resolve(dist, "." + decodeURIComponent(new URL(request.url ?? "/", "http://x").pathname));
  if (!file.startsWith(resolve(dist)) || !existsSync(file) || statSync(file).isDirectory()) { response.statusCode = 404; return void response.end(); }
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  response.end(readFileSync(file));
});
await new Promise(accept => server.listen(0, "127.0.0.1", accept));
const browser = await chromium.launch({ channel: "chromium" });
for (const id of ids) {
  const clip = process.env.SHOT_CLIP ? Object.fromEntries(process.env.SHOT_CLIP.split(",").map((v, i) => [["x", "y", "width", "height"][i], Number(v)])) : undefined;
  const page = await browser.newPage({ deviceScaleFactor: clip ? 4 : 1, viewport: { width, height }, colorScheme: variant.includes("dark") ? "dark" : "light", forcedColors: variant.includes("forced") ? "active" : "none" });
  if (variant.includes("reduced-transparency")) await (await page.context().newCDPSession(page)).send("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-transparency", value: "reduce" }] });
  const errors = [];
  page.on("pageerror", error => errors.push(error.message.slice(0, 160)));
  await page.goto(`http://127.0.0.1:${server.address().port}/iframe.html?id=${encodeURIComponent(id)}&viewMode=story${variant.includes("de") ? "&globals=locale:de" : ""}`, { waitUntil: "load" });
  await page.waitForSelector("#storybook-root > *", { timeout: 30000 }).catch(error => errors.push(error.message.slice(0, 80)));
  if (variant.includes("rtl")) await page.evaluate(() => { document.documentElement.dir = "rtl"; });
  if (id.includes("select--default")) await page.locator("[role=combobox]").first().click().catch(() => {});
  if (id.includes("contextmenu--grouped") || id.includes("contextmenu--default")) await page.locator("#storybook-root > *").first().click({ button: "right" }).catch(() => {});
  await page.waitForTimeout(1500);
  const facts = await page.evaluate(() => {
    const visible = element => { const box = element.getBoundingClientRect(); return box.width > 0 && box.height > 0; };
    const chips = [...document.querySelectorAll("[data-window-frame-chip]")].filter(visible);
    const outside = chips.filter(chip => { const frame = chip.closest("[data-window-frame]"); if (!frame) return true; const a = chip.getBoundingClientRect(), b = frame.getBoundingClientRect(); return a.left < b.left - 1 || a.right > b.right + 1 || a.top < b.top - 1 || a.bottom > b.bottom + 1; }).length;
    const all = [...document.querySelectorAll("#storybook-root *")].filter(visible);
    const bare = [...document.querySelectorAll("[data-window-frame-chip],[data-glass-chip]")].filter(visible).filter(chip => getComputedStyle(chip).backgroundImage === "none").length;
    return { frames: document.querySelectorAll("[data-window-frame]").length, chips: chips.length, outside, glassChips: document.querySelectorAll("[data-glass-chip]").length, bare, clipPaths: all.filter(e => getComputedStyle(e).clipPath !== "none").length, radius: all.filter(e => parseFloat(getComputedStyle(e).borderTopLeftRadius) > 0).length, oldNames: document.querySelectorAll("[data-window-silhouette],[data-window-silhouette-gap]").length };
  });
  console.log(id, JSON.stringify(facts), errors.join(" | "));
  writeFileSync(join(out, id.replace(/^.*react-/u, "") + suffix + ".png"), await page.screenshot({ animations: "disabled", clip }));
  await page.close();
}
await browser.close();
server.close();
