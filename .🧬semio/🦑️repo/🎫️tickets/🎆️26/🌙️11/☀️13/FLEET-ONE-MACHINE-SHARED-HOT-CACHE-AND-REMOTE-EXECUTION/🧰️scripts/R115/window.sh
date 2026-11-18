#!/bin/zsh
R=/Users/ueli/Documents/semio
T=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R115
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R115
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=$(cat $SC/script.path)
LIB=$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
G=$LIB/🏗️build/🕸️graph
W=$G/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm
PN=$G/📥️inference/📦️project-nodes
m=$((10#$(date -u +%M)))
if ! { (( m <= 4 )) || (( m >= 30 && m <= 34 )); }; then echo "outside the landing window"; exit 2; fi
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R115 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
echo "start $(date -u)"
mkdir -p $SC/backup
cp $PN/🟨️.mjs $SC/backup/pn.mjs; cp $PN/🧪️tests/🚪️away/🟦️.ts $SC/backup/away.ts; cp $W/👷️child/🟨️.mjs $SC/backup/child.mjs
shasum -a 256 $PN/🟨️.mjs $W/👷️child/🟨️.mjs $PN/🧪️tests/🚪️away/🟦️.ts | cut -c1-12
t() { (cd $R && $BUN $S --file="$1" --birth=setup --root=$R 2>&1 | grep -E "^ *[0-9]+ (pass|fail)|^\(fail\)|build:genuine-test" | tr '\n' ' '; echo " <- ${1:t:h:t}"); }
restore() {
  cp $SC/backup/pn.mjs $PN/🟨️.mjs; cp $SC/backup/away.ts $PN/🧪️tests/🚪️away/🟦️.ts; cp $SC/backup/child.mjs $W/👷️child/🟨️.mjs
  rm -f "${PN:?}/🧪️tests/🧭️stage-names/🟦️.ts" "${W:?}/🧪️tests/🪦️busy-orphan/🟦️.ts" "${W:?}/🧪️tests/🪦️busy-orphan/🪦️busy-deriver.mjs"
  rmdir "${PN:?}/🧪️tests/🧭️stage-names" "${W:?}/🧪️tests/🪦️busy-orphan" 2>/dev/null
}
mkdir -p $PN/🧪️tests/🧭️stage-names $W/🧪️tests/🪦️busy-orphan
cp $D/stage-names.test.ts $PN/🧪️tests/🧭️stage-names/🟦️.ts
cp $SC/stage/test/🟦️.ts $SC/stage/test/🪦️busy-deriver.mjs $W/🧪️tests/🪦️busy-orphan/
echo "--- red (before)"; t $PN/🧪️tests/🧭️stage-names/🟦️.ts; t $W/🧪️tests/🪦️busy-orphan/🟦️.ts
python3 $D/stage-patch.py $PN/🟨️.mjs || { restore; rm -r "$T/landing.lock"; exit 3; }
python3 - "$PN/🧪️tests/🚪️away/🟦️.ts" <<'PY'
import sys
p=sys.argv[1];s=open(p,encoding="utf8").read()
a='return away(()=>farm.prepare('
assert s.count(a)==1
open(p,"w",encoding="utf8").write(s.replace(a,'return stagedPrepareV1(name,away,()=>farm.prepare('))
PY
cp $SC/stage/child/new.mjs $W/👷️child/🟨️.mjs
echo "--- green (after)"
OUT=$SC/after.txt; : > $OUT
for f in \
  $PN/🧪️tests/🧭️stage-names/🟦️.ts $W/🧪️tests/🪦️busy-orphan/🟦️.ts $W/🧪️tests/🟦️.ts $PN/🧪️tests/🚪️away/🟦️.ts \
  $PN/🧪️tests/📈️inferred/🟦️.ts $PN/🧪️tests/🐌️slow-last/🟦️.ts $PN/🧪️tests/🛣️lanes/🟦️.ts $PN/🧪️tests/🪂️set-aside/🟦️.ts \
  $PN/🧪️tests/🚦️governed-dispatch/🟦️.ts $PN/🧪️tests/🛑️too-heavy/🟦️.ts $PN/🧪️tests/🧹️parent-budget/🟦️.ts $PN/🧪️tests/🧵️ready-context/🟦️.ts \
  $G/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts $G/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts; do
  [ -f "$f" ] && t "$f" | tee -a $OUT
done
if python3 $D/compare.py $SC/before.txt $OUT; then echo GREEN; else echo RED; restore; fi
rm -r "$T/landing.lock"
echo "end $(date -u)"
