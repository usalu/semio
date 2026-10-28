#!/bin/sh
# 🧑‍🤝‍🧑 T9 turnkey (agents3): the real three-agent acceptance run on the declared 16 GB machine, or its dry run.
# Usage: sh run.sh [--dry-run | --preflight] [--prime] [--wait-quiet] [--force] [--tag T]   (see `python3 -I run.py --help`)
# Real run, quiet host: sh run.sh --prime   -> prints the verdict table, writes 🗑️generated/agents3/verdict.json
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
exec python3 -I "$HERE/run.py" "$@"
