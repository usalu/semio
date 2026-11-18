/** 🚦️ Serves the staffage release folder like a static CDN (no SPA fallback, `404.html` for misses) and opens the routes a reader uses in headless Chrome on desktop and phone, recording page errors, console errors, failed requests and CSP violations. Usage: `node smoke.mjs <dist/release>`. */
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { extname, join, normalize } from "node:path";
import playwright from "/Users/ueli/Documents/semio/node_modules/playwright/index.js";

const root = process.argv[2];
const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".svg": "image/svg+xml", ".json": "application/json", ".webmanifest": "application/manifest+json", ".png": "image/png", ".woff2": "font/woff2", ".wasm": "application/wasm", ".txt": "text/plain", ".glb": "model/gltf-binary" };
const server = createServer(async (request, response) => {
  const path = normalize(decodeURIComponent(new URL(request.url, "http://x").pathname)).replace(/^\/+/, "");
  const file = join(root, path === "" ? "index.html" : path);
  const found = await stat(file).then((s) => s.isFile(), () => false);
  response.writeHead(found ? 200 : 404, { "content-type": types[extname(found ? file : ".html")] ?? "application/octet-stream" });
  response.end(await readFile(found ? file : join(root, "404.html")));
});
await new Promise((done) => server.listen(0, "127.0.0.1", done));
const base = `http://127.0.0.1:${server.address().port}/`;
const routes = ["#/", "#/all", "#/humans", "#/furniture", "#/humans/walking-commuter", "#/humans/walking-commuter?view=thermal", "#/humans/walking-commuter?view=forces", "#/compare", "#/playground", "#/collections"];
const browser = await playwright.chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", headless: true });
let failures = 0;
for (const [device, viewport] of [["desktop", { width: 1440, height: 900 }], ["phone", { width: 390, height: 844 }]]) {
  const page = await (await browser.newContext({ viewport })).newPage();
  const problems = [];
  page.on("pageerror", (error) => problems.push(`pageerror ${error.message.split("\n")[0]}`));
  page.on("console", (message) => { if (message.type() === "error" || /Content Security Policy/u.test(message.text())) problems.push(`console ${message.text().slice(0, 300)}`); });
  page.on("requestfailed", (request) => problems.push(`requestfailed ${request.url()}`));
  page.on("response", (response) => { if (response.status() >= 400) problems.push(`http ${response.status()} ${response.url()}`); });
  for (const route of routes) {
    problems.length = 0;
    await page.goto(base + route, { waitUntil: "networkidle", timeout: 60000 });
    const english = page.getByRole("button", { name: "English" });
    if (await english.count()) await english.first().click();
    await page.waitForTimeout(2000);
    const svgs = await page.locator("svg").count();
    const text = (await page.locator("body").innerText()).replace(/\s+/gu, " ").slice(0, 120);
    const ok = problems.length === 0 && svgs > 0;
    if (!ok) failures++;
    console.log(`${ok ? "PASS" : "FAIL"} ${device} ${route} svg=${svgs} problems=${problems.length} text=${text}`);
    for (const problem of problems) console.log(`  ${problem}`);
  }
}
const missing = await fetch(base + "no-such-module.json");
console.log(`${missing.status === 404 ? "PASS" : "FAIL"} miss answers ${missing.status}`);
if (missing.status !== 404) failures++;
await browser.close();
server.close();
console.log(`failures ${failures}`);
process.exit(failures ? 1 : 0);
