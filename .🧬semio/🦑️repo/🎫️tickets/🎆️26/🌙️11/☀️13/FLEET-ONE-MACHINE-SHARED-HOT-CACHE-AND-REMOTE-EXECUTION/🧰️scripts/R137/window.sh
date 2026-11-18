#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R137
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R137
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; done
echo "claude-R137 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
python3 $D/land.py --dry || { rm -r "$T/landing.lock"; exit 5; }
START=$(date -u +%H:%M:%S)
python3 $D/land.py || { python3 $D/restore.py; rm -r "$T/landing.lock"; exit 3; }
OK=1
FILES=("$B/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🎯️probes/🟦️.ts" "$B/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🪜️stage-plan/🟦️.ts" "$B/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🔭️scope/🟦️.ts" "$B/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$B/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts")
for file in "${FILES[@]}"; do
 one() { $BUN "$SCRIPT" --file="$file" --birth=setup --root=/Users/ueli/Documents/semio > $SC/win.txt 2>&1; }
 one || one || { echo "RED $file"; grep -E "\(fail\)|error:|Expected|Received" $SC/win.txt | head -14; cp $SC/win.txt $SC/win-red.txt; OK=0; }
done
if [ $OK = 1 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo RED; python3 $D/restore.py; fi
rm -r "$T/landing.lock"
