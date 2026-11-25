#!/bin/zsh
# 🧾️ The proof steps of assets, one process after the other. It starts the pinned bun itself, so it must run INSIDE a grant of the memory queue: start it only through prove.sh. steps.sh <what,…> <id>…   with what out of scan, validate, print, check, suites, matrix (in this order); logs in 🗑️generated/matrix/MU/proof, one verdict line per step on stdout.
source "${0:A:h}/env.sh"
what="$1"; shift; d="$GEN/proof"; mkdir -p "$d"
has() { [[ ",$what," == *",$1,"* ]]; }
if has scan; then (cd "$T" && timeout 3000 "$B" 🔎️scan.ts --language en --fresh --out "$d/scan.jsonl" --report "$d/scan.json" "$@") > "$d/scan.log" 2>&1; echo "scan exit $? | $(tail -1 "$d/scan.log" | cut -c1-200)"; fi
if has validate; then (cd "$P" && timeout 1800 "$B" "$W/validate.ts" "$@") > "$d/validate.log" 2>&1; echo "validate: $(grep -c "	0 issues" "$d/validate.log") of $# with 0 issues"; grep -v "	0 issues" "$d/validate.log" | head -20; fi
if has print; then (cd "$T" && timeout 1800 "$B" "$W/print.ts" "$@") > "$d/print.log" 2>&1; echo "print: $(grep -c "every authored scale" "$d/print.log") of $# print at every authored scale and range end"; grep -v "every authored scale" "$d/print.log" | cut -c1-300 | head; fi
if has check; then for a in "$@"; do (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/check-$a.log" 2>&1; echo "check $a: $(grep -m1 -E '^(✓|✗)' "$d/check-$a.log" | cut -c1-160)"; done; fi
if has suites; then
  any="(${(j:|:)@})"
  run() {
    (cd "$P" && timeout ${VT_TIMEOUT:-3000} "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$1/🟦️.ts" -t "$2") > "$d/vt-$1.log" 2>&1
    echo "$1 exit $? | $(grep -E '^ *(Test Files|Tests) ' "$d/vt-$1.log" | tr -s ' ' | tr '\n' ';')"; grep -E "^ *(×|✗|FAIL)" "$d/vt-$1.log" | cut -c1-240 | sort -u | head -20
  }
  run "🧊️catalog-silhouette" "$any( |\$)"
  run "🧊️three-views" "$any( |\$)"
  run "🌡️catalog-physics" "$any( |\$)"
  run "🫁️breathing-catalog" "$any:"
  run "✅️asset-validation" "(accepts the sample figure|validates the manifest|$any)"
  run "⚡️asset-power" "$any:"
fi
if has matrix; then for a in "$@"; do
  out="$GEN/matrix/$a"; rm -rf "$out"; mkdir -p "$out"; start=$(date +%s)
  (cd "$T" && timeout ${MATRIX_TIMEOUT:-3600} "$B" 📊️matrix.ts --language en --out "$out" --jobs 1 --fresh --assets "$a") > "$out/run.log" 2>&1
  echo "matrix $a | exit $? | $(( $(date +%s) - start )) s | $(python3 "$M/cells.py" "$out")"
done; fi
