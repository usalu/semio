#!/usr/bin/env bash
set -u
ticket="$(cd "$(dirname "$0")" && pwd)"
setsid "$ticket/lguest-build.sh" > >(tee -a "$1") 2>&1 &
pid=$!
while kill -0 "$pid" 2>/dev/null; do
  if grep -aq "Compiling semio-hub-puzzle " "$1"; then
    printf '%s\n' '[DEBUG] dependency phase complete, stopping before hub crate'
    kill -TERM -- "-$pid" 2>/dev/null
    sleep 5
    exit 0
  fi
  if grep -aq "^error: could not\|^error\[" "$1"; then wait "$pid"; exit 1; fi
  sleep 5
done
wait "$pid"
