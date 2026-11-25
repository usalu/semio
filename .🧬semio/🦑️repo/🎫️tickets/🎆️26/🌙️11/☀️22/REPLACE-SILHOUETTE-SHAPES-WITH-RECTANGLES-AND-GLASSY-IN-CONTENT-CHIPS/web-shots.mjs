/** 📸️ Screenshots of the statically built course site (generated/web-dist) at 1440 and 375 into evidence/web. A local static server answers the build; the proctor is unreachable on purpose (the client works on the device). Usage (repo root): node web-shots.mjs [chooser] */
import { createServer } from "node:http";
import { existsSync, mkdirSync, writeFileSync, readFileSync, statSync } from "node:fs";
import { extname, join, resolve } from "node:path";
import { chromium } from "playwright";
const dist = join(import.meta.dirname, "🗑️generated/web-dist");
const evidence = join(import.meta.dirname, "evidence/web");
mkdirSync(evidence, { recursive: true });
const chooser = process.argv[2] === "chooser";
const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml", ".png": "image/png", ".webp": "image/webp", ".jpg": "image/jpeg", ".woff2": "font/woff2", ".pdf": "application/pdf" };
const server = createServer((request, response) => {
  const path = decodeURIComponent(new URL(request.url ?? "/", "http://x").pathname);
  let file = resolve(dist, "." + path);
  if (!file.startsWith(dist) || !existsSync(file) || statSync(file).isDirectory()) file = join(dist, "index.html");
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  let body = readFileSync(file);
  if (chooser && file.endsWith(".js")) body = Buffer.from(body.toString("utf8").replace(/language:\s*[A-Za-z_$][\w$]*\.language\b/u, "language:void 0"));
  response.end(body);
});
await new Promise((accept) => server.listen(0, "127.0.0.1", accept));
const base = `http://127.0.0.1:${server.address().port}/`;
const browser = await chromium.launch({ channel: "chromium" });
const shots = [];
for (const [name, width, height] of [["1440", 1440, 900], ["375", 375, 812]]) {
  const context = await browser.newContext({ viewport: { width, height }, locale: chooser ? "fr-FR" : "en-US", deviceScaleFactor: 1, reducedMotion: "reduce" });
  const page = await context.newPage();
  await page.route("**/127.0.0.1:59999/**", () => undefined);
  page.on("pageerror", (error) => console.log("[DEBUG] pageerror", error.message));
  await page.goto(base, { waitUntil: "load" });
  await page.waitForTimeout(chooser ? 2500 : 5000);
  const take = async (label) => {
    const path = join(evidence, `${chooser ? "chooser" : label}-${name}.png`);
    let previous = null;
    for (let attempt = 0; attempt < 12; attempt++) {
      const bytes = await page.screenshot({ animations: "disabled", timeout: 90000 });
      if (previous && Buffer.compare(previous, bytes) === 0) break;
      previous = bytes;
      await page.waitForTimeout(700);
    }
    writeFileSync(path, previous);
    shots.push(path);
  };
  if (!chooser) {
    await page.keyboard.press("Escape");
    await page.waitForTimeout(1500);
  }
  await take("home");
  if (!chooser) {
    console.log(name, "cards", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-card]")].map((card) => card.getAttribute("data-card")))));
    for (const step of (process.env.STEPS ?? "").split(",").filter(Boolean)) {
      const [label, kind, target] = step.split("|");
      if (kind === "goto") { await page.goto(base + target, { waitUntil: "load" }); await page.waitForTimeout(3500); await page.keyboard.press("Escape"); }
      if (kind === "mouse") { const [x, y] = target.split(":").map(Number); await page.mouse.click(x * (width >= 1000 ? 1 : 0.26), y); }
      if (kind === "click") await page.locator(target).first().click({ timeout: 8000 }).catch((error) => console.log("[DEBUG] click failed", target, error.message.slice(0, 80)));
      if (kind === "scroll") await page.evaluate((selector) => { const el = document.querySelector(selector); if (el) el.scrollTop = el.scrollHeight / 2; }, target);
      await page.waitForTimeout(1500);
      await take(label);
    }
  }
  await context.close();
}
await browser.close();
server.close();
console.log(shots.join("\n"));
