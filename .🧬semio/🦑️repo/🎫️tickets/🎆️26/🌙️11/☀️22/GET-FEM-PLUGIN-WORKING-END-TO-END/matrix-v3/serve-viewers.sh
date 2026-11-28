#!/usr/bin/env bash
# Serves the standalone FEM viewer apps in dev next to (not instead of) the editors: 2D viewer on 6088, 3D viewer on 6089.
# Usage: serve-viewers.sh start|stop|check [2d|3d|both]   (default both). Same env as $M25/native-fem-dev.sh serve2d/serve3d,
# except VITE_SEMIO_APP_ID selects the `#viewer` app of the same `semio:fem` component (descriptor apps s.fem.fem{2,3}d@1/*#viewer). The app id alone names the
# role (an unstated boot role never overrides it, ShellHost surface-switch); `?role=` / VITE_SEMIO_APP_ROLE still project it. The editors can also be opened as viewer with `?role=viewer`.
# It never touches 6086/6087; `stop` only kills the pids this script recorded.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
G="$HERE/../🗑️generated/matrix-v3"
R=/c/semio
mkdir -p "$G"
VITE=$(for d in $(ls -dt "$R"/.cache/b/d/m/j/*/); do [ -f "$d/node_modules/vite/dist/client/client.mjs" ] && [ -d "$d/node_modules/react" ] && { echo "$d/node_modules/vite/bin/vite.js"; break; }; done)
cmd=${1:-start}; which=${2:-both}
variants() { case "$which" in 2d) echo fem2d;; 3d) echo fem3d;; *) echo fem2d fem3d;; esac; }
portof() { [ "$1" = fem2d ] && echo 6088 || echo 6089; }
case "$cmd" in
start)
  [ -n "$VITE" ] || { echo "vite not found under $R/.cache/b/d/m/j"; exit 2; }
  for v in $(variants); do
    port=$(portof $v)
    if curl -s -m 2 -o /dev/null "http://127.0.0.1:$port/"; then echo "$v viewer already up on $port"; continue; fi
    (
      cd "$R" || exit 2
      export SEMIO_PLUGIN=$v SEMIO_RENDERER=react SEMIO_BUILD_MODE=prototype VITE_SEMIO_PLUGIN=$v VITE_SEMIO_RENDERER=react
      export VITE_SEMIO_APP_ID="s.fem.$v@1/*#viewer" SEMIO_VITE_HMR=0 S_OS_PORT=$port S_HUB_URL= SEMIO_BRAND=
      export SEMIO_VITE_CACHE_DIRECTORY="C:\semio\.cache\b\p\m\vite\prototype\native-$v-viewer"
      nohup bun "$VITE" --mode prototype --config "$R/✏️s/🧑‍💻dev/🎭️variants/🏗️fem/🏗️builder/🌐️vite/🟦️.ts" \
        --configLoader native --host 127.0.0.1 --port $port --strictPort --clearScreen false \
        > "$G/viewer-$v.log" 2>&1 &
      echo $! > "$G/viewer-$v.pid"
    )
    echo "$v viewer starting on $port (pid $(cat "$G/viewer-$v.pid"), log $G/viewer-$v.log)"
  done ;;
stop)
  for v in $(variants); do
    [ -f "$G/viewer-$v.pid" ] || { echo "no pid file for $v"; continue; }
    pid=$(cat "$G/viewer-$v.pid"); kill "$pid" 2>/dev/null && echo "stopped $v viewer pid $pid" || echo "pid $pid of $v not running"; rm -f "$G/viewer-$v.pid"
  done ;;
check)
  for v in $(variants); do p=$(portof $v); echo "$v viewer :$p -> $(curl -s -m 5 -o /dev/null -w 'html %{http_code} %{size_download}' "http://127.0.0.1:$p/")"; done ;;
*) echo "usage: $0 start|stop|check [2d|3d|both]"; exit 2 ;;
esac
