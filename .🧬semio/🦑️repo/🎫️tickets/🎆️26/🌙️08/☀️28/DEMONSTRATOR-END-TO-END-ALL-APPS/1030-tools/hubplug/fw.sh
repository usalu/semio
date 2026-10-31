#!/bin/bash
D="$(dirname "$0")"
"$D/chk.sh" 2 "🧰️framework/Cargo.toml" wasm -p semio-framework-surface-component -p semio-framework-editor-component
"$D/chk.sh" 3 "Cargo.toml" wasm -p semio-framework-os-flow-component -p semio-framework-os-node-graph-component
"$D/chk.sh" 4 "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml" wasm -p semio-s-plugin-playbook-procedural-component
