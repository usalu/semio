import { resolve, join } from "node:path";
import { mkdirSync } from "node:fs";
const root = resolve(import.meta.dir, "../../../../../../../.."), output = join(import.meta.dir, "../🗑️generated");
mkdirSync(output, { recursive: true });
const { testWasmOptimizer } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🟦️.ts"));
await testWasmOptimizer(root, output);
