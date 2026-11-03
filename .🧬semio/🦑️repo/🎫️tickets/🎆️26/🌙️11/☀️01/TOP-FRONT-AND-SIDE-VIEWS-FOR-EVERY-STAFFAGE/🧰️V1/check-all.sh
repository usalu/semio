#!/bin/sh
cd /c/git/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
mkdir -p "$T/🗑️generated/V1"
for a in agave apple-tree bamboo-clump date-palm english-oak hornbeam-hedge hosta-clump ivy-on-trellis; do
  repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$a" < /dev/null > "$T/🗑️generated/V1/check-$a.txt" 2>&1
done
echo done > "$T/🗑️generated/V1/check-all.done"
