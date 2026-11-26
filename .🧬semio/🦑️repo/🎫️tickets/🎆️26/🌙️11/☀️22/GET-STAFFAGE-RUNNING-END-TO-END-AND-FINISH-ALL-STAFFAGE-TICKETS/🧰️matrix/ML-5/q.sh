#!/bin/zsh
# 🧺️ One script under ONE grant of the memory queue: q.sh <MB> <timeout-s> <script.zsh> [args…]   the script is sourced with the paths and `cap` of env.sh inside `repo capacity run`; its output goes to $GEN/<script>.log, the queue lines to $GEN/capacity.log.
source "${0:A:h}/env.sh"
mb="$1"; s="$2"; script="${3:A}"; shift 3
log="$GEN/${script:t:r}${1:+.$1}.log"
(cd "$R" && repo capacity run --memory="$mb" -- timeout "$s" /bin/zsh -c 'source "$1/env.sh"; script="$2"; shift 2; source "$script" "$@"' q "$W" "$script" "$@") > "$log.all" 2>&1
code=$?
grep "^\[capacity\]" "$log.all" >> "$GEN/capacity.log"; grep -v "^\[capacity\]" "$log.all" > "$log"; rm -f "$log.all"
echo "[DEBUG] ${script:t} $* exit $code $(date +%H:%M)" >> "$GEN/q.log"
exit $code
