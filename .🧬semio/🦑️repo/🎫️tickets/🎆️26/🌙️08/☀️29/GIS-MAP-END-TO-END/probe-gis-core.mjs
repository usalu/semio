import { chromium, firefox, webkit } from "playwright";
import { createReadStream } from "node:fs";
import { mkdir, mkdtemp, stat } from "node:fs/promises";
import { createServer } from "node:http";
import { pipeline } from "node:stream/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const [engine] = process.argv.slice(2);
const browsers = { chromium, firefox, webkit };
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const source = join(generated, "native-transpile-proof", "semio_hub_gis_component.core.wasm");
const { size } = await stat(source);
const log = (phase, data = {}) => console.log(JSON.stringify({ engine, phase, ...data }));
let transfers = 0;
const server = createServer((request, response) => {
  if (request.url === "/") {
    response.writeHead(200, {"Content-Type":"text/html","Cache-Control":"no-store"});
    response.end("<!doctype html><title>GIS core streaming diagnostic</title>");
  } else if (request.url === "/gis.wasm") {
    transfers++;
    response.writeHead(200, {"Content-Type":"application/wasm","Content-Length":size,"Cache-Control":"no-store"});
    const started = performance.now();
    void pipeline(createReadStream(source), response).then(
      () => log("stream-complete", { milliseconds: Math.round(performance.now() - started), bytes:size }),
      error => log("stream-error", { error:String(error) }),
    );
  } else {
    response.writeHead(404).end();
  }
});
await new Promise((resolve, reject) => { server.once("error", reject); server.listen(0, "127.0.0.1", resolve); });
let browser;
let deadline;
try {
  const env = { ...process.env };
  if (engine === "firefox") {
    const data = await mkdtemp(join(generated, "compile-firefox-"));
    env.MOZ_APP_DATA = data;
    env.MOZ_LOCAL_APP_DATA = join(data, "local");
    await mkdir(env.MOZ_LOCAL_APP_DATA);
  }
  browser = await browsers[engine].launch({ headless:true, timeout:10000, env,
    ...(engine === "chromium" && process.env.GIS_CHROMIUM_CHANNEL ? {channel:process.env.GIS_CHROMIUM_CHANNEL} : {}),
  });
  const version = browser.version();
  const page = await browser.newPage();
  await page.goto("http://127.0.0.1:" + server.address().port + "/", { timeout:7000 });
  log("compile-start", {version, bytes:size});
  const result = await Promise.race([
    page.evaluate(async () => {
      const start = performance.now();
      const response = await fetch("/gis.wasm", {cache:"no-store"});
      const headers = performance.now();
      const module = await WebAssembly.compileStreaming(response);
      const end = performance.now();
      return { totalMilliseconds: Math.round(end-start), headersMilliseconds: Math.round(headers-start),
        compileAfterHeadersMilliseconds: Math.round(end-headers),
        imports: WebAssembly.Module.imports(module).length,
        exports: WebAssembly.Module.exports(module).length };
    }),
    new Promise((_, reject) => { deadline = setTimeout(() => reject(new Error("Compile exceeded 55 seconds")),55000); }),
  ]);
  if (transfers !== 1) throw new Error("Expected exactly one core transfer, saw " + transfers);
  log("compiled", {version, ...result});
} catch (error) {
  log("failed", { error:String(error) });
  process.exitCode = 1;
} finally {
  clearTimeout(deadline);
  server.closeAllConnections();
  server.close();
  await browser?.close();
}
