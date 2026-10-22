#!/bin/sh
# After any set landed (or before applying): recompute the feature states, regenerate the edits from the pristine README, verify, dry-run.
# Usage: sh refresh.sh [--write]     (--write applies; never run it while the live-apply freeze is on)
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE"
python3 -I make-edits.py | sed -n 1,2p
python3 -I check.py | tail -3
python3 -I apply.py "$@" | tail -12
