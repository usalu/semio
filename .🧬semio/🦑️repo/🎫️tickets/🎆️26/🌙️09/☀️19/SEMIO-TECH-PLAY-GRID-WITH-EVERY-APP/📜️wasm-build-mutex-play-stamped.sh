#!/bin/zsh
# 🔒 Fleet wasm build mutex (preamble rule 27): run ONE wasm32 plugin build/activation at a time, first come first served.
# usage: zsh 📜️wasm-build-mutex.sh <slice> -- <command …>
slice="$1"; shift; [ "$1" = "--" ] && shift
lock="/tmp/semio-wasm-build.lock"
queue="/tmp/semio-wasm-build.queue"
mkdir -p "$queue"
ticket="$queue/${PLAY_MUTEX_STAMP:-$(date +%Y%m%d%H%M%S)}-$$-$slice"
echo $$ > "$ticket"
trap 'rm -f "$ticket"' EXIT INT TERM
while true; do
  for t in "$queue"/*(N); do
    tp=$(cat "$t" 2>/dev/null)
    if [ -z "$tp" ] || ! kill -0 "$tp" 2>/dev/null; then rm -f "$t"; fi
  done
  first=$(ls "$queue" 2>/dev/null | sort | head -1)
  if [ "$queue/$first" = "$ticket" ]; then
    if mkdir "$lock" 2>/dev/null; then break; fi
    holder_pid=$(cat "$lock/pid" 2>/dev/null)
    if [ -n "$holder_pid" ] && ! kill -0 "$holder_pid" 2>/dev/null; then rm -rf "$lock"; continue; fi
    if [ -z "$holder_pid" ] && [ -n "$(find "$lock" -maxdepth 0 -mmin +3 2>/dev/null)" ]; then rm -rf "$lock"; continue; fi
  fi
  sleep 15
done
echo $$ > "$lock/pid"; echo "$slice $(date '+%H:%M:%S')" > "$lock/owner"
trap 'rm -rf "$lock"; rm -f "$ticket"' EXIT INT TERM
"$@"
