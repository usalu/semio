#!/bin/zsh
# 🔬️ One script of lane VOL in one grant of 800 MB, asked for again when the queue gives up: run1.sh <script.ts> <log> [args…]
source "${0:A:h}/env.sh"; cd "$R"; mkdir -p "$GEN/run"; s="$1"; out="$GEN/run/$2"; shift 2; : > "$out"
until SEMIO_LANGUAGE=en repo capacity run --memory=800 -- timeout 2400 "$B" "$W/$s" "$@" >> "$out" 2>&1 && ! tail -3 "$out" | grep -q 'error:'; do sleep 5; done
echo "run done" >> "$out"
