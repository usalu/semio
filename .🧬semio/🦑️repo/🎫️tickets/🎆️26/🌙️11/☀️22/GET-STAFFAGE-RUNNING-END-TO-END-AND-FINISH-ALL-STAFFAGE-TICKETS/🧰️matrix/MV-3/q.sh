#!/bin/zsh
# 🚦️ Runs one tool process of batch MV-3 through the machine-wide memory queue (binding rule of the coordinator of 2026-11-24: every bun or node run goes through `repo capacity run`): q.sh <megabytes> <working directory> <timeout seconds> <command> <args…>. The command waits in the queue until the host can carry it; the timeout counts the run only.
mb="$1"; dir="$2"; t="$3"; shift 3
cd /Users/ueli/Documents/semio && repo capacity run --memory="$mb" -- zsh -c 'cd "$1" && shift && exec timeout "$@"' q "$dir" "$t" "$@"
