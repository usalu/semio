#!/bin/zsh
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ctc-tools
typeset -A M
for line in ${(f)"$(bun ctc-find.ts $(echo $@))"}; do M[${line%%$'\t'*}]=${line#*$'\t'}; done
while read c o f; do
  [[ -z "$c" ]] && continue
  if [[ -z "${M[$c]}" ]]; then echo "NOMANIFEST $c"; continue; fi
  bun ctc-devfeat.ts "${M[$c]}" "$o" "$f" 2>&1 | tail -1
done
