#!/usr/bin/env bash
set -euo pipefail
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests"
cd /home/deck/semio
for case in "$@"; do
  repo tools exec uv -- run --with scipy --with CoolProp --with numpy python -I "$S/$case/🐍️.py" ${WRITE:+--write}
done
