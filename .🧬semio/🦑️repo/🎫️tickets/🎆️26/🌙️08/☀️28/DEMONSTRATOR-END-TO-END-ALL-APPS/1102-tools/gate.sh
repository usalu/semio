#!/bin/bash
# usage: gate.sh <label>  — wasm lib check of the framework crates the panes share
exec "$(dirname "$0")/chk.sh" "$1" "🌎️hub/Cargo.toml" wasm --lib -p semio-framework-value -p semio-framework-value-retained-clone -p semio-framework-value-native-owned -p semio-framework-job -p semio-framework-os-store-replay-operation -p semio-framework-plugin-reactor-turn -p semio-framework-os -p semio-framework-tool-machine -p semio-framework-tool-run-entities -p semio-framework-os-kernel-neural-engine-model -p semio-framework-os-infinite-board-schema
