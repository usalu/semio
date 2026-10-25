#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg2-d-tools"
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
sh "$T/check.sh" crates-root.txt > "$D/cargo-root2.log" 2>&1; echo "root2 rc=$?" > "$D/cargo-root2-done.txt"
