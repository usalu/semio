#!/bin/zsh
# 🔁️ Recycles the fem2d React dev serve on :6086 by pid (a peer's registry regen makes Vite "restart" and the
# restart comes back without a listener — memory: release serve wedges after host edit bursts). Runs the serve
# script directly (bypassing nx's dependsOn re-activation, which a peer's broken fem3d can block) with a log.
set -u
ROOT=/Users/ueli/Documents/semio
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/INPUT-CAUSALITY-LEDGER/🗑️generated"
mkdir -p "$T"
if [ -f "$T/serve-6086.pid" ]; then P=$(cat "$T/serve-6086.pid"); pkill -TERM -P "$P" 2>/dev/null; kill "$P" 2>/dev/null; fi
sleep 1
lsof -nP -iTCP:6086 -sTCP:LISTEN 2>/dev/null | awk 'NR>1{print $2}' | xargs -r kill 2>/dev/null
sleep 1
cd "$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
nohup env SEMIO_RENDERER=react S_OS_PORT=6086 bun ./📜️script.ts serve fem2d react dev > "$T/serve-6086-$(date +%H%M%S).log" 2>&1 &
echo $! > "$T/serve-6086.pid"; disown
for i in $(seq 1 60); do code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 5 http://127.0.0.1:6086/ 2>/dev/null); if [ "$code" = "200" ]; then echo "serve up (pid $(cat "$T/serve-6086.pid")) after ${i}x5s"; exit 0; fi; sleep 5; done
echo "serve did not answer"; exit 1
