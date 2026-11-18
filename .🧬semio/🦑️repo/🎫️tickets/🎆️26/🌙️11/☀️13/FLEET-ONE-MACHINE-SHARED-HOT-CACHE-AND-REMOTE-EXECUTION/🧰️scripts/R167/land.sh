#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
D=$T/🧰️scripts/R167
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R167
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B="${L}🏗️build/"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("🏗️build/🕸️graph/📥️load/🟦️.ts" "🏗️build/🕸️graph/🔁retry/🧬️schema/🔣️.json" "🏗️build/🧬️schema/📥️authority.json" "🏗️build/📏️workspace-scale/🧬️schema/🔣️.json" "🏗️build/🧪️tests/🕸️graph/🔄️candidate/🟦️.ts" "🏗️build/🧪️tests/🕸️graph/🔄️candidate/🔣️.json" "🏗️build/🧪️tests/🕸️graph/🔄️candidate/🧬️schema/🔣️.json")
run(){ (cd "$1" && shift && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 "$@" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)" | tr -d ' \n'); }
mkdir "$T/landing.lock" 2>/dev/null || { echo "lock held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R167 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
echo "start $(date -u +%H:%M:%S)"
rm -rf $S/bk; mkdir -p $S/bk; i=0; for f in $FILES; do cp "$L$f" "$S/bk/$i"; i=$((i+1)); done
restore(){ i=0; for f in $FILES; do cp "$S/bk/$i" "$L$f"; i=$((i+1)); done; echo "restored"; }
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; echo "released $(date -u +%H:%M:%S)"; }
python3 $D/patches.py apply "${B}🕸️graph/📥️load/🟦️.ts" || { restore; release; exit 2; }
python3 $D/scale.py "$L" || { restore; release; exit 3; }
cp $S/stage/candidate.test.ts "${B}🧪️tests/🕸️graph/🔄️candidate/🟦️.ts"; cp $S/stage/corpus.json "${B}🧪️tests/🕸️graph/🔄️candidate/🔣️.json"; cp $S/stage/corpus.schema.json "${B}🧪️tests/🕸️graph/🔄️candidate/🧬️schema/🔣️.json"
mkdir -p "${B}🧪️tests/🕸️graph/🧲️adopt-registry"; cp $S/stage/law/law.ts "${B}🧪️tests/🕸️graph/🧲️adopt-registry/🟦️.ts"
r1=$(run "${B}🧪️tests/🕸️graph/🔄️candidate"); r2=$(run "${B}🧪️tests/🕸️graph/🧲️adopt-registry"); r3=$(run "${B}🧪️tests/🕸️graph/🌊️seal-under-churn"); r4=$(run "${B}🧪️tests/🕸️graph/🩹️repair-rounds"); r5=$(run "${L}⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth" -t ceiling); r6=$(run "${B}🕸️graph/🔁retry/🧪️tests/🧮️projection")
echo "candidate=$r1 (baseline 1pass2fail before; 2 fail are the inventory model) adopt=$r2 churn=$r3 repair=$r4 growth-ceiling=$r5 projection=$r6"
ok=1; [[ $r1 == 1pass2fail ]] || ok=0; [[ $r2 == 3pass0fail ]] || ok=0; [[ $r3 == 3pass0fail ]] || ok=0; [[ $r4 == 11pass0fail ]] || ok=0; [[ $r5 == *0fail ]] || ok=0; [[ $r6 == 4pass0fail ]] || ok=0
if [ $ok = 0 ]; then restore; rm -f "${B}🧪️tests/🕸️graph/🧲️adopt-registry/🟦️.ts"; fi
release; echo "ok=$ok"
