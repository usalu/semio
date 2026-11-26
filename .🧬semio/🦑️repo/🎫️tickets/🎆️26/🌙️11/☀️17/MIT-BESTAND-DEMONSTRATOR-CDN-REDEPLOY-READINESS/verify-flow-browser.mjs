import { createHash } from "node:crypto";
import { readFile, readdir } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const [input, second, third] = process.argv.slice(2);
const site = input === "--site";
const packageRoot = site ? third : second;
if (!input || (site ? !second || !third || process.argv.length !== 5 : process.argv.length > 4)) {
  throw new Error("Usage: verify-flow-browser.mjs module.wasm [managed-package] | --site captured-output managed-package");
}
let modulePath = input;
if (site) {
  const assets = join(resolve(second), "assets");
  const files = await readdir(assets);
  const wasmFiles = files.filter(name => /^flow_core_bg-[a-zA-Z0-9_-]+[.]wasm$/.test(name));
  if (wasmFiles.length !== 1) throw new Error("The captured site must deliver exactly one standalone Flow Wasm");
  const [name] = wasmFiles;
  const chunks = files.filter(name => /^flow_core-[a-zA-Z0-9_-]+[.]js$/.test(name));
  if (chunks.length !== 1 || !(await readFile(join(assets, chunks[0]), "utf8")).includes(name)) {
    throw new Error("The captured Flow binding chunk must reference its delivered Wasm");
  }
  modulePath = join(assets, name);
}
const root = resolve(import.meta.dirname, "../../../../../../..");
const contract = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🧬️schema/🔣️.json");
const { bridgeExports } = JSON.parse(await readFile(contract, "utf8")).definitions.Protocol.const;
const bytes = await readFile(modulePath);
const wasm = await WebAssembly.compile(bytes);
const exports = WebAssembly.Module.exports(wasm);
const memory = exports.filter(item => item.kind === "memory");
const imports = WebAssembly.Module.imports(wasm).filter(item => item.kind === "memory");
const missing = bridgeExports.filter(name => !exports.some(item => item.kind === "function" && item.name === name));
const sha256 = createHash("sha256").update(bytes).digest("hex");
console.log(JSON.stringify({ module: modulePath, bytes: bytes.length, sha256, memory, memoryImports: imports, missing }));
if (memory.length !== 1 || memory[0].name !== "memory" || missing.length) {
  throw new Error("The standalone Flow browser ABI is incomplete");
}

if (packageRoot) {
  const location = resolve(packageRoot);
  const exact = await readFile(join(location, "flow_core_bg.wasm"));
  if (!bytes.equals(exact)) throw new Error("The browser package initializes different Flow bytes");
  const bindings = await import(pathToFileURL(join(location, "flow_core.js")).href);
  const { createFlowBrowserRuntime } = await import(pathToFileURL(join(location, "🌐️browser", "🟨️.js")).href);
  const source = await bindings.default({ module_or_path: bytes });
  if (!(source.memory instanceof WebAssembly.Memory) || bridgeExports.some(name => typeof source[name] !== "function")) {
    throw new Error("The binding initializer did not return the exact Flow memory and functions");
  }
  const timeLimit = setTimeout(() => {
    console.error("Original Flow session did not settle within 30 seconds");
    process.exit(1);
  }, 30_000);
  try {
    const runtime = await createFlowBrowserRuntime({ source, bindings });
    try {
      const session = runtime.openSession();
      const viewport = await session.viewport().result;
      if (viewport === undefined || viewport === null) throw new Error("Flow session returned no viewport");
      console.log(JSON.stringify({ viewport }));
    } finally {
      await runtime.close();
    }
    if (!runtime.terminalIsEmpty()) throw new Error("Flow session retained native ownership after closing");
  } finally {
    clearTimeout(timeLimit);
  }
}
