#!/bin/zsh
# R124 landing: run only inside the first 5 minutes of a half hour UTC; foreground; releases the lock on every path.
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R124
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
$BUN $D/land.mjs --dry || { echo "dry failed"; exit 5; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R124 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
$D/battery.sh $SC/after.txt 6
rm -f $SC/reds.txt
if ! python3 $D/compare.py $SC/before2.txt $SC/after.txt $SC/reds.txt; then
 echo "retry the red files once"
 R124_ONLY="$(cat $SC/reds.txt)" $D/battery.sh $SC/retry.txt 3
 python3 - <<PY
import re
def load(p):
    return {l.rsplit("::",1)[0].strip():l for l in open(p,encoding="utf8") if "::" in l}
a=load("$SC/after.txt");a.update(load("$SC/retry.txt"))
open("$SC/after.txt","w",encoding="utf8").write("".join(a[k] for k in sorted(a)))
PY
 python3 $D/compare.py $SC/before2.txt $SC/after.txt || { echo RED; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 4; }
fi
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
