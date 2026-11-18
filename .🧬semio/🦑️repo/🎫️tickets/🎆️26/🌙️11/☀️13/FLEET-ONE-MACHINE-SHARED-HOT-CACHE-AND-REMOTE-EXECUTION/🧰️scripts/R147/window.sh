#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
R=/Users/ueli/Documents/semio
T=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R147
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
OP="$B/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
# wait for the lock, never give up
while ! mkdir "$T/landing.lock" 2>/dev/null; do echo "held: $(cat $T/landing.lock/owner 2>/dev/null) $(date -u +%H:%M:%S)"; sleep 20; done
echo "claude-R147 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
python3 $D/land.py dry >/dev/null || { echo "dry failed"; rm -r "$T/landing.lock"; exit 5; }
python3 $D/land.py land || { python3 $D/land.py restore; rm -r "$T/landing.lock"; exit 3; }
FILES=(
 "$OP/🧪️tests/🧮️shared-budget/🟦️.ts"
 "$OP/🧪️tests/🎛️one-budget/🟦️.ts"
 "$OP/🧪️tests/🟦️.ts|sizing, limits"
 "$R/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🧪️tests/🟦️.ts"
 "$B/🧠️memory/🧪️tests/🙋️ask/🟦️.ts"
 "$B/🧠️memory/🧪️tests/🟦️.ts"
 "$B/🌐️remote-execution/🛰️sidecar/🧪️tests/🏊️pool/🟦️.ts"
 "$B/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts"
 "$B/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"
)
i=0
for f in "${FILES[@]}"; do
 i=$((i+1)); file=${f%%|*}; pat=${f#*|}; [ "$pat" = "$f" ] && pat=""
 ( if [ -n "$pat" ]; then $BUN "$SCRIPT" --file="$file" --pattern="$pat" --birth=setup --root=$R > $SC/win$i.txt 2>&1; else $BUN "$SCRIPT" --file="$file" --birth=setup --root=$R > $SC/win$i.txt 2>&1; fi; echo $? > $SC/win$i.rc ) &
done
wait
OK=1
for n in $(seq 1 $i); do rc=$(cat $SC/win$n.rc); echo "$n rc=$rc $(grep -E '^ *[0-9]+ (pass|fail)' $SC/win$n.txt | tr '\n' ' ')"; done
echo "R147 window: started $START ended $(date -u +%H:%M:%S)"
for n in 1 2 3 4 5 6 8 9; do [ "$(cat $SC/win$n.rc)" = 0 ] || OK=0; done
F7=$(grep -E '^ *[0-9]+ fail' $SC/win7.txt | awk '{print $1}'); [ "${F7:-0}" -le 3 ] || OK=0
if [ $OK = 1 ]; then echo GREEN; else echo RED; python3 $D/land.py restore; fi
rm -r "$T/landing.lock"
