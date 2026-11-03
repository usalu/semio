#!/usr/bin/env bash
set -uo pipefail
PY=/home/deck/semio/.venv/bin/python
NORM=/home/deck/semio/✏️s/🔌️plugins/📕️norm
HERE="$(cd "$(dirname "$0")" && pwd)"
F="$NORM/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party-v3-extended.json"
"$PY" -I "$HERE/oracle-en1991-v3-gen.py"
"$PY" -I "$NORM/🔮️oracles/🏃️harness/🐍️.py" --cases "$F" --bless 2>&1 | grep -v '^ok' || true
"$PY" -I "$NORM/🔮️oracles/🏃️harness/🐍️.py" --cases "$F" | awk '{print $1}' | sort | uniq -c
