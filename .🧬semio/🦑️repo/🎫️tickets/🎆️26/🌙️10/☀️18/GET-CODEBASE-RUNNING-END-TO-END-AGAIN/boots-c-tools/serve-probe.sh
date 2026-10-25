#!/bin/sh
# usage: serve-probe.sh <variant> <port> (runs probe.mjs boot probe, then interact-<variant>.mjs when present; interact probes read SEMIO_PROBE_URL, default 6058 forms / 6080 note) : starts the serve target (own session), waits for the port, probes headless, stops ONLY the pid it started.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
V="$1"; PORT="$2"; G="$C/🗑️generated/boots-c"
export PLAYWRIGHT_BROWSERS_PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
nice -n 10 bun repo --tool="${SEMIO_TOOL:-bazel}" run "@semio-tech/framework-os-dev:serve-$V-react-prototype" > "$G/serve-$V.out" 2>&1 &
SP=$!
echo "[DEBUG] serve pid $SP $(ps -o lstart= -p $SP)"
i=0; while [ $i -lt 360 ]; do kill -0 $SP 2>/dev/null || { echo "[DEBUG] serve died"; break; }; curl -s -o /dev/null "http://127.0.0.1:$PORT/" && { echo "[DEBUG] port up after $((i*5))s"; break; }; sleep 5; i=$((i+1)); done
bun "$C/boots-c-tools/probe.mjs" "$V" "$PORT" 300 > "$G/probe-$V.out" 2>&1
echo "[DEBUG] probe rc=$?"
if [ -f "$C/boots-c-tools/interact-$V.mjs" ]; then bun "$C/boots-c-tools/interact-$V.mjs" > "$G/interact-$V.out" 2>&1; echo "[DEBUG] interact rc=$?"; fi
kill $SP 2>/dev/null; sleep 5; kill -0 $SP 2>/dev/null && kill -9 $SP
echo "[DEBUG] done $(date -u +%FT%TZ)"
