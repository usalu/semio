#!/bin/zsh
# 🚂️ Runs try.sh for the assets given, one after the other, each output in $GEN/trylog/<id>.out: try-all.sh <id>…
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN/trylog"
for a in "$@"; do s=$SECONDS; "$W/try.sh" "$a" > "$GEN/trylog/$a.out" 2>&1; echo "$a done $((SECONDS-s))s"; done
