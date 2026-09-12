#!/bin/zsh
# 🧊️ Detached procedural 3d WGPU dev serve (trunk, port 6118) for runtime verification.
# Run under `screen -dmS g3dwgpu` so the serve outlives the agent shell.
#
# Primary path = the `procedural3d-wgpu` launch row (`@semio-tech/framework-os-dev:dev -- generation3d`
# with SEMIO_RENDERER=wgpu), plus `served` so `activatePlaygroundRuntime` is skipped: the plugin wasm is
# the component already staged under `🧑‍💻dev/🔌️plugin-modules`, and this lane must not restage the guest.
#
# Fallback = trunk directly. `TrunkServeScript` runs `checkFrameWorkerCarrierCensus` first, and on
# 2026-09-12 a live peer lane added `globalThis.localStorage` to `🎭️actor/🧵️shard-runtime/🟦️.ts`
# (runtime-diagnostics arming), which the frame Worker bundles — the census forbids that literal, so the
# dev path refuses to start on a bundle that is otherwise current. The fallback runs the same
# `trunk serve --config Trunk.toml --port <port>` the script would have run, after the generated
# `🎞️frame-worker.js` has been refreshed by hand. Delete the fallback once that peer lane lands.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/wgpu-serve.txt"
exec > "$LOG" 2>&1
export NX_DAEMON=false SEMIO_RENDERER=wgpu SEMIO_PLUGIN=generation3d S_OS_PORT=6118 CARGO_PROFILE_WASM_DEV_DEBUG=false
date
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
bun ./📜️script.ts dev generation3d served
echo "DEV_EXIT=$?"
date
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust"
echo "[DEBUG] falling back to trunk serve directly (see header)"
trunk serve --config Trunk.toml --port 6118
echo "TRUNK_EXIT=$?"
date
