#!/bin/bash
# one native cycle: pipeline law, static fuel law, Resolve dispatch laws (retained lib + unit), JSON writer law
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools/flow/tst.sh"
"$T" flow-r1 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
"$T" flow-r2 Cargo.toml -p semio-os-kernel-neural-engine-budgeted --lib -- every_retained_evaluation_action_charges_at_least_one_fuel
"$T" flow-r3 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained --lib
"$T" flow-r4 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-unit-tests --lib
"$T" flow-r5 Cargo.toml -p semio-framework-pack-json --lib
echo done > "$C/🗑️generated/1105b/flow-run-all.rc"
