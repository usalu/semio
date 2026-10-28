#!/bin/sh
# 🧪 [pinpub] runs every Nx graph store test file (and the new sparse law) from <root>; usage: neighbors.sh <root> <tag>  -> one summary line per file in C/🗑️generated/pinpub/nb-<tag>.txt
R="$1"; tag="$2"; C="$(cd "$(dirname "$0")/.." && pwd)"; out="$C/🗑️generated/pinpub/nb-$tag.txt"; : > "$out"
cd "$R" || exit 2
find "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests" -name '🟦️.ts' | sort | while read -r f; do
 sh "$C/pinpub-tools/run1.sh" "$R" "$f" "$C/🗑️generated/pinpub/nb-$tag-$(echo "$f" | shasum | cut -c1-8).log" >> "$out" 2>&1
done
cat "$out"
