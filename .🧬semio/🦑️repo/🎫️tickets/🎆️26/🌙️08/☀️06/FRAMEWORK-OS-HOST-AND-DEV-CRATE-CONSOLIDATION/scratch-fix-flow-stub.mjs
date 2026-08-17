import { readFileSync, writeFileSync, existsSync } from "fs";
import { join } from "path";

const ticket = process.argv[2];
const paths = readFileSync(join(ticket, "🧪paths-e2e.txt"), "utf8").trim().split("\n");
const flow = paths[0];
const wasmLoader = paths[1];
const flowPkgJs = join(flow, "🙰core/pkg/flow_core.js");
// actual core dir
import { readdirSync } from "fs";
const coreDir = readdirSync(flow).find((n) => /core/.test(n));
const pkgJs = join(flow, coreDir, "pkg", "flow_core.js");
const pkgJson = join(flow, coreDir, "pkg", "package.json");
console.log("pkgJs", pkgJs, existsSync(pkgJs));

const stubFile = "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
// discover stub file from prior knowledge via reading ticket log if needed
const stubCandidates = [
  "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts",
];
// use path from 854203 exactly - framework emoji is 🧰️
const stubPath = "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts".replace("𝒯framework", "🧰️framework");
console.log("stubPath", stubPath, existsSync(stubPath));
const stubSrc = readFileSync(stubPath, "utf8");
const m = stubSrc.match(/PLAYGROUND_WASM_JS_STUB\s*=\s*`([\s\S]*?)`;/);
console.log("PLAYGROUND_WASM_JS_STUB:\n", m?.[1] ?? "(not found)");

const loader = readFileSync(wasmLoader, "utf8");
const typeIdx = loader.indexOf("export type FlowWasmSession");
console.log("FlowWasmSession type:\n", loader.slice(typeIdx, typeIdx + 1200));

const editorStub = "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/framework_editor.js".replace("𝒯framework", "𝒯framework");
const editorStub2 = "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/framework_editor.js".replaceAll("𝒯", "�");
const editorPath = "/Users/ueli/Documents/semio/𝒯framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/framework_editor.js".replace("𝒯framework", "🧰️framework");
console.log("editor", editorPath, existsSync(editorPath));
console.log(readFileSync(editorPath, "utf8"));

// Write proper flow stub mirroring editor pattern + FlowSession
const flowStub = `/** @emoji 🧪 Dev stub for \`@semio-tech/flow-core\` until \`bun ./📜️script.ts wasm\` emits pkg/. */
export default async function init(_input) { return undefined; }
export class FlowSession {
  free() {}
  syncFromSceneJson(_json) {}
  setSize(_w, _h, _dpr) {}
  render() {}
  pointerDownScreen() {}
  pointerMoveScreen() {}
  pointerUpScreen() {}
  wheelScrollScreen() {}
  cameraJson() { return "{}"; }
  setCanvasThemeJson(_json) {}
}
`;

writeFileSync(pkgJs, flowStub);
// also update package.json description
const pkg = JSON.parse(readFileSync(pkgJson, "utf8"));
pkg.description = "flow core wasm pkg stub (init + FlowSession) until wasm build emits pkg/";
writeFileSync(pkgJson, JSON.stringify(pkg, null, 2) + "\n");
console.log("wrote stub", pkgJs, "bytes", flowStub.length);
console.log(readFileSync(pkgJs, "utf8"));
