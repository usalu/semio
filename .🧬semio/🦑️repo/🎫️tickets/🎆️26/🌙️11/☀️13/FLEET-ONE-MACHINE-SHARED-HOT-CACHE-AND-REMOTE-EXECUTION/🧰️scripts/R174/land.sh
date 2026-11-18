#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R174
D=🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📂️source-names
BASE_MJS=$(cat $S/base.mjs.sha); BASE_JSON=$(cat $S/base.json.sha)
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R174 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ cp $S/bk/mjs "$D/🟨️.mjs"; cp $S/bk/json "$D/🧬️schema/🔣️.json"; }
fail(){ echo "RED: $1"; restore; echo "restored $(date -u +%H:%M:%S) mjs=$(shasum -a 256 "$D/🟨️.mjs"|cut -c1-64) json=$(shasum -a 256 "$D/🧬️schema/🔣️.json"|cut -c1-64)"; release; exit 3; }
[ "$(shasum -a 256 "$D/🟨️.mjs"|cut -c1-64)" = "$BASE_MJS" ] && [ "$(shasum -a 256 "$D/🧬️schema/🔣️.json"|cut -c1-64)" = "$BASE_JSON" ] || { echo "live differs from base"; release; exit 2; }
mkdir -p $S/bk; cp "$D/🟨️.mjs" $S/bk/mjs; cp "$D/🧬️schema/🔣️.json" $S/bk/json
cp "$S/stage/$D/🟨️.mjs" "$D/🟨️.mjs"; cp "$S/stage/$D/🧬️schema/🔣️.json" "$D/🧬️schema/🔣️.json"
$S/law.sh 80 > $S/after-law.txt; cat $S/after-law.txt | grep -E "^ *[0-9]+ (pass|fail)|^\(fail\)"
grep -q "^ 3 pass" $S/after-law.txt && grep -q "^ 0 fail" $S/after-law.txt || fail law
$S/srcnames.sh > $S/after-sn.txt; cat $S/after-sn.txt
grep -q "8 pass" $S/after-sn.txt && grep -q " 2 fail" $S/after-sn.txt || fail source-names
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
