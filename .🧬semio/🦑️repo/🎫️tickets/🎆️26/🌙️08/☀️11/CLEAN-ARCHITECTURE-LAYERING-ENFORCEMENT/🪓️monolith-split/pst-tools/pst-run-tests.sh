#!/bin/zsh
# usage: pst-run-tests.sh <listfile> <outdir> [parallel]
list="$1"; out="$2"; par="${3:-4}"
mkdir -p "$out"; : > "$out/summary.txt"
tr '\n' '\0' < "$list" | xargs -0 -n 1 -P "$par" "$(dirname "$0")/pst-run-one.sh" "$out"
print done > "$out/DONE"
