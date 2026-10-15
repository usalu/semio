#!/bin/zsh
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/hubsplit-tools
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/hubo
for i in 1 2 3 4; do
  python3 $T/apply.py >/dev/null || exit 1
  $S/run.sh plain >/dev/null; $S/run.sh test --cfg test >/dev/null
  r=$(python3 $T/oraclefix.py); echo "$r"
  echo "$r" | /usr/bin/grep -q '^stable' && break
done
