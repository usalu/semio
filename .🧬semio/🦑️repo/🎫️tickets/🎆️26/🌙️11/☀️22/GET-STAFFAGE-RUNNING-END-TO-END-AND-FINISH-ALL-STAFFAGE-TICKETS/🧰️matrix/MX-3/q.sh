#!/bin/zsh
# 🎟️ One command line inside one queue grant with the paths of the lane set: q.sh <MB> <command line for zsh>
source "${0:A:h}/env.sh"; mkdir -p "$GEN"
mb="$1"; shift
cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory="$mb" -- /bin/zsh -c "$*"
