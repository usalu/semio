#!/bin/zsh
# 🚥️ Every heavy tool process of this fixer: q.sh <command…> runs one at a time (the lock of the batch, a file outside the shared worktree because the replica daemon rewrites files inside it) and only through the machine-wide memory queue of the repo, `repo capacity run --memory=<MB>` from the repo root (coordinator rule of 2026-11-24 after the host ran out of memory): 7000 MB for the matrix, 5000 MB for the check CLI and for the proof script that contains it, 2500 MB for everything else (silhouette, gate, measure, vitest, sheets); MEM=<MB> states another reservation. The command runs in the directory q.sh was called from; waiting in the queue is intended. Inside a grant this fixer already holds (MH5_GRANT=1, set by the caller that started a whole chain under one q.sh) the command runs directly: one process after the other inside the one reservation.
[[ -n "$MH5_GRANT" ]] && exec "$@"
mem="$MEM"
if [[ -z "$mem" ]]; then
  case "$*" in
    *matrix.ts*) mem=800 ;;
    *🩺️check*) mem=1000 ;;
    *vitest*|*vt.sh*|*prove.sh*) mem=2500 ;;
    *) mem=800 ;;
  esac
fi
here="$PWD"
cd /Users/ueli/Documents/semio
exec lockf -k "${MH5_LOCK:-${TMPDIR:-/tmp}/semio-matrix-MH-5.lock}" repo capacity run --memory="$mem" -- /bin/zsh -c 'cd "$1" && shift && exec "$@"' q "$here" "$@"
