#!/usr/bin/env bash
# usage: carve-test.sh <manifest-dir> <log-name> <cargo test args...>
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
D="$1"; L="$T/🗑️generated/$2.log"; shift 2
cd "$D" || exit 1
python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test "$@" --offline > "$L" 2>&1
echo "EXIT $?" >> "$L"
