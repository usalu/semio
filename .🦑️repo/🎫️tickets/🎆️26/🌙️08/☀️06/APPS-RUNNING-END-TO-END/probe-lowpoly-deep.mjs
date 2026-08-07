import { chromium } from "playwright";
import { join } from "path";
const ticketDir = import.meta.dirname;
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer", "--use-angle=swiftshader"],
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleMsgs = [];
page.on("console", (msg) => consoleMsgs.push({ type: msg.type(), text: msg.text().slice(0, 2000) }));
page.on("pageerror", (err) => consoleMsgs.push({ type: "pageerror", text: String(err.stack || err).slice(0, 2000) }));
await page.goto("http://127.0.0.1:6078/?nocache=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(10000);
const diag = await page.evaluate(async () => {
  const status = [...document.querySelectorAll('[role="status"]')].map((el) => el.textContent?.trim());
  const canvases = [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height, cls: c.className }));
  // try dynamic import of World3dHost via same path vite serves? skip
  const gpu = "gpu" in navigator ? await (async () => {
    try {
      const a = await navigator.gpu.requestAdapter();
      return { hasGpu: true, adapter: !!a };
    } catch (e) {
      return { hasGpu: true, error: String(e) };
    }
  })() : { hasGpu: false };
  return {
    status,
    canvases,
    gpu,
    bodySample: document.body.innerText.split("\n").map(s=>s.trim()).filter(Boolean).slice(0,60),
    reactRoot: !!document.getElementById("root") || !!document.querySelector("#app") || !!document.querySelector("[data-reactroot]"),
    rootHtml: (document.getElementById("root") || document.querySelector("#app") || document.body).innerHTML.slice(0, 500),
  };
});
const out = { diag, consoleMsgs: consoleMsgs.slice(0, 80) };
await Bun.write(join(ticketDir, "🧪lowpoly-deep.json"), JSON.stringify(out, null, 2));
await page.screenshot({ path: join(ticketDir, "🧪lowpoly-deep.png") });
console.log(JSON.stringify(out, null, 2).slice(0, 6000));
await browser.close();
