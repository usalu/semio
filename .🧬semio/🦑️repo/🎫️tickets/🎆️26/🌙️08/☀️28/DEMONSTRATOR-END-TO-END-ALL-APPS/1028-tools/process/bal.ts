import { selectRustSourceV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFileSync } from "node:fs";
for (const f of process.argv.slice(2)) { try { selectRustSourceV1(readFileSync(f, "utf8"), ["target_arch=\"wasm32\""]); } catch (e) { console.log(f, String(e)); } }
