#!/bin/zsh
# 🧾️ The whole proof list of the multiview brief for assets on their catalog documents, in ONE queue grant of 2500 MB, one process after the other, into 🗑️generated/matrix/MX-2/proof/<id>.out: the authors' gate (`T/🔎️scan.ts`), the fixer's gate over every subject (`MR/gate.ts --twins`), silhouette guard, `validateAsset` and three-views sweep (`MR/measure.ts`), the print check of every band and its range ends (`MC/prints.ts`), the check CLI, the real tests of the asset by name (`🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power`), the matrix of the asset (`T/📊️matrix.ts --fresh --jobs 1`) and the summary of its cells: prove.sh <id>…   (STEPS="scan gate measure prints check tests matrix" selects steps)
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=2500 -- "${0:A}" --inside "$@"; fi
shift
d="$GEN/proof"; mkdir -p "$d" "$GEN/vt"; P="$S/📦️packages/🟦️typescript"; MC="$U/🧰️matrix/MC"
steps="${STEPS:-scan gate measure prints check tests matrix}"
vt() { log="$GEN/vt/$1.log"; suite="$2"; shift 2; pat="(${(j:|:)@})${SUFFIX-( |$)}"; (cd "$P" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1; echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"; }
for a in "$@"; do
{
echo "== $a $(date +%H:%M) $(shasum -a 256 "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" | cut -c1-16)"
[[ " $steps " == *" scan "* ]] && { (cd "$T" && timeout 3000 "$B" 🔎️scan.ts --language en "$a") > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"; }
[[ " $steps " == *" gate "* ]] && { (cd "$T" && timeout 3600 "$B" "$M/gate.ts" "$a" --twins) > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"; }
[[ " $steps " == *" measure "* ]] && { (cd "$P" && timeout 2400 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; grep -E "1:|silhouette:|validate:|sweep:" "$d/$a-measure.log" | cut -c1-260; }
[[ " $steps " == *" prints "* ]] && { (cd "$T" && timeout 900 "$B" "$MC/prints.ts" "$a") 2>&1 | grep -v '^\[' | cut -c1-300; }
[[ " $steps " == *" check "* ]] && { (cd "$S" && timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"; }
if [[ " $steps " == *" tests "* ]]; then
vt "$a-silhouette" "🧊️catalog-silhouette" "$a"
vt "$a-three-views" "🧊️three-views" "$a"
vt "$a-physics" "🌡️catalog-physics" "$a"
SUFFIX=":" vt "$a-breathing" "🫁️breathing-catalog" "$a"
SUFFIX="" vt "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SUFFIX=":" vt "$a-power" "⚡️asset-power" "$a"
fi
if [[ " $steps " == *" matrix "* ]]; then
rm -rf "$GEN/matrix-$a"; (cd "$T" && timeout 14400 "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log" | cut -c1-200)"
python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -12 | cut -c1-240
fi
echo "== $a done $(date +%H:%M)"
} 2>&1 | tee "$d/$a.out"
done
