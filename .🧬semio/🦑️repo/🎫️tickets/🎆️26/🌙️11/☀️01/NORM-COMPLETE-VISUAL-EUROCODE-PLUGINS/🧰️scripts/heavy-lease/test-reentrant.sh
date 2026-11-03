#!/usr/bin/env bash
# Regression test: a command that holds the heavy slot must be able to call heavy.sh again (evolve.py --apply runs `schema twins generate/check` and `verify build-owners sync` through heavy.sh while itself wrapped in heavy.sh). Uses a scratch runner directory (own lock), never the shared one.
# Usage: test-reentrant.sh <path-to-heavy.sh>   (exit 0 = nested call returned, 1 = nested call deadlocked)
set -u
SRC=$(readlink -f "$1"); S=$(mktemp -d); trap 'rm -rf "$S"' EXIT
cp "$SRC" "$S/heavy.sh"
start=$(date +%s)
timeout 10 bash "$S/heavy.sh" bash -c "timeout 5 bash '$S/heavy.sh' true; echo \$?" >"$S/out" 2>&1
inner=$(tail -1 "$S/out"); end=$(date +%s)
printf 'nested heavy.sh exit=%s elapsed=%ss (124 = deadlock until timeout)\n' "$inner" "$((end - start))"
[ "$inner" = 0 ]
