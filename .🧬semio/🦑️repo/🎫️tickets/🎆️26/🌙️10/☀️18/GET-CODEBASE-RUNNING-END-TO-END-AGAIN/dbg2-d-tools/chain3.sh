#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg2-d-tools"
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
rm -f "$D"/chain3-*.txt
sh "$T/check.sh" crates-root.txt > "$D/cargo-root4.log" 2>&1; echo "root4 rc=$?" > "$D/chain3-root.txt"
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-async --lib publication > "$D/cargo-testpub3.log" 2>&1; echo "testpub rc=$?" > "$D/chain3-testpub.txt"
