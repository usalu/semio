#!/bin/sh
# usage: lock-then-activate.sh <name>: refresh the hub lock through the repo cargo ingress, then run the real demonstrator activation on Bazel
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1
for m in "🌎️hub/Cargo.toml" "Cargo.toml"; do
SEMIO_LANGUAGE=en nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --manifest-path "$m" --workspace --offline >> "$C/🗑️generated/slice-demo2/$n.lock.log" 2>&1
echo "lock $m rc=$?" >> "$C/🗑️generated/slice-demo2/$n.lock.log"
done
SEMIO_LANGUAGE=en sh "$C/slice-demo2-tools/run.sh" "$n" --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype
