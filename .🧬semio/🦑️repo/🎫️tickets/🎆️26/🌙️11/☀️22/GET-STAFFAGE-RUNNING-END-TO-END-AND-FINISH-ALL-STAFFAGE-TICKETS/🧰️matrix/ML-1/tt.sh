#!/bin/zsh
# 🧫️ Runs trial.ts for several assets one after the other under one grant of the memory queue (2500 MB) and writes 🗑️generated/matrix/ML-1/try/<id>.<tag>.txt each: tt.sh <tag> "<trial flags>" <id>…
source "${0:A:h}/env.sh"
tag="$1"; flags="$2"; shift 2; mkdir -p "$GEN/try"
cmd="cd '$P'"
for a in "$@"; do cmd="$cmd; '$B' '$W/trial.ts' $a $flags > '$GEN/try/$a.$tag.txt' 2>&1"; done
qs ${MB:-2500} ${TO:-5400} "$cmd" > "$GEN/try/queue.$tag.log" 2>&1
for a in "$@"; do echo "trial $a $tag: $(grep -E 'hard findings|the chain|^print|^measure' "$GEN/try/$a.$tag.txt" | tr '\n' ';' | cut -c1-400)"; done
