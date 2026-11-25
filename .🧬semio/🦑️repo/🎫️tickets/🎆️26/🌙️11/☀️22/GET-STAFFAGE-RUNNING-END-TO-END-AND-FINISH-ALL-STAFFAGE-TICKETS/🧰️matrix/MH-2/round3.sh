#!/bin/zsh
# 🧮️ The proof round of batch MH-2 under the memory queue (every tool process through `q.sh`, one after the other), ordered so that the cheap proofs of every asset are in before the matrices: phase A per asset: the repair applied once more (`apply.sh`, must change nothing or, for an asset not yet written, write it), the gate over every pose, clip instant and transition (`../MR/gate.ts --twins`), the official scan, the check CLI and ONE vitest run over the six real test files (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power) filtered by the asset id and the two manifest cases; phase B: the sheets before and after of the scales the ops name, with their pixel difference; phase C: `T/📊️matrix.ts --assets <id>` for the assets named in MATRIX_IDS. Blocks are appended to $GEN/proof/summary.txt. round3.sh <id>…   (PHASES="A B C" selects)
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d" "$GEN/vt" "$GEN/sheets"; sum="$d/summary.txt"; P="$S/📦️packages/🟦️typescript"; phases="${PHASES:-A B C}"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
if [[ " $phases " == *" A "* ]]; then for a in "$@"; do {
  echo "=== A $a $(date +%H:%M)"
  "$W/apply.sh" "$a" | cut -c1-200
  "$W/q.sh" 2500 3600 "$T" "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"
  "$W/q.sh" 1000 1800 "$T" "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
  "$W/q.sh" 2500 3600 "$P" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🧊️catalog-silhouette/🟦️.ts" "../../🧪️tests/🧊️three-views/🟦️.ts" "../../🧪️tests/🌡️catalog-physics/🟦️.ts" "../../🧪️tests/🫁️breathing-catalog/🟦️.ts" "../../🧪️tests/✅️asset-validation/🟦️.ts" "../../🧪️tests/⚡️asset-power/🟦️.ts" -t "(accepts the sample figure|validates the manifest|$a)" > "$GEN/vt/$a-tests.log" 2>&1
  echo "tests exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/$a-tests.log" | tr -s ' ' | tr '\n' ';') $(grep -E '^ *(✓|❯|×|FAIL) ' "$GEN/vt/$a-tests.log" | sed -E 's/ +[0-9]+ms$//' | tr -s ' ' | tr '\n' ';' | cut -c1-600)"
  "$W/q.sh" 5000 1800 "$S" "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
  echo "--- A $a $(date +%H:%M)"
} >> "$sum" 2>&1; done; fi
if [[ " $phases " == *" B "* ]]; then for a in "$@"; do {
  echo "=== B $a $(date +%H:%M)"
  pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$GEN/backup/$a.json")"; n=0
  for s in $(python3 -c "import json,sys; print(' '.join(sorted({o['scale'] for o in json.load(open(sys.argv[1])) if 'scale' in o}, key=int)))" "$W/ops/$a.json"); do
    for tag in before after; do
      [[ -f "$GEN/sheets/$a.s$s.$pose.$tag.png" && $tag == before ]] && { n=$((n+1)); continue; }
      src="$GEN/backup/$a.json"; [[ $tag == after ]] && src="$S/🗂️catalog/👤️humans/$a/🔣️.json"
      work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"; cp "$src" "$work/🔣️.json"
      png="$("$W/q.sh" 1000 600 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>/dev/null | tail -1)"
      [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$a.s$s.$pose.$tag.png" && n=$((n+1)); rm -rf "$GEN/sheets/work-$tag"
    done
  done
  echo "sheets drawn: $n"; "$R/.venv/bin/python3" -W ignore "$W/sheetdiff.py" "$GEN/sheets" "$a"
  echo "--- B $a $(date +%H:%M)"
} >> "$sum" 2>&1; done; fi
if [[ " $phases " == *" C "* ]]; then for a in ${=MATRIX_IDS}; do {
  echo "=== C $a $(date +%H:%M)"
  rm -rf "$GEN/matrix-$a"; "$W/q.sh" 7000 ${MATRIX_TIMEOUT:-10800} "$T" "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log")"
  python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-260
  echo "--- C $a $(date +%H:%M)"
} >> "$sum" 2>&1; done; fi
echo "ROUND3DONE $(date +%H:%M)" >> "$sum"
