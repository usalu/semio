#!/bin/sh
# usage: bounded.sh <seconds> <log> <cmd...> : runs cmd in its own pid, kills ONLY that pid after <seconds> (child first).
S="$1"; L="$2"; shift 2
"$@" > "$L" 2>&1 &
P=$!
echo "[DEBUG] pid $P $(ps -o lstart= -p $P)"
i=0; while [ $i -lt "$S" ]; do kill -0 $P 2>/dev/null || break; sleep 1; i=$((i+1)); done
if kill -0 $P 2>/dev/null; then echo "[DEBUG] bound hit; killing $P"; pkill -P $P 2>/dev/null; kill $P; sleep 3; kill -0 $P 2>/dev/null && kill -9 $P; fi
wait $P 2>/dev/null; echo "[DEBUG] rc=$?"
