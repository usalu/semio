#!/bin/sh
# 🏗️ Rework 05: builds the candidate document of one asset (roots, model, drawings) and validates it: rework-05-build.sh <slug> [--volumes]
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; shift
python3 "rework-05-$slug-roots.py" > /dev/null
python3 "rework-05-$slug-model.py" > /dev/null
python3 rework-05-draw2d.py "$slug" > /dev/null
[ -f "rework-05-$slug-finish.py" ] && python3 "rework-05-$slug-finish.py"
"$BUN" r3-validate.ts --file "🗑️generated/r3-$slug/🔣️.json" "$@" > "🗑️generated/rework-05/$slug.validate.txt" 2>&1 || true
tail -1 "🗑️generated/rework-05/$slug.validate.txt" | cut -c1-220
grep -E '^(ASSET|PRINT)' "🗑️generated/rework-05/$slug.validate.txt" | cut -d' ' -f1-2 | sort | uniq -c | sort -rn | head -20
grep -c '^SCHEMA' "🗑️generated/rework-05/$slug.validate.txt" || true
