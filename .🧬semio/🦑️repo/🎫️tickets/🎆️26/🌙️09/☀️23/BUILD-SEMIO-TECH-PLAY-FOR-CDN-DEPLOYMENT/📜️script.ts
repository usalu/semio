/// <reference types="@webgpu/types" />
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import type { chromium as chromiumType } from "playwright";
import { repoPlaywrightBrowsersPath } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

/** 🔭 Measures actual browser prerequisites using the production acceptance launch settings. */
async function browserCapabilities(workspace: string): Promise<void> {
  const play = join(workspace, "🏢️semio-tech", "🎡️play");
  const require = createRequire(join(play, "package.json"));
  const packagePath = require.resolve("playwright/package.json");
  const requireCore = createRequire(packagePath);
  const coreRoot = dirname(requireCore.resolve("playwright-core/package.json"));
  const browserManifest = JSON.parse(readFileSync(join(coreRoot, "browsers.json"), "utf8"));
  process.env.PLAYWRIGHT_BROWSERS_PATH = repoPlaywrightBrowsersPath(workspace);
  const { chromium } = require("playwright") as { chromium: typeof chromiumType };
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: () => new Response("<!doctype html><html lang=\"en\"><title>Play Browser Probe</title></html>", { headers: { "Content-Type": "text/html" } }) });
  let browser: Awaited<ReturnType<typeof chromium.launch>> | undefined;
  let timeout: ReturnType<typeof setTimeout> | undefined;
  try {
    process.env.PLAYWRIGHT_BASE_URL = server.url.href;
    const { default: config } = await import(pathToFileURL(join(play, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts")).href);
    browser = await chromium.launch({ ...config.projects[0].use.launchOptions, timeout: 15_000 });
    const page = await browser.newPage();
    await page.goto(server.url.href, { timeout: 15_000 });
    const revisions = browserManifest.browsers.filter((row: { name: string }) => ["chromium", "chromium-headless-shell"].includes(row.name));
    console.log(`[DEBUG] Play browser ready ${JSON.stringify({ version: browser.version(), revisions, cache: process.env.PLAYWRIGHT_BROWSERS_PATH, launch: config.projects[0].use.launchOptions })}`);
    const measure = page.evaluate(async () => {
      const wasm = WebAssembly as unknown as Record<string, unknown>;
      const gl = document.createElement("canvas").getContext("webgl2");
      const glPixel = new Uint8Array(4);
      gl?.clearColor(0, 1, 0, 1);
      if (gl) {
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, glPixel);
      }
      const debug = gl?.getExtension("WEBGL_debug_renderer_info");
      const webglRenderer = debug ? gl?.getParameter(debug.UNMASKED_RENDERER_WEBGL) : null;
      const gpu = navigator.gpu;
      const draw = async () => {
        const adapter = await gpu?.requestAdapter();
        if (!adapter) return null;
        const device = await adapter.requestDevice();
        try {
          const canvas = document.createElement("canvas");
          canvas.width = canvas.height = 1;
          document.body.append(canvas);
          const context = canvas.getContext("webgpu");
          if (!context) return null;
          context.configure({ device, format: gpu.getPreferredCanvasFormat(), usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
          const texture = context.getCurrentTexture();
          const buffer = device.createBuffer({ size: 256, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
          const commands = device.createCommandEncoder();
          commands.beginRenderPass({
            colorAttachments: [{ view: texture.createView(), clearValue: { r: 0, g: 1, b: 0, a: 1 }, loadOp: "clear", storeOp: "store" }],
          }).end();
          commands.copyTextureToBuffer({ texture }, { buffer, bytesPerRow: 256 }, [1, 1]);
          device.queue.submit([commands.finish()]);
          await buffer.mapAsync(GPUMapMode.READ);
          const pixel = Array.from(new Uint8Array(buffer.getMappedRange(), 0, 4));
          buffer.destroy();
          context.unconfigure();
          return { vendor: adapter.info.vendor, architecture: adapter.info.architecture, fallback: adapter.info.isFallbackAdapter, pixel };
        } finally {
          device.destroy();
        }
      };
      const webgpu = await draw();
      return { jspi: typeof wasm.Suspending === "function" && typeof wasm.promising === "function", webgl: { renderer: webglRenderer, pixel: Array.from(glPixel) }, webgpu };
    });
    const observed = await Promise.race([measure, new Promise<never>((_, reject) => {
      timeout = setTimeout(() => reject(new Error("Browser GPU draw probe exceeded 15 seconds")), 15_000);
    })]);
    console.log(`[DEBUG] Play browser capabilities ${JSON.stringify(observed)}`);
    const green = [0, 255, 0, 255].join();
    if (!observed.jspi || observed.webgl.pixel.join() !== green || observed.webgpu?.pixel.join() !== green) throw new Error("Play browser prerequisites unavailable");
  } finally { clearTimeout(timeout); await browser?.close(); await server.stop(true); }
}

const [command, workspace] = process.argv.slice(2);
if (command !== "browser-capabilities" || !workspace) throw new Error("browser-capabilities <workspace>");
await browserCapabilities(workspace);
