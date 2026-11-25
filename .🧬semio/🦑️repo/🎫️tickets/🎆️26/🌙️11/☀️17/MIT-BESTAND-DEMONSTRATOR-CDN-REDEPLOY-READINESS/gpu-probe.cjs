/** 🎮️ Reports the WebGPU and WebGL adapters headless Chromium gets, and the console of the map pane with every worker. Usage: node gpu-probe.cjs <origin> [headed] */
const { createRequire } = require("node:module");
const { resolve, join } = require("node:path");
const root = resolve(__dirname, "../../../../../../..");
const appRequire = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = appRequire("@playwright/test");
(async () => {
  const browser = await chromium.launch({ headless: process.argv[3] !== "headed", args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"] });
  const page = await browser.newPage({ locale: "en-US" });
  page.on("console", message => { const text = message.text(); if (/webgpu|gpu|canvas|adapter|device|map|worker|surface/i.test(text)) console.log(message.type(), text.slice(0, 400)); });
  page.on("pageerror", error => console.log("pageerror", String(error).slice(0, 400)));
  page.on("requestfailed", request => console.log("failed", decodeURI(request.url()).slice(0, 200), request.failure()?.errorText));
  page.on("response", response => { if (response.status() >= 400) console.log(response.status(), decodeURI(response.url()).slice(0, 200)); });
  const debug = await page.context().newCDPSession(page); await debug.send("Debugger.enable"); await debug.send("Debugger.setPauseOnExceptions", { state: "all" });
  debug.on("Debugger.paused", event => { console.log("THROW", (event.data?.description ?? JSON.stringify(event.data?.value ?? "")).slice(0, 500), event.callFrames.slice(0, 4).map(frame => `${frame.functionName}@${decodeURI(frame.url).split("/").pop().slice(0, 40)}:${frame.location.lineNumber}:${frame.location.columnNumber}`).join(" < ")); void debug.send("Debugger.resume").catch(() => {}); });
  await page.addInitScript(() => { window.__lost = []; const timer = window.setTimeout; window.setTimeout = (run, delay, ...rest) => timer(() => { if (delay === 15000 || delay === 4000) window.__lost.push(`timer ${delay}`); run(...rest); }, delay); addEventListener("contextlost", event => window.__lost.push("contextlost"), true); const request = GPUAdapter.prototype.requestDevice; GPUAdapter.prototype.requestDevice = async function (descriptor) { const device = await request.call(this, descriptor); window.__lost.push("device"); void device.lost.then(info => window.__lost.push(`device lost ${info.reason} ${info.message}`)); device.addEventListener("uncapturederror", event => window.__lost.push(`gpu ${event.error.message.slice(0, 300)}`)); return device; }; });
  await page.goto(`${process.argv[2]}/#verfolgen`);
  console.log(await page.evaluate(async () => { const adapter = await navigator.gpu?.requestAdapter(); const gl = document.createElement("canvas").getContext("webgl2"); return JSON.stringify({ secure: isSecureContext, gpu: !!navigator.gpu, adapter: adapter ? [adapter.info?.vendor, adapter.info?.architecture] : null, gl: gl?.getParameter(gl.VERSION) ?? null }); }));
  for (let second = 0; second < 12; second += 1) { await page.waitForTimeout(1000); console.log(second, await page.evaluate(() => JSON.stringify({ renderers: [...document.querySelectorAll("canvas")].map(canvas => [canvas.dataset.mapRenderer ?? null, canvas.width, canvas.height]), unavailable: document.body.innerText.includes("Canvas unavailable"), lost: window.__lost ?? [] }))); }
  console.log((await page.evaluate(() => document.body.innerText)).includes("Canvas unavailable") ? "CANVAS UNAVAILABLE" : "canvas ok");
  await browser.close();
})();
