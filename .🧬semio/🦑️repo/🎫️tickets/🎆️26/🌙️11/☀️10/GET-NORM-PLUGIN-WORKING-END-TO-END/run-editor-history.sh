#!/usr/bin/env bash
# 🦀️ Runs one integration test of an editor crate: run-editor-history.sh <family dir> <crate> <test name> <log>
T="$(cd "$(dirname "$0")" && pwd)"
cd "$T/../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/$1" || exit 1
python3 "$T/lane.py" timeout 2700 repo cargo --store=shared --configuration=prototype -- test -p "$2" --test integration --offline -- $3 > "$T/🗑️generated/$4" 2>&1
echo "exit $?" >> "$T/🗑️generated/$4"
