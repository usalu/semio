import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const extRoot = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
for (const id of ["core","math","text","logic","dictionary","list"]) {
  const dir = fs.readdirSync(extRoot).find((n) => n.endsWith(id));
  const comp = path.join(extRoot, dir, "🦀️component.rs");
  const t = fs.readFileSync(comp, "utf8");
  const issues = [];
  if ((t.match(/fn module_registry/g) || []).length !== 1) issues.push("module_registry count="+((t.match(/fn module_registry/g)||[]).length));
  if (!t.includes("extension_manifest_json")) issues.push("no extension_manifest_json");
  if (!t.includes("ExtensionGuest")) issues.push("no ExtensionGuest");
  if (t.includes("WasmExt")) issues.push("still has WasmExt");
  if (t.includes("standalone-wasm")) issues.push("still standalone-wasm");
  // check for broken build_manifest in extension_manifest_json
  const idx = t.indexOf("pub fn extension_manifest_json");
  const snippet = t.slice(idx, idx+500);
  const proj = path.join(extRoot, dir, "📦️packages", "🦀️rust", "📋️project.json");
  const pj = JSON.parse(fs.readFileSync(proj,"utf8"));
  console.log(JSON.stringify({id, dir, issues, namedInput: pj.namedInputs.default[0], cwd: pj.targets.test.options.cwd, manifestSnippet: snippet.replace(/\s+/g," ").slice(0,200)}, null, 0));
}
// framework extensions remaining
const fw = fs.readdirSync(root).find(n=>n.endsWith("framework"));
const fwExt = path.join(root, fw, "🛍️products","💻️os","🔨️modules","🌊️flow","️️extensions");
const fwExt2 = path.join(root, fw, "🛍️products","💻️os","🔨️modules","🌊️flow",
  fs.readdirSync(path.join(root, fw, "🛍️products","💻️os","🔨️modules","🌊️flow")).find(n=>n.includes("extensions")));
console.log("fw remaining", fs.readdirSync(fwExt2));
