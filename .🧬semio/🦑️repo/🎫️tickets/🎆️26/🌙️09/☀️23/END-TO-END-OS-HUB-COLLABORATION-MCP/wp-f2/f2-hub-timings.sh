#!/bin/zsh
# 👥️ F2 — hub document open + catalog install timings on hub 7800 (catalog B3) through the permanent two-human gate:
# starts the hub-bound `s` serve on 6580 detached, waits for it, then runs `verify two-human` detached (ONE headless browser,
# two profiles) on one kind per B3 package plus a second puzzle kind (same plugin: the catalog install is not repeated).
# usage: zsh f2-hub-timings.sh <tag>   — stop with: kill -TERM <verify pid> (closes its browser), kill -TERM -<serve pgid>
setopt no_bg_nice
TAG="${1:-s13-f2-b3}"
REPO=/Users/ueli/Documents/semio
LOGS="$REPO/.🧬semio/🌐hub/s13-f2-logs"
DEV="$REPO/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
DETACH="$REPO/.tmp-ticket/wp-w2/w2-detach.py"
SERVE_PID=$(python3 "$DETACH" "$LOGS/serve-6580-$TAG.txt" zsh -c "cd '$DEV' && exec env NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6580 S_HUB_URL=http://127.0.0.1:7800 SEMIO_PLUGIN=s S_LOCAL_ONLY=1 nice -n 10 bun ./📜️script.ts serve s react dev")
echo "serve pgid $SERVE_PID"
for i in $(seq 1 150); do curl -s -o /dev/null --max-time 3 http://127.0.0.1:6580/ && break; sleep 2; done
curl -s -o /dev/null -w "serve 6580 %{http_code}\n" --max-time 5 http://127.0.0.1:6580/
VERIFY_PID=$(python3 "$DETACH" "$LOGS/two-human-$TAG.txt" zsh -c "cd '$DEV' && exec nice -n 10 bun ./📜️script.ts verify two-human --hub http://127.0.0.1:7800 --serve http://127.0.0.1:6580/ --locale en --kinds text.document,2d.drawing,2d.puzzle,3d.puzzle,2d.block,animate.presentation --tag $TAG")
echo "verify pid $VERIFY_PID"
