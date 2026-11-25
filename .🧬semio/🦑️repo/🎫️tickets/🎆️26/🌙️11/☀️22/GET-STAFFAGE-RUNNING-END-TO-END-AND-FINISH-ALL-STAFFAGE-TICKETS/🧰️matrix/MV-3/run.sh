#!/bin/zsh
# 🧵️ Runs several scripts of batch MV-3 one after the other under one grant: `grant.sh <MB> <timeout> run.sh '<script args…>' '<script args…>' …` (each argument is one command line of a script of this folder).
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
for line in "$@"; do words=(${=line}); echo "## $line"; zsh "$W/$words[1]" $words[2,-1]; done
