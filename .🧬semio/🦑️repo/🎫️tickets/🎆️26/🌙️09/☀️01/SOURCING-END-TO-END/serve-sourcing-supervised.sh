#!/bin/sh
# 🩺️ Keeps the sourcing react dev serve on :6081 answering: a peer regenerating
# 🔌️plugin/📇️registry/🤖️generated/*.ts makes vite "restart" and never rebind (twice tonight),
# so the serve is recycled whenever the port stops answering for 30 s.
cd "$(dirname "$0")/../../../../../../.." || exit 1
LOG="$1"
while :; do
  bun nx run @semio-tech/framework-os-dev:serve-sourcing-react-dev >> "$LOG" 2>&1 &
  SERVE=$!
  DEAD=0
  while kill -0 "$SERVE" 2>/dev/null; do
    sleep 10
    if curl -s -m 5 -o /dev/null -w "%{http_code}" http://127.0.0.1:6081/ | grep -q 200; then DEAD=0; else DEAD=$((DEAD + 10)); fi
    if [ "$DEAD" -ge 30 ]; then
      echo "[supervisor] :6081 silent for ${DEAD}s — recycling $(date)" >> "$LOG"
      pkill -P "$SERVE" 2>/dev/null; kill "$SERVE" 2>/dev/null
      pgrep -f "vite.*--port 6081" | xargs kill 2>/dev/null
      sleep 3
      break
    fi
  done
done
