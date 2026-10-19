#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$1"; shift
P="🧰️framework/🛍️products"
node --max-old-space-size=12000 node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" \
  --exclude '(^|/)(dist|target|pkg|node_modules)/' \
  --do-not-follow "^(?!$P/(🦑️repo|❓️quiz|🎤️presentation|🐾️pets|📓️print|🖥️server)/)" \
  --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$out"
