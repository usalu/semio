#!/bin/zsh
# ⏲️ Waits at most 170 s until one of the files named is no longer empty, then prints the time and which are: wt.sh <file>…
n=0
until [[ $n -ge 34 ]]; do for f in "$@"; do [[ -s "$f" ]] && break 2; done; sleep 5; n=$((n+1)); done
echo "$(date +%H:%M) $(for f in "$@"; do [[ -s "$f" ]] && echo "${f:t} ready"; done | tr '\n' ' ')"
