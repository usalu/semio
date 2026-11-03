#!/usr/bin/env bash
set -uo pipefail
PY=/home/deck/semio/.venv/bin/python
NORM=/home/deck/semio/✏️s/🔌️plugins/📕️norm
HERE="$(cd "$(dirname "$0")" && pwd)"
"$PY" -I "$HERE/oracle-wave2-v2-gen.py"
for f in "$NORM"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party*v2-extended.json; do
  "$PY" -I "$NORM/🔮️oracles/🏃️harness/🐍️.py" --cases "$f" --bless 2>&1 | grep -v '^ok' || true
done
"$PY" -I "$NORM/🔮️oracles/🏃️harness/🐍️.py" --cases "$NORM"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party*v2-extended.json | awk '{print $1}' | sort | uniq -c
