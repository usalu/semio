#!/bin/sh
# 🔗️ Waits for the compiler process of one crate below the process <root-pid>, writes its argument vector to <out> and registers this process as a live user of the build lane its dependencies live in (a marker `<lane>/v/<pid>-hold`), so that garbage collection keeps the lane after the build ended; it then sleeps until it is killed, and its marker dies with it.
# Usage: link-hold.sh <root-pid> <crate-name> <out-file>
set -eu
root=$1; crate=$2; out=$3
export LC_ALL=en_US.UTF-8
while :; do
  pid=""
  for candidate in $(pgrep -f -- "^rustc --crate-name $crate " || true); do
    up=$candidate
    while [ "$up" -gt 1 ]; do
      [ "$up" = "$root" ] && { pid=$candidate; break; }
      up=$(ps -o ppid= -p "$up" | tr -d ' ')
      [ -n "$up" ] || break
    done
    [ -n "$pid" ] && break
  done
  if [ -n "$pid" ]; then ps -ww -o args= -p "$pid" > "$out" 2>/dev/null && break; fi
  sleep 0.05
done
lane=$(tr ' ' '\n' < "$out" | sed -n 's#^--remap-path-prefix=\(.*/\.cache/b/r/m/u/l/[a-z0-9]*\)=/cargo/build-dir$#\1#p' | head -1)
mkdir -p "$lane/v"; printf '{"pid":%s}' "$$" > "$lane/v/$$-hold"
echo "held $pid lane $lane"
trap 'rm -f "$lane/v/$$-hold"; exit 0' TERM INT
while :; do sleep 5; touch "$lane/v/$$-hold"; done
