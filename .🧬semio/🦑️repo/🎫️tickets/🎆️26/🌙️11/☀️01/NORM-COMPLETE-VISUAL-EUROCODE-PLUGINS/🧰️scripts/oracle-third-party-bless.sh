#!/usr/bin/env bash
set -euo pipefail
PY=/home/deck/semio/.venv/bin/python
NORM=/home/deck/semio/✏️s/🔌️plugins/📕️norm
HERE="$(cd "$(dirname "$0")" && pwd)"
"$PY" -I "$HERE/oracle-third-party-gen.py"
for f in "$NORM"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party*.json; do
  "$PY" -I "$NORM/🔮️oracles/🏃️harness/🐍️.py" --cases "$f" --bless
done | grep -v '^ok' || true
"$PY" -I "$NORM/🔮️oracles/🏃️harness/🐍️.py" --cases "$NORM"/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json "$NORM/🧪️tests/🧫️fixtures/🔮️oracle-cases/numerical.json" | sort | uniq -c | awk '{print $2}' | sort | uniq -c
