const { createRequire } = require("node:module");
const { resolve } = require("node:path");
const { createWriteStream } = require("node:fs");
const { once } = require("node:events");
const [url, seconds = "240", browsers, diagnostics] = process.argv.slice(2);
process.env.PLAYWRIGHT_BROWSERS_PATH = browsers;
const { chromium } = createRequire(resolve("package.json"))("playwright");
let moreWork = 0;
const stamp = () => new Date().toISOString().slice(11, 23);
async function sampleWorker(session, path, ms) {
  await session.send("Tracing.start", { categories: "disabled-by-default-v8.cpu_profiler", transferMode: "ReturnAsStream" });
  await new Promise((done) => setTimeout(done, ms));
  const completed = new Promise((done) => session.once("Tracing.tracingComplete", done));
  await session.send("Tracing.end");
  const { stream } = await completed;
  const output = createWriteStream(path);
  for (;;) {
    const { data, base64Encoded, eof } = await session.send("IO.read", { handle: stream });
    if (!output.write(base64Encoded ? Buffer.from(data, "base64") : data)) await once(output, "drain");
    if (eof) break;
  }
  output.end();
  await once(output, "finish");
  await session.send("IO.close", { handle: stream });
}
(async () => {
  const browser = await chromium.launch({});
  const page = await browser.newPage({ locale: "en-US" });
  page.on("console", (message) => {
    const line = message.text();
    if (/GL Driver|turn phase|guest linear|typed-operation slots/.test(line)) return;
    if (diagnostics && /reactor more-work/.test(line) && (++moreWork % 400) !== 1) return;
    if (diagnostics || message.type() === "error" || /flowEval|preview|toolRun|refus|fault|stall/i.test(line)) process.stdout.write(`[DEBUG] ${stamp()} console ${message.type()} ${diagnostics ? line.slice(0, 6000) : line.slice(0, 400)}\n`);
  });
  if (diagnostics) await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  await page.goto(url, { waitUntil: "domcontentloaded" });
  const profilePath = process.argv[6];
  if (profilePath) { const cdp = await browser.newBrowserCDPSession(); setTimeout(() => sampleWorker(cdp, profilePath, 6000).then(() => process.stdout.write("[DEBUG] profile stored\n"), (e) => process.stdout.write("[DEBUG] profile error " + e + "\n")), Number(process.argv[7] ?? 60) * 1000); }
  const end = Date.now() + Number(seconds) * 1000;
  while (Date.now() < end) {
    const state = await page.evaluate(() => {
      const host = document.querySelector('[data-shell-id="generator"] .semio-world-3d-host');
      const len = (text) => { try { return JSON.parse(text ?? "[]").length; } catch { return -1; } };
      const status = [...document.querySelectorAll('[data-shell-id="generator"] [role="status"]')].map((el) => el.textContent.trim()).filter(Boolean).join(" | ");
      const runs = document.querySelector('[data-shell-id="generator"] [aria-label="Preview evaluation"]')?.textContent.trim().slice(0, 200);
      return { host: !!host, meshes: host ? len(host.getAttribute("data-meshes-json")) : null, instances: host ? len(host.getAttribute("data-instances-json")) : null, status: status.slice(0, 300), runs };
    }).catch((error) => ({ error: String(error).slice(0, 100) }));
    process.stdout.write(`[DEBUG] ${stamp()} ${JSON.stringify(state)}\n`);
    if (state.meshes > 0 || state.instances > 0) break;
    await new Promise((done) => setTimeout(done, 10000));
  }
  await browser.close();
})().catch((error) => { console.error(error); process.exit(1); });
