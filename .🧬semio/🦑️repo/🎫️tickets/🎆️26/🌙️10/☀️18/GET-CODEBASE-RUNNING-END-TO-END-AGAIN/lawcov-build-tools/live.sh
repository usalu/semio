#!/bin/zsh
# 🚀 Installs a work set live into its unit and runs it there: live.sh <slug> <unit> [tail]
set -e
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lawcov-build-tools
T=$(python3 lc.py live "$1" "$2" | grep '^TEST ' | cut -c6-)
load=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1)
if [ "$load" -ge 45 ]; then echo "load $load >= 45, not running"; exit 3; fi
cd "$(dirname "$T")"
nice -n 10 bun test "$T" 2>&1 | tail -${3:-30}
