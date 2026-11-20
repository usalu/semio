import assert from "node:assert/strict";
import { createServer } from "node:http";
import { chromium } from "playwright";

const backend = process.argv[2];
const args = ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"];
if (backend) args.push(`--use-angle=${backend}`);
const server = createServer((_, response) => {
  response.writeHead(200, { "Content-Type": "text/html" });
  response.end("<!doctype html><title>WebGPU browser preflight</title>");
});
await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
let browser;
try {
  browser = await chromium.launch({ channel: process.argv[3] === "chromium" ? "chromium" : undefined, headless: true, args, timeout: 15000 });
  console.log("[DEBUG] chromium", browser.version(), args);
  const page = await browser.newPage();
  const response = await page.goto(`http://127.0.0.1:${server.address().port}`, { timeout: 10000 });
  assert.ok(response?.ok());
  const status = await page.evaluate(async () => {
    const within = fn => Promise.race([fn(), new Promise(resolve => setTimeout(() => resolve("timeout"), 8000))]);
    const adapter = await within(() => navigator.gpu?.requestAdapter());
    const device = adapter && adapter !== "timeout" ? await within(() => adapter.requestDevice()) : undefined;
    if (device && device !== "timeout") device.destroy();
    return { secure: isSecureContext, webgpu: !!navigator.gpu, adapter: adapter === "timeout" ? adapter : !!adapter, device: device === "timeout" ? device : !!device };
  });
  console.log("[DEBUG]", status);
  assert.deepEqual(status, { secure: true, webgpu: true, adapter: true, device: true });
} finally {
  server.closeAllConnections();
  server.close();
  await browser?.close();
}
