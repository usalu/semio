#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
out="$1"; shift
inc="^($(printf '%s|' "$@" | sed 's/|$//'))"
node --max-old-space-size=8000 node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" --exclude '(^|/)(dist|target|pkg|node_modules)/' --include-only "$inc" \
  --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$out"
