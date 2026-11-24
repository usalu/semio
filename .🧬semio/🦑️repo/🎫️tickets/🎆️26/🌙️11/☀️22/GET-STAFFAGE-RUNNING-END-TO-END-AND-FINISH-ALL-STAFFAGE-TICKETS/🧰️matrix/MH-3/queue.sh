#!/bin/zsh
# 📬️ Runs the dry run (try.sh) of several assets one after the other, each output in $GEN/try/<id>.out: GATE_FLAGS="" queue.sh <id>…
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
mkdir -p "$GEN/try"
for a in "$@"; do
  "$here/try.sh" "$a" > "$GEN/try/$a.out" 2>&1
  echo "$a done: $(grep -E '^gate exit' "$GEN/try/$a.out") $(grep -E 'silhouette:' "$GEN/try/$a.out")"
done
