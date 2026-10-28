#!/bin/sh
# 🧪 [nxdeadline] runs the neighbour suites of batch `release` in mirror <name> through the envelope fallback (red-g mrun.sh chain mode), one file at a time, resumable: neighbors.sh <name> <tag> [max-seconds]  -> C/🗑️generated/nxdeadline/nb-<tag>.txt (one summary + failing names per file)
n="$1"; tag="$2"; max="${3:-560}"; C="$(cd "$(dirname "$0")/.." && pwd)"; out="$C/🗑️generated/nxdeadline/nb-$tag.txt"; touch "$out"; start=$(date +%s)
E="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
while read -r f; do
 /usr/bin/grep -qF "== $f" "$out" && continue
 [ $(( $(date +%s) - start )) -gt "$max" ] && { echo "PAUSED (time) before $f"; exit 3; }
 { echo "== $f"; sh "$C/red-g-tools/mrun.sh" "$n" chain "$f" --timeout 300000; } >> "$out" 2>&1
done <<LIST
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🤝️yield/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🧭️ambient-scope/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔓️pool-order/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪶️sparse/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🏦️funded/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️store/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/🧪️tests/🟦️.ts
$E/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts
$E/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts
$E/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️phase/🟦️.ts
$E/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️queued/🟦️.ts
$E/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🆔️identity/🟦️.ts
$E/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts
$E/🧪️tests/🔒️nx/🟦️.ts
$E/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts
LIST
echo "DONE"; cat "$out"
