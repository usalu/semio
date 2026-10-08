import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { chromium } from "playwright";

/** 🔭 Measures actual browser prerequisites using the production acceptance launch settings. */
async function browserCapabilities(workspace: string): Promise<void> {
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: () => new Response("<!doctype html><html lang=\"en\"><title>Play Browser Probe</title></html>", { headers: { "Content-Type": "text/html" } }) });
  let browser: Awaited<ReturnType<typeof chromium.launch>> | undefined;
  try {
    process.env.PLAYWRIGHT_BASE_URL = server.url.href;
    const { default: config } = await import(pathToFileURL(join(workspace, "🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts")).href);
    browser = await chromium.launch(config.projects[0].use.launchOptions);
    const page = await browser.newPage();
    await page.goto(server.url.href);
    const observed = await page.evaluate(async () => {
      const wasm = WebAssembly as unknown as Record<string, unknown>;
      const gpu = (navigator as unknown as { gpu?: { requestAdapter(): Promise<unknown> } }).gpu;
      return { jspi: typeof wasm.Suspending === "function" && typeof wasm.promising === "function", gpu: gpu !== undefined, adapter: gpu !== undefined && await gpu.requestAdapter() !== null, version: navigator.userAgent };
    });
    console.log(`[DEBUG] Play browser capabilities ${JSON.stringify(observed)}`);
    if (!observed.jspi || !observed.adapter) throw new Error("Play browser prerequisites unavailable");
  } finally { await browser?.close(); await server.stop(true); }
}

const [command, workspace] = process.argv.slice(2);
if (command !== "browser-capabilities" || !workspace) throw new Error("browser-capabilities <workspace>");
await browserCapabilities(workspace);
