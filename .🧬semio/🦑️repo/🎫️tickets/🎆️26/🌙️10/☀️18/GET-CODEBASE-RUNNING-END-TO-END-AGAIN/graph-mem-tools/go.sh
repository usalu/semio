#!/bin/sh
# after the coordinator's GO and a finished graph build: regenerate ops, check, apply, run the laws of every touched area against the live tree
# usage: go.sh [--write]      (default: dry run of the apply; nothing is written)
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && while [ ! -e .git ]; do cd ..; done; pwd)
python3 "$HERE/gen_memory_fixture.py" >/dev/null
for gen in "$HERE"/gen_*_fixture.py; do python3 "$gen" >/dev/null; done
python3 "$HERE/ops_def.py"
python3 "$HERE/apply.py" check || true
[ "$1" = "--write" ] || { echo "dry run; use --write to apply"; exit 0; }
python3 "$HERE/apply.py" apply --write
sh "$HERE/guard.sh"
sh "$HERE/verify.sh" "$ROOT"
echo "next: bun repo tools setup nx (the loaded graph module authority changed), then a cold bun repo show projects with SEMIO_GRAPH_PHASES=1 and sh $HERE/measure-after-go.sh"
