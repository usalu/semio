#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg2-d-tools"
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
sh "$T/check.sh" crates-fw.txt --manifest-path 🧰️framework/Cargo.toml > "$D/cargo-fw.log" 2>&1; echo "fw rc=$?" > "$D/cargo-done.txt"
sh "$T/check.sh" crates-root.txt > "$D/cargo-root.log" 2>&1; echo "root rc=$?" >> "$D/cargo-done.txt"
