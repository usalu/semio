#!/usr/bin/env bash
cd /home/deck/semio
R=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/opus1122-react-run.sh"
S="🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"; E="🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust"; N="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust"
"$R" wasm-surface "$S" @semio-tech/framework-surface-rs:wasm ./📜️script.ts wasm
"$R" wasm-editor "$E" @semio-tech/framework-editor-rs:wasm ./📜️script.ts wasm
"$R" wasm-nodegraph "$N" @semio-tech/framework-os-node-graph-rs:wasm ./📜️script.ts wasm
