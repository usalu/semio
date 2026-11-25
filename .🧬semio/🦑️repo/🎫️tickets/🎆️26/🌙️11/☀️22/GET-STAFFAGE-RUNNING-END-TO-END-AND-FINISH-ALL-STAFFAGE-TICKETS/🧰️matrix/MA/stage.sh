#!/bin/zsh
# 🎬️ One proof or work stage over several assets as ONE process tree, made to be started through the machine-wide memory queue from the repo root (`q.sh`): every stage changes to the folder its tool needs, runs its bun processes one after the other and writes its logs to `$GEN/proof`. stage.sh <stage> <id>…
#   apply     the repair on the catalog files: `fit.ts <id> --ops ops/<id>.json --turns turns/<id>.json --write` until a run changes nothing, then compared with the trial copy
#   gate      `../MR/gate.ts <id> --twins` per asset (poses, clip instants, transitions)
#   scan      the official `T/🔎️scan.ts --language en <ids>`
#   measure   `../MR/measure.ts` (silhouette guard, validateAsset, three-views sweep) of backup and catalog
#   check     the product check CLI, `--asset <id>` per asset
#   vt-<suite> one real test suite with the name filter of the assets: silhouette, three-views, physics, breathing, validation, power
#   matrix    `T/📊️matrix.ts --language en --jobs 1 --fresh --assets <ids>` into `$GEN/matrix`
#   sheets    product sheets (side, front, plan) of every authored scale, backup and catalog, default pose and the poses of POSES="a b"
BATCH=MA source "${0:A:h}/../MR/env.sh"
stage="$1"; shift; d="$GEN/proof"; mkdir -p "$d"
case "$stage" in
  apply)
    for a in "$@"; do
      for i in 1 2 3 4; do (cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" --turns "$W/turns/$a.json" --limit ${LIMIT:-0.75} --write > "$d/$a-apply$i.log" 2>&1); grep -q " 0 changed something" "$d/$a-apply$i.log" && break; done
      echo "$a: run $i $(tail -1 "$d/$a-apply$i.log") | $(cmp -s "$GEN/try/$a.json" "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" && echo "catalog equals the trial copy" || echo "CATALOG DIFFERS FROM THE TRIAL COPY")"
    done ;;
  gate) for a in "$@"; do (cd "$T" && timeout 1800 "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1); echo "$a gate exit $? $(tail -1 "$d/$a-gate.log")"; done ;;
  scan) (cd "$T" && timeout 3600 "$B" 🔎️scan.ts --language en "$@" > "$d/scan.log" 2>&1); echo "scan exit $? $(tail -1 "$d/scan.log" | cut -c1-200)" ;;
  measure) for a in "$@"; do (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a" > "$d/$a-measure.log" 2>&1); grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" | tr '\n' ';'; echo; done ;;
  check) for a in "$@"; do (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1); echo "check exit $?: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"; done ;;
  vt-*)
    case "${stage#vt-}" in
      silhouette) suite="🧊️catalog-silhouette"; pat="(${(j:|:)@})( |$)" ;;
      three-views) suite="🧊️three-views"; pat="(${(j:|:)@})( |$)" ;;
      physics) suite="🌡️catalog-physics"; pat="(${(j:|:)@})( |$)" ;;
      breathing) suite="🫁️breathing-catalog"; pat="(${(j:|:)@}):" ;;
      validation) suite="✅️asset-validation"; pat="(accepts the sample figure|validates the manifest|${(j:|:)@})" ;;
      power) suite="⚡️asset-power"; pat="(${(j:|:)@}):" ;;
    esac
    (cd "$S/📦️packages/🟦️typescript" && timeout 5400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$d/$stage.log" 2>&1)
    echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$d/$stage.log" | tr -s ' ' | tr '\n' ';')" ;;
  matrix)
    (cd "$T" && timeout ${MATRIX_TIMEOUT:-30000} "$B" 📊️matrix.ts --language en --out "$GEN/matrix" --jobs 1 --fresh --assets "${(j:,:)@}" > "$d/matrix.log" 2>&1); echo "matrix exit $? $(tail -1 "$d/matrix.log" | cut -c1-300)"
    python3 "$M/cells.py" "$GEN/matrix" > "$d/matrix-cells.txt" 2>&1; head -3 "$d/matrix-cells.txt" ;;
  sheets)
    for a in "$@"; do
      scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$GEN/backup/$a.json"))
      TAG=before BATCH=MA "$M/look.sh" "$a" $scales; BATCH=MA "$M/look.sh" "$a" $scales
      for pose in ${=POSES}; do TAG=before POSE=$pose BATCH=MA "$M/look.sh" "$a" $scales[1] $scales[-1]; POSE=$pose BATCH=MA "$M/look.sh" "$a" $scales[1] $scales[-1]; done
    done ;;
  *) echo "unknown stage $stage"; exit 2 ;;
esac
