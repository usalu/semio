#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
out="$1"; shift
node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" --exclude '(^|/)node_modules(/|$)' --do-not-follow '^(🧰️framework|✏️s|🌎️hub|♻️mit-bestand)/' --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$out"
