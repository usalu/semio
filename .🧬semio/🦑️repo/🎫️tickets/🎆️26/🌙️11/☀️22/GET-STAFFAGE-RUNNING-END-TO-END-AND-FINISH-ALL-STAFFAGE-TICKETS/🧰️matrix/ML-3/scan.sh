#!/bin/zsh
# 🔎️ The authors' gate (`T/🔎️scan.ts`) over plants of batch ML-3 in one process through the memory queue (1000 MB): scan.sh <tag> <id>…   (listing `$GEN/log/scan-<tag>.log`, compact report `$GEN/log/scan-<tag>.json`)
source "${0:A:h}/env.sh"
tag="$1"; shift; mkdir -p "$GEN/log"
"$W/q.sh" ${SCAN_MB:-1000} "$T" ${SCAN_TIMEOUT:-3000} "$B" 🔎️scan.ts --language en --report "$GEN/log/scan-$tag.json" "$@" > "$GEN/log/scan-$tag.log" 2>&1; echo "scan-$tag exit $? $(tail -1 "$GEN/log/scan-$tag.log" | cut -c1-300)"
