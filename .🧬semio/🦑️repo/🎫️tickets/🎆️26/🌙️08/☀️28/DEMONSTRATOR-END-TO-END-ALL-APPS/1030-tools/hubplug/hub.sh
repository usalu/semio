#!/bin/bash
exec "$(dirname "$0")/chk.sh" "$1" "🌎️hub/Cargo.toml" wasm -p semio-hub-playbook-component -p semio-hub-puzzle-component -p semio-hub-process-component -p semio-hub-sourcing-component
