#!/bin/bash
# usage: hub.sh <N> [extra -p args] — procedural hub component check under the gen lane prefix
exec "$(dirname "$0")/chk.sh" "$1" "🌎️hub/Cargo.toml" wasm -p semio-hub-procedural-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component
