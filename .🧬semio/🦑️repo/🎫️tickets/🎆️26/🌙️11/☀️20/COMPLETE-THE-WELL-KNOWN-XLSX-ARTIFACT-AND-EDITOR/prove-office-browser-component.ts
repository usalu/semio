import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, existsSync, mkdirSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve, sep } from "node:path";

const repoRoot = process.cwd();
const output = join(import.meta.dir, "🗑️generated");
const browserBundle = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const materialization = join(browserBundle, "🏗️materialization");
const component = join(output, "stdio-office.wasm");
const moduleRoot = join(output, "office-web");
const moduleBase = "semio_hub_stdio_office_component";
const componentSha256 = "2ae0845ec0987eaab2d1d584fed4fb1f7745d8d5de819f5db88145b90f163a7b";
const provedPack = join(output, "office-emitted-replay", "🛂️.descriptor.semio");

if (process.argv[2] === "transpile") {
  const sha = createHash("sha256");
  for await (const chunk of createReadStream(component)) sha.update(chunk);
  assert.equal(sha.digest("hex"), componentSha256);
  const { transpilePluginComponentAsync } = await import(join(materialization, "🟦️.ts"));
  const { nativeTranspileBinaryV1 } = await import(join(materialization, "🦀️native-transpile/🟦️.ts"));
  const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source/🟦️.ts"));
  const { ensurePreview2ShimVendorAt } = await import(join(materialization, "🩹️preview2-shim-patches/🟦️.ts"));
  const { PREVIEW2_VENDOR_RELATIVE } = await import(join(browserBundle, "🕸️imports/🟦️.ts"));
  const preview2VendorDir = join(moduleRoot, PREVIEW2_VENDOR_RELATIVE);
  mkdirSync(moduleRoot);
  ensurePreview2ShimVendorAt(preview2VendorDir, repoRoot);
  writeFileSync(join(moduleRoot, "🟨️.js"), hostShimSource(), { flag: "wx" });
  const tool = nativeTranspileBinaryV1(repoRoot);
  const toolHash = createHash("sha256");
  for await (const chunk of createReadStream(tool)) toolHash.update(chunk);
  console.log(`[DEBUG] Transpiling Office component ${componentSha256} with native tool ${toolHash.digest("hex")}`);
  await transpilePluginComponentAsync(component, moduleRoot, moduleBase, { repoRoot, preview2VendorDir, optimize: false });
  assert.ok(existsSync(join(moduleRoot, moduleBase + ".js")));
  console.log("Production prototype transpile completed for the captured Office component");
} else if (process.argv[2] === "chromium") {
  const { PLUGIN_DESCRIPTOR_PROBE_SOURCE } = await import(join(browserBundle, "🛂️descriptor/🟦️.ts"));
  const repositoryBrowser = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts");
  const { ensureRepositoryBrowserPathV1 } = await import(repositoryBrowser);
  const require = createRequire(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/package.json"));
  ensureRepositoryBrowserPathV1(repoRoot, process.env);
  const { chromium } = require("playwright") as typeof import("playwright");
  const replacements = [
    ['import { pathToFileURL } from "node:url";', ""],
    ["pathToFileURL(process.argv[1]).href", JSON.stringify("/" + moduleBase + ".js")],
    ['process.stdout.write(Buffer.from(bytes).toString("base64"));', 'window.officeDescriptor = Array.from(bytes);'],
  ] as const;
  let probe = PLUGIN_DESCRIPTOR_PROBE_SOURCE as string;
  for (const [before, after] of replacements) {
    assert.ok(probe.includes(before));
    probe = probe.replace(before, after);
  }
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch(request) {
    const route = decodeURIComponent(new URL(request.url).pathname);
    if (route === "/") return new Response('<script type="module" src="/probe.js"></script>', { headers: { "Content-Type": "text/html" } });
    if (route === "/probe.js") return new Response(probe, { headers: { "Content-Type": "text/javascript" } });
    const path = resolve(moduleRoot, "." + route);
    return path.startsWith(moduleRoot + sep) && existsSync(path)
      ? new Response(Bun.file(path), { headers: { "Content-Type": path.endsWith(".wasm") ? "application/wasm" : "text/javascript" } })
      : new Response(null, { status: 404 });
  } });
  try {
    const browser = await chromium.launch({ args: ["--js-flags=--experimental-wasm-jspi"] });
    try {
      const page = await browser.newPage();
      const failure = page.waitForEvent("pageerror", { timeout: 0 }).then(error => { throw error; });
      await page.goto(server.url.href);
      await Promise.race([failure, page.waitForFunction(() => "officeDescriptor" in window, undefined, { timeout: 120_000 })]);
      const bytes = await page.evaluate(() => (window as Window & { officeDescriptor?: number[] }).officeDescriptor);
      const emitted = Buffer.from(await Bun.file(provedPack).arrayBuffer());
      assert.deepEqual(Buffer.from(bytes ?? []), emitted);
      console.log(`Chromium JSPI exposed the production actor exports and independently returned the native Office descriptor ${createHash("sha256").update(emitted).digest("hex")}`);
    } finally {
      await browser.close();
    }
  } finally {
    server.stop(true);
  }
} else {
  throw Error("Expected transpile or chromium");
}
