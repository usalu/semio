#!/bin/zsh
# 📋️ One line per asset: short.sh [wait-seconds]   waits like status.sh, then prints for every asset the time of its newest output and its last verdict line (hard findings, READY, STOP, matrix, done).
source "${0:A:h}/env.sh"
"$W/status.sh" "${1:-0}" > /dev/null
date +%H:%M
for f in "$GEN"/do/*.out; do echo "${f:t:r} $(stat -f %Sm -t %H:%M "$f") | $(grep -E "^hard findings|^READY|^STOP|^matrix exit|^status|done [0-9]|^== " "$f" | tail -2 | tr '\n' ' ' | cut -c1-200)"; done
