#!/usr/bin/env bash
# usage: cycle.sh <slug>   (baseline "before" must exist)
REPO=/Users/ueli/Documents/semio
cd "$REPO"
C="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gocanon-logs"
slug=$1
dir=$(awk -v s="$slug" '$2==s {print $1}' "$C/gocanon/modules.txt")
bun "$C/gocanon/convert.ts" "$slug" apply || exit 1
python3 "$C/gocanon/registry.py" "$G/$slug-additions.json" || exit 1
[ -f "$dir/📜️script.ts" ] && python3 "$C/gocanon/patch-script.py" "$dir/📜️script.ts"
"$C/gocanon/all.sh" after "$slug"
for k in results list list-exh; do diff "$G/$slug-before-$k.txt" "$G/$slug-after-$k.txt" > "$G/$slug-diff-$k.txt" && echo "  $k identical" || echo "  $k DIFF ($(wc -l < "$G/$slug-diff-$k.txt") lines)"; done
