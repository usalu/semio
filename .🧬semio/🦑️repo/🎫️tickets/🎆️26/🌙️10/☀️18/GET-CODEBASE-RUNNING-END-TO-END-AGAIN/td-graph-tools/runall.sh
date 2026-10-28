#!/bin/sh
# 🧾 [td-graph] runs every suite of suites.txt (or the list in $3) single-file at <root>, logs to <outdir>/<n>.log; usage: runall.sh <root> <outdir> [list]
C="$(cd "$(dirname "$0")/.." && pwd)"; R="$1"; O="$2"; LIST="${3:-$C/td-graph-tools/suites.txt}"; i=0
while IFS= read -r f; do i=$((i+1)); [ -n "$f" ] && sh "$C/td-graph-tools/run1.sh" "$R" "$f" "$O/$i.log" | sed 's#^🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/##'; done < "$LIST"
