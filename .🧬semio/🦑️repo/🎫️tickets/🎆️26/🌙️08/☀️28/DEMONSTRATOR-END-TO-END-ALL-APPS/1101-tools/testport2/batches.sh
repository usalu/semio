#!/bin/bash
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
p(){ for c in "$@"; do printf -- '-p semio-framework-%s ' "$c"; done; }
$C/1101-tools/testport/t.sh p2-b1 Cargo.toml $(p os-infinite-board-engine plugin-time-travel plugin-typing-edit)
$C/1101-tools/testport/t.sh p2-b2 Cargo.toml $(p plugin-guest-contract plugin-publication-mounted plugin-reactor-jobs)
$C/1101-tools/testport/t.sh p2-b3 Cargo.toml $(p plugin-retained-command plugin-instance-jobs)
$C/1101-tools/testport/t.sh p2-b4 Cargo.toml $(p plugin-instance-render plugin-instance-dispatch)
