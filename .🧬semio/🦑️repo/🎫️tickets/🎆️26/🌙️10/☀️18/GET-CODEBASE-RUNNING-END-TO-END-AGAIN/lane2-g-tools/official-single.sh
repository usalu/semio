#!/bin/sh
# lane2-g: one real artifact consumer through the repo CLI: usage official-single.sh <label> <manifest relative to repo root>
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/lane2-g/official-$1.log"
S=$(date +%s)
SEMIO_BUILD_GRAPH_POLICY=current /usr/bin/time -l repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts" native cargo build --manifest "$2" > "$OUT" 2>&1
echo "exit $? wall $(($(date +%s)-S)) s" >> "$OUT"
