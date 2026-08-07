const fs = require("fs");
const path = require("path");
const fw = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const coreName = fs.readdirSync(fw).find((n) => n.includes("core") && !n.includes("extensions"));
const extsName = fs.readdirSync(fw).find((n) => n.includes("extensions"));
const exts = path.join(fw, extsName);
const brep = fs.readdirSync(exts).find((n) => n.includes("brep"));
const wasm = fs.readdirSync(exts).find((n) => n.includes("wasm"));
console.log({ coreName, extsName, brep, wasm });

const glue = `//! 🌊️ OS flow family glue — wires core and remaining built-in extensions (brep) plus wasm SDK.

extern crate self as flow_core;
extern crate self as flow_extension_brep;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../${coreName}/🦀️component.rs"]
pub mod core;
pub use core::*;

#[path = "."]
pub mod extensions {
  #[path = "../../../${extsName}/${brep}/🦀️component.rs"]
  pub mod brep;

  #[path = "../../../${extsName}/${wasm}/🦀️component.rs"]
  pub mod wasm;
}

pub use extensions::brep::*;
pub use extensions::wasm::*;
`;
const gluePath = path.join(fw, "📦️packages/🦀️rust/📦️glue.rs");
fs.writeFileSync(gluePath, glue);
console.log("wrote glue");

const base = path.join(fw, "📦️packages/🦀️rust");
for (const m of glue.matchAll(/#\[path = "([^"]+)"\]/g)) {
  const p = path.resolve(base, m[1]);
  console.log(fs.existsSync(p) ? "OK" : "MISSING", m[1]);
}

const corePath = path.join(fw, coreName, "🦀️component.rs");
let core = fs.readFileSync(corePath, "utf8");
console.log("BEFORE:\n", core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/)?.[0]);
core = core.replace(
  /pub fn install_builtin_flow_extensions\(registry: &mut neural::Registry\) \{[\s\S]*?\n\}/,
  `pub fn install_builtin_flow_extensions(registry: &mut neural::Registry) {
    flow_extension_brep::register(registry);
}`,
);
if (core.includes("flow_extension_draw")) {
  console.log("WARNING still has flow_extension_draw refs:");
  core.split("\n").forEach((l, i) => {
    if (l.includes("flow_extension_draw")) console.log(i + 1 + ":" + l);
  });
}
fs.writeFileSync(corePath, core);
console.log("AFTER:\n", core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/)?.[0]);
