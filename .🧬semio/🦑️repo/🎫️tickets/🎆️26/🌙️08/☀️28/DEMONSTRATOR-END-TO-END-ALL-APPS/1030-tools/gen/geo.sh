#!/bin/bash
# usage: geo.sh <N> — wasm lib check of framework-3d + 3d io-geometry
exec "$(dirname "$0")/chk.sh" "$1" "🌎️hub/Cargo.toml" wasm -p semio-framework-3d -p semio-s-artifact-procedural-3d-io-geometry
