#!/bin/zsh
# 🧩️ Graph-free build of the procedural plugin component into the browser plugin-modules root (prototype), through the repo's own
# `component --manifest` driver under a native-owner-command envelope (same launcher pattern as the Play native fallback).
cd /Users/ueli/Documents/semio || exit 1
TF="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
export SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio SEMIO_BUILD_MODE=prototype
MANIFEST="${1:-🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml}"
exec "$BUN" "$TF/native-launch.ts" . "$BUN" "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest "$MANIFEST"
