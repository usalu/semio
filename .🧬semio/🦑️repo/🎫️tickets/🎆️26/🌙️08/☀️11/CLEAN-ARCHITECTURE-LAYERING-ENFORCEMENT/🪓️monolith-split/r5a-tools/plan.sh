#!/bin/zsh
# usage: plan.sh <tag> <artifact-dir-name> [extra flags]  -> F/🗑️generated/r5a-<tag>-plan.log (slices + groups)
cd /Users/ueli/Documents/semio || exit 99
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
tag=$1; art=$2; shift 2
ulimit -n 65536
P="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/$art/📦️packages/🦀️rust"
for i in 1 2 3 4 5 6; do
  F3_SLICES=1 nice -n 10 bun ./📜️script.ts new crates "$P" --dry-run --explain "$@" > "$F/🗑️generated/r5a-$tag-plan.log" 2>&1
  if /usr/bin/grep -q 'SyntaxError\|Cannot find module\|Export named' "$F/🗑️generated/r5a-$tag-plan.log"; then sleep 40; else break; fi
done
/usr/bin/grep -v '^note:\|^  slice\|^    {\|^  group\|^\[script\]' "$F/🗑️generated/r5a-$tag-plan.log" | cut -c1-1200
