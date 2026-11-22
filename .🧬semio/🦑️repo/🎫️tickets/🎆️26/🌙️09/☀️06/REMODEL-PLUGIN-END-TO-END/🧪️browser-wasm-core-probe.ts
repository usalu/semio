import assert from "node:assert/strict";
import { readdir, stat, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { chromium } from "playwright";

const directory = resolve(import.meta.dir, "🗑️generated/browser-materialization");
const modules = (await readdir(directory)).filter(name => /^remodel\.core\d*\.wasm$/u.test(name)).sort();
assert.ok(modules.length, "Jco did not emit any core Wasm files");
const records = [];
const server = Bun.serve({
  hostname: "127.0.0.1",
  port: 0,
  fetch(request) {
    const path = new URL(request.url).pathname;
    if (path === "/") return new Response("<title>Remodel core compilation</title>", { headers: { "content-type": "text/html" } });
    const name = decodeURIComponent(path.slice(1));
    return modules.includes(name)
      ? new Response(Bun.file(join(directory, name)), { headers: { "content-type": "application/wasm" } })
      : new Response("Unknown core module", { status: 404 });
  },
});
try {
  const browser = await chromium.launch({ headless: true, timeout: 8_000 });
  try {
    const page = await browser.newPage();
    await page.goto(server.url.href);
    for (const name of modules) {
      const size = (await stat(join(directory, name))).size;
      console.log(`[DEBUG] Chromium is compiling ${name} (${size} bytes)`);
      let deadline: ReturnType<typeof setTimeout> | undefined;
      const timeout = new Promise<never>((_, reject) => {
        deadline = setTimeout(() => reject(Error(`Chromium compilation timed out for ${name}`)), 65_000);
      });
      const compilation = page.evaluate(async name => {
        const started = performance.now();
        const controller = new AbortController();
        const timer = setTimeout(() => controller.abort(), 60_000);
        try {
          const compiled = await WebAssembly.compileStreaming(fetch("/" + encodeURIComponent(name), { signal: controller.signal }));
          return { imports: WebAssembly.Module.imports(compiled).length, exports: WebAssembly.Module.exports(compiled).length, elapsedMs: Math.round(performance.now() - started) };
        } finally {
          clearTimeout(timer);
        }
      }, name);
      try {
        const result = await Promise.race([compilation, timeout]);
        records.push({ chromium: browser.version(), name, bytes: size, ...result });
      } finally {
        clearTimeout(deadline);
      }
    }
  } finally {
    await browser.close();
  }
} finally {
  await server.stop(true);
}
await writeFile(join(directory, "chromium-observations.json"), JSON.stringify(records, null, 2) + "\n");
console.log(JSON.stringify(records));
