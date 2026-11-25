/** 🔎️ Tells whether Playwright's Chromium starts on this host and which WebGL renderer it has. Usage: node browser-probe.mjs */
import { chromium } from "playwright";

const browser = await chromium.launch({ channel: "chromium", args: ["--disable-crash-reporter", "--disable-crashpad", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage();
  await page.goto("about:blank");
  console.log("[DEBUG]", browser.version(), JSON.stringify(await page.evaluate(() => {
    const gl = document.createElement("canvas").getContext("webgl2");
    const info = gl?.getExtension("WEBGL_debug_renderer_info");
    return { webgl2: gl !== null, renderer: gl && info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : null, agent: navigator.userAgent, wasm: typeof WebAssembly, webm: MediaRecorder.isTypeSupported("video/webm") };
  })));
} finally {
  await browser.close();
}
