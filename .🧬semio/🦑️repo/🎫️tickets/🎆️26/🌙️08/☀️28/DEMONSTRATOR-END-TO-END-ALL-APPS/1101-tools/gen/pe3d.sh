#!/bin/bash
D="$(dirname "$0")"; exec "$D/../chk.sh" "gen-pe3d$1" "🌎️hub/Cargo.toml" wasm --lib -p semio-s-artifact-procedural-3d-preview-eval
