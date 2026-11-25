#!/bin/zsh
# 🖨️ The print suitability (`suit.ts`) of plants or trial files of batch MV-3 in one process through the memory queue (1000 MB): suit.sh <tag> <id|file.json>…   (log `$GEN/log/suit-<tag>.log`)
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
tag="$1"; shift
q 1000 "$T" 900 "$B" "$W/suit.ts" "$@" > "$GEN/log/suit-$tag.log" 2>&1; echo "suit exit $?: $(grep -E ': (prints|[0-9]+ scales unsuitable)$' "$GEN/log/suit-$tag.log" | tr '\n' ';')"
