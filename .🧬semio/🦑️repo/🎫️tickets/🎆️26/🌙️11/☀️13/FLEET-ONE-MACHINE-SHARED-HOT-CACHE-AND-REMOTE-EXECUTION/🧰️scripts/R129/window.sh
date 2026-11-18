#!/bin/zsh
# usage: window.sh   (run it in the foreground inside the first 5 minutes of a half hour UTC; after the pre-validated staged battery)
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R129
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
FT="$L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
export R129_SET=${R129_SET:-all}
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R129 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
$BUN $D/land.mjs --dry || { echo "dry failed"; rm -r "$T/landing.lock"; exit 5; }
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
OK=1
for f in "$FT/🧽️retained/🟦️.ts" "$FT/🛑️hold/🟦️.ts" "$FT/🦶️footprint-recycle/🟦️.ts" "$FT/♻️retirement/🟦️.ts" "$FT/🧯️cap/🟦️.ts" "$FT/🔱️warm-jobs/🟦️.ts" "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"; do
 one() { $BUN "$SCRIPT" --file="$1" --birth=setup --root=/Users/ueli/Documents/semio > $SC/win.txt 2>&1; }
 one "$f" || one "$f" || { echo "RED $f"; tail -5 $SC/win.txt; OK=0; }
done
$BUN "$SCRIPT" --file="$FT/🟦️.ts" --birth=setup --root=/Users/ueli/Documents/semio > $SC/win.txt 2>&1
grep -E "^ *[0-9]+ (pass|fail)$" $SC/win.txt | tr '\n' ' '; echo "<- main (baseline 16 pass 14 fail under load)"
PASS=$(grep -E "^ *[0-9]+ pass$" $SC/win.txt | tr -d ' pass'); [ "${PASS:-0}" -ge 16 ] || { echo "RED main"; OK=0; }
if [ $OK = 1 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo RED; $BUN $D/restore.mjs; fi
rm -r "$T/landing.lock"
