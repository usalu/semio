#!/bin/zsh
# 🔁 Round 3 loop of machinery/oil-pumpjack: build, validate the candidate, apply, check every scale (outputs under 🗑️generated/r3-oil-pumpjack).
cd "${0:A:h}"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-oil-pumpjack"
python3 r3-oil-pumpjack-build.py > $G/build.txt || { cat $G/build.txt; exit 1; }
$BUN r3-validate.ts --file "$G/🔣️.json" | tail -5 | tee $G/validate.txt | grep -q "^VALID" || { cat $G/validate.txt; exit 1; }
python3 r3-oil-pumpjack-build.py --apply | tail -2
$BUN r3-validate.ts machinery/oil-pumpjack | tail -1
scales=($@); (( $#scales )) || scales=(50 100 200 500)
for s in $scales; do
  $BUN volumes-check.ts machinery/oil-pumpjack --product --scale $s > $G/check-$s.txt 2>&1
  echo "== $s: $(tail -1 $G/check-$s.txt)"
  grep -o '![^ ]*' $G/check-$s.txt
done
true
