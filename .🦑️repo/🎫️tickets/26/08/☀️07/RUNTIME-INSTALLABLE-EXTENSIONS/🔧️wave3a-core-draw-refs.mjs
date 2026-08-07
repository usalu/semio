import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const flowFw = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const core = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs");
const t = fs.readFileSync(core, "utf8");
const lines = t.split(/\n/);
lines.forEach((l,i)=>{
  if (/flow_extension_draw|install_builtin_flow_extensions|install_first_party_light|flow_extension_brep::register/.test(l))
    console.log((i+1)+":"+l);
});
console.log("--- install_builtin ---");
const m = t.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/);
console.log(m?.[0]);
console.log("--- helper present", t.includes("fn install_first_party_light_flow_extensions_for_tests"));
