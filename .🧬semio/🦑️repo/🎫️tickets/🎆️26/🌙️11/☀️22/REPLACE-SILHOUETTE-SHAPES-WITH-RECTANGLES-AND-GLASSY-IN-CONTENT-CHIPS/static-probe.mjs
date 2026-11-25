/** 🔬️ Evaluates a JS expression in a story of a statically built Storybook. Usage: node static-probe.mjs <static dir> <story id> '<expression>' [click selector] */
import { createServer } from "node:http";
import { existsSync, readFileSync, statSync } from "node:fs";
import { extname, resolve } from "node:path";
import { chromium } from "playwright";
const [dist, id, expr, click] = process.argv.slice(2);
const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml", ".woff2": "font/woff2" };
const server = createServer((request, response) => {
  const file = resolve(dist, "." + decodeURIComponent(new URL(request.url ?? "/", "http://x").pathname));
  if (!file.startsWith(resolve(dist)) || !existsSync(file) || statSync(file).isDirectory()) { response.statusCode = 404; return void response.end(); }
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  response.end(readFileSync(file));
});
await new Promise(accept => server.listen(0, "127.0.0.1", accept));
const browser = await chromium.launch({ channel: "chromium" });
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto(`http://127.0.0.1:${server.address().port}/iframe.html?id=${encodeURIComponent(id)}&viewMode=story`, { waitUntil: "load" });
await page.waitForSelector("#storybook-root > *", { timeout: 30000 });
if (click) await page.locator(click).first().click({ button: process.env.RIGHT ? "right" : "left" });
await page.waitForTimeout(1200);
console.log(JSON.stringify(await page.evaluate(expr), null, 1));
await browser.close();
server.close();
