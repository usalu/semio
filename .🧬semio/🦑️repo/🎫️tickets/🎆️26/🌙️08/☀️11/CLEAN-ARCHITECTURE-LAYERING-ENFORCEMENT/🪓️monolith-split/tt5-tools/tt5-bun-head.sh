#!/bin/zsh
# usage: tt5-bun-head.sh <phase> <list-file> ; like tt5-bun.sh but runs the HEAD version of each test (git show) as a snapshot copy, so the pre-edit behaviour is measured against the current sources.
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
P="$1"; LIST="$2"
mkdir -p "$G/tt5-bun-$P"
n=0
cd "$R" || exit 1
while IFS= read -r f; do
  n=$((n+1)); [ -z "$f" ] && continue
  rel="${f#$R/}"
  d="$R/$(dirname "$rel")"; b="$(basename "$rel")"
  out="$G/tt5-bun-$P/$(printf %03d $n).txt"
  run="tt5-snap-$n.ts"
  git -c core.quotepath=false -c core.fsmonitor=false show "HEAD:$rel" > "$d/$run" 2>/dev/null || { echo "NOHEAD $rel" > "$out"; rm -f "$d/$run"; continue; }
  if /usr/bin/grep -q 'from "vitest"' "$d/$run" 2>/dev/null; then echo "VITEST $rel" > "$out"; rm -f "$d/$run"; continue; fi
  res=$(cd "$d" && perl -e 'alarm 300; exec @ARGV' -- nice -n 10 bun test "./$run" 2>&1)
  rm -f "$d/$run"
  { echo "FILE $rel"; echo "$res" | /usr/bin/grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]//' | sort; echo "$res" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)|error:' | head -5; } > "$out"
done < "$LIST"
echo done > "$G/tt5-bun-$P/DONE"
