#!/bin/zsh
# 🧾️ The proof of several written assets in three grants of the memory queue (proof.sh needs twelve per asset): in-prove.sh (2500 MB: gates, scan, measure, suitability, the six test files), in-check.sh (5000 MB: the check CLI per asset) and with MATRIX=1 ONE `T/📊️matrix.ts --assets a,b,… --jobs 1 --fresh` (7000 MB; each asset in a process of its own, one after the other) with the cells per asset by status and failing check: TAG=<name> REGATE="<id> …" MATRIX=1 prove-many.sh <id>…   (logs in $GEN/proof, the matrix in $GEN/matrix-<TAG>)
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
tag="${TAG:-many}"; export TAG="$tag"
[[ -z "$NOLIGHT" ]] && QMEM=2500 QTIME=${LIGHT_TIMEOUT:-14400} "$here/qsh" "$here/in-prove.sh" "$@" 2>&1 | grep -v '^\[capacity\]'
[[ -z "$NOCHECK" ]] && QMEM=5000 QTIME=${CHECK_TIMEOUT:-10800} "$here/qsh" "$here/in-check.sh" "$@" 2>&1 | grep -v '^\[capacity\]'
if [[ -n "$MATRIX" ]]; then
  (cd "$T" && QMEM=7000 QTIME=${MATRIX_TIMEOUT:-28800} "$here/qbun" 📊️matrix.ts --language en --out "$GEN/matrix-$tag" --jobs 1 --fresh --assets "${(j:,:)@}") > "$GEN/proof/$tag-matrix.log" 2>&1; echo "matrix exit $? $(grep -v '^\[capacity\]' "$GEN/proof/$tag-matrix.log" | tail -1 | cut -c1-200)"
  python3 "$here/cells-by-asset.py" "$GEN/matrix-$tag" 2>&1 | cut -c1-300
fi
