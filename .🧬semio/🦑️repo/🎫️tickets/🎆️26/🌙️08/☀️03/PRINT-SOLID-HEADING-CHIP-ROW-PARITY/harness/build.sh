#!/bin/sh
# 🧪️ Ticket-local compile harness: ASCII symlinks of print's .sty/.cls (LaTeX cannot
# resolve the astral-plane emoji file names on disk) plus an ASCII font stack.
set -e
H="$(cd "$(dirname "$0")" && pwd)"
TEC="/Users/ueli/Documents/semio/.🦑️repo/⚡️cache/tectonic/0.16.9/tectonic"
JOB="${1:-verify-headings}"
cd "$H"
"$TEC" --keep-logs --reruns 2 -Z search-path="$H/tex" --outdir "$H" "$JOB.tex"
