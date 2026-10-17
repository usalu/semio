#!/bin/zsh
# usage: plan.sh <pkgdir> <tag> [extra args]; dry-run explain with slices, writes r8b-<tag>.log/.slices
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P=$1; tag=$2; shift; shift
F3_SLICES=1 $F/r8b-tools/step.sh $tag bun ./📜️script.ts new crates "$P" --dry-run --explain "$@"
/usr/bin/grep -E '^(crate|problem|  cycle)' $F/🗑️generated/r8b-$tag.log | cut -c1-230
/usr/bin/grep -E '^blocker' $F/🗑️generated/r8b-$tag.log | cut -c1-120
for i in 0 1 2; do python3 $F/r8b-tools/slices.py $F/🗑️generated/r8b-$tag.log $i > $F/🗑️generated/r8b-$tag.slices$i 2>/dev/null; done
