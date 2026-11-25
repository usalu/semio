#!/bin/zsh
# 🔗️ Small measuring steps in ONE queue grant of MEM (default 800) MB, one process after the other: chain-b.sh <step>… with the steps `column` (the plate variants of `steel-column` on trial copies, column-try.sh), `tour` (the loft trial of `tour-guide`: catalog copy and trial copy, guard, gate with twins, prints, validate; tour.sh) and `lateral` (the 15 assets that author `lateralScale`, the guard on the catalog document and on a copy without the member; output 🗑️generated/matrix/MX-2/lateral.txt).
self="${0:A}"; export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=${MEM:-800} -- "$self" --inside "$@"; fi
shift
P="$S/📦️packages/🟦️typescript"
for step in "$@"; do
  echo "[DEBUG] step $step $(date +%H:%M:%S)"
  case "$step" in
    column) "$W/column-try.sh" 0.035 0.02 ;;
    tour) "$W/tour.sh" ;;
    lateral) [[ -z "$LATERAL" ]] && { echo "[DEBUG] lateral is the job 50-lateral of the lane"; continue; }
      ids=($(python3 "$W/lateral.py" "$GEN/lateral"))
      : > "$GEN/lateral.txt"
      for a in $ids; do
        (cd "$P" && timeout 1200 "$B" "$W/guard.ts" "$a" --tag "read " 2>&1 | grep -v '^\[' ; timeout 1200 "$B" "$W/guard.ts" "$GEN/lateral/$a.json" --tag "ignored " 2>&1 | grep -v '^\[') >> "$GEN/lateral.txt"
        echo "[DEBUG] lateral $a $(date +%H:%M:%S) $(grep -c silhouette "$GEN/lateral.txt")"
      done
      grep silhouette "$GEN/lateral.txt" ;;
  esac
done
echo "[DEBUG] chain done $(date +%H:%M:%S)"
[[ -z "$LANE" ]] && { export B R S T U W M GEN LANE=b; source "$W/jobs.sh" b; }
