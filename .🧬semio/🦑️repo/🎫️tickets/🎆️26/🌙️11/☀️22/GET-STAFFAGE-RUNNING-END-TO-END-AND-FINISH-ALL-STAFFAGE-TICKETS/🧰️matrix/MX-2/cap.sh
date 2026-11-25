#!/bin/zsh
# 🚏️ Starts one run through the machine-wide memory queue of the repo: cap.sh <megabytes> <timeout seconds> <working directory> <command> <args…> waits in `repo capacity run` until the host can carry the reservation, then runs the command under `timeout` in the directory. Reservations (queue rule of 2026-11-25 09:20): 800 scan, gate and small scripts, 1000 check CLI, 1200 matrix of one asset, 2500 a vitest suite with a name filter; at most two requests of MX-2 in the queue.
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"
mb="$1"; secs="$2"; dir="$3"; shift 3
cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory="$mb" -- timeout "$secs" sh -c 'cd "$1" && shift && exec "$@"' sh "$dir" "$@"
