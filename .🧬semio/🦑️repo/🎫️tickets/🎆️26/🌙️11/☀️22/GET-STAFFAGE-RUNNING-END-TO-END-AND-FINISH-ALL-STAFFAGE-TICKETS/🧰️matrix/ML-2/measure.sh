#!/bin/zsh
# 📏️ State of assets on their catalog documents in ONE queue grant of 800 MB (rigid assets, figures of MM): the authors' gate (`T/🔎️scan.ts`) over all of them, then the matrix of each (`T/📊️matrix.ts --fresh --jobs 1`) with the summary of its cells, into 🗑️generated/matrix/ML-2/<tag>/: measure.sh <tag> <id>…
export BATCH=ML-2; source "${0:A:h}/../MR/env.sh"
self="${0:A}"; if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- "$self" --inside "$@"; fi
shift; tag="$1"; shift; d="$GEN/$tag"; mkdir -p "$d"
(cd "$T" && timeout 3000 "$B" 🔎️scan.ts --language en "$@") > "$d/scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/scan.log" | cut -c1-200)" | tee "$d/summary.txt"
for a in "$@"; do
  rm -rf "$d/matrix-$a"; (cd "$T" && timeout 7200 "$B" 📊️matrix.ts --language en --out "$d/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1; rc=$?
  { echo "== $a $(date +%H:%M) $(shasum -a 256 "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" | cut -c1-16) matrix exit $rc $(tail -1 "$d/$a-matrix.log" | cut -c1-200)"; python3 "$M/cells.py" "$d/matrix-$a" 2>&1 | cut -c1-330; } | tee -a "$d/summary.txt"
done
echo "[DEBUG] measure done $(date +%H:%M)" | tee -a "$d/summary.txt"
