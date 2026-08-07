import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
console.log("flowFw listing", fs.readdirSync(flowFw));
const coreDir = fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions"));
console.log("coreDir listing", fs.readdirSync(path.join(flowFw, coreDir)));
const extDir = fs.readdirSync(flowFw).find((n) => n.includes("extensions"));
console.log("fw ext listing", fs.readdirSync(path.join(flowFw, extDir)));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
console.log("plugin ext listing", fs.readdirSync(pluginExt));

const core = fs.readFileSync(path.join(flowFw, coreDir, "🦀️component.rs"), "utf8");
console.log("--- install_builtin ---\n", core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/)?.[0]);
console.log("flow_extension_brep refs", (core.match(/flow_extension_brep::/g)||[]).length);
console.log("helper present", core.includes("fn install_first_party_light_flow_extensions_for_tests"));
// helper body - does it still call flow_extension_brep?
const helper = core.match(/fn install_first_party_light_flow_extensions_for_tests[\s\S]*?\n    \}/)?.[0];
console.log("--- helper ---\n", helper);
