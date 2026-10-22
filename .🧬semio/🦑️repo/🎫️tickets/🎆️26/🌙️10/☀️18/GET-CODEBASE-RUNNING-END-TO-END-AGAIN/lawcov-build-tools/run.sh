#!/bin/zsh
# 🏃 Re-installs one work set into its overlay and runs it: run.sh <slug> <unit> [tail]
set -e
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lawcov-build-tools
rm -rf /private/tmp/lawcov-build/$1
T=$(LAWCOV_PATCH="${LAWCOV_PATCH:-}" python3 lc.py overlay "$1" "$2" | grep '^TEST ' | cut -c6-)
load=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1)
if [ "$load" -ge 45 ]; then echo "load $load >= 45, not running"; exit 3; fi
cd /private/tmp/lawcov-build/$1
SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio nice -n 10 bun test "$T" 2>&1 | tail -${3:-60}
