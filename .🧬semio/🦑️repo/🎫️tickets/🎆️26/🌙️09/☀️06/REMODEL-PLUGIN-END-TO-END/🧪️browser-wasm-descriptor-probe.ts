import assert from "node:assert/strict";
import { readFile, readdir, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { rewritePreview2ShimImports } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentAssetUrlsAt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧬️jco-module-rewrites/🟦️.ts";
import { ensurePreview2ShimVendorAt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🩹️preview2-shim-patches/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const staging = join(import.meta.dir, "🗑️generated", "browser-materialization");
const modulePath = join(staging, "module", "remodel.js");
const packedPath = join(staging, "guest-descriptor.semio");
const [phase, engine] = process.argv.slice(2);
if (phase === "stage") {
  const vendor = join(staging, "vendor");
  ensurePreview2ShimVendorAt(vendor, root);
  rewriteJcoAsyncResultLiftingAt(modulePath);
  rewriteJcoComponentAssetUrlsAt(modulePath);
  rewritePreview2ShimImports(modulePath, vendor);
  await writeFile(join(staging, "module", "🟨️.js"), hostShimSource());
  await writeFile(join(staging, "descriptor-probe-source.mjs"), PLUGIN_DESCRIPTOR_PROBE_SOURCE);
  const webSource = PLUGIN_DESCRIPTOR_PROBE_SOURCE
    .replace('import { pathToFileURL } from "node:url";', "")
    .replace("pathToFileURL(process.argv[1]).href", '"./module/remodel.js"')
    .replace('process.stdout.write(Buffer.from(bytes).toString("base64"));', "export default bytes;");
  assert.ok(!webSource.includes("process.") && webSource.includes("export default bytes;"));
  await writeFile(join(staging, "descriptor-probe-web.js"), webSource);
  await writeFile(join(staging, "descriptor-probe.mjs"), `
import assert from "node:assert/strict";
import { writeFile } from "node:fs/promises";
const originalWrite = process.stdout.write;
let base64 = "";
process.argv.splice(1, 1);
try {
  process.stdout.write = chunk => {
    base64 += chunk;
    assert.ok(base64.length <= 12 * 1024 * 1024);
    return true;
  };
  await import("./descriptor-probe-source.mjs");
} finally {
  process.stdout.write = originalWrite;
}
assert.match(base64, /^[A-Za-z0-9+/]+={0,2}$/u);
const pack = Buffer.from(base64, "base64");
await writeFile(new URL("./guest-descriptor.semio", import.meta.url), pack);
console.log(JSON.stringify({ node: process.version, descriptorBytes: pack.length }));
`);
} else if (phase === "chromium") {
  const assets = new Map([["/descriptor-probe-web.js", join(staging, "descriptor-probe-web.js")]]);
  for (const role of ["module", "vendor"]) {
    for (const file of await readdir(join(staging, role))) {
      if (file.endsWith(".js") || file.endsWith(".wasm")) assets.set("/" + role + "/" + file, join(staging, role, file));
    }
  }
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    fetch(request) {
      const path = decodeURIComponent(new URL(request.url).pathname);
      if (path === "/") return new Response("<title>Remodel descriptor probe</title>", { headers: { "content-type": "text/html" } });
      const file = assets.get(path);
      if (file === undefined) return new Response("Unknown probe asset", { status: 404 });
      return new Response(Bun.file(file), { headers: { "content-type": path.endsWith(".wasm") ? "application/wasm" : "text/javascript" } });
    },
  });
  try {
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true, timeout: 8_000 });
    try {
      const page = await browser.newPage();
      await page.goto(server.url.href, { timeout: 8_000 });
      let deadline: ReturnType<typeof setTimeout> | undefined;
      const timeout = new Promise<never>((_, reject) => {
        deadline = setTimeout(() => reject(Error("Chromium describe() timed out")), 74_000);
      });
      try {
        const describe = page.evaluate(async () => {
          const { default: bytes } = await import(location.origin + "/descriptor-probe-web.js");
          return Array.from(bytes as Uint8Array);
        });
        const bytes = await Promise.race([describe, timeout]);
        await writeFile(join(staging, "chromium-descriptor.semio"), new Uint8Array(bytes));
        console.log(JSON.stringify({ chromium: browser.version(), descriptorBytes: bytes.length }));
      } finally {
        clearTimeout(deadline);
      }
    } finally {
      await browser.close();
    }
  } finally {
    await server.stop(true);
  }
} else if (phase === "verify" || phase === "verify-examples") {
  const component = join(root, "🌎️hub", "🧩️compositions", "📸️remodel", "🧩️component");
  const guest = await readFile(engine === "chromium" ? join(staging, "chromium-descriptor.semio") : packedPath);
  const canonical = finalizePluginDescriptor(guest, "remodel");
  const [expectedPack, expectedJson] = await Promise.all([
    readFile(join(component, "🛂️.descriptor.semio")),
    readFile(join(component, "🔣️.json"), "utf8"),
  ]);
  const current = JSON.parse(canonical.json);
  const checkedInPackEqual = guest.equals(expectedPack);
  if (phase === "verify-examples") {
    const subset = join(root, "✏️s", "🔌️plugins", "📸️remodel", "🗿️artifacts", "📸️remodeling", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any");
    const fixture = JSON.parse(await readFile(join(subset, "✏️editor", "📚️examples", "🧪️tests", "🧫️picker", "🔣️.json"), "utf8")) as { examples: { id: string; asset: string }[] };
    const observed = current.manifest.examples as { id: string; artifactJson: string }[];
    assert.deepEqual(observed.map(row => row.id).sort(), ["demo", "demo-session", "synthetic-orbit"]);
    assert.deepEqual(fixture.examples.map(row => row.id).sort(), ["demo", "demo-session", "synthetic-orbit"]);
    const examples = await Promise.all(fixture.examples.map(async ({ id, asset }) => {
      const text = await readFile(join(subset, asset), "utf8");
      assert.ok(observed.find(row => row.id === id)?.artifactJson === text, `Guest example ${id} differs from its source file`);
      return { id, bytes: Buffer.byteLength(text), sha256: new Bun.CryptoHasher("sha256").update(text).digest("hex") };
    }));
    await writeFile(join(staging, `${engine === "chromium" ? "chromium" : "guest"}-descriptor.json`), canonical.json);
    console.log(JSON.stringify({ descriptorBytes: guest.length, sha256: new Bun.CryptoHasher("sha256").update(guest).digest("hex"), checkedInPackEqual, examples }));
  } else {
    assert.ok(checkedInPackEqual, "Guest descriptor differs from the checked-in Pack");
    assert.deepEqual(current, JSON.parse(expectedJson));
    console.log(JSON.stringify({ descriptorBytes: guest.length, sha256: new Bun.CryptoHasher("sha256").update(guest).digest("hex"), checkedInPackEqual, checkedInJsonEqual: true }));
  }
} else {
  throw Error("Expected stage, chromium, verify or verify-examples");
}
