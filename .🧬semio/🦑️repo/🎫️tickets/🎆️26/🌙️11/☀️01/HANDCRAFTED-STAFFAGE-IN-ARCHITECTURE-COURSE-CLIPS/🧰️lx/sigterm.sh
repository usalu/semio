set -u
BUN=$(ls /home/ueli/.cache/semio/runtimes/bun/*/bun | head -1)
R=/mnt/c/git/semio
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/lx"
W=$(mktemp -d /tmp/lx.XXXX)
python3 - "$S/🧫️fixtures/🎭️cast-cancellation/🔣️.json" "$W" <<'PY'
import json,sys
d=json.load(open(sys.argv[1]));json.dump(d["manifest"],open(sys.argv[2]+"/m.json","w"))
print(d["slowIds"],d["completeIds"],d["cancelledExitCode"])
open(sys.argv[2]+"/slow","w").write(",".join(d["slowIds"]))
PY
CLI="$S/🔨️modules/📤️export/🎭️cast/🟦️.ts"
echo "== SIGTERM"
"$BUN" "$CLI" --manifest "$W/m.json" --ids "$(cat $W/slow)" --out "$W/out" 2> "$G/sigterm.err" &
P=$!
for i in $(seq 1 100); do grep -q '\[cast 1/' "$G/sigterm.err" 2>/dev/null && break; sleep 0.3; done
sleep 0.3; kill -TERM $P; wait $P; echo "exit=$?"
tail -3 "$G/sigterm.err"; echo "out:"; ls -A "$W/out" 2>&1
echo "== parent SIGKILL"
rm -rf "$W/out2"
setsid sleep 120 & PAR=$!
"$BUN" "$CLI" --manifest "$W/m.json" --ids "$(cat $W/slow)" --out "$W/out2" --parent $PAR 2> "$G/parent.err" &
C=$!
for i in $(seq 1 100); do grep -q '\[cast 1/' "$G/parent.err" 2>/dev/null && break; sleep 0.3; done
sleep 0.3; T0=$(date +%s); kill -KILL $PAR; wait $C; echo "exit=$? after $(( $(date +%s)-T0 ))s"
tail -2 "$G/parent.err"; ls -A "$W/out2" 2>&1
rm -rf "$W"
pgrep -fa "lx\.|🟦️.ts" || echo "no survivors"
