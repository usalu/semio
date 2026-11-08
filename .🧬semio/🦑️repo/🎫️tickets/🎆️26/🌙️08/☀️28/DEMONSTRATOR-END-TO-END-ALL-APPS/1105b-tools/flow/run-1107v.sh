#!/bin/bash
# serial chain; a run is retried every 10 min (max 24) while the first "could not compile" crate is foreign to flow
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; T="$C/1105b-tools/flow/tst.sh"
while [ ! -f "$G/flow-run-1107u.rc" ]; do sleep 20; done
try(){ n=$1; shift; for k in $(seq 1 24); do "$@"; [ "$(cat "$G/$n.rc")" = 0 ] && return; cp "$G/$n.txt" "$G/$n-try$k.txt"; c=$(/usr/bin/grep -a -m1 "could not compile \`" "$G/$n.txt" | sed 's/.*`\(.*\)`.*/\1/'); case "$c" in semio-os-flow*|semio-s-artifact-flow*) return;; esac; sleep 600; done; }
try flow-181 "$T" flow-181 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
try flow-182 "$T" flow-182 Cargo.toml -p semio-os-flow-host-read --lib
try flow-183 "$T" flow-183 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained --lib -- --test-threads=1
try flow-184 "$T" flow-184 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-unit-tests --lib
try flow-185 "$C/1105b-tools/chk.sh" flow-185 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" wasm -p semio-s-artifact-flow-flow-editor -p semio-s-artifact-flow-flow
try flow-186 "$T" flow-186 "🧰️framework/Cargo.toml" -p semio-framework-pack-json --tests
try flow-187 "$C/1105b-tools/chk.sh" flow-187 Cargo.toml native -p semio-os-flow-drawing
try flow-188 "$C/1105b-tools/chk.sh" flow-188 Cargo.toml wasm -p semio-os-flow-wasm
echo done > "$G/flow-run-1107v.rc"
