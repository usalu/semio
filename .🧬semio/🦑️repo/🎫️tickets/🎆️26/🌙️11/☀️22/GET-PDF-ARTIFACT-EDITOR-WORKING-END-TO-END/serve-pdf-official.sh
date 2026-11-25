#!/bin/sh
# Starts the official PDF 1.7 dev target without its dependency chain (no wasm build) and retries while the shared tree is mid-edit.
# Usage: sh serve-pdf-official.sh [attempts]; log: 🗑️generated/dev-official.log; stop with: pkill -f 'dev-stdio-pdf-1-7-react-prototype'
cd "$(dirname "$0")/../../../../../../.." || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END/🗑️generated/dev-official.log"
n=0
while [ "$n" -lt "${1:-12}" ]; do
  n=$((n + 1))
  echo "[attempt $n] $(date)" >> "$LOG"
  repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype --lang=en --exclude-dependencies --output=stream >> "$LOG" 2>&1
  code=$?
  echo "[attempt $n] exit $code" >> "$LOG"
  [ "$code" = 75 ] || exit "$code"
  sleep 60
done
