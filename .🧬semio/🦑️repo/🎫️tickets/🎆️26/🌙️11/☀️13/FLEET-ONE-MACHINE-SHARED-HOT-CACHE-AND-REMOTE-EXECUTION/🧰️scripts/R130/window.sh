#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R130
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R130
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
$BUN $D/land.mjs --dry || { echo "dry failed"; exit 5; }
if [ -n "$R130_WAIT" ]; then python3 -c "
import time,datetime
now=datetime.datetime.utcnow()
t=now.replace(minute=0 if now.minute>=30 else 30,second=0,microsecond=0)
if now.minute>=30: t=t+datetime.timedelta(hours=1)
d=0 if now.minute%30<5 else (t-now).total_seconds()
print('waiting',d,'s for',t)
time.sleep(max(0,d+0.2))
"; fi
GOT=0; for try in $(seq 1 ${R130_TRIES:-1}); do
 M=$(date -u +%M); S=$(date -u +%S)
 { [ $((10#$M % 30)) -lt 2 ] || { [ $((10#$M % 30)) -eq 2 ] && [ $((10#$S)) -lt 30 ]; }; } || { echo "outside window"; exit 2; }
 if mkdir "$T/landing.lock" 2>/dev/null; then GOT=1; break; fi
 echo "held: $(cat $T/landing.lock/owner 2>/dev/null)"; python3 -c "import time;time.sleep(1)"
done
[ $GOT = 1 ] || exit 1
echo "claude-R130 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
rm -rf $SC/backup
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
PRE1=("$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts")
for f in "${PRE1[@]}"; do ( $BUN "$SCRIPT" --file="$f" --birth=setup --root=/Users/ueli/Documents/semio > "$SC/pre-$(echo $f | cksum | cut -d' ' -f1).txt" 2>&1; echo $? > "$SC/pre-$(echo $f | cksum | cut -d' ' -f1).rc" ) & done
R130_SUBSET=1 R130_JOBS=${R130_JOBS:-3} $D/battery.sh $SC/after.txt
wait
OK=1
python3 - <<PY || OK=0
def load(p):
    r={}
    for l in open(p,encoding="utf8"):
        if "::" in l:
            k,rc,s=l.split("::",2)
            import re
            m=re.search(r"(\d+) fail",s); r[k]=(rc,int(m.group(1)) if m else 0)
    return r
a=load("$SC/before2.txt");b=load("$SC/after.txt");bad=[k for k in b if k in a and b[k][0]!="rc=0" and (a[k][0]=="rc=0" or b[k][1]>a[k][1])]
new=[k for k in b if k not in a and b[k][0]!="rc=0"]
print("regressions",bad,"new red",new)
raise SystemExit(1 if bad or new else 0)
PY
for f in "${PRE1[@]}"; do rc=$(cat "$SC/pre-$(echo $f | cksum | cut -d' ' -f1).rc"); [ "$rc" = 0 ] || { echo "PREFLIGHT RED $f"; tail -5 "$SC/pre-$(echo $f | cksum | cut -d' ' -f1).txt"; OK=0; }; done
if [ $OK = 1 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo RED; $BUN $D/restore.mjs; fi
rm -r "$T/landing.lock"
