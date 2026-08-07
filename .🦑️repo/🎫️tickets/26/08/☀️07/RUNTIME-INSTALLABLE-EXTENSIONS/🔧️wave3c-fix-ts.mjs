import fs from "fs";
import path from "path";

const TICKET = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS";
const before = fs.readFileSync(path.join(TICKET, "3d-index-before.ts"), "utf8");

const mods = "/Users/ueli/Documents/semio/✏️s/🔨️modules";
const d3 = path.join(mods, fs.readdirSync(mods).find((n) => n.includes("3d")));
const ts = path.join(d3, "📦️packages", fs.readdirSync(path.join(d3, "📦️packages")).find((n) => n.includes("typescript")));
const idxFile = path.join(ts, fs.readdirSync(ts).find((n) => n.includes("index") && n.endsWith(".ts")));

const flowCoreImport = before.match(/import\("([^"]*flow_core\.js)"\)/)?.[1];
const flowCoreWasm = before.match(/import\("([^"]*flow_core_bg\.wasm\?url)"\)/)?.[1];
if (!flowCoreImport || !flowCoreWasm) {
  console.error("missing flow_core import paths in backup");
  process.exit(1);
}

const newEnsure = `/** @emoji ⏳️ Loads brep tessellation WASM via flow_core (standalone \`flow_extension_brep\` pack removed in Wave 3.c). */
export async function ensureBrepWasmLoaded(): Promise<BrepWasmModule> {
  if (brepWasm.current) return brepWasm.current;
  const [{ default: initFlow, tessellate, dispose }, { default: wasmUrl }] = await Promise.all([
    import("${flowCoreImport}"),
    import("${flowCoreWasm}"),
  ]);
  if (typeof tessellate !== "function" || typeof dispose !== "function") {
    throw new Error("flow_core brep tessellation exports missing — rebuild flow/core wasm");
  }
  if (initFlow) await initFlow({ module_or_path: wasmUrl });
  brepWasm.current = { tessellate, dispose };
  return brepWasm.current;
}

`;

const newModule = `/** @emoji ⏳️ Brep operator WASM loader — standalone \`flow_extension_brep\` pack removed in Wave 3.c; install the packaged brep extension instead. */
export async function ensureBrepModuleWasmLoaded(): Promise<BrepModuleWasm> {
  if (brepModuleWasm.current) return brepModuleWasm.current;
  throw new Error(
    "flow_extension_brep wasm pack removed (Wave 3.c). Geometry IO operators now live in the packaged flow-extension-brep extension; install/enable it or call host flow tessellate/export APIs.",
  );
}

`;

let text = fs.readFileSync(idxFile, "utf8");

// Replace ensureBrepWasmLoaded function
{
  const start = text.indexOf("/** @emoji ⏳️ Loads brep tessellation WASM");
  const end = text.indexOf("export async function createDefaultBrepWasmBridge");
  if (start < 0 || end < 0) {
    console.error("ensureBrepWasmLoaded markers missing", { start, end });
    process.exit(1);
  }
  text = text.slice(0, start) + newEnsure + text.slice(end);
}

// Replace ensureBrepModuleWasmLoaded function
{
  const start = text.indexOf("/** @emoji ⏳️ Loads flow brep module WASM");
  // after broken patch the emoji comment may differ — also try alternate
  let s = start;
  if (s < 0) s = text.indexOf("export async function ensureBrepModuleWasmLoaded");
  const end = text.indexOf("function brepGeometryInput");
  if (s < 0 || end < 0) {
    console.error("ensureBrepModuleWasmLoaded markers missing", { s, end });
    process.exit(1);
  }
  // include doc comment if we landed on the function
  if (!text.slice(s, s + 10).includes("/**")) {
    // find preceding doc
    const doc = text.lastIndexOf("/**", s);
    if (doc >= 0 && s - doc < 300) s = doc;
  }
  text = text.slice(0, s) + newModule + text.slice(end);
}

fs.writeFileSync(idxFile, text);
console.log("rewrote loaders in", idxFile);

// Verify no flow_extension_brep.js left
const left = text.split("\n").filter((l) => l.includes("flow_extension_brep.js") || l.includes("flow_extension_brep_bg"));
console.log("stale js refs", left.length, left);

// Show resulting functions
const a = text.indexOf("export async function ensureBrepWasmLoaded");
const b = text.indexOf("export async function ensureBrepModuleWasmLoaded");
const c = text.indexOf("function brepGeometryInput");
console.log("--- ensureBrepWasmLoaded ---");
console.log(text.slice(a, text.indexOf("export async function createDefaultBrepWasmBridge")));
console.log("--- ensureBrepModuleWasmLoaded ---");
console.log(text.slice(b, c));
