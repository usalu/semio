#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
. "$T/nr-env.sh"
LABEL=$1; shift
cd "/c/semio/🧰️framework"
cargo "$@" > "$T/🗑️generated/nr-fw-$LABEL.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/nr-fw-$LABEL.log"
