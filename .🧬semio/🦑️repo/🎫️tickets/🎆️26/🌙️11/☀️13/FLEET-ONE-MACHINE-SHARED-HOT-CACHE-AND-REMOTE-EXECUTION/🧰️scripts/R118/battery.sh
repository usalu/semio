#!/bin/zsh
LABEL=$1
PRE=$2
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R118
D="$(cd "$(dirname "$0")" && pwd)"
cd /Users/ueli/Documents/semio
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
BD=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
I=$BD/🕸️graph/📥️inference
export R118_STAGE=$SC/stage/L R118_LIVE=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
LIST=()
for d in $BD/👁️observation/🧪️tests $BD/👁️observation/🏚️relocation/🧪️tests $BD/👁️observation/🔢️row-table/🧪️tests $I/🧭️script-inputs/🧪️tests $I/🧱️project-assembly/🧪️tests $I/🔎️owner-discovery/🧪️tests $I/🦀️native-manifests/🧪️tests; do
  while IFS= read -r f; do LIST+=("$f"); done < <(find "$d" \( -name '🟦️.ts' -o -name '*.test.ts' \) -not -path '*/🧫️fixtures/*' -not -path '*/🐍️oracle/*' | sort)
done
LIST+=("$I"/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts "$I"/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts)
mkdir -p $SC/out $SC/logs; rm -f $SC/out/*
one() {
  f=$1; n=$(echo "$f" | shasum | cut -c1-8)
  args=(); [ -n "$PRE" ] && args=(--preload "$D/preload.ts")
  for attempt in 1 2; do
    out=$(cd /Users/ueli/Documents/semio && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test $args "$f" --timeout 240000 2>&1)
    pass=$(echo "$out" | grep -E "^ *[0-9]+ pass" | tail -1 | tr -s ' ')
    fail=$(echo "$out" | grep -E "^ *[0-9]+ fail" | tail -1 | tr -s ' ')
    [ -n "$pass$fail" ] && { echo "$fail" | grep -qE "[1-9][0-9]* fail" || break; }
  done
  echo "$out" | grep -E "^\(fail\)|error:" | head -12 > $SC/logs/$n.$LABEL.log
  echo "${f#$BD/} :${pass}${fail}" > $SC/out/$n
}
i=0
for f in $LIST; do
  [ -f "$f" ] || continue
  one "$f" &
  i=$((i+1)); (( i % ${PAR:-4} == 0 )) && wait
done
wait
cat $SC/out/* | sort > $SC/$LABEL.txt; echo "files ${#LIST}" >> $SC/$LABEL.txt; echo done >> $SC/$LABEL.txt
