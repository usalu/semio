import { chromium } from "/home/user/semio/node_modules/playwright/index.mjs";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createServer } from "node:http";
const here = fileURLToPath(new URL("./", import.meta.url));
const html = readFileSync("/home/user/semio/semio/client/bin/engine/dist/mcp-app.html", "utf8");
const surfaces = (process.env.SURFACES ?? "design").split(",");
const args = "--use-angle=swiftshader,--enable-unsafe-swiftshader,--ignore-gpu-blocklist,--enable-unsafe-webgpu,--enable-features=Vulkan,--use-vulkan=swiftshader,--use-webgpu-adapter=swiftshader".split(",");
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium", args });
for (const surface of surfaces) {
  const page = await browser.newPage({ viewport: { width: 1400, height: 800 } });
  const logs = [];
  page.on("console", (m) => { if (process.env.ALL || ["error", "warning"].includes(m.type()) || m.text().startsWith("[DEBUG]")) logs.push(`[${m.type()}] ${m.text().slice(0, 300)}`); });
  page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}`));
  const payload = readFileSync(`${here}mcp-${surface}-payload.json`, "utf8");
  const HOST_HTML = `<!doctype html><html><body style="margin:0"><iframe id="app" src="http://127.0.0.1:8766/mcp-app.html" style="border:0;width:100vw;height:100vh"></iframe><script>
window.__contexts = [];
const iframe = document.getElementById("app");
const reply = (id, result) => iframe.contentWindow.postMessage({ jsonrpc: "2.0", id, result }, "*");
window.addEventListener("message", async (ev) => {
  const msg = ev.data;
  if (!msg || msg.jsonrpc !== "2.0" || ev.source !== iframe.contentWindow) return;
  if (msg.method === "ui/initialize") reply(msg.id, { protocolVersion: msg.params?.protocolVersion ?? "2026-01-26", hostInfo: { name: "harness", version: "1.0.0" }, hostCapabilities: {}, hostContext: {} });
  else if (msg.method === "ui/notifications/initialized") {
    const payload = await (await fetch("http://127.0.0.1:8765/payload.json")).json();
    iframe.contentWindow.postMessage({ jsonrpc: "2.0", method: "ui/notifications/tool-result", params: { content: [{ type: "text", text: JSON.stringify({ mode: payload.mode, surface: payload.surface }) }], structuredContent: payload } }, "*");
  } else if (msg.method === "ui/update-model-context") { window.__contexts.push(msg.params); reply(msg.id, {}); }
  else if (msg.id !== undefined && msg.method) reply(msg.id, {});
});
</script></body></html>`;
  const hostServer = createServer((req, res) => {
    if (req.url === "/payload.json") { res.writeHead(200, { "content-type": "application/json" }); res.end(payload); return; }
    res.writeHead(200, { "content-type": "text/html" }); res.end(HOST_HTML);
  }).listen(8765, "127.0.0.1");
  const appServer = createServer((req, res) => {
    if (req.url === "/mcp-app.html") { res.writeHead(200, { "content-type": "text/html" }); res.end(html.replace('data-mcp-viewer="design"', `data-mcp-viewer="${surface}"`)); return; }
    res.writeHead(404); res.end();
  }).listen(8766, "127.0.0.1");
  try {
    await page.goto("http://127.0.0.1:8765/", { waitUntil: "load", timeout: 300000 });
    await page.waitForTimeout(Number(process.env.WAIT ?? 30000));
    await page.screenshot({ path: `${here}shots/mcp-built-${surface}.png` });
    const frame = page.frame({ url: /8766/ });
    console.log("[DEBUG] shot", surface, JSON.stringify(await frame?.evaluate(() => [document.title, document.body.innerText.slice(0, 120)])), JSON.stringify(await page.evaluate(() => window.__contexts)).slice(0, 300));
  } catch (e) {
    console.log("[DEBUG] failed", e.message.slice(0, 400));
  }
  console.log([...new Set(logs)].slice(0, 20).join("\n"));
  await page.close();
  hostServer.close();
  appServer.close();
}
await browser.close();
