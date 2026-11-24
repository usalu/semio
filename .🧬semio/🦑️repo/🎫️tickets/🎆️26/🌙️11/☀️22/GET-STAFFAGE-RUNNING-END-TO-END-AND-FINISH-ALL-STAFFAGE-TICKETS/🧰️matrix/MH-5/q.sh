#!/bin/zsh
# 🚥️ Runs one heavy tool process of this fixer at a time: q.sh <command…> waits for the lock of the batch, then runs the command in the foreground (exactly one tool process of MH-5 on the shared host).
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN"
exec lockf -k "$GEN/.lock" "$@"
