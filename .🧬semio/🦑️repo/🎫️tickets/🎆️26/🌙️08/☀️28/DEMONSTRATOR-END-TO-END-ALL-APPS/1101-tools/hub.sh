#!/bin/bash
# usage: hub.sh <label>  — wasm check of every demonstrator hub component
exec "$(dirname "$0")/chk.sh" "$1" "🌎️hub/Cargo.toml" wasm -p semio-hub-cad-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component -p semio-hub-energy-component -p semio-hub-fem-component -p semio-hub-flow-component -p semio-hub-gis-component -p semio-hub-playbook-component -p semio-hub-procedural-component -p semio-hub-process-component -p semio-hub-puzzle -p semio-hub-puzzle-component -p semio-hub-sourcing-component
