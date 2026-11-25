#!/bin/zsh
# 🔧️ Runs one measuring or repairing tool of batch ML-3 (a `.ts` of this folder unless the name holds a slash) from the suite folder through the memory queue and keeps its output: tool.sh <log name> <megabytes> <script.ts> <args…>   (log `$GEN/log/<log name>.log`, last line echoed)
source "${0:A:h}/env.sh"
name="$1"; mb="$2"; script="$3"; shift 3; [[ "$script" == */* ]] || script="$W/$script"; mkdir -p "$GEN/log"
"$W/q.sh" "$mb" "$T" ${TOOL_TIMEOUT:-3000} "$B" "$script" "$@" > "$GEN/log/$name.log" 2>&1; echo "$name exit $? $(tail -1 "$GEN/log/$name.log" | cut -c1-300)"
