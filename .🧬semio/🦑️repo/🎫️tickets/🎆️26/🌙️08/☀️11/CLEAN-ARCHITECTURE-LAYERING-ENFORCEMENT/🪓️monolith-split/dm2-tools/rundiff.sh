#!/bin/zsh
# 🔬️ Runs the scoped differential for every "label:dir[:flags]" line of the given spec file; logs go to 🗑️generated/dm2-diff-<set>.log
cd /Users/ueli/Documents/semio || exit 1
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/dm2-tools"
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/dm2-diff-$2.log"
: > "$LOG"
while IFS='|' read -r label dir flags; do
  [ -z "$label" ] && continue
  echo "## $label $dir $flags" >> "$LOG"
  perl -e 'alarm 300; exec @ARGV' -- python3 "$D/diff_oracle.py" "$label" "$dir" ${=flags} >> "$LOG" 2>&1
done < "$1"
