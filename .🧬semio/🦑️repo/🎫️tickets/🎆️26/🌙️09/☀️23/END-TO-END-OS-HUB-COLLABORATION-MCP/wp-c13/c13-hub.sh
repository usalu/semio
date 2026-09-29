#!/bin/zsh
# 🌎️ C13 private hub on an APFS clone of a published catalog (C13_CATALOG, default s14-w4-catalog-p33; production mode, loopback,
# credential sign-in) with fresh users user1/user2 whose passwords come from the environment only (C12_USER{1,2}_PASSWORD via
# stdin, never argv, never printed).
#   c13-hub.sh prepare <root-name> <binary>       fresh root .🧬semio/🌐hub/<root-name> (clone of the catalog's generation + users)
#   c13-hub.sh start <root-name> <port> <binary>  detached hub, pid → <root>.pid, log → s14-c13-logs/<root-name>-<port>.log
#   c13-hub.sh stop <root-name>                   SIGTERM the recorded pid, wait ≤ 60 s
set -u
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
SRC="$H/${C13_CATALOG:-s14-w4-catalog-p33}"
cmd=$1; shift
case $cmd in
  prepare)
    NAME=$1; BIN=$2; ROOT="$H/$NAME"
    test ! -e "$ROOT" || { echo "exists: $ROOT"; exit 1; }
    GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" "$SRC/trusted-catalog/current.json")
    mkdir -p "$ROOT/trusted-catalog/generations"; chmod 700 "$ROOT" "$ROOT/trusted-catalog" "$ROOT/trusted-catalog/generations"
    cp -Rc "$SRC/trusted-catalog/generations/$GEN" "$ROOT/trusted-catalog/generations/"
    cp -p "$SRC/trusted-catalog/current.json" "$ROOT/trusted-catalog/current.json"
    printf '%s' "$C12_USER1_PASSWORD" | OS_HUB_DATA="$ROOT" "$BIN" credential set --email user1@semio.dev --display-name user1 >/dev/null || exit 1
    printf '%s' "$C12_USER2_PASSWORD" | OS_HUB_DATA="$ROOT" "$BIN" credential set --email user2@semio.dev --display-name user2 >/dev/null || exit 1
    echo "prepared $ROOT generation $GEN";;
  start)
    NAME=$1; PORT=$2; BIN=$3; ROOT="$H/$NAME"
    lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null && { echo "port $PORT busy"; exit 1; }
    mkdir -p "$H/s14-c13-logs"
    pid=$(OS_HUB_DATA="$ROOT" OS_HUB_MODE=production OS_HUB_BIND=127.0.0.1 OS_HUB_PORT=$PORT OS_HUB_CREDENTIAL_SIGN_IN=true \
      OS_HUB_ADMIN_SUBJECTS="credential.password.v1:user1@semio.dev" SEMIO_TRACE_LEVEL=info SEMIO_TRACE_SINK=stderr \
      python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/w2-detach.py "$H/s14-c13-logs/$NAME-$PORT.log" "$BIN")
    echo "$pid" > "$ROOT.pid"
    echo "started pid=$pid port=$PORT";;
  stop)
    NAME=$1; ROOT="$H/$NAME"; pid=$(cat "$ROOT.pid")
    ps -o pid=,comm= -p $pid | /usr/bin/grep -q os-hub || { echo "pid $pid is not an os-hub"; exit 1; }
    kill -TERM $pid
    for i in $(seq 1 600); do ps -p $pid >/dev/null || break; sleep 0.1; done
    ps -p $pid >/dev/null && { echo "pid $pid still alive after 60 s"; exit 1; }
    echo "stopped pid=$pid";;
esac
