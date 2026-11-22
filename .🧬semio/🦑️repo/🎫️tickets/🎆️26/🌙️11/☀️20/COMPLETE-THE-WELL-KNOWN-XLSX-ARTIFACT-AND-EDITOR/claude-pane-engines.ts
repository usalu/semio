import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const os = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
const { frameworkOsEngineDeliveriesV1 } = await import(join(root, os));
const products = join(root, ".cache/b/p/m");
for (const d of frameworkOsEngineDeliveriesV1(products, { version: 1, mode: "prototype", facet: "renderer" })) console.log(d.id, d.root, d.entry, d.files.join(","), d.entrypoint);
