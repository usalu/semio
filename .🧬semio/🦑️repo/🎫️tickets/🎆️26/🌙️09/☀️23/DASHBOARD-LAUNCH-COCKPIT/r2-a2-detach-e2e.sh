#!/usr/bin/env bash
# Isolated end-to-end of the detached daemon: start, status, run --detach --wait-ready, tasks, stop.
# Usage: r2-a2-detach-e2e.sh <path to semio.exe>
set -u
SEMIO="$1"
ROOT=$(mktemp -d /tmp/semio-a2-e2e.XXXXXX)
WROOT=$(cygpath -w "$ROOT")
PORT=$(node -e "const s=require('net').createServer().listen(0,'127.0.0.1',()=>{console.log(s.address().port);s.close()})")
mkdir -p "$ROOT/.git"
cat > "$ROOT/package.json" <<EOF
{ "name": "e2e", "private": true }
EOF
cat > "$ROOT/project.json" <<EOF
{ "name": "e2e", "metadata": { "semio": { "dashboard": { "tools": [ { "id": "srv", "verb": "dev", "continuous": true, "command": ["node", "-e", "require('http').createServer((q,r)=>r.end('ok')).listen($PORT,'127.0.0.1',()=>console.log('Local: http://localhost:$PORT/'))"], "ready": { "port": $PORT, "path": "/" } } ] } } } }
EOF
cd "$ROOT"
echo "== daemon start";  "$SEMIO" daemon start --root "$WROOT"; echo "exit=$?"
echo "== daemon status"; "$SEMIO" daemon status --root "$WROOT"; echo "exit=$?"
echo "== commands";      "$SEMIO" commands --root "$WROOT" 2>&1 | head -5
echo "== run --detach --wait-ready"; "$SEMIO" run tool:e2e/srv --detach --wait-ready --root "$WROOT"; echo "exit=$?"
echo "== http";          curl -s "http://127.0.0.1:$PORT/"; echo
echo "== tasks";         "$SEMIO" tasks --root "$WROOT"; echo "exit=$?"
echo "== daemon stop";   "$SEMIO" daemon stop --root "$WROOT"; echo "exit=$?"
echo "== status after";  "$SEMIO" daemon status --root "$WROOT"
echo "== port released"; curl -s -m 2 "http://127.0.0.1:$PORT/" || echo "closed"
echo "== two instances on one root"
"$SEMIO" daemon start --root "$WROOT" > /dev/null; echo "default pid: $("$SEMIO" daemon status --root "$WROOT")"
SEMIO_DASHBOARD_INSTANCE=smoke "$SEMIO" daemon start --root "$WROOT" > /dev/null; echo "smoke pid:   $(SEMIO_DASHBOARD_INSTANCE=smoke "$SEMIO" daemon status --root "$WROOT")"
SEMIO_DASHBOARD_INSTANCE=smoke "$SEMIO" run tool:e2e/srv --detach --wait-ready --root "$WROOT" | tail -1
echo "default tasks:"; "$SEMIO" tasks --root "$WROOT"
echo "smoke tasks:";   SEMIO_DASHBOARD_INSTANCE=smoke "$SEMIO" tasks --root "$WROOT"
SEMIO_DASHBOARD_INSTANCE=smoke "$SEMIO" daemon stop --root "$WROOT"
echo "default still: $("$SEMIO" daemon status --root "$WROOT")"
"$SEMIO" daemon stop --root "$WROOT"
cd /; rm -rf "$ROOT"
