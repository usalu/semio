/** 🕹️ Keeps one headless FEM editor page open and evaluates posted async JS bodies against it (`page`, `shot(name)`, `log`). Usage: node page-driver.mjs <url> <port> */
import { createServer } from "node:http";
import { mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/", port = Number(process.argv[3] ?? 6196);
const out = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(out, { recursive: true });
const browser = await chromium.launch();
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
const log = [];
page.on("pageerror", error => log.push(`pageerror ${error.message.slice(0, 400)}`));
page.on("console", m => { const t = m.text(); if (m.type() === "error" && !/follow unit/.test(t)) log.push(`${m.type()} ${t.slice(0, 400)}`); });
if (process.env.INSTRUMENT) await page.context().route(u => decodeURIComponent(u.pathname).endsWith("/🌉️bridge.js"), async route => {
  const response = await route.fetch();
  const source = await response.text();
  const timed = (seam, kind) => source.includes(seam) ? [seam, `{const __t=performance.now(); try { ${seam} } finally { (globalThis.__bridgeCalls ??= []).push([${JSON.stringify(kind)}, performance.now()-__t, performance.now(), ${kind === "poll" ? "[String(slice?.fuel), String(result?.retainedReceipt?.workUnitsSpent), result?.uiPatches?.length, result?.effects?.length, events?.map?.(e=>e.kind).join(\"+\"), String(result?.nextWake?.tag ?? result?.nextWake ?? \"\"), JSON.stringify(result?.retainedReceipt?.spent ?? {}, (_, v) => typeof v === \"bigint\" ? String(v) : v)]" : "null"}]); }}`] : null;
  let body = source;
  for (const [seam, kind] of [["result = await reactor.poll(slice);", "poll"], ["result = await this.#guest.reactor.poll(originalContextBudgetWit(turn));", "guestPoll"]]) { const t = timed(seam, kind); if (t) body = body.replace(...t); }
  body = body.replace("stepJob: async (job, budget, control = {}) => fundedJob(budget, async (funds) => {", "stepJob: async (job, budget, control = {}) => fundedJob(budget, async (funds) => { const __t=performance.now(); try {").replace("if (step.status !== \"running\") succession.settled(job); return step; }),", "if (step.status !== \"running\") succession.settled(job); return step; } finally { (globalThis.__bridgeCalls ??= []).push([\"stepJob\", performance.now()-__t, performance.now()]); } }),");
  log.push(`instrumented ${body !== source}`);
  await route.fulfill({ response, body });
});
if (process.env.DIAG) await page.context().route(u => /preview2-shim\/(environment|cli)\.js$/.test(decodeURIComponent(u.pathname)), async route => {
  const response = await route.fetch();
  const body = (await response.text()).replace("let _env = [];", 'let _env = [["SEMIO_RUNTIME_DIAGNOSTICS","1"]];').replace("Object.entries(config.environment ?? {})", 'Object.entries({ SEMIO_RUNTIME_DIAGNOSTICS: "1", ...(config.environment ?? {}) })');
  log.push(`diag ${route.request().url().slice(-20)}`);
  await route.fulfill({ response, body });
});
page.on("console", m => { const t = m.text(); if (/more-work/.test(t)) ((globalThis.__diag ??= []).push([Date.now(), t.slice(0, 600)]), globalThis.__diag.length > 50000 && globalThis.__diag.splice(0, 10000)); });
await page.goto(url,{ waitUntil: "domcontentloaded", timeout: 120000 });
const shot = async name => { const path = `${out}${name}.png`; await page.screenshot({ path }); return path; };
const AsyncFunction = (async () => {}).constructor;
createServer(async (request, response) => {
  let body = "";
  for await (const chunk of request) body += chunk;
  try {
    const value = await new AsyncFunction("page", "shot", "log", "diag", body)(page, shot, log, () => globalThis.__diag ?? []);
    const drained = log.splice(0);
    response.end(JSON.stringify({ value, log: drained }, null, 1));
  } catch (error) {
    response.end(JSON.stringify({ error: String(error?.stack ?? error).slice(0, 2000), log: log.splice(0) }, null, 1));
  }
}).listen(port, "127.0.0.1", () => console.log(`driver ${url} on ${port}`));
