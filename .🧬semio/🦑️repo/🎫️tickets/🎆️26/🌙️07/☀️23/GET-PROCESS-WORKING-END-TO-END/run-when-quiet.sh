#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
build_mb(){ ps -axo rss,comm | awk '$2 ~ /(rustc|\/ld|cargo|clang|\/bun)$/ {s+=$1} END{print int(s/1024)}'; }
quiet=0
while (( quiet < 3 )); do
  if (( $(build_mb) < ${QUIET_BUILD_MB:-3500} )); then quiet=$((quiet+1)); else quiet=0; fi
  sleep 20
done
for i in $(seq 1 ${E2E_ATTEMPTS:-3}); do
  rm -f $T/🗑️generated/e2e-report.json
  $T/run-forest-render.sh
  grep -q "^fin 2" $T/🗑️generated/${TAG:-forest}-out.txt || break
  echo "attempt $i killed" >> $T/🗑️generated/render-attempts.txt
done
echo end > $T/🗑️generated/quiet-end.txt
