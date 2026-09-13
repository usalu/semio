#!/bin/zsh
# 🧊️ Detached procedural 3d WGPU browser serve (port 6118) for runtime verification.
# Run under `screen -dmS g3dwgpu` so the serve outlives the agent shell:
#
#   screen -dmS g3dwgpu "<ticket>/📜️serve-generation3d-wgpu.sh"
#
# 🌐️ 2026-09-13: this now starts the ONE browser listener that owns the WGPU target —
# `🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve generation3d dev`, the Vite host a peer stood up at
# 02:13–02:29 to replace the deleted `trunk serve` (`Trunk.toml`/`🌐️.html`/`📋️project.json` are gone
# from `📦️packages/🦀️rust`, so the old fallback could not start at all). It serves the Nx-COMPLETED
# artifacts and compiles nothing itself:
#
#   /renderer-modules/wgpu  ← 📦️packages/🦀️rust/dist/wasm-dev   (nx run @semio-tech/framework-renderer-wgpu:wasm)
#   /🚀️boot.js             ← 🚀️browser-boot/🤖️generated        (…:generate-browser-boot)
#   /🎞️frame-worker.js      ← 🎞️frame-worker/🤖️generated        (…:generate-frame-worker)
#   <plugin module routes>  ← 🧑‍💻dev/🔌️plugin-modules            (…:activate-generation3d-wgpu-dev)
#
# so it refuses to boot when any of them is missing rather than serving a stale page. Rebuild the
# renderer wasm with `bunx nx run @semio-tech/framework-renderer-wgpu:wasm` and simply RELOAD the page;
# the serve never has to restart for new wasm.
#
# The registered equivalent is the `🎮️serve🧩️generation3d🧊️wgpu dev` launch row
# (`@semio-tech/framework-os-dev:serve-generation3d-wgpu-dev`), which additionally runs
# `activate-generation3d-wgpu-dev` first. This script deliberately does NOT: the procedural guest is
# the component already staged under `🧑‍💻dev/🔌️plugin-modules` and this lane must not restage it.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/wgpu-serve.txt"
exec > "$LOG" 2>&1
export NX_DAEMON=false SEMIO_RENDERER=wgpu SEMIO_PLUGIN=generation3d S_OS_PORT=6118 CARGO_PROFILE_WASM_DEV_DEBUG=false
date
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server"
bun ./📜️script.ts serve generation3d dev
echo "SERVE_EXIT=$?"
date
