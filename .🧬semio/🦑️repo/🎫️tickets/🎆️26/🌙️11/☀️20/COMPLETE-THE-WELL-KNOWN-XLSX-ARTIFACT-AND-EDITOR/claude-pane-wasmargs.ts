import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const m = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts"));
const sel = m.wasmCompilerSelectionArgumentsV1({ mode: "prototype", manifest: "/X/Cargo.toml", configuration: null, selectors: [], threads: false, features: [], noDefaultFeatures: false });
console.log(JSON.stringify(sel));
console.log(JSON.stringify(m.wasmPackBuildArgumentsV1(sel.cargo, "prototype", "/OUT", "base")));
