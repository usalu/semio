#!/bin/bash
L="/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE"
cd "$L"
for a in "$@"; do
  for pair in $(bun "$T/🛠️scripts/vegetation/symlist2.ts" $a); do
    sym=${pair%%:*}; den=${pair##*:}
    bun 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $a --symbol $sym --scale $den --views --projection third --dpi 60 --view light-solid --language en --out "$T/🗑️generated/vegetation/proof-sym/$a" >/dev/null 2>&1 || echo "FAIL $a sym $sym $den"
  done
done
echo done
