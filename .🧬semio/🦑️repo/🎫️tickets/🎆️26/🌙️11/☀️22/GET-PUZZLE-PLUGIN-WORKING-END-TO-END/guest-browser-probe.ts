/** 🔍️ Runs the repository's browser descriptor probe on the isolated Puzzle component. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { rewritePreview2ShimImports } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentFactoryAt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧬️jco-module-rewrites/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { monolithicComponentTable, pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";

const base = "semio_hub_puzzle_component";
const output = join(import.meta.dir, "🗑️generated", process.argv[3] === "current" ? "guest-browser-current" : "guest-browser");
const componentSource = join(process.cwd(), "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component");
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
    const document = JSON.parse(descriptor.json);
    const expected = [2, 3, 5].flatMap(dimension => ["editor", "viewer"].map(role => `s.puzzle.puzzle${dimension}d@1/*#${role}`));
    const actual = document.manifest.apps.map((app: { id: string }) => app.id);
    if (JSON.stringify(actual.sort()) !== JSON.stringify(expected)) throw new Error("The built component does not advertise all six Puzzle apps");
    if (process.argv[3] === "current") {
      const editor = document.manifest.apps.find((app: { id: string }) => app.id === "s.puzzle.puzzle5d@1/*#editor");
      const rewrite = editor?.io?.ports[1];
      const committed = JSON.parse(readFileSync(join(componentSource, "🔣️.json"), "utf8"));
      const committedEditor = committed.manifest.apps.find((app: { id: string }) => app.id === editor?.id);
      const expectedRewrite = committedEditor?.io?.ports[1];
      if (expectedRewrite?.id !== "rewrite:in" || !isDeepStrictEqual(rewrite, expectedRewrite)) throw new Error("The built Puzzle 5D editor's rewrite input differs from its declared input");
    }
    writeFileSync(join(output, "🛂️.descriptor.semio"), descriptor.pack);
    writeFileSync(join(output, "🔣️.json"), descriptor.json);
    writeFileSync(join(output, "🌉️bridge.js"), pluginComponentBridgeSource(base, monolithicComponentTable(document, base)));
    const committed = readFileSync(join(componentSource, "🛂️.descriptor.semio"));
    console.log(`Puzzle actor described ${actual.length} apps in ${descriptor.pack.length} canonical bytes, matches committed descriptor=${committed.equals(descriptor.pack)}`);
    break;
  }
  default:
    throw new Error("Usage: guest-browser-probe.ts prepare|finalize [current]");
}
