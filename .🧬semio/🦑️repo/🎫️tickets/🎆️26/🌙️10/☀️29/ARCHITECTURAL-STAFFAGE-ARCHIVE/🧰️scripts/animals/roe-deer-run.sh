#!/usr/bin/env bash
# [scratch] re-emits, assembles, checks and previews the roe deer; usage: roe-deer-run.sh [preview args...]
set -u
D="$(cd "$(dirname "$0")" && pwd)"; T="$D/../.."; O="$T/🗑️generated/animals"; S=/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
mkdir -p "$O/roe-deer"
python3 "$D/roe-deer-gen.py" && python3 "$D/roe-deer-emit.py" "$D" "$O/roe-deer.json" && bash "$D/category.sh" "$O/roe-deer.json" roe-deer || exit 1
cd /home/deck/semio
repo tools exec bun -- "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$O/roe-deer.json" 2>&1 | grep -v '^\[' 
