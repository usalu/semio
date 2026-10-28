#!/bin/sh
# 🧾 [rusttests] runs every declared file bare in the mirror (mrun plain), one line each + failing names. usage: runall.sh <mirror> <mode: plain|chain|env> [file-substring-filter]
C="$(cd "$(dirname "$0")/.." && pwd)"; m="$1"; mode="${2:-plain}"; flt="$3"
grep -v '^#' "$C/rusttests-tools/suites.tsv" | cut -f1 | while IFS= read -r f; do
  case "$f" in *"$flt"*) ;; *) continue;; esac
  printf '%s ' "$(echo "$f" | awk -F/ '{print $(NF-2)}')"; sh "$C/red-g-tools/mrun.sh" "$m" "$mode" "$f" --timeout 170000 2>&1 | head -12
done
