#!/bin/zsh
# ⏳️ Blocks until a line matching the pattern is in the file or the seconds are over (default 160), then prints the time and the last lines of the file without the queue's waiting lines: wait.sh <file> <pattern> [seconds] [lines]
end=$(( $(date +%s) + ${3:-160} ))
until grep -q -- "$2" "$1" 2>/dev/null || [[ $(date +%s) -ge $end ]]; do sleep 5; done
grep -q -- "$2" "$1" 2>/dev/null && echo "DONE $(date +%H:%M)" || echo "waiting $(date +%H:%M)"
grep -v '^\[capacity\]' "$1" 2>/dev/null | tail -${4:-3} | cut -c1-240
