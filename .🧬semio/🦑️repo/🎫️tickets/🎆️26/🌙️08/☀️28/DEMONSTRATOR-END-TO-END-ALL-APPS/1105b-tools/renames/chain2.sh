#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
B="🧰️framework/🛍️products/💻️os/🔨️modules"
"$C/1105b-tools/chk.sh" renames-13 "$B/🔌️plugin/📜️declaration/🧩️app/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-plugin-decl-app >/dev/null 2>&1
"$C/1105b-tools/chk.sh" renames-14 "$B/🚪️io/📦️packages/🦀️rust/Cargo.toml" native -p semio-os-io >/dev/null 2>&1
"$C/1105b-tools/chk.sh" renames-15 "$B/🌉️mcp/🔬️probe-document/📦️packages/🦀️rust/Cargo.toml" native -p semio-os-mcp-probe-document >/dev/null 2>&1
touch "$C/🗑️generated/1105b/renames-chain2-done"
