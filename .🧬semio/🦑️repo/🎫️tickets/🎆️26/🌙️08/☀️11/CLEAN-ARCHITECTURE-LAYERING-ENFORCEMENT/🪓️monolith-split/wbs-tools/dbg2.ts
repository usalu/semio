import { readWasmPackageOwnerV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { readFileSync } from "node:fs";
const m = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/package.json", "utf8"));
try { console.log(readWasmPackageOwnerV1(m.semioWasmOwner)); } catch (e) { console.log(String((e as Error).stack).slice(0, 1500)); }
