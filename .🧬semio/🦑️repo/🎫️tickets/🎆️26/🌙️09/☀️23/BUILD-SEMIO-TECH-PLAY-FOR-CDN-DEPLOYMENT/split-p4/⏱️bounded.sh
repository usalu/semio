#!/bin/bash
# ⏱️ Runs one command in its own process group and interrupts the whole group (SIGINT, then SIGTERM) at a deadline, so a
# cargo pass stays inside one foreground tool call and resumes from the shared build dir on the next call. Exit 124 = deadline.
# usage: ⏱️bounded.sh <seconds> -- <command...>
SECONDS_LIMIT="$1"; shift; [ "$1" = "--" ] && shift
set -m
"$@" &
PID=$!
for ((i = 0; i < SECONDS_LIMIT; i++)); do
  kill -0 "$PID" 2>/dev/null || { wait "$PID"; exit $?; }
  sleep 1
done
kill -INT -"$PID" 2>/dev/null
for ((i = 0; i < 30; i++)); do kill -0 "$PID" 2>/dev/null || break; sleep 1; done
kill -TERM -"$PID" 2>/dev/null
wait "$PID" 2>/dev/null
exit 124
