#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg2-d-tools"
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
printf 'semio-framework-os-store-command\t🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/📦️packages/🦀️rust\n' > "$D/crates-recheck-root.txt"
sh "$T/check.sh" crates-recheck-root.txt > "$D/cargo-recheck3.log" 2>&1; echo "recheck3 rc=$?" > "$D/chain2-recheck.txt"
