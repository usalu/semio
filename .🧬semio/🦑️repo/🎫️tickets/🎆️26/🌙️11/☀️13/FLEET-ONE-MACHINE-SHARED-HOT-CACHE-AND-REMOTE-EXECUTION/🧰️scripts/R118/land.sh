#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R118
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
BD=$G/🏗️build; I=$BD/🕸️graph/📥️inference; O=$BD/👁️observation; N=$O/📂️source-names
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R118 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
echo "start $(date -u)"
rm -rf $SC/backup; mkdir -p $SC/backup $SC/work
TARGETS=("$O/🟨️.mjs:observation" "$O/🟨️.d.mts:declarations" "$I/🧱️project-assembly/🟨️.mjs:assembly" "$I/🧭️script-inputs/🟨️.mjs:script-inputs" "$I/🧭️script-inputs/🧪️tests/🔣️.json:corpus")
restore() {
  [ -n "$APPLIED" ] || { echo "nothing applied"; return; }
  for row in $TARGETS; do f=${row%%:*}; k=${row##*:}; [ -f "$SC/backup/$k" ] && cp "$SC/backup/$k" "$f.r118tmp" && mv "$f.r118tmp" "$f"; done
  rm -rf "$N"
  echo restored
}
fail() { echo "FAIL $1"; restore; rm -r "$T/landing.lock"; exit 3; }
for row in $TARGETS; do
  f=${row%%:*}; k=${row##*:}
  cp "$f" "$SC/backup/$k"; cp "$f" "$SC/work/$k"
  python3 "$D/patch.py" "$k" "$SC/work/$k" || fail "patch $k"
done
for row in $TARGETS; do f=${row%%:*}; k=${row##*:}; shasum -a 256 "$f" | cut -c1-12 >> $SC/backup/before.sha; done
APPLIED=1
mkdir -p "$N/🧬️schema" "$N/🧪️tests"
cp "$SC/stage/L/🏗️build/👁️observation/📂️source-names/🧬️schema/🔣️.json" "$N/🧬️schema/🔣️.json"
cp "$SC/stage/L/🏗️build/👁️observation/📂️source-names/🟨️.mjs" "$N/🟨️.mjs"
cp "$SC/stage/L/🏗️build/👁️observation/📂️source-names/🧪️tests/🟦️.ts" "$N/🧪️tests/🟦️.ts"
for row in $TARGETS; do f=${row%%:*}; k=${row##*:}; cp "$SC/work/$k" "$f.r118tmp" && mv "$f.r118tmp" "$f"; done
cd /Users/ueli/Documents/semio
run() { env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN $S --file="$1" --birth=setup --root=/Users/ueli/Documents/semio 2>&1; }
LIST=("$N/🧪️tests/🟦️.ts" "$O/🧪️tests/🆔️record-revision/🟦️.ts" "$O/🧪️tests/📥️reuse/🟦️.ts" "$O/🧪️tests/🧾️record-header/🟦️.ts" "$O/🧪️tests/🔎️lookup-holds/🟦️.ts" "$I/🧭️script-inputs/🧪️tests/🟦️.ts" "$I/🧱️project-assembly/🧪️tests/🟦️.ts" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$I/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts")
rm -f $SC/landed.txt; rm -rf $SC/lt; mkdir -p $SC/lt; i=0
for f in $LIST; do ( out=$(run "$f"); echo "${f#$BD/} :$(echo "$out" | grep -E "^ *[0-9]+ pass" | tail -1 | tr -s ' ')$(echo "$out" | grep -E "^ *[0-9]+ fail" | tail -1 | tr -s ' ')" > $SC/lt/$i; echo "$out" | grep -E "^\(fail\)|error:" | head -6 >> $SC/lt/$i.err ) & i=$((i+1)); (( i % 5 == 0 )) && wait; done; wait
cat $SC/lt/? > $SC/landed.txt
cat $SC/landed.txt
if grep -qE "[1-9][0-9]* fail" $SC/landed.txt || [ $(grep -c pass $SC/landed.txt) -lt ${#LIST} ]; then cat $SC/lt/*.err 2>/dev/null | head -20; fail "tests"; fi
for row in $TARGETS; do f=${row%%:*}; shasum -a 256 "$f" | cut -c1-12; done
rm -r "$T/landing.lock"
cat $SC/landed.txt
echo "GREEN end $(date -u)"
