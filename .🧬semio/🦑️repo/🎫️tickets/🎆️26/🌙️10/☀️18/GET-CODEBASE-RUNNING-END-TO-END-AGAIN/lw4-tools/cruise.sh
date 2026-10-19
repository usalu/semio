#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$1"
mkdir -p "$C/🗑️generated/lw4-run"
find 🌎️hub \( -name node_modules -o -name target -o -name .cache -o -name '🗑️generated' \) -prune -o -type f \( -name '*.ts' -o -name '*.tsx' -o -name '*.js' -o -name '*.mjs' -o -name '*.cjs' \) -print > "$C/🗑️generated/lw4-run/files.txt"
node node_modules/dependency-cruiser/bin/dependency-cruise.mjs --do-not-follow '^(🧰️framework|✏️s|♻️mit-bestand)/' --config "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type json --output-to "$out" ${(f)"$(<$C/🗑️generated/lw4-run/files.txt)"}
