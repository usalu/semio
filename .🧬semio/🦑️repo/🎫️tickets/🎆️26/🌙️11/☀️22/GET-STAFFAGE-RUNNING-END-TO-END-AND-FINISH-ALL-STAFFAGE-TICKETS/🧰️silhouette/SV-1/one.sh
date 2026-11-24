#!/bin/zsh
# 🛠️ The repair of one plant up to a candidate file, one process after the other: bounding box (bbox.ts), drawing (fix.ts), the ops replayed on a copy in 🗑️generated/silhouette/SV-1/work/<id>, then the gate with every volume rule and `validateAsset` on that copy, each compared with the catalog file as it was before, the print suitability and the sheets of 1:50 and 1:500. Nothing is written to the catalog. one.sh <id> [bbox args…]   (SKIP_BBOX=1 leaves the bounding box alone, FIXARGS passes arguments to fix.ts)
source "${0:A:h}/env.sh"; cd "$W"
id="$1"; shift
mkdir -p "$GEN/fix" "$GEN/work/$id" "$GEN/cand" "$GEN/validate"
mkdir -p "$GEN/basevol"
[[ -s "$GEN/basevol/$id.log" ]] || timeout 2400 "$B" measure.ts --volumes "$id" > "$GEN/basevol/$id.log" 2>&1
[[ -s "$GEN/validate/before-$id.txt" ]] || timeout 900 "$B" validate.ts "$id" "$GEN/validate/before-$id.txt" | head -1
[[ -n "$SKIP_BBOX" ]] || timeout 1800 "$B" bbox.ts "$id" "$@" 2>&1 | tee "$GEN/fix/$id-bbox.log" | cut -c1-400
timeout 3000 "$B" fix.ts "$id" ${=FIXARGS} > "$GEN/fix/$id.log" 2>&1
grep -E "round|nothing|DEBUG|rror|no template" "$GEN/fix/$id.log" | cut -c1-220
timeout 600 "$B" apply.ts "$id" --out "$GEN/work/$id/🔣️.json" | cut -c1-200
timeout 1800 "$B" measure.ts --volumes "$GEN/work/$id/🔣️.json" > "$GEN/cand/$id.log" 2>&1; grep -v "^   " "$GEN/cand/$id.log" | cut -c1-330
echo "volume rules new in the candidate:"; diff <(grep "^   " "$GEN/basevol/$id.log" | sed -E "s/[0-9]+\.[0-9]+/#/g") <(grep "^   " "$GEN/cand/$id.log" | sed -E "s/[0-9]+\.[0-9]+/#/g") | grep "^>" | cut -c1-420
timeout 900 "$B" validate.ts "$GEN/work/$id/🔣️.json" "$GEN/validate/cand-$id.txt"
echo "validateAsset issues new in the candidate:"; diff "$GEN/validate/before-$id.txt" "$GEN/validate/cand-$id.txt" | grep "^>" | cut -c1-300
N=4 timeout 1800 "$B" suit.ts "$GEN/work/$id/🔣️.json" 2>&1 | grep -v ": authored$" | cut -c1-300
for s in 50 500; do ./look.sh cand "$id" $s "$GEN/work/$id/🔣️.json" | sed 's|.*/||'; done
