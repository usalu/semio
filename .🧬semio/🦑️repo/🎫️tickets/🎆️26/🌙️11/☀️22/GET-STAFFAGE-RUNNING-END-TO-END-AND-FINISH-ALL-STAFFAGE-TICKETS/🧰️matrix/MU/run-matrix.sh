#!/bin/zsh
# 📊️ The coverage matrix of the suite over the whole batch, one asset process at a time, resumable (`T/📊️matrix.ts`, records in $GEN/matrix/cells): run-matrix.sh rigid|trees   (rigid: everything but the plants, small documents first is not possible, the generator keeps the catalog order; run it through q.sh)
source "${0:h}/env.sh"
kind="$1"
if [[ "$kind" == trees ]]; then ids=($(awk -F'\t' '$1=="vegetation"{print $2}' "$U/🧰️matrix/batches/MU.tsv")); else ids=($(awk -F'\t' '$1!="vegetation"{print $2}' "$U/🧰️matrix/batches/MU.tsv")); fi
mkdir -p "$GEN/matrix-$kind" "$GEN/log"
cd "$T" && "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$kind" --jobs 1 --timeout-s 14400 --assets "${(j:,:)ids}" >> "$GEN/log/matrix-$kind.log" 2>&1
