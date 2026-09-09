#!/bin/zsh
cd /Users/ueli/Documents/semio
export SEMIO_VITE_HMR=0 S_OS_PORT=6013 SEMIO_RENDERER=react SEMIO_PLUGIN=puzzle3d SEMIO_BUILD_MODE=ship
PLUGIN_SCRIPT="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"
DEV_SCRIPT="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"
MANIFEST="✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml"
echo "[direct] support release $(date)"; bun "$PLUGIN_SCRIPT" support release
echo "[direct] materialize release $(date)"; bun "$PLUGIN_SCRIPT" materialize release --manifest "$MANIFEST"
echo "[direct] prepare $(date)"; bun "$DEV_SCRIPT" prepare puzzle3d react release
echo "[direct] activate $(date)"; bun "$DEV_SCRIPT" activate puzzle3d react release
echo "[direct] serve $(date)"; exec bun "$DEV_SCRIPT" serve puzzle3d react release
