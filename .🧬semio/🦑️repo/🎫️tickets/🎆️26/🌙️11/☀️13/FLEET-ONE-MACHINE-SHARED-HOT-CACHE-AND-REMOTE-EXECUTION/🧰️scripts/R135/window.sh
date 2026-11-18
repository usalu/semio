#!/bin/zsh
# usage: window.sh   (foreground, inside the first 5 minutes of a half hour UTC; after the staged battery)
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R135
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
export R135_STEP=${R135_STEP:-1}
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R135 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
$BUN $D/land.mjs --dry || { echo "dry failed"; rm -r "$T/landing.lock"; exit 5; }
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
OK=1
FILES=(${(f)"$(cat $SC/window$R135_STEP.list)"})
for f in "${FILES[@]}"; do
 file=${f%%|*}; pat=${f#*|}; [ "$pat" = "$f" ] && pat=""
 one() { if [ -n "$pat" ]; then $BUN "$SCRIPT" --file="$file" --pattern="$pat" --birth=setup --root=/Users/ueli/Documents/semio > $SC/win.txt 2>&1; else $BUN "$SCRIPT" --file="$file" --birth=setup --root=/Users/ueli/Documents/semio > $SC/win.txt 2>&1; fi; }
 one || one || { echo "RED $f"; grep -E "\(fail\)|error:|Expected|Received" $SC/win.txt | head -14; cp $SC/win.txt $SC/win-red.txt; OK=0; }
done
if [ $OK = 1 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo RED; $BUN $D/restore.mjs; fi
rm -r "$T/landing.lock"
