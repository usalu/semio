#!/bin/sh
# 🩺 Runs the staffage check CLI per category, three categories at a time, each through the machine-wide memory queue (`repo capacity run`, 5000 MB), into 🗑️generated/check-<category>.txt.
# Usage: sh check-categories.sh <category>...
U="$(cd "$(dirname "$0")" && pwd)"
S="$U/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
mkdir -p "$U/🗑️generated"
cd "$S" || exit 1
printf '%s\n' "$@" | xargs -P 3 -I{} sh -c 'repo capacity run --memory=5000 -- "$0" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --category {} > "$1/🗑️generated/check-{}.txt" 2>&1; echo "[DEBUG] {} exit $?"' "$BUN" "$U"
