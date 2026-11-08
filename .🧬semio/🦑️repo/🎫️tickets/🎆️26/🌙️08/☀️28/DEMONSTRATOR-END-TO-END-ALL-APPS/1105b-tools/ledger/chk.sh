#!/bin/bash
# usage: chk.sh <label> — native check of the three ledger-owned value crates
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
exec "$C/1105b-tools/chk.sh" "$1" 🧰️framework/Cargo.toml native $2 -p semio-framework-value-equality -p semio-framework-value-paged-text -p semio-framework-value-clone-source-testing
