#!/usr/bin/env bash
# 🔏️ Slice C1b — publishes the trusted stdio+GIS catalog ONCE into a warm hub data root, so every later
# `verify collab` run seeds it instead of paying two wasm-release component builds plus a default-features
# `cargo build --bin os-hub` per run. `OS_HUB_DATA` must be a realpath: the catalog loader walks it with
# O_NOFOLLOW and macOS `/var` is a symlink.
set -euo pipefail
ROOT=/Users/ueli/Documents/semio
cd "$ROOT/🌎️hub/📦️packages/🦀️rust"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export OS_HUB_DATA="$ROOT/.🧬semio/🌐hub/c1b-warm"
mkdir -p "$OS_HUB_DATA"
exec bun ./📜️script.ts trusted-stdio-gis-bootstrap
