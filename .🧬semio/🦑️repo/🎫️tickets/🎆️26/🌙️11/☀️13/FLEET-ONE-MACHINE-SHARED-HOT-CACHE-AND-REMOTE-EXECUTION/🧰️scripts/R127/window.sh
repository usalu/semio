#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R127
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R127
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SCRIPT="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
$BUN $D/land.mjs --dry || { echo "dry failed"; exit 5; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R127 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
rm -rf $SC/backup
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
R127_SUBSET=1 $D/battery.sh $SC/after.txt
OK=1
python3 - <<PY || OK=0
def load(p):
    r={}
    for l in open(p,encoding="utf8"):
        if "::" in l:
            k,rc,_=l.split("::",2); r[k]=rc
    return r
a=load("$SC/before.txt");b=load("$SC/after.txt");bad=[k for k in b if b[k]!="rc=0" and a.get(k)=="rc=0"]
new=[k for k in b if k not in a and b[k]!="rc=0"]
print("regressions",bad,"new red",new)
raise SystemExit(1 if bad or new else 0)
PY
for f in "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts"; do
 $BUN "$SCRIPT" --file="$f" --birth=setup --root=/Users/ueli/Documents/semio > $SC/pre.txt 2>&1 || { echo "PREFLIGHT RED $f"; tail -5 $SC/pre.txt; OK=0; }
done
if [ $OK = 1 ]; then echo "GREEN started $START ended $(date -u +%H:%M:%S)"; else echo RED; $BUN $D/restore.mjs; fi
rm -r "$T/landing.lock"
