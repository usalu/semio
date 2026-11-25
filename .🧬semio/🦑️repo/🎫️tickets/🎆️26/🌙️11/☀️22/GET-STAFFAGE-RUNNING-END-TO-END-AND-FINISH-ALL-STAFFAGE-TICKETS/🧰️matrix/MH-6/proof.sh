#!/bin/zsh
# 🧾️ The whole proof of one asset, into $GEN/proof/<id>.out, every run through the memory queue (cap.sh): the gate over every pose, clip instant and transition (not run a second time when the catalog file is byte for byte the trial copy $GEN/try/<id>.json whose gate log ran to its end in this session; the log is then cited), the official scan, silhouette, validate and sweep, the check CLI, the real tests with a name filter, the matrix of the asset and the summary of its cells. proof.sh <id>   (STEPS="matrix" repeats only the named steps: gate scan measure check tests matrix; reservations of the queue rule of 2026-11-25 09:20: 800 MB gate, scan and measure, 1000 MB check CLI, 2500 MB vitest, MATRIX_MB or 1200 MB matrix)
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; a="$1"; d="$GEN/proof"; mkdir -p "$d" "$GEN/vt"
steps="${STEPS:-gate scan measure check tests matrix}"
[[ -z "$STEPS" ]] && : > "$d/$a.out"
vt() { log="$GEN/vt/$1.log"; suite="$2"; shift 2; pat="(${(j:|:)@})${SUFFIX-( |$)}"; "$W/cap.sh" 2500 2400 "$S/📦️packages/🟦️typescript" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$log" 2>&1; echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"; }
{
f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
if [[ " $steps " == *" gate "* ]]; then
if cmp -s "$f" "$GEN/try/$a.json" && grep -q "^hard findings:" "$GEN/trylog/$a.gate.log"; then echo "gate (trial copy identical to the catalog file) $(tail -1 "$GEN/trylog/$a.gate.log")"; cp "$GEN/trylog/$a.gate.log" "$d/$a-gate.log"
else "$W/cap.sh" 800 3600 "$T" "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"; fi; fi
if [[ " $steps " == *" scan "* ]]; then "$W/cap.sh" 800 1800 "$T" "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"; fi
if [[ " $steps " == *" measure "* ]]; then "$W/cap.sh" 800 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log"; fi
if [[ " $steps " == *" check "* ]]; then "$W/cap.sh" 1000 1800 "$S" "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"; fi
if [[ " $steps " == *" tests "* ]]; then
vt "$a-silhouette" "🧊️catalog-silhouette" "$a"
vt "$a-three-views" "🧊️three-views" "$a"
vt "$a-physics" "🌡️catalog-physics" "$a"
SUFFIX=":" vt "$a-breathing" "🫁️breathing-catalog" "$a"
SUFFIX="" vt "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SUFFIX=":" vt "$a-power" "⚡️asset-power" "$a"
fi
if [[ " $steps " == *" matrix "* ]]; then
rm -rf "$GEN/matrix-$a"; "$W/cap.sh" "${MATRIX_MB:-1200}" 14400 "$T" "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log")"
python3 "$M/cells.py" "$GEN/matrix-$a"
fi
} >> "$d/$a.out" 2>&1
