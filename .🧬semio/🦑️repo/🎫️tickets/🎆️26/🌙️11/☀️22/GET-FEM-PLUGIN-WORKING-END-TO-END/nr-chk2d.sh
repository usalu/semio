#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
. "$T/nr-env.sh"
L=$1; shift
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
cargo "$@" > "$T/🗑️generated/nr-chk-$L.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/nr-chk-$L.log"
