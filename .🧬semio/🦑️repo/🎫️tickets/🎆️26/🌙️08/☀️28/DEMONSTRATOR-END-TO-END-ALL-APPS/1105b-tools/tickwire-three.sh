#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools/chk.sh"
$T tickwire-16 "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-os-inference
$T tickwire-17 "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-os-infinite-board-schema
$T tickwire-18 "🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-framework-tool-typing-edit
echo done > "$C/🗑️generated/1105b/tickwire-three.rc"
