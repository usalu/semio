#!/bin/zsh
# 🌎️ H12 hub lifecycle on a clone of a published catalog (production mode, loopback, credential sign-in, user1 admin).
#   h12-hub.sh prepare <root-name> <catalog-root> <binary>   fresh root .🧬semio/🌐hub/<root-name> (APFS clone + user1/user2)
#   h12-hub.sh start <root-name> <port> <binary> [residency-bytes]   detached hub (NI 0); pid → <root>.pid; log → s13-h12-logs/
#   h12-hub.sh stop <root-name>                                     SIGTERM the recorded pid (verified os-hub), wait ≤ 60 s
set -u
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
LOGS="$H/s13-h12-logs"
cmd=$1; shift
case $cmd in
  prepare)
    NAME=$1; SRC=$2; BIN=$3; ROOT="$H/$NAME"
    test ! -e "$ROOT" || { echo "exists: $ROOT"; exit 1; }
    GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
    mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
    cp -Rc "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
    cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
    for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
      E=${u%%|*}; R=${u#*|}; N=${R%%|*}; P=${R#*|}
      printf '%s' "$P" | OS_HUB_DATA="$ROOT" "$BIN" credential set --email "$E" --display-name "$N" || exit 1
    done
    echo "prepared $ROOT generation $GEN";;
  start)
    NAME=$1; PORT=$2; BIN=$3; BYTES=${4:-}; ROOT="$H/$NAME"
    lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null && { echo "port $PORT busy"; exit 1; }
    n=1; while [ -e "$LOGS/$NAME-$PORT-$n.log" ]; do n=$((n+1)); done
    LOG="$LOGS/$NAME-$PORT-$n.log"
    setopt no_bg_nice
    ENVS=(OS_HUB_DATA="$ROOT" OS_HUB_MODE=production OS_HUB_BIND=127.0.0.1 OS_HUB_PORT=$PORT OS_HUB_CREDENTIAL_SIGN_IN=true
      OS_HUB_ADMIN_SUBJECTS=credential.password.v1:user1@semio.dev SEMIO_TRACE_LEVEL=info SEMIO_TRACE_SINK=stderr)
    [ -n "$BYTES" ] && ENVS+=(OS_HUB_GUEST_RESIDENCY_BYTES=$BYTES)
    nohup env "${ENVS[@]}" "$BIN" > "$LOG" 2>&1 & disown
    pid=$!
    echo "$pid" > "$ROOT.pid"
    echo "started pid=$pid port=$PORT residency=${BYTES:-default} log=$LOG at $(python3 -c 'import time;print(int(time.time()*1000))')";;
  stop)
    NAME=$1; ROOT="$H/$NAME"; pid=$(cat "$ROOT.pid")
    ps -o pid=,comm= -p $pid | /usr/bin/grep -q os-hub || { echo "pid $pid is not an os-hub"; exit 1; }
    t0=$(python3 -c 'import time;print(int(time.time()*1000))'); kill -TERM $pid
    for i in $(seq 1 600); do ps -p $pid >/dev/null || break; sleep 0.1; done
    t1=$(python3 -c 'import time;print(int(time.time()*1000))')
    ps -p $pid >/dev/null && { echo "pid $pid still alive after 60 s"; exit 1; }
    echo "stopped pid=$pid in $((t1 - t0)) ms";;
esac
