#!/bin/sh
# 🧪️ Locked admitted cargo run of the history/import-export Rust tests: the curated io format round trips and the checkout-without-id law.
# Usage: sh test-history.sh   (run from the workspace root)
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
lock="$T/🗑️generated/build.lock"
until mkdir "$lock" 2>/dev/null; do sleep 30; done
trap 'rmdir "$lock"' EXIT
sh "$T/native-cargo.sh" checkout-law "." test -p semio-plugin-tests-14-2 --lib checkout_checkpoint_without_an_id -- --test-threads=1
