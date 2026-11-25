/** 🛂️ Finalizes one semantic FEM descriptor pack into the published pair, for A/B serving. Usage: bun finalize-descriptor.ts <semantic.semio> <outDir> */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const { finalizePluginDescriptor } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts");
const descriptor = finalizePluginDescriptor(readFileSync(process.argv[2]!), "fem");
writeFileSync(join(process.argv[3]!, "🛂️.descriptor.semio"), descriptor.pack);
writeFileSync(join(process.argv[3]!, "🔣️.json"), descriptor.json);
