#!/bin/bash
L="/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE"
cd "$L"
for a in "$@"; do
  for s in 500 200 100 50; do
    bun 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $a --scale $s --views --projection third --dpi 60 --view light-solid --language en --out "$T/🗑️generated/vegetation/proof/$a" >/dev/null 2>&1 || echo "FAIL $a $s"
  done
  for sym in $(bun "$T/🛠️scripts/vegetation/symlist.ts" $a); do
    bun 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $a --symbol $sym --views --projection third --dpi 60 --view light-solid --language en --out "$T/🗑️generated/vegetation/proof/$a" >/dev/null 2>&1 || echo "FAIL $a sym $sym"
  done
done
echo done
