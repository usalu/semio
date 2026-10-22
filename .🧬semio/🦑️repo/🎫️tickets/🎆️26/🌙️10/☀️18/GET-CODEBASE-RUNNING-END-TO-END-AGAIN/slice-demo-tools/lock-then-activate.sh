#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --manifest-path "🌎️hub/Cargo.toml" --workspace --offline > "$C/🗑️generated/slice-demo/$n.lock.log" 2>&1
sh "$C/slice-demo-tools/run.sh" "$n" --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype
