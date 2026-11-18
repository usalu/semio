#!/bin/zsh
# R126 landing: run only inside the first 5 minutes of a half hour UTC. usage: window.sh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R126
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R126
M=$(date -u +%M); [ $((M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R126 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
$D/battery.sh $SC/after-live.txt live $D/inlock-after.txt
python3 $D/compare.py $SC/before-live.txt $SC/after-live.txt $SC/worse.txt || {
 echo "retrying the worse files once"
 $D/battery.sh $SC/retry-live.txt live $SC/worse.txt
 python3 - $SC <<'PY'
import sys
S=sys.argv[1]+"/"
retry={l.split(" :: ")[0]:l for l in open(S+"retry-live.txt",encoding="utf8") if " :: " in l}
out=[retry.get(l.split(" :: ")[0],l) for l in open(S+"after-live.txt",encoding="utf8")]
open(S+"after-live.txt","w",encoding="utf8").write("".join(out))
PY
 python3 $D/compare.py $SC/before-live.txt $SC/after-live.txt $SC/worse.txt || { echo RED; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 4; }
}
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
