/** 🪫️ The page driver of `page-driver.mjs` with the served bridge's stock succession traced: every low-water decision, job registration and refusal is logged with a `[DEBUG] stock` prefix. Usage: node stock-driver.mjs <url> <port> */
import { createServer } from "node:http";
import { mkdirSync, appendFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/", port = Number(process.argv[3] ?? 6296);
const out = fileURLToPath(new URL("./🗑️generated/stock/", import.meta.url));
mkdirSync(out, { recursive: true });
const trace = `${out}trace.log`;
const browser = await chromium.launch();
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
const log = [];
page.on("pageerror", error => log.push(`pageerror ${error.message.slice(0, 400)}`));
page.on("console", m => { const t = m.text(); if (t.startsWith("[DEBUG] stock")) appendFileSync(trace, `${Date.now()} ${t}\n`); else if (m.type() === "error" && !/follow unit/.test(t)) log.push(`${m.type()} ${t.slice(0, 400)}`); });
await page.context().route(u => decodeURIComponent(u.pathname).endsWith("/🌉️bridge.js"), async route => {
  const response = await route.fetch();
  const source = await response.text();
  const seams = [
    ["  started(job) {\n    this.#jobs.add(job);", "  started(job) {\n    console.log(`[DEBUG] stock started ${job} jobs=${[...this.#jobs]}`);\n    this.#jobs.add(job);"],
    ["  settled(job) {\n    this.#jobs.delete(job);", "  settled(job) {\n    console.log(`[DEBUG] stock settled ${job} jobs=${[...this.#jobs]}`);\n    this.#jobs.delete(job);"],
    ["  #defer(refusal) {\n", "  #defer(refusal) {\n    console.log(`[DEBUG] stock defer ${String(refusal?.message ?? refusal)} deferral=${this.#deferral}`);\n"],
    ["  async advance() {\n    const source = this.#source;\n", "  async advance() {\n    const source = this.#source;\n    if (source.lowWater && (globalThis.__stockAt ?? 0) + 2000 < Date.now()) { globalThis.__stockAt = Date.now(); console.log(`[DEBUG] stock low copy=${source.retainedRemaining.maximumCopyBytes} tenure=${source.tenure} outstanding=${source.outstanding} held=${this.#held.size} jobs=${[...this.#jobs]} wait=${this.#wait} deferral=${this.#deferral}`); }\n"],
    ["this.#successor = void 0;", "console.log(`[DEBUG] stock renewed tenure=${this.#source.tenure}`);this.#successor = void 0;"],
  ];
  let body = source;
  for (const [seam, traced] of seams) { if (!body.includes(seam)) log.push(`seam missing ${seam.slice(0, 40)}`); body = body.replace(seam, traced); }
  log.push(`instrumented ${body !== source}`);
  await route.fulfill({ response, body });
});
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
const shot = async name => { const path = `${out}${name}.png`; await page.screenshot({ path }); return path; };
const AsyncFunction = (async () => {}).constructor;
createServer(async (request, response) => {
  let body = "";
  for await (const chunk of request) body += chunk;
  try {
    const value = await new AsyncFunction("page", "shot", "log", body)(page, shot, log);
    response.end(JSON.stringify({ value, log: log.splice(0) }, null, 1));
  } catch (error) {
    response.end(JSON.stringify({ error: String(error?.stack ?? error).slice(0, 2000), log: log.splice(0) }, null, 1));
  }
}).listen(port, "127.0.0.1", () => console.log(`driver ${url} on ${port}`));
