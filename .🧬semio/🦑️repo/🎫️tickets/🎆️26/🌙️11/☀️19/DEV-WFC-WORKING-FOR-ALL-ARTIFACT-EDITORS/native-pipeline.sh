#!/bin/bash
# Drives the activate-<variant>-react-prototype dependsOn chain by hand (graph blocked); one heavy build at a time.
ROOT="$HOME/Documents/semio"; T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS"; S="$T/native-step.sh"
OS="🧰️framework/🛍️products/💻️os"; P="$OS/🔨️modules/🔌️plugin"
export SEMIO_BUILD_MODE=prototype
step(){ "$S" "$@"; if ! tail -1 "$T/🗑️generated/$1.log" | grep -q '^rc=0'; then echo "FAILED $1" >> "$T/🗑️generated/pipeline.status"; exit 1; fi; echo "done $1: $(tail -1 "$T/🗑️generated/$1.log")" >> "$T/🗑️generated/pipeline.status"; }
step transpile-build "$P/🌐️browser-bundle/🏗️materialization/🦀️native-transpile" ./📜️script.ts build --host-platform=darwin
step font-assets 🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust ./📜️script.ts build
step fonts $OS/🔨️modules/♾️infinite/📦️packages/🦀️rust ./📜️script.ts fonts
step wasm-surface 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust ./📜️script.ts wasm
step wasm-editor 🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust ./📜️script.ts wasm
step wasm-graph $OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust ./📜️script.ts wasm
step component . "$P/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest "🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust/Cargo.toml"
