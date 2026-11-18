#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R134
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R134
$BUN $D/land.mjs --dry || { echo "dry failed"; exit 5; }
python3 -c "
import time,datetime
n=datetime.datetime.utcnow()
if n.minute%30>=3:
    t=(n.replace(minute=0 if n.minute>=30 else 30,second=0,microsecond=0)+(datetime.timedelta(hours=1) if n.minute>=30 else datetime.timedelta(0)))
    time.sleep(max(0,(t-n).total_seconds()))
"
M=$(date -u +%M); S=$(date -u +%S)
[ $((10#$M % 30)) -lt 4 ] || { echo "outside window $M:$S"; exit 2; }
GOT=0; for try in $(seq 1 3000); do if mkdir "$T/landing.lock" 2>/dev/null; then GOT=1; break; fi; M=$(date -u +%M); S=$(date -u +%S); [ $((10#$M % 30)) -ge 4 ] && [ $((10#$M % 30)) -lt 29 ] && break; python3 -c "import time;time.sleep(0.1)"; done
[ $GOT = 1 ] || exit 1
echo "claude-R134 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
rm -rf $SC/backup
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
( cd $D && $BUN check.mjs ) ; RC=$?
if [ $RC = 0 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo "RED started $START ended $(date -u +%H:%M:%S)"; $BUN $D/restore.mjs; fi
rm -r "$T/landing.lock"
exit $RC
