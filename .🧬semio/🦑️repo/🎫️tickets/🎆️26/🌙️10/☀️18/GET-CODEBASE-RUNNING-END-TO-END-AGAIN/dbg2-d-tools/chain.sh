#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg2-d-tools"
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
rm -f "$D"/chain-*.txt
sh "$T/check.sh" crates-root.txt > "$D/cargo-root3.log" 2>&1; echo "root3 rc=$?" > "$D/chain-root.txt"
sh "$T/check.sh" crates-recheck.txt --manifest-path 🧰️framework/Cargo.toml > "$D/cargo-recheck2.log" 2>&1; echo "recheck2 rc=$?" > "$D/chain-recheck.txt"
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-async --lib publication > "$D/cargo-testpub2.log" 2>&1; echo "testpub rc=$?" > "$D/chain-testpub.txt"
