import fs from "fs";
import path from "path";

const FLOW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const mods = [
  "📄️document",
  "📚️catalogue",
  "📔️registry",
  "🌉️bridge",
  "🖥️host",
  "🖍️drawing",
  "🌉️wasm",
  "🌿️vcs",
];

for (const dir of mods) {
  const p = path.join(FLOW, dir, "🦀️component.rs");
  let text = fs.readFileSync(p, "utf8");
  if (!text.includes("use crate::dag;")) {
    text = text.replace(
      "use crate::dag::{",
      "use crate::dag;\nuse crate::dag::{",
    );
    fs.writeFileSync(p, text);
    console.log("added use crate::dag in", dir);
  } else {
    console.log("already has", dir);
  }
}

// Also check ui_wgpu usages and other bare paths that assumed same-file scope
// Catalogue uses ui_wgpu - was that imported? Check original - might come from somewhere in the big file later or prelude missing
for (const dir of mods) {
  const p = path.join(FLOW, dir, "🦀️component.rs");
  const text = fs.readFileSync(p, "utf8");
  const needs = [];
  if (/\bui_wgpu::/.test(text) && !/use\s+.*ui_wgpu/.test(text)) needs.push("ui_wgpu");
  if (/\bsemio_s_2d::/.test(text) && !/use\s+.*semio_s_2d/.test(text) && !/semio_s_2d::/.test(text.split("\n").slice(0,40).join("\n"))) {
    // used with full path already OK
  }
  if (/\bwasm_bindgen::/.test(text)) needs.push("wasm_bindgen(cfg)");
  if (/\bjs_sys::/.test(text)) needs.push("js_sys");
  console.log(dir, "extra needs?", needs.join(",") || "none", "ui_wgpu=", /\bui_wgpu::/.test(text));
}
