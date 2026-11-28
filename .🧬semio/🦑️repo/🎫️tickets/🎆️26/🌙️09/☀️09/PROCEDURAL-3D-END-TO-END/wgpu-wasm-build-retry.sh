#!/bin/zsh
# 🔁️ Retries the wgpu wasm build while the host memory guard cancels it; stops on success or on any other failure.
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END
for n in 1 2 3 4 5 6; do
  "$T/wgpu-native-wasm-build.sh" > "$T/🗑️generated/wgpu-wasm-build.txt" 2>&1
  grep -q BUILD_DONE "$T/🗑️generated/wgpu-wasm-build.txt" && { echo RETRY_DONE >> "$T/🗑️generated/wgpu-wasm-build.txt"; exit 0; }
  grep -qE "warden|Wächter" "$T/🗑️generated/wgpu-wasm-build.txt" || break
  sleep 120
done
echo RETRY_FAILED >> "$T/🗑️generated/wgpu-wasm-build.txt"
