#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
. "$T/nr-env.sh"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
cargo test -j 3 -p semio-s-artifact-fem-2d-snapshot-tests --lib --no-run --message-format short > "$T/🗑️generated/nr-build2d-$1.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/nr-build2d-$1.log"
