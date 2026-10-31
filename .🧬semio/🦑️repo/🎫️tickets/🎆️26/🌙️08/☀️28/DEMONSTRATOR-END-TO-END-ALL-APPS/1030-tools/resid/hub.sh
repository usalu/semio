#!/bin/bash
# usage: hub.sh <N>  — the hub batch of matrix/run.sh under own prefix
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
exec "$(dirname "$0")/chk.sh" "$1" "🌎️hub/Cargo.toml" wasm -p semio-hub-cad-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component -p semio-hub-energy-component -p semio-hub-fem-component -p semio-hub-flow-component -p semio-hub-gis-component -p semio-hub-playbook-component -p semio-hub-procedural-component -p semio-hub-process-component -p semio-hub-puzzle -p semio-hub-puzzle-component -p semio-hub-sourcing-component
