#!/bin/zsh
# 🧮️ The matrix confirmation of one asset with its memory measured: starts the sampler rss.sh on the output folder of the asset, runs the matrix step of proof.sh under the reservation MATRIX_MB (default 1200 MB, the queue rule of 2026-11-25 09:20) and appends the peak to the proof file: mx.sh <id>
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; a="$1"; mkdir -p "$GEN/t2"
: > "$GEN/t2/rss-$a.log"
"$W/rss.sh" "matrix-$a" "$GEN/t2/rss-$a.log" &
STEPS=matrix MATRIX_MB="${MATRIX_MB:-1200}" "$W/proof.sh" "$a"
echo "matrix memory: $(awk '{if($3>s)s=$3; if($4>m)m=$4} END{printf "peak sum %d MB, largest process %d MB, %d samples", s, m, NR}' "$GEN/t2/rss-$a.log")" >> "$GEN/proof/$a.out"
