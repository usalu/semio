#!/bin/sh
# 📏️ Measures one asset through the capacity queue: measure.sh <label> <asset id> <memory MB> <timeout s> [one.ts flags…]
# SNAP=<dir> names the folder of the frozen asset documents (<asset id>.json); `--suite <dir>` among the flags runs another copy of the suite files.
# Writes <generated>/<label>.cells.json (cells), <label>.log (probe lines and /usr/bin/time -l).
set -u
R=/Users/ueli/Documents/semio
U="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
G="$U/🗑️generated/matrix/MM"
SNAP="${SNAP:-/private/tmp/claude-501/-Users-ueli-Documents-semio/77b365fc-dfc7-4b6f-9d54-4f0eb23231c9/scratchpad/snap}"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
label=$1; asset=$2; memory=$3; limit=$4; shift 4
mkdir -p "$G"
cd "$R" || exit 1
queued=$(date +%s)
repo capacity run --memory="$memory" -- /usr/bin/time -l timeout "$limit" "$BUN" "$U/🧰️matrix/MM/one.ts" --snapshot "$SNAP/$asset.json" --out "$G/$label.cells.json" "$@" > "$G/$label.log" 2>&1
code=$?
echo "exit $code, queued+run $(( $(date +%s) - queued )) s" >> "$G/$label.log"
grep -E "maximum resident|real|peak memory|\] [a-z-]+ (done|failed)|^exit" "$G/$label.log"
