#!/bin/sh
# 🧾️ The measurements after the shared repairs, one queue grant each, in a row: the old and new paint depth of the solid-sweep vectors, MO's depth table of `used-classical-guitar`, the authors' gate and the fresh matrix of the assets the repairs clear (`used-harmonium`, `steel-column`), and the type check of the staffage package. Output: 🗑️generated/matrix/MX/after-*.txt.
R=/Users/ueli/Documents/semio
U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$S/🧪️tests/🔭️multiview-coverage"
G="$U/🗑️generated/matrix/MX"
B=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$R" || exit 2
repo capacity run --memory=800 -- timeout 600 "$B" "$U/🧰️matrix/MX/depth-old.ts" > "$G/after-depth-old.txt" 2>&1
repo capacity run --memory=800 -- timeout 600 "$B" "$U/🧰️matrix/MO/depth.ts" used-classical-guitar 50 stored plan > "$G/after-depth-guitar.txt" 2>&1
repo capacity run --memory=800 -- timeout 1200 "$B" "$T/🔎️scan.ts" --language en used-harmonium steel-column > "$G/after-scan.txt" 2>&1
repo capacity run --memory=1200 -- timeout 3000 "$B" "$T/📊️matrix.ts" --language en --out "$G/matrix-harmonium" --jobs 1 --fresh --assets used-harmonium > "$G/after-matrix-harmonium.txt" 2>&1
repo capacity run --memory=1200 -- timeout 3000 "$B" "$T/📊️matrix.ts" --language en --out "$G/matrix-column" --jobs 1 --fresh --assets steel-column > "$G/after-matrix-column.txt" 2>&1
(cd "$S/📦️packages/🟦️typescript" && repo capacity run --memory=2500 -- timeout 1800 "$B" x tsc --noEmit -p tsconfig.json > "$G/after-tsc.txt" 2>&1; echo "tsc exit $?" >> "$G/after-tsc.txt")
echo done
