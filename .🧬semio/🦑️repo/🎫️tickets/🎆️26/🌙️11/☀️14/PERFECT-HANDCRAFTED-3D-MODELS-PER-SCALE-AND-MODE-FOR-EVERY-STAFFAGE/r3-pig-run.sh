#!/bin/sh
# 🔁 Rebuilds the four scales of animals/pig, redraws the 2D from them into a candidate document, validates it (schema, validateAsset, print suitability) and writes representations and dimensions into the catalog only when it is valid. Usage: sh r3-pig-run.sh [check]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-pig"
"$BUN" r3-pig-50.ts --write | tail -1
for s in 100 200 500; do "$BUN" r3-pig-coarse.ts $s --write | tail -1; done
"$BUN" r3-pig-2d.ts --dimensions 0.684 1.27 0.331 | tail -1
"$BUN" r3-validate.ts --file "$G/🔣️.json" > "$G/validate.txt"; tail -1 "$G/validate.txt"
"$BUN" r3-pig-suit.ts "$G/🔣️.json" > "$G/suit.txt"; grep -c " false " "$G/suit.txt"
if grep -q "^VALID" "$G/validate.txt" && ! grep -q " false " "$G/suit.txt"; then python3 r3-pig-decisions.py && python3 r3-pig-apply.py; else echo "candidate not written"; grep -v "^VALID\|^INVALID" "$G/validate.txt" | head; grep -A3 " false " "$G/suit.txt" | head; fi
if [ "$1" = "check" ]; then "$BUN" volumes-check.ts animals/pig --product > "$G/check.txt" 2>&1; grep -c "^ok" "$G/check.txt"; grep "^FAIL" "$G/check.txt" | sed -E 's/ info:[^ ]+//g' | cut -c1-700; fi
