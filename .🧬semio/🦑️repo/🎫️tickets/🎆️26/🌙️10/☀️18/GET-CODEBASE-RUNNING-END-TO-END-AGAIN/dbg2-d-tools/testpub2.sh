#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
rm -f "$D/testpub4-done.txt"
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-async-tests --lib publication -- --nocapture > "$D/cargo-testpub4.log" 2>&1; echo "testpub4 rc=$?" > "$D/testpub4-done.txt"
