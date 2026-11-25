#!/usr/bin/env bash
# 🪂️ Runs a command in a session of its own, its output in <log> and its exit code as a last line "exit <n>". Usage: detach.sh <log> <command> [args...]
LOG="$1"; shift
perl -MPOSIX -e 'POSIX::setsid(); exec @ARGV' bash -c '"$@" > "$0" 2>&1; echo "exit $?" >> "$0"' "$LOG" "$@" &
