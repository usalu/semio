import { join } from "node:path";
const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
const output = process.env.SEMIO_LAYOUT_OUTPUT;
if (!root || !output) throw new Error("Explicit repository and output paths are required");
const { testPluginCoreOptimization } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕸️native-optimization/🟦️.ts"));
await testPluginCoreOptimization(root, output);
