#!/bin/bash
D="$(dirname "$0")"
"$D/chk.sh" 18 "🌎️hub/Cargo.toml" wasm -p semio-hub-playbook-component
"$D/chk.sh" 19 "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml" wasm -p semio-s-plugin-playbook-procedural-component
