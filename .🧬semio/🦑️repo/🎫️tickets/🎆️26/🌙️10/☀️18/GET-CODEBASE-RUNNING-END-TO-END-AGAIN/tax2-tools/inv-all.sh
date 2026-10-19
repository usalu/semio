#!/bin/zsh
# usage: inv-all.sh <tag> <scope>...
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag=$1; shift
for s in "$@"; do bun "$C/tax-tools/inventory.ts" "$s" "$C/🗑️generated/tax2/inv-$tag-$s.json" 2> "$C/🗑️generated/tax2/inv-$tag-$s.log"; done
echo done > "$C/🗑️generated/tax2/inv-$tag.done"
