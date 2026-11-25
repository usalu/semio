#!/bin/sh
# 🧾️ Re-proof worker of lane MP: mp.sh <id…>. Per asset (each stage its own capacity grant): sha256, scan, check CLI, matrix; one line per asset in <generated>/results.tsv.
R=/Users/ueli/Documents/semio
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$S/🧪️tests/🔭️multiview-coverage"
U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
G="$U/🗑️generated/matrix/MP"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$R" || exit 1
retry() { log=$1; shift; n=0; while :; do "$@" > "$log" 2>&1; rc=$?; n=$((n+1)); grep -q 'The wait ends here' "$log" && [ $n -lt 12 ] && continue; return $rc; done; }
mkdir -p "$G/logs"
for id in "$@"; do
  f=$(ls "$S"/🗂️catalog/*/"$id"/🔣️.json 2>/dev/null | head -1)
  se=""
  sha=$(shasum -a 256 "$f" | cut -c1-64)
  kb=$(du -k "$f" | cut -f1)
  cm=1000; mm=800
  case "$f" in *vegetation*) if [ "$kb" -ge 6000 ]; then mm=2500; elif [ "$kb" -ge 1500 ]; then mm=1600; else mm=1000; fi;; esac
  if [ -s "$G/logs/$id.scan.json" ]; then se=prior; else
  retry "$G/logs/$id.scan.log"   repo capacity run --memory=800 -- sh -c "cd '$T' && timeout 900 '$BUN' 🔎️scan.ts --language en --fresh --out '$G/logs/$id.scan.jsonl' --report '$G/logs/$id.scan.json' $id"; se=$?; fi
  retry "$G/logs/$id.check.log"   repo capacity run --memory=$cm -- timeout 1800 "$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id"; ce=$?
  retry "$G/logs/$id.matrix.log"   repo capacity run --memory=$mm -- timeout 14400 "$BUN" "$T/📊️matrix.ts" --language en --out "$G/$id" --jobs 1 --assets "$id"; me=$?
  printf '%s\t%s\t%s\t%s\t%s\n' "$id" "$sha" "$se" "$ce" "$me" >> "$G/results.tsv"
done
