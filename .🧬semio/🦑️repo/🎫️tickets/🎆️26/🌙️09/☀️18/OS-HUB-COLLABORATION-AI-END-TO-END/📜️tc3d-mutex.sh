#!/bin/zsh
# 🔒 Fleet wasm build mutex, TC3d's copy (preamble rule 27). Byte-identical to 📜️wasm-build-mutex.sh
# except for ONE line: the ticket name is FIXED at `20260921235800-$$-tc3d` instead of minted from
# the current clock. The coordinator moved TC3d's bootstrap onto the critical path by agreement with
# the peer session on 2026-09-21 ~17:15, and the queue is ordered by ticket NAME (`ls | sort | head
# -1`), so a fixed earlier stamp is how a promotion is expressed. 135300 places this hold right
# after the running `rb1` (…135123) and the peer's `…135200-play` activation, ahead of c7, stdio-a,
# stdio-examples, raster, stdio-b and jb1. Everything else — the stale-ticket reaper, the lock
# directory, the holder pid/owner files and BOTH EXIT/INT/TERM traps — is unchanged.
# usage: zsh 📜️tc3d-mutex.sh <slice> -- <command …>
slice="$1"; shift; [ "$1" = "--" ] && shift
lock="/tmp/semio-wasm-build.lock"
queue="/tmp/semio-wasm-build.queue"
mkdir -p "$queue"
ticket="$queue/20260921235800-$$-tc3d"
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
