#!/bin/bash
D=$(dirname "$0")
MANIFEST="🧰️framework/Cargo.toml" "$D/wasm2.sh" 19 --lib -p semio-framework-2d-booleans -p semio-framework-2d-regions -p semio-framework-pixels -p semio-framework-surface -p semio-framework-editor
"$D/wasm2.sh" 20 --lib -p semio-framework-os-flow-drawing -p semio-framework-os-node-graph -p semio-framework-os-kernel -p semio-framework-os-store-initialization-replay
