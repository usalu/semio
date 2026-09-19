#!/usr/bin/env bun
/** 🧩️ Loads the VS Code extension module against a generated `vscode` host, proving its top-level
 * `⌛️Queries` constants evaluate — before T2c they threw `ReferenceError: documents is not defined`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const hostSource = readFileSync(join(import.meta.dir, "🐍️t2c-vscode-host.mjs"), "utf8");
Bun.plugin({
  name: "virtual-vscode-host",
  setup(build) {
    build.module("vscode", () => ({ contents: hostSource, loader: "js" }));
  },
});

const extension = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts");
const documents = Object.entries(extension).filter(([name]) => name.endsWith("Document"));
console.log(`exports=${Object.keys(extension).length} queryDocuments=${documents.length} graphql=${typeof extension.graphql}`);
for (const [name, value] of documents.slice(0, 3)) console.log(`  ${name}: ${typeof value} chars=${String(value).length}`);
