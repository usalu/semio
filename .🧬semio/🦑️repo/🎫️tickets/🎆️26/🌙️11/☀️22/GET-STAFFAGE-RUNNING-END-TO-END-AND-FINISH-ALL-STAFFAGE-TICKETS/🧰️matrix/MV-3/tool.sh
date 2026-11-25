#!/bin/zsh
# 🔧️ Runs one measuring tool of batch MV-3 (a `.ts` of this folder or of `M`) from the suite folder through the memory queue and keeps its output: tool.sh <log name> <megabytes> <script.ts> <args…>   (log `$GEN/log/<log name>.log`; a script name without a slash is taken from this folder)
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
q() { if [[ -n "$MV3_GRANT" ]]; then local dir="$2" t="$3" r="$GEN/log/rss-$$.txt" code; shift 3; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; else "$W/q.sh" "$@"; fi }
name="$1"; mb="$2"; script="$3"; shift 3; [[ "$script" == */* ]] || script="$W/$script"
q "$mb" "$T" 3000 "$B" "$script" "$@" > "$GEN/log/$name.log" 2>&1; echo "$name exit $? $(tail -1 "$GEN/log/$name.log" | cut -c1-200)"
