#!/bin/zsh
# 🩻️ Several small diagnoses under ONE grant of the memory queue, one after the other: diag.sh <megabytes> <timeout seconds> <file of command lines> runs every line of the file (each may use $W, $M, $GEN, $B, $S, $T and changes into its own folder) and prints its output under a heading; the queue is waited for once instead of once per line.
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory="$1" -- timeout "$2" zsh -c 'while IFS= read -r line; do [[ -z "$line" ]] && continue; echo "=== $line"; eval "$line" 2>&1; done < "$1"' zsh "$3"
