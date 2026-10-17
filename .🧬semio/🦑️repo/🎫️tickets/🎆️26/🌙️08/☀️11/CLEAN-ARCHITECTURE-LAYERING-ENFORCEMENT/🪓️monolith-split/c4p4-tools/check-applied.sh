#!/bin/zsh
# usage: check-applied.sh <tag> <pkg-dir>...   (baseline/after runs of new crates --check-applied)
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p4-work
tag=$1; shift
n=0
for pkg in "$@"; do
  n=$((n+1))
  bun ./📜️script.ts new crates "$pkg" --check-applied --base=f856dff0813 --sandbox=$S/ca-$tag-$n > "$F/🗑️generated/c4p4-checkapplied-$tag-$n.log" 2>&1
  echo "pkg=$pkg exit=$?" >> "$F/🗑️generated/c4p4-checkapplied-$tag.exit"
done
