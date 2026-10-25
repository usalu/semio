// Headless browser probe of the Jack LSP worker: serves the worker + wasm through Vite, replays the shared conformance vectors, fails on any console error.
// usage: bun probe.mjs <wasm-package-dir> <output-dir>   (the package dir holds trinity_jack_lsp.js + trinity_jack_lsp_bg.wasm)
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { copyFileSync, mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repo = "/Users/ueli/Documents/semio";
const here = dirname(fileURLToPath(import.meta.url));
const [pkgDir, outDir] = [resolve(process.argv[2]), resolve(process.argv[3])];
const lsp = join(repo, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp");
const vectors = join(lsp, "🧪️tests/🔖️protocol/🔣️.json");
const worker = join(lsp, "🟦️.ts");
const transient = join(repo, "🧰️framework/🔨️modules/🫧️transient/🟦️.ts");
for (const file of [join(pkgDir, "trinity_jack_lsp.js"), join(pkgDir, "trinity_jack_lsp_bg.wasm"), vectors, worker, transient]) if (!existsSync(file)) throw new Error(`missing ${file}`);
mkdirSync(outDir, { recursive: true });
const site = join(outDir, "site");
mkdirSync(site, { recursive: true });
copyFileSync(join(here, "index.html"), join(site, "index.html"));
writeFileSync(join(site, "driver.mjs"), readFileSync(join(here, "driver.mjs"), "utf8").replace("VECTORS_PATH", vectors).replace("WORKER_URL", `/@fs${worker}`));

const server = await createServer({
  configFile: false,
  root: site,
  logLevel: "warn",
  server: { host: "127.0.0.1", port: 6091, strictPort: true, fs: { strict: false }, hmr: false },
  resolve: { alias: [{ find: "@semio-tech/trinity-jack-lsp", replacement: join(pkgDir, "trinity_jack_lsp.js") }, { find: "@semio-tech/framework", replacement: transient }] },
  worker: { format: "es" },
});
await server.listen();
const lines = [];
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage();
  page.on("console", message => lines.push(`${message.type()} ${message.text().slice(0, 1200)}`));
  page.on("pageerror", error => lines.push(`pageerror ${String(error).slice(0, 1200)}`));
  page.on("requestfailed", request => lines.push(`requestfailed ${request.url().slice(0, 300)}`));
  await page.goto("http://127.0.0.1:6091/", { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => window.__done === true, undefined, { timeout: 240000 });
  const results = await page.evaluate(() => window.__results);
  const failed = results.filter(row => !row.ok);
  const errors = lines.filter(line => /^(error|pageerror|requestfailed)/.test(line));
  writeFileSync(join(outDir, "results.json"), JSON.stringify({ results, console: lines }, null, 1));
  console.log(`[lsp-probe] ${results.length - failed.length}/${results.length} steps conform; ${errors.length} console errors`);
  for (const row of failed) console.log(`[lsp-probe] FAIL ${row.session} #${row.step}: ${row.detail}`);
  for (const line of errors) console.log(`[lsp-probe] ${line}`);
  process.exitCode = failed.length === 0 && errors.length === 0 && results.length > 0 ? 0 : 1;
} finally {
  await browser.close();
  await server.close();
}
