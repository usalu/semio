import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
const mathDir = fs.readdirSync(pluginExt).find((n) => n.endsWith("math"));
const p = path.join(pluginExt, mathDir, "🦀️component.rs");
let t = fs.readFileSync(p, "utf8");
t = t.replace('ExtensionBundle::new(EXTENSION_ID, EXTENSION_LABEL, "0.1.0")', 'ExtensionBundle::new(EXTENSION_ID, EXTENSION_LABEL, "0.2.0")');
fs.writeFileSync(p, t);
console.log("math bundle version aligned to 0.2.0");

// Verify testkit seeds
const app = path.join(root, sDir, "🔌️plugins", "🌊️flow", "🎛️apps", "🌊️flow", "🦀️component.rs");
const at = fs.readFileSync(app, "utf8");
console.log("testkit has seed", at.includes("install_first_party_light_flow_extensions_for_tests"));
console.log("flow_app calls seed", /flow_app\(\)[\s\S]{0,200}install_first_party/.test(at));
