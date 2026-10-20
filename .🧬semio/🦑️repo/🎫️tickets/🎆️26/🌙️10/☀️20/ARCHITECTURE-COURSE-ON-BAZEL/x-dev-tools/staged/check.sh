#!/usr/bin/env bash
# x-dev check: proves the staged manifest edits and the manifest test on a scratch copy, then reports whether they are applied to the live tree.
# usage: bash check.sh            (scratch proof; then live state)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE" && while [ ! -f AGENTS.md ]; do cd ..; done; pwd)"
SCRATCH="$HERE/../../🗑️generated/x-dev/scratch"
COURSE="🎓️teaching/🏛️architecture/📚️course"
rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/$COURSE/📦️packages/🟦️typescript" "$SCRATCH/🎓️teaching/🛂️proctor/📦️packages/🦀️rust" "$SCRATCH/$COURSE/🧪️tests/🟢️continuous"
SCRATCH="$(cd "$SCRATCH" && pwd)"
cp "$ROOT/$COURSE/📦️packages/🟦️typescript/📋️project.json" "$SCRATCH/$COURSE/📦️packages/🟦️typescript/"
cp "$ROOT/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json" "$SCRATCH/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/"
cp -R "$ROOT/$COURSE/🧫️fixtures" "$SCRATCH/$COURSE/🧫️fixtures"
ln -s "$ROOT/🧰️framework" "$SCRATCH/🧰️framework"
ln -s "$ROOT/node_modules" "$SCRATCH/node_modules"
X_DEV_ROOT="$SCRATCH" python3 "$HERE/apply.py"
X_DEV_ROOT="$SCRATCH" python3 "$HERE/apply.py"
mkdir -p "$SCRATCH/🎓️teaching/🛂️proctor/🏗️bootstrap"
(cd "$SCRATCH/$COURSE" && bun test "./🧪️tests/🟢️continuous/🧾️manifests/🟦️.ts")
echo "--- live tree"
python3 "$HERE/apply.py" --check
