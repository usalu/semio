/** 🔍️ Runs the repository's browser descriptor probe on the isolated Puzzle component. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { rewritePreview2ShimImports } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentFactoryAt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧬️jco-module-rewrites/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { monolithicComponentTable, pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";

const base = "semio_hub_puzzle_component";
const output = join(import.meta.dir, "🗑️generated", "guest-browser");
switch (process.argv[2]) {
  case "prepare": {
    const module = join(output, `${base}.js`);
    const vendor = join(process.cwd(), ".cache/b/p/m/browser/prototype/modules", PREVIEW2_VENDOR_RELATIVE);
    rewriteJcoAsyncResultLiftingAt(module);
    rewriteJcoComponentFactoryAt(module);
    rewritePreview2ShimImports(module, vendor);
    writeFileSync(join(output, "🟨️.js"), hostShimSource());
    writeFileSync(join(output, "probe-source.mjs"), PLUGIN_DESCRIPTOR_PROBE_SOURCE);
    break;
  }
  case "finalize": {
    const encoded = readFileSync(join(output, "probe.b64"), "utf8").trim();
    if (!/^[A-Za-z0-9+/]+={0,2}$/u.test(encoded)) throw new Error(`Expected a single base64 descriptor, received ${encoded.length} characters`);
    const descriptor = finalizePluginDescriptor(Buffer.from(encoded, "base64"), "puzzle");
    writeFileSync(join(output, "🛂️.descriptor.semio"), descriptor.pack);
    writeFileSync(join(output, "🔣️.json"), descriptor.json);
    writeFileSync(join(output, "🌉️bridge.js"), pluginComponentBridgeSource(base, monolithicComponentTable(JSON.parse(descriptor.json), base)));
    const committed = readFileSync(join(process.cwd(), "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/🛂️.descriptor.semio"));
    console.log(`Puzzle actor returned ${descriptor.pack.length} canonical descriptor bytes, matches committed descriptor=${committed.equals(descriptor.pack)}`);
    break;
  }
  default:
    throw new Error("Usage: guest-browser-probe.ts prepare|finalize");
}
