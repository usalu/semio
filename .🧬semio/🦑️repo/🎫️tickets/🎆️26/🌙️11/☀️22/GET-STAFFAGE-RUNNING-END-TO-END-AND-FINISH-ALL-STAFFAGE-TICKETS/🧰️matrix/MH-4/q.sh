#!/bin/zsh
# 🧺️ Several short runs of the takeover under ONE grant of the memory queue, strictly one after the other: q.sh <MB> <timeout-s> <script.zsh>   the script is sourced with the paths of env.sh (B, R, S, T, U, W, M, GEN) inside `repo capacity run`, and cap.sh calls of at most that memory run directly under the grant (MH4_GRANT); the queue lines go to $GEN/capacity.log.
source "${0:A:h}/env.sh"
mb="$1"; s="$2"; script="${3:A}"
mkdir -p "$GEN"; tmp="$(mktemp "$GEN/q.XXXXXX")"
(cd "$R" && repo capacity run --memory="$mb" -- timeout "$s" /bin/zsh -c 'export MH4_GRANT="$3"; source "$1/env.sh"; source "$2"' q "$W" "$script" "$mb") > "$tmp" 2>&1
code=$?
grep "^\[capacity\]" "$tmp" >> "$GEN/capacity.log"
grep -v "^\[capacity\]" "$tmp"
rm -f "$tmp"
exit $code
