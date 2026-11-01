#!/bin/bash
# usage: hub.sh <N> — wasm check of procedural hub component (+ extra -p args)
D="$(dirname "$0")"; N=$1; shift
exec "$D/../chk.sh" "gen-hub$N" "🌎️hub/Cargo.toml" wasm -p semio-hub-procedural-component "$@"
