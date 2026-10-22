#!/bin/sh
# usage: verify.sh [--full]
# Overlay verification of the agent-session set, never touching the checkout: clones the live tree twice (APFS clones under /private/tmp/agent-session), applies the set to one clone,
# regenerates the help snapshots, runs the test battery on both clones and prints the differences, then type-checks, runs the Go module tests and the mutation check.
# Every step runs under nice -n 10, one heavy process at a time; it refuses to start while the load average is 30 or higher.
set -e
T="$(cd "$(dirname "$0")" && pwd)"
C="$(dirname "$T")"
load="$(uptime | sed -E 's/.*load averages?: ([0-9.]+).*/\1/')"
if [ "$(echo "$load >= 30" | bc)" = 1 ]; then echo "load $load is 30 or higher: wait"; exit 3; fi
BASE=/private/tmp/agent-session/ov-v0
NEW=/private/tmp/agent-session/ov-v
rm -rf "$BASE" "$NEW"
nice -n 10 python3 "$T/overlay.py" --tag v0 > /dev/null
nice -n 10 cp -c -R "$BASE" "$NEW"
python3 "$T/apply.py" --root "$NEW" --group core,routing,nx-watchers          # dry run
python3 "$T/apply.py" --root "$NEW" --group core,routing,nx-watchers --write --backup /private/tmp/agent-session/backup-v --ignore-running
python3 "$T/apply.py" --root "$NEW" --group core,routing,nx-watchers          # second run: everything applied
bun "$C/mem-gov-tools/regen-help.ts" "$NEW"
nice -n 10 python3 "$T/battery.py" "$BASE" > /private/tmp/agent-session/results/base.jsonl
nice -n 10 python3 "$T/battery.py" "$NEW" > /private/tmp/agent-session/results/new.jsonl
python3 "$T/compare.py" /private/tmp/agent-session/results/base.jsonl /private/tmp/agent-session/results/new.jsonl
sh "$T/go-run.sh" "$NEW"                                  # Go tests of the repo MCP module (canonical Go overlay)
sh "$T/tsc-check.sh" "$NEW" "$T/tsc-files.txt"            # strict type check of the new and edited TypeScript
nice -n 10 python3 "$T/mutation-check.py" "$NEW"                                # the tests can fail
if [ "$1" = "--full" ]; then
  HC=/private/tmp/agent-session/ov-hc3
  rm -rf "$HC"; nice -n 10 python3 "$T/overlay.py" --tag hc3 > /dev/null
  python3 "$C/hot-cache-tools/staged/apply.py" --root "$HC" --write
  python3 "$T/apply.py" --root "$HC" --group all --write --backup /private/tmp/agent-session/backup-hc3 --ignore-running
fi
