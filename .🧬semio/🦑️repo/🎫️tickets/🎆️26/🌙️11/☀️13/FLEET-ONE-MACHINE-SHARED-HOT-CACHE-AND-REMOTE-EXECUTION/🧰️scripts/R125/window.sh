#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R125
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R125 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
$BUN $D/land.mjs --dry || { echo "dry failed"; rm -r "$T/landing.lock"; exit 5; }
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
R125_BASE=1 $D/battery.sh $SC/after2.txt
rm -f $SC/reds.txt
OK=1
if ! python3 $D/compare.py $SC/before.txt $SC/after2.txt $SC/reds.txt; then
 echo "retry the red files once"
 R125_BASE=1 R125_ONLY="$(cat $SC/reds.txt)" $D/battery.sh $SC/retry.txt
 python3 - <<PY
def load(p):
    return {l.rsplit("::",1)[0].strip():l for l in open(p,encoding="utf8") if "::" in l}
a=load("$SC/after2.txt");a.update(load("$SC/retry.txt"))
open("$SC/after2.txt","w",encoding="utf8").write("".join(a[k] for k in sorted(a)))
PY
 python3 $D/compare.py $SC/before.txt $SC/after2.txt || OK=0
fi
for f in "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"; do
 $BUN "$SCRIPT" --file="$f" --birth=setup --root=/Users/ueli/Documents/semio > $SC/pre.txt 2>&1 || { echo "PREFLIGHT RED $f"; tail -5 $SC/pre.txt; OK=0; }
done
if [ $OK = 1 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo RED; $BUN $D/restore.mjs; fi
rm -r "$T/landing.lock"
