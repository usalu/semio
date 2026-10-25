#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg2-d-tools"
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
while ! grep -q root "$D/cargo-done.txt" 2>/dev/null; do sleep 15; done
sh "$T/check.sh" crates-recheck.txt --manifest-path 🧰️framework/Cargo.toml > "$D/cargo-recheck.log" 2>&1; echo "recheck rc=$?" > "$D/cargo-recheck-done.txt"
