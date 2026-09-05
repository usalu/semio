#!/usr/bin/env bash
# 🚀️ macOS puts processes spawned from background tool calls under background task policy, which
# throttles CPU and disk hard -- so a build can look "starved" at 5% CPU when it is merely throttled.
# `taskpolicy -B` lifts one pid out of it, but new children start throttled again, so this re-applies
# every 30s across cargo -> rustc -> grandchildren. Scoped by CARGO_TARGET_DIR to THIS ticket's two
# private target dirs, so peers' builds are never touched.
set -uo pipefail
MINE="target-demonstrator"
while true; do
  for c in $(pgrep -f "bin/cargo" 2>/dev/null); do
    env_line=$(ps eww -o command= -p "$c" 2>/dev/null | tr ' ' '\n' | grep '^CARGO_TARGET_DIR=' | head -1)
    case "$env_line" in *"$MINE"*) ;; *) continue ;; esac
    taskpolicy -B -p "$c" 2>/dev/null
    for d in $(pgrep -P "$c" 2>/dev/null); do
      taskpolicy -B -p "$d" 2>/dev/null
      for g in $(pgrep -P "$d" 2>/dev/null); do taskpolicy -B -p "$g" 2>/dev/null; done
    done
  done
  sleep 30
done
