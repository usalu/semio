import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const uiCargo = readFileSync("/tmp/semio-ui-cargo.txt", "utf8").trim();
let cargo = readFileSync(uiCargo, "utf8");
cargo = cargo.replace(
  "[target.'cfg(not(target_os = \"wasi\"))'..dependencies]",
  "[target.'cfg(not(target_os = \"wasi\"))'.dependencies]",
);
writeFileSync(uiCargo, cargo);
const lines = cargo.split("\n");
console.log("cargo target lines:");
lines.forEach((l, i) => {
  if (l.includes("winit") || l.includes("wasi") || l.startsWith("[target.")) console.log(`${i + 1}:${l}`);
});

const glue = readFileSync("/tmp/semio-ui-wgpu-glue.txt", "utf8").trim();
const gpuPath = join(dirname(glue), "🦀️gpu.rs");
let gpu = readFileSync(gpuPath, "utf8");
if (!gpu.includes('#[cfg(not(target_os = "wasi"))]\n    pub async fn from_window')) {
  gpu = gpu.replace(
    "    pub async fn from_window(window: std::sync::Arc<winit::window::Window>) -> Result<Self, String> {",
    '    #[cfg(not(target_os = "wasi"))]\n    pub async fn from_window(window: std::sync::Arc<winit::window::Window>) -> Result<Self, String> {',
  );
}

// Gate both schedule_frame overloads that take winit Window
gpu = gpu.replace(
  '#[cfg(target_arch = "wasm32")]\npub fn schedule_frame(window: &winit::window::Window, callback: impl FnMut() + \'static) {',
  '#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]\npub fn schedule_frame(window: &winit::window::Window, callback: impl FnMut() + \'static) {',
);
if (!gpu.includes('#[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]\npub fn schedule_frame')
  && gpu.includes("pub fn schedule_frame(window: &winit::window::Window, _callback")) {
  gpu = gpu.replace(
    "pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {",
    '#[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]\npub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + \'static) {',
  );
}

writeFileSync(gpuPath, gpu);
console.log("gpu schedule/from_window cfgs:");
gpu.split("\n").forEach((l, i) => {
  if (/from_window|schedule_frame|cfg\(/.test(l) && /wasi|wasm32|from_window|schedule_frame/.test(l)) {
    console.log(`${i + 1}:${l}`);
  }
});

// Also ensure glue schedule_frame reexport is gated
let g = readFileSync(glue, "utf8");
const before = g;
g = g.replace(
  '#[cfg(feature = "wgpu-engine")]\npub use gpu::schedule_frame;',
  '#[cfg(all(feature = "wgpu-engine", not(target_os = "wasi")))]\npub use gpu::schedule_frame;',
);
if (g !== before) {
  writeFileSync(glue, g);
  console.log("glue schedule_frame reexport gated");
}
