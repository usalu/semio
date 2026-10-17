#!/bin/zsh
# usage: tt5-bun.sh <phase> <list-file> [snapshot]  ; runs bun test per file (snapshot: run a temp copy so the original can be edited meanwhile); results in 🗑️generated/tt5-bun-<phase>/<n>.txt
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
P="$1"; LIST="$2"; SNAP="$3"
mkdir -p "$G/tt5-bun-$P"
n=0
cd "$R" || exit 1
while IFS= read -r f; do
  n=$((n+1)); [ -z "$f" ] && continue
  case "$f" in /*) rel="${f#$R/}";; *) rel="$f";; esac
  d="$R/$(dirname "$rel")"; b="$(basename "$rel")"
  out="$G/tt5-bun-$P/$(printf %03d $n).txt"
  if /usr/bin/grep -q 'from "vitest"' "$R/$rel" 2>/dev/null; then echo "VITEST $rel" > "$out"; continue; fi
  run="$b"
  if [ -n "$SNAP" ]; then run="tt5-snap-$n.ts"; cp "$R/$rel" "$d/$run"; fi
  res=$(cd "$d" && perl -e 'alarm 300; exec @ARGV' -- nice -n 10 bun test "./$run" 2>&1)
  [ -n "$SNAP" ] && rm -f "$d/$run"
  { echo "FILE $rel"; echo "$res" | /usr/bin/grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]//' | sort; echo "$res" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)|error:' | head -5; } > "$out"
done < "$LIST"
echo done > "$G/tt5-bun-$P/DONE"
