#!/bin/zsh
# 🚏️ Starts one bun or node run through the machine-wide memory queue of the repo (binding since 2026-11-24 23:00): q.sh <MB> <timeout s> <working directory> <command…> runs `repo capacity run --memory=<MB> -- timeout <s> <command…>` from the repo root, the command itself in its working directory; it waits in the queue until the host can carry it. Reserve (queue rule of 2026-11-25 09:20) 800 for scans, gates and small scripts, 1000 for the check CLI with --asset, 1200 for the matrix of one asset, 2500 for a vitest suite with a name filter; more only after the peak of that very run was measured.
mb="$1"; t="$2"; dir="$3"; shift 3
cd /Users/ueli/Documents/semio && exec repo capacity run --memory="$mb" -- timeout "$t" /bin/zsh -c 'cd "$1" && shift && exec "$@"' _ "$dir" "$@"
