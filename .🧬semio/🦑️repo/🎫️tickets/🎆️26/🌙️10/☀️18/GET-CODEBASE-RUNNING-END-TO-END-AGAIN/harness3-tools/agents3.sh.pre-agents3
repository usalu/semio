#!/bin/sh
# 🧑‍🤝‍🧑 harness3: three concurrent agents on the declared 16 GB machine (official runs: course edit, print edit, dashboard no-op/peer/no-op).
# Usage: sh agents3.sh [name]  -> output below 🗑️generated/harness3/<name>, summary.md at the end
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
C=$(dirname "$HERE")
NAME=${1:-agents3}
exec python3 -I "$C/agents3-g-tools/agents3.py" run --tag "$NAME" --out "$C/🗑️generated/harness3/$NAME"
