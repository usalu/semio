/** 🩺️ F2 — traces why one leaf write yields two HMR updates: logs every replayed watcher event, every freshness-guard
 * retirement and every HMR payload Vite sends, per write style. usage: bun f2-hot-debug2.ts */
import { mkdirSync, mkdtempSync, renameSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createServer, type Plugin } from "vite";
import { chromium } from "playwright";
import { semioSourceFreshnessVitePlugins } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";
const t0 = Date.now();
const log = (...parts: unknown[]) => console.log(String(Date.now() - t0).padStart(6), ...parts);
const root = mkdtempSync(join(tmpdir(), "f2-hot-"));
mkdirSync(join(root, "🧰️framework"), { recursive: true });
const leaf = join(root, "🧰️framework/🍃️leaf.ts");
writeFileSync(leaf, 'export const value: string = "0";\n');
writeFileSync(join(root, "🧰️framework/🟦️.ts"), ['import { value } from "./🍃️leaf.ts";', "const state = { value, updates: 0 };", 'Object.defineProperty(window, "__hot", { value: state });', 'if (import.meta.hot) import.meta.hot.accept("./🍃️leaf.ts", (next) => { if (next) { state.value = next.value; state.updates += 1; } });', ""].join("\n"));
writeFileSync(join(root, "index.html"), '<!doctype html><html><head><meta charset="utf-8"></head><body><script type="module" src="/🧰️framework/🟦️.ts"></script></body></html>');
const tap: Plugin = {
  name: "f2-tap",
  configureServer(server) {
    const emit = server.watcher.emit.bind(server.watcher);
    server.watcher.emit = ((event: string, ...rest: unknown[]) => { if (["add", "change", "unlink", "addDir"].includes(event)) log("watcher", event, String(rest[0]).slice(-40)); return emit(event, ...rest); }) as typeof server.watcher.emit;
    const send = server.hot.send.bind(server.hot);
    server.hot.send = ((payload: unknown, ...rest: unknown[]) => { log("hmr send", JSON.stringify(payload).slice(0, 200)); return (send as (...a: unknown[]) => void)(payload, ...rest); }) as typeof server.hot.send;
    server.middlewares.use((req, _res, next) => { if (!/@vite\/client|@id|node_modules/.test(req.url ?? "")) log("request", decodeURIComponent(req.url ?? "")); next(); });
  },
  handleHotUpdate(context) { log("handleHotUpdate", context.file.slice(-30), context.modules.length); },
};
const server = await createServer({ configFile: false, root, logLevel: "silent", cacheDir: join(root, ".vite"), optimizeDeps: { noDiscovery: true, include: [] }, server: { host: "127.0.0.1", port: 6587, strictPort: true, watch: null }, plugins: [tap, ...semioSourceFreshnessVitePlugins({ repoRoot: root })] });
await server.listen();
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.goto("http://127.0.0.1:6587/");
await page.waitForFunction(() => (window as any).__hot?.value === "0", undefined, { timeout: 60_000 });
log("booted", JSON.stringify(await page.evaluate(() => (window as any).__hot)));
if (process.env.F2_PAUSE !== "0") await page.waitForTimeout(1500);
log("--- in-place write");
writeFileSync(leaf, 'export const value: string = "1";\n');
if (process.env.F2_GAP === "0") await page.waitForFunction(() => (window as any).__hot.value === "1", undefined, { polling: 20 }); else await page.waitForTimeout(3000);
log("state", JSON.stringify(await page.evaluate(() => (window as any).__hot)));
log("--- atomic write");
writeFileSync(`${leaf}.tmp`, 'export const value: string = "2";\n');
renameSync(`${leaf}.tmp`, leaf);
await page.waitForTimeout(3000);
log("state", JSON.stringify(await page.evaluate(() => (window as any).__hot)));
await browser.close();
await server.close();
