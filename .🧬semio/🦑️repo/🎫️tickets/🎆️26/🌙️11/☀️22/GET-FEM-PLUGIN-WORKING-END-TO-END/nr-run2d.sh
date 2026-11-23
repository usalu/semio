#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
. "$T/nr-env.sh"
L=$1; shift
export RUST_MIN_STACK=268435456
/c/t/nrtmp/snap2d.exe "$@" > "$T/🗑️generated/nr-run2d-$L.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/nr-run2d-$L.log"
