import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const ticket = process.argv[2];
const flow = readFileSync(join(ticket, "🧪paths-e2e.txt"), "utf8").trim().split("\n")[0];
const { readdirSync } = await import("fs");
const coreDir = readdirSync(flow).find((n) => /core/.test(n));
const pkgJs = join(flow, coreDir, "pkg", "flow_core.js");

const stubPath = "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts".replace("𝒯framework", "🧰️framework");
const stubSrc = readFileSync(stubPath, "utf8");
const m = stubSrc.match(/PLAYGROUND_WASM_JS_STUB\s*=\s*`([\s\S]*?)`;/);
if (!m) throw new Error("PLAYGROUND_WASM_JS_STUB not found");
// The template has ${JSON.stringify(...)} interpolation for lod scales — evaluate it the same way
const PLAYGROUND_WASM_JS_STUB = new Function(`return \`${m[1]}\`;`)();
writeFileSync(pkgJs, `/** @emoji 🧪 Dev stub for \`@semio-tech/flow-core\` until wasm-pack emits pkg/. */\n${PLAYGROUND_WASM_JS_STUB}\n`);
console.log("wrote", pkgJs, "bytes", readFileSync(pkgJs).length);
console.log(readFileSync(pkgJs, "utf8").slice(0, 400));
