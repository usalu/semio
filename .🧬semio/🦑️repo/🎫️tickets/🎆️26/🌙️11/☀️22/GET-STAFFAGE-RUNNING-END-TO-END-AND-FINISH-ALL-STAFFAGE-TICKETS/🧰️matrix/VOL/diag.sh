#!/bin/zsh
# 🔬️ files.ts in one grant of 800 MB, asked for again when the queue gives up: diag.sh <asset id>…
source "${0:A:h}/env.sh"; cd "$R"; mkdir -p "$GEN/run"; out="$GEN/run/files-$1.log"; : > "$out"
until SEMIO_LANGUAGE=en repo capacity run --memory=800 -- timeout 2400 "$B" "$W/files.ts" "$@" >> "$out" 2>&1 && grep -q ' geometry ' "$out"; do sleep 5; done
echo "files done" >> "$out"
