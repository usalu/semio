#!/bin/zsh
# ⏳️ Waits up to 170 s until a log of 🗑️generated/silhouette/SH-1 has grown by <n> lines and prints the new lines: wait.sh <file> <n>
source "${0:A:h}/env.sh"
f="$GEN/$1"; n=$(wc -l < "$f" 2>/dev/null || echo 0); end=$((SECONDS+170))
while (( SECONDS < end )) && [[ $(wc -l < "$f" 2>/dev/null || echo 0) -lt $((n+$2)) ]]; do sleep 5; done
tail -n +$((n+1)) "$f"
