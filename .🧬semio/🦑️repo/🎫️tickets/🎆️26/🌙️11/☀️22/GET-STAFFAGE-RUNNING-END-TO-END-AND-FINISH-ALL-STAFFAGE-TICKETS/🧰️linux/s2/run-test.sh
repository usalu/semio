#!/bin/sh
# 🧪️ Installs test cases of `tests/` into a mirror (never into the worktree) and runs them: with the bun test runner (what `repo test-file` runs, below 500 MB for one file), or with `RUNNER=vitest` under the config of the staffage library through the memory queue of the repo.
# usage: run-test.sh <mirror staffage root> <case> [<case> ...]     (a case that is no folder of tests/ is taken as an existing case of the mirror)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$HERE/../../../../../../../../.." && pwd)
M="$1"
shift
[ -d "$M/🧪️tests" ] || { echo "no mirror at $M" >&2; exit 2; }
[ -L "$M/🧫️fixtures" ] && { echo "the fixtures of $M are a link into the worktree: rebuild the mirror with mirror.sh" >&2; exit 2; }
case "$M" in "$R"/*) echo "refusing to install into the worktree" >&2; exit 2 ;; esac
FILES=""
for c in "$@"; do
  if [ -d "$HERE/tests/$c" ]; then
    mkdir -p "$M/🧪️tests/$c" "$M/🧫️fixtures/$c"
    for f in "$HERE/tests/$c"/*; do
      case "$f" in *🔣️.json) cp "$f" "$M/🧫️fixtures/$c/" ;; *) cp "$f" "$M/🧪️tests/$c/" ;; esac
    done
  fi
  FILES="$FILES ../../🧪️tests/$c/🟦️.ts"
done
BUN=$(ls "$HOME"/.cache/semio/runtimes/bun/*/bun | head -1)
cd "$M/📦️packages/🟦️typescript"
if [ "${RUNNER:-bun}" = "vitest" ]; then
  # shellcheck disable=SC2086
  (cd "$R" && exec repo capacity run --memory=2500 -- timeout 900 sh -c "cd '$M/📦️packages/🟦️typescript' && '$BUN' node_modules/vitest/vitest.mjs run --configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts $FILES")
else
  # shellcheck disable=SC2086
  exec timeout 600 "$BUN" test --timeout 600000 $FILES
fi
