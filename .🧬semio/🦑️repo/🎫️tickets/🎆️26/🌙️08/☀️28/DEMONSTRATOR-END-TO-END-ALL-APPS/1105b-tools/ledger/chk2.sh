#!/bin/bash
# usage: chk2.sh <label> <cargo check args...> — native check of the given -p packages in the framework workspace
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
L=$1; shift
exec "$C/1105b-tools/chk.sh" "$L" 🧰️framework/Cargo.toml native "$@"
