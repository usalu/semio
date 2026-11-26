#!/bin/zsh
# 🚦️ Runs one command of round MV-L2 through the machine-wide memory queue: q.sh <megabytes> <working directory> <timeout seconds> <command> <args…>.
mb="$1"; dir="$2"; t="$3"; shift 3
cd /Users/ueli/Documents/semio && repo capacity run --memory="$mb" -- zsh -c 'cd "$1" && shift && exec timeout "$@"' q "$dir" "$t" "$@"
