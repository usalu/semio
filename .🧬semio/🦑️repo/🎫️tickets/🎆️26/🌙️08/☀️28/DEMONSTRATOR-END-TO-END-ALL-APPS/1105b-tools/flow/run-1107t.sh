#!/bin/bash
# serial chain; a run is retried every 10 min (max 24) while the first "could not compile" crate is foreign to flow
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; T="$C/1105b-tools/flow/tst.sh"
while [ ! -f "$G/flow-go" ]; do sleep 20; done
try(){ n=$1; shift; for k in $(seq 1 24); do "$@"; [ "$(cat "$G/$n.rc")" = 0 ] && return; cp "$G/$n.txt" "$G/$n-try$k.txt"; c=$(/usr/bin/grep -a -m1 "could not compile \`" "$G/$n.txt" | sed 's/.*`\(.*\)`.*/\1/'); case "$c" in semio-os-flow*|semio-s-artifact-flow*) return;; esac; sleep 600; done; }
try flow-161 "$T" flow-161 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
try flow-162 "$T" flow-162 Cargo.toml -p semio-os-flow-host-read --lib
try flow-163 "$T" flow-163 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained --lib -- --test-threads=1
try flow-164 "$T" flow-164 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-unit-tests --lib
try flow-165 "$C/1105b-tools/chk.sh" flow-165 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" wasm -p semio-s-artifact-flow-flow-editor -p semio-s-artifact-flow-flow
try flow-166 "$T" flow-166 "🧰️framework/Cargo.toml" -p semio-framework-pack-json --tests
echo done > "$G/flow-run-1107u.rc"
