#!/bin/zsh
# 🚂️ S5-AGNOSTIC: runs `🧪️s5-agnostic-run-acceptance.sh` for the named crates ONE AFTER ANOTHER (one cargo at a time, rule 48) and
# refreshes the report table after each. Stops when free disk is below the runner's floor, when my own harness landing takes the lock
# (`run/landing-hold`), or after the last crate. Single-flight and re-issuable: a call that finds a batch running waits for it (a Bash
# call is capped at 10 min) and prints the rows written so far. A crate that already has a result is skipped by the runner.
# Usage: <crate>…
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
G="$T/🗑️generated/s5-agnostic"
mkdir -p "$G/run"
pidf="$G/run/batch.pid"
alive() { [ -f "$pidf" ] && kill -0 "$(cat "$pidf")" 2>/dev/null }
rows() { awk -F'\t' '{print $1, $2, $4, $5 "/" $6 "/" $7 "/" $8}' "$G/acceptance-results.tsv" 2>/dev/null | tail -${1:-6} }
if alive; then
  waited=0
  while alive && [ "$waited" -lt 540 ]; do sleep 10; waited=$((waited + 10)); done
  if alive; then echo "BATCH STILL RUNNING: $(cat "$G/run/batch.current" 2>/dev/null)"; rows; exit 3; fi
  echo "batch finished: $(cat "$G/run/batch.end" 2>/dev/null)"; rows 12; exit 0
fi
echo $$ > "$pidf"; rm -f "$G/run/batch.end"
reason="all crates done"
for crate in "$@"; do
  if [ -e "$G/run/landing-hold" ]; then reason="stopped before $crate: my harness landing holds the lock"; break; fi
  echo "$crate since $(date '+%T')" > "$G/run/batch.current"
  zsh "$T/🧪️s5-agnostic-run-acceptance.sh" "$crate" > "$G/run/batch.last" 2>&1
  code=$?
  python3 "$T/🧪️s5-agnostic-table.py" > /dev/null 2>&1
  if [ "$code" = 4 ]; then reason="stopped at $crate: $(tail -1 "$G/run/batch.last")"; break; fi
  if [ "$code" = 7 ]; then reason="stopped at $crate: interrupted for my harness landing"; break; fi
done
echo "$reason $(date '+%T')" | tee "$G/run/batch.end"
rm -f "$pidf" "$G/run/batch.current"
rows 12
