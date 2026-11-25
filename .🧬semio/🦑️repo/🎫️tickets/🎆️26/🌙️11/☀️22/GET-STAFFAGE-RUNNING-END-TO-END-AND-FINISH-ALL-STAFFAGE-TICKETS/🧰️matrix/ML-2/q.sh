#!/bin/zsh
# 🎟️ Runs one command line inside a queue grant with the paths of the batch set (B, R, S, T, U, W, M, GEN): q.sh <MB> <command line for zsh>
export BATCH=ML-2; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN"
mb="$1"; shift
cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory="$mb" -- /bin/zsh -c "$*"
