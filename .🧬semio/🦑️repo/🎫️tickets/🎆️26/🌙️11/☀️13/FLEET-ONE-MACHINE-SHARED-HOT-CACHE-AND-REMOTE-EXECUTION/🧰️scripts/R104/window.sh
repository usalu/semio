#!/bin/zsh
D="$(cd "$(dirname "$0")" && pwd)"
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R104
B=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
I=$B/🕸️graph/📥️inference
N=$I/📦️project-nodes
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R104 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
python3 "$D/land.py" land || { python3 "$D/restore.py" 2>/dev/null; python3 "$D/land.py" restore; rm -r "$T/landing.lock"; exit 3; }
cd /Users/ueli/Documents/semio
FILES=("$N/🌐️owner-pool/🧪️tests/🌊️early-wave/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🎚️lanes/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🔁️no-record/🟦️.ts" "$N/🌐️owner-pool/🧪️tests/🏋️heavy/🟦️.ts" "$B/🌐️remote-execution/🛰️sidecar/🧪️tests/🏊️pool/🟦️.ts" "$N/🧪️tests/📥️take/🟦️.ts" "$I/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$I/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts")
rm -f $SC/out.*.txt
i=0
for f in $FILES; do
  i=$((i+1))
  ( out=$($BUN $H --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); echo "$out" | grep -E "^ +[0-9]+ (pass|fail)" | tr -s ' ' | tr '\n' ' ' > $SC/out.$i.txt; echo "|${f##*/🏗️build/}" >> $SC/out.$i.txt; echo "$out" > $SC/full.$i.txt ) &
done
wait
cat $SC/out.*.txt
red=$(cat $SC/out.*.txt | grep -c "[1-9][0-9]* fail")
if (( red > 0 )) || [ $(cat $SC/out.*.txt | grep -c "pass") -lt 9 ]; then echo "RED: restoring"; python3 "$D/land.py" restore; rm -r "$T/landing.lock"; date -u; exit 4; fi
rm -r "$T/landing.lock"; echo LANDED; date -u
