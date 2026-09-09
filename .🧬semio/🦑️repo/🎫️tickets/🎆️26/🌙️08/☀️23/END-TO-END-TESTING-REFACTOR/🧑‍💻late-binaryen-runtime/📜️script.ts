import { join } from "node:path";
const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
const output = process.env.SEMIO_LAYOUT_OUTPUT;
if (!root || !output) throw new Error("Explicit repository and output paths are required");
const { testBinaryenToolchain } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts"));
await testBinaryenToolchain(root, output);
