#!/bin/zsh
# 🔎️ The official pose gate (`T/🔎️scan.ts`) over plants of batch MV-3 through the memory queue (1000 MB); the readable listing goes to `$GEN/log/scan-<tag>.log`, the compact report to `$GEN/log/scan-<tag>.json`. scan.sh <tag> <id>…
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
tag="$1"; shift; mkdir -p "$GEN/log"
"$W/q.sh" 1000 "$T" ${SCAN_TIMEOUT:-3000} "$B" 🔎️scan.ts --language en --report "$GEN/log/scan-$tag.json" "$@" > "$GEN/log/scan-$tag.log" 2>&1; echo "scan exit $? $(tail -1 "$GEN/log/scan-$tag.log" | cut -c1-300)"
