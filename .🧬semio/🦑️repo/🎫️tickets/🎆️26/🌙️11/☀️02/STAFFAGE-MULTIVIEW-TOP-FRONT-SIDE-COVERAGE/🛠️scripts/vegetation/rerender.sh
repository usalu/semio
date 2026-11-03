#!/bin/bash
L="/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVAGE"
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE"
cd "$L"
for a in apple-tree english-oak; do
  bun 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $a --scale 500 --views --projection third --dpi 60 --view light-solid --language en --out "$T/🗑️generated/vegetation/proof/$a" >/dev/null 2>&1 || echo "FAIL $a 500"
done
bash "$T/🛠️scripts/vegetation/proof-sym.sh" agave apple-tree bamboo-clump date-palm english-oak hornbeam-hedge hosta-clump ivy-on-trellis japanese-maple lombardy-poplar london-plane-pollarded miscanthus-grass multi-stem-birch scots-pine small-leaved-lime weeping-willow
