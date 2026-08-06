import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const glue = readFileSync("/tmp/semio-ui-wgpu-glue.txt", "utf8").trim();
const dir = dirname(glue);

// Fix duplicate cfg on gpu schedule_frame native
const gpuPath = join(dir, "🦀️gpu.rs");
let gpu = readFileSync(gpuPath, "utf8");
gpu = gpu.replace(
  `#[cfg(not(target_arch = "wasm32"))]
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]
pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {`,
  `#[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]
pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {`,
);
writeFileSync(gpuPath, gpu);

const cursorPath = join(dir, "🦀️cursor.rs");
let cursor = readFileSync(cursorPath, "utf8");
console.log("--- cursor winit lines ---");
cursor.split("\n").forEach((l, i) => {
  if (/winit|cfg\(/.test(l)) console.log(`${i + 1}:${l}`);
});

// If winit is referenced without cfg in signatures/types at module level, wrap import
if (cursor.includes("winit::") && !cursor.includes('cfg(not(target_os = "wasi"))\nuse winit') && !cursor.includes("extern crate winit")) {
  // no top-level use; references are fully qualified. Ensure apply_window_cursor body still compiles by keeping cfg on fn.
}

const hostPath = join(dir, "🦀️host.rs");
let host = readFileSync(hostPath, "utf8");
console.log("--- host first 30 lines ---");
console.log(host.split("\n").slice(0, 30).join("\n"));

writeFileSync(gpuPath, gpu);
console.log("gpu deduped");
