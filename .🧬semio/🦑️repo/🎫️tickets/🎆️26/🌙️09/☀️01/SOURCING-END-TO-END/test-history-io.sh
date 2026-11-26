#!/bin/sh
# 🧪️ Locked admitted cargo run of the curated io format laws of the sourcing curation unit target.
# Usage: sh test-history-io.sh   (run from the workspace root)
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
lock="$T/🗑️generated/build.lock"
until mkdir "$lock" 2>/dev/null; do sleep 30; done
trap 'rmdir "$lock"' EXIT
sh "$T/native-cargo.sh" curation-io2 "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation" test -p semio-s-artifact-sourcing-curation --test curation_unit curated_state -- --test-threads=1
