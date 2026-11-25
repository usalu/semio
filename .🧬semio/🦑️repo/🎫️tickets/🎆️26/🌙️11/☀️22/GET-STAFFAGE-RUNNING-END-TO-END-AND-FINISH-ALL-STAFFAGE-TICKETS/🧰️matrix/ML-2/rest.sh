#!/bin/zsh
# 🧾️ The proof list of the multiview brief without scan and matrix (those are `measure.sh`) for several assets on their catalog documents in ONE queue grant of 2500 MB: per asset the fixer's gate over every subject (`MR/gate.ts --twins`), silhouette guard, `validateAsset` and three-views sweep (`MR/measure.ts`), the print check of every band and its range ends (`MC/prints.ts`) and the check CLI; then each real suite once with the name filter over all of them (`🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power`; the heater cases of `🧊️model-bounds` when `electric-fan-heater` is named), into 🗑️generated/matrix/ML-2/<tag>/: rest.sh <tag> <id>…
export BATCH=ML-2; source "${0:A:h}/../MR/env.sh"
self="${0:A}"; if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=2500 -- "$self" --inside "$@"; fi
shift; tag="$1"; shift; d="$GEN/$tag"; mkdir -p "$d"; P="$S/📦️packages/🟦️typescript"; MC="$U/🧰️matrix/MC"; out="$d/rest.txt"; : > "$out"
vt() { log="$d/vt-$1.log"; suite="$2"; shift 2; pat="(${(j:|:)@})${SUFFIX-( |$)}"; (cd "$P" && timeout 3000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1; echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" | tee -a "$out"; grep -E '✗|×|FAIL|AssertionError' "$log" | head -12 | cut -c1-300 | tee -a "$out"; }
for a in "$@"; do
{
echo "== $a $(date +%H:%M) $(shasum -a 256 "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" | cut -c1-16)"
(cd "$T" && timeout 3600 "$B" "$M/gate.ts" "$a" --twins) > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log" | cut -c1-200)"
(cd "$P" && timeout 2400 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; grep -E "1:|silhouette:|validate:|sweep:" "$d/$a-measure.log" | cut -c1-260
(cd "$T" && timeout 900 "$B" "$MC/prints.ts" "$a") 2>&1 | grep -v '^\[' | cut -c1-400
(cd "$S" && timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
} 2>&1 | tee -a "$out"
done
if [[ -z "$NO_SUITES" ]]; then
vt silhouette "🧊️catalog-silhouette" "$@"
vt three-views "🧊️three-views" "$@"
vt physics "🌡️catalog-physics" "$@"
SUFFIX=":" vt breathing "🫁️breathing-catalog" "$@"
SUFFIX="" vt validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$@"
SUFFIX=":" vt power "⚡️asset-power" "$@"
[[ " $* " == *" electric-fan-heater "* ]] && SUFFIX="" vt model-bounds "🧊️model-bounds" "heater"
fi
echo "[DEBUG] rest done $(date +%H:%M)" | tee -a "$out"
