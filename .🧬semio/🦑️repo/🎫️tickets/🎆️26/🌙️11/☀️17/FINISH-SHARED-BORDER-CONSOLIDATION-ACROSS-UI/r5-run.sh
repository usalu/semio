#!/bin/zsh
# usage: r5-run.sh <tag> <crates-file>; runs every crate's tests, one log per crate
F="/Users/ueli/Documents/semio/🧰️framework"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/🗑️generated"
tag=$1; cd "$F"
i=0
while read -r m; do
  i=$((i+1)); name=$(grep -m1 '^name' "$m" | cut -d'"' -f2)
  timeout 1500 repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path "$F/$m" > "$T/r5-$tag-$name.log" 2>&1
  echo "$name exit $? $(grep -h 'test result' "$T/r5-$tag-$name.log" | awk '{p+=$4;f+=$6} END{print "passed="p" failed="f}') $(grep -c 'error: could not compile' "$T/r5-$tag-$name.log") compileerr" >> "$T/r5-$tag-summary.log"
done < "${2}"
echo ALLDONE >> "$T/r5-$tag-summary.log"
