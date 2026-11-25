#!/bin/zsh
# 🚦️ The fixer's gate (`M/gate.ts`: every pose, clip instant and transition of the matrix) over plants of batch MV-3, one process per plant, through the memory queue: gate.sh <tag> <id|file.json>…   (log per plant in `$GEN/log/<name>-gate-<tag>.log`, one summary line per plant on stdout; GATE_FLAGS adds flags such as `--soft`)
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
tag="$1"; shift; mkdir -p "$GEN/log"
for a in "$@"; do n="${${a:t}%.json}"; q ${GATE_MB:-1000} "$T" 2400 "$B" "$M/gate.ts" "$a" ${=GATE_FLAGS} > "$GEN/log/$n-gate-$tag.log" 2>&1; echo "$n gate exit $? $(tail -1 "$GEN/log/$n-gate-$tag.log")"; done
