#!/bin/bash
# usage: run-1107r.sh <wait-marker> ; serial retry chain: a run whose log shows a foreign planning/source refusal is retried after 4 min (max 30 tries)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; T="$C/1105b-tools/flow/tst.sh"
while [ ! -f "$G/$1" ]; do sleep 20; done
try(){ n=$1; shift; for k in $(seq 1 30); do "$@"; rc=$(cat "$G/$n.rc"); [ "$rc" = 0 ] && return; if /usr/bin/grep -aqE "Source refused before compiler|multiple times with different names|could not find|failed to select a version|failed to load manifest|cargo:planning-capture failed|not enough free memory.*timeout" "$G/$n.txt" && ! /usr/bin/grep -aq "^error\[E" "$G/$n.txt"; then cp "$G/$n.txt" "$G/$n-try$k.txt"; sleep 240; else return; fi; done; }
try flow-141 "$T" flow-141 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
try flow-142 "$T" flow-142 Cargo.toml -p semio-os-flow-host-read --lib
try flow-143 "$T" flow-143 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained --lib -- --test-threads=1
try flow-144 "$T" flow-144 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-unit-tests --lib
try flow-145 "$C/1105b-tools/chk.sh" flow-145 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" wasm -p semio-s-artifact-flow-flow-editor -p semio-s-artifact-flow-flow
try flow-146 "$T" flow-146 "🧰️framework/Cargo.toml" -p semio-framework-pack-json --tests
echo done > "$G/flow-run-1107s.rc"
