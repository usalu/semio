#!/bin/zsh
# 🌎️ H11 hub lifecycle on a copy of catalog B2 (production mode, loopback, credential sign-in).
#   h11-hub.sh prepare <root-name>                 fresh data root under .🧬semio/🌐hub/<root-name> (APFS clone of B2 + 2 users)
#   h11-hub.sh start <root-name> <port> <binary>   detached hub (NI 0), pid → <root>.pid, log → s13-h11-logs/<root-name>-<port>-<n>.log
#   h11-hub.sh stop <root-name>                    SIGTERM the recorded pid, wait for exit (≤ 60 s), print exit timing
#   h11-hub.sh wait-ready <port> <start-epoch-ms> <timeout-s>   poll /readyz until 200; prints elapsed ms
set -u
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
LOGS="$H/s13-h11-logs"
SRC="$H/w2-catalog-b2"
cmd=$1; shift
case $cmd in
  prepare)
    NAME=$1; ROOT="$H/$NAME"
    test ! -e "$ROOT" || { echo "exists: $ROOT"; exit 1; }
    GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
    mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
    cp -Rc "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
    cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
    BIN=${2:-"$H/s13-h11-bin/os-hub-c11-1907"}
    for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
      E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
      printf '%s' "$P" | OS_HUB_DATA="$ROOT" "$BIN" credential set --email "$E" --display-name "$N" || exit 1
    done
    echo "prepared $ROOT generation $GEN";;
  start)
    NAME=$1; PORT=$2; BIN=$3; ROOT="$H/$NAME"
    lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null && { echo "port $PORT busy"; exit 1; }
    n=1; while [ -e "$LOGS/$NAME-$PORT-$n.log" ]; do n=$((n+1)); done
    LOG="$LOGS/$NAME-$PORT-$n.log"
    setopt no_bg_nice
    OS_HUB_DATA="$ROOT" OS_HUB_MODE=production OS_HUB_BIND=127.0.0.1 OS_HUB_PORT=$PORT OS_HUB_CREDENTIAL_SIGN_IN=true \
      OS_HUB_ADMIN_SUBJECTS=credential.password.v1:user1@semio.dev SEMIO_TRACE_LEVEL=info SEMIO_TRACE_SINK=stderr \
      nohup "$BIN" > "$LOG" 2>&1 & disown
    pid=$!
    echo "$pid" > "$ROOT.pid"
    echo "started pid=$pid port=$PORT log=$LOG at $(python3 -c 'import time;print(int(time.time()*1000))')";;
  stop)
    NAME=$1; ROOT="$H/$NAME"; pid=$(cat "$ROOT.pid")
    ps -o pid=,comm= -p $pid | /usr/bin/grep -q os-hub || { echo "pid $pid is not an os-hub"; exit 1; }
    t0=$(python3 -c 'import time;print(int(time.time()*1000))'); kill -TERM $pid
    for i in $(seq 1 600); do ps -p $pid >/dev/null || break; sleep 0.1; done
    t1=$(python3 -c 'import time;print(int(time.time()*1000))')
    ps -p $pid >/dev/null && { echo "pid $pid still alive after 60 s"; exit 1; }
    echo "stopped pid=$pid in $((t1 - t0)) ms";;
  wait-ready)
    PORT=$1; START=$2; TIMEOUT=$3
    end=$(( $(date +%s) + TIMEOUT ))
    while [ $(date +%s) -lt $end ]; do
      code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 5 "http://127.0.0.1:$PORT/readyz")
      if [ "$code" = 200 ]; then now=$(python3 -c 'import time;print(int(time.time()*1000))'); echo "ready after $((now - START)) ms"; exit 0; fi
      sleep 1
    done
    echo "not ready within $TIMEOUT s (last $code)"; curl -s --max-time 5 "http://127.0.0.1:$PORT/readyz" | head -c 1500; exit 1;;
esac
