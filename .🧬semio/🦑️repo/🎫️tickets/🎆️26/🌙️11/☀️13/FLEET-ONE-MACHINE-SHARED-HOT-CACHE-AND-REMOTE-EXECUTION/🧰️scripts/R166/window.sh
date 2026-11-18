#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R166
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R166
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
C="$L/⚡️caching/🚀️bootstrap/🛠️tools"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
m=$((10#$(date -u +%M))); until [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; do sleep 1; m=$((10#$(date -u +%M))); done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 200 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R166 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land.py restore $S/bk; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $S/bk $S/after; mkdir -p $S/after
python3 $D/land.py apply $S/bk || fail apply
run(){ n=$(echo "$1"|md5 -q); cd "$L/$1" || fail cd; env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test ./🟦️.ts --timeout 120000 > $S/after/$n.txt 2>&1; echo "$1 $(grep -E '^ *[0-9]+ (pass|fail)' $S/after/$n.txt | tr -d ' ' | tr '\n' ' ')"; grep -q "^ *0 fail" $S/after/$n.txt; }
run "⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📏️artefact-scale" || fail artefact-scale
run "⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🎯️need-scope" || fail need-scope
run "⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🧾️sealed-plan" || fail sealed-plan
run "🏗️build/📖️texts/🧪️tests" || fail texts
run "🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight" || fail preflight
run "🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests" || fail heavy
cd "$C/📦️sdk/🎟️capacity/🧪️tests" && env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test ./🟦️.ts ./📤️derivative/🟦️.ts --timeout 120000 > $S/after/cap.txt 2>&1; echo "capacity $(grep -E '^ *[0-9]+ (pass|fail)' $S/after/cap.txt | tr -d ' ' | tr '\n' ' ')"; grep "^(pass)" $S/after/cap.txt | cut -c1-100
grep -q "^(fail) .*\(descriptor\|capacity charges\|clone publication charges\|shared-pristine\)" $S/after/cap.txt && fail capacity-pure
g="$C/🔏️authorizers/🧪️tests/📈️growth"; cd "$g" && env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test ./🟦️.ts --timeout 120000 > $S/after/growth.txt 2>&1; grep -E '^ *[0-9]+ (pass|fail)' $S/after/growth.txt | tr -d ' ' | tr '\n' ' '; grep -q "^ *4 fail" $S/after/growth.txt || fail growth
echo "GREEN $START $(date -u +%H:%M:%S)"; release
