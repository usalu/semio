#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/🔌️plugins --do-not-follow '^(🧰️framework|🌎️hub|♻️mit-bestand|✏️s/(?!🔌️plugins))' --exclude '(^|/)node_modules(/|$)|(^|/)target(/|$)' --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$1"
