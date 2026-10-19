#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
out="$1"
OS="🧰️framework/🛍️products/💻️os"; REPO="🧰️framework/🛍️products/🦑️repo"
node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$OS" "$REPO/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests" "$REPO/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🧪️tests" --include-only "^($OS|$REPO)/" --do-not-follow "(^|/)node_modules/|^$REPO/" --exclude "(^|/)node_modules/" --config "$REPO/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$out"
