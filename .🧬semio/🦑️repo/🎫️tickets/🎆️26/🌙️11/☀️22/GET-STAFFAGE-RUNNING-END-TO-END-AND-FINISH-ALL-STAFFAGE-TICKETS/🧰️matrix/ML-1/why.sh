#!/bin/zsh
# 🩻️ Runs why.ts (the gate over poses, clip instants and transitions with the reasons of each hard code) through the memory queue and writes 🗑️generated/matrix/ML-1/why/<tag>.txt without the queue's waiting lines: why.sh <tag> <id|file.json>[,…] [flags…]
source "${0:A:h}/env.sh"
tag="$1"; shift; mkdir -p "$GEN/why"
q 1000 ${WHY_TIMEOUT:-3000} "$T" "$W/why.ts" "$@" 2>&1 | grep -v '^\[capacity\]' > "$GEN/why/$tag.txt"
echo "why $tag: $(tail -1 "$GEN/why/$tag.txt")"
