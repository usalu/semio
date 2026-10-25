#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d"
while ! [ -f "$D/cargo-recheck-done.txt" ]; do sleep 15; done
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-async --lib publication > "$D/cargo-testpub.log" 2>&1; echo "testpub rc=$?" > "$D/cargo-testpub-done.txt"
