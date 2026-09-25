import { chromium } from "/home/user/semio/node_modules/playwright/index.mjs";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const base = process.env.BASE ?? "http://127.0.0.1:5173";
const here = fileURLToPath(new URL("./", import.meta.url));
const surfaces = (process.env.SURFACES ?? "design").split(",");
const args = "--use-angle=swiftshader,--enable-unsafe-swiftshader,--ignore-gpu-blocklist,--enable-unsafe-webgpu,--enable-features=Vulkan,--use-vulkan=swiftshader,--use-webgpu-adapter=swiftshader".split(",");
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium", args });
for (const surface of surfaces) {
  const page = await browser.newPage({ viewport: { width: 1400, height: 800 } });
  const logs = [];
  page.on("console", (m) => { if (["error", "warning"].includes(m.type()) || m.text().startsWith("[DEBUG]")) logs.push(`[${m.type()}] ${m.text().slice(0, 400)}`); });
  page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}\n${(e.stack ?? "").slice(0, 1500)}`));
  await page.route(`${base}/mcp-test.html`, (route) => route.fulfill({ contentType: "text/html", body: `<!doctype html><html><head><title>t</title><script type="module">import RefreshRuntime from "/@react-refresh"; RefreshRuntime.injectIntoGlobalHook(window); window.$RefreshReg$ = () => {}; window.$RefreshSig$ = () => (type) => type; window.__vite_plugin_react_preamble_installed__ = true;</script></head><body style="margin:0"><div id="root" style="width:100vw;height:100vh"></div><script type="module">
const { mountMcpKitViewer, mountMcpDesignViewer, mountMcpSceneViewer, mountMcpDiagramViewer } = await import("/boot.tsx");
const mount = { kit: mountMcpKitViewer, design: mountMcpDesignViewer, scene: mountMcpSceneViewer, diagram: mountMcpDiagramViewer }["${surface}"];
const contexts = []; window.__mcpContexts = contexts;
mount(document.getElementById("root"), { connect: async (onPayload) => { onPayload(await (await fetch("/mcp-payload.json")).json()); }, reportContext: (text) => contexts.push(text) });
</script></body></html>` }));
  await page.route(`${base}/mcp-payload.json`, (route) => route.fulfill({ contentType: "application/json", body: readFileSync(`${here}mcp-${surface}-payload.json`) }));
  try {
    await page.goto(`${base}/mcp-test.html`, { waitUntil: "load", timeout: 180000 });
    await page.waitForTimeout(Number(process.env.WAIT ?? 20000));
    await page.screenshot({ path: `${here}shots/mcp-${surface}.png` });
    console.log("[DEBUG] shot", surface, await page.evaluate(() => [document.title, document.body.innerText.slice(0, 200), JSON.stringify(window.__mcpContexts)]));
    if (process.env.CLICK) {
      const [x, y] = process.env.CLICK.split(":").map(Number);
      await page.mouse.click(x, y);
      await page.waitForTimeout(2000);
      await page.screenshot({ path: `${here}shots/mcp-${surface}-click.png` });
      console.log("[DEBUG] contexts", await page.evaluate(() => JSON.stringify(window.__mcpContexts)));
    }
  } catch (e) {
    console.log("[DEBUG] failed", e.message.slice(0, 400));
  }
  console.log([...new Set(logs)].slice(0, 30).join("\n"));
  await page.close();
}
await browser.close();
