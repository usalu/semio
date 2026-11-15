#!/bin/bash
# 👀 Waits for the driver's try line <n>, stops my own driver (pids given) in its wait, runs step 0 once, then a fresh series. Usage: after-try.sh <tries file> <n> <next round> <driver pid>...
M="$(cd "$(dirname "$0")" && pwd)"; f="$1"; n="$2"; r="$3"; shift 3
until grep -q "^try $n ended" "$f" 2>/dev/null; do sleep 5; done
for p in "$@"; do for c in $(pgrep -P "$p"); do ps -o comm= -p "$c" | grep -q '^sleep$' && kill -TERM "$c"; done; done
for p in "$@"; do kill -TERM "$p" 2>/dev/null; done
echo "driver stopped at $(date -u +%Y-%m-%dT%H:%M:%SZ) for step 0 rerun (round 24)" >> "$f"
bash "$M/tools-setup-round-16.sh"
bash "$M/drive-round.sh" "$r" 1 4
