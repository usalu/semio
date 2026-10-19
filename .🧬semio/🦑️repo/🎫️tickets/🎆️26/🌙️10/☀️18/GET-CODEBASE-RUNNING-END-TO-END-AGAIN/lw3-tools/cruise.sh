#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="${1:-$C/🗑️generated/lw3/cruise.json}"
NODE_OPTIONS="--max-old-space-size=12000 ${LW3_PRELOAD:+--require=$LW3_PRELOAD}" node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s --include-only '^✏️s/' --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$out"
