#!/usr/bin/env bash
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
retry() { log=$1; shift; while true; do "$T/l5d-gated-run.sh" "$@" > /dev/null 2>&1; if grep -q "needs at least" "$log"; then sleep 120; else break; fi; done; }
until grep -q "^RC:" "$G/l5d-final-m5.log" 2>/dev/null; do sleep 30; done
retry "$G/l5d-final-v1.log" 0 7000 "$G/l5d-final-v1.log" --no-fail-fast --lib -p semio-s-artifact-puzzle-5d-v1-unit-tests
retry "$G/l5d-light.log" 0 3000 "$G/l5d-light.log" --no-fail-fast --lib -p semio-s-artifact-puzzle-5d-mut-any-5 $(sed 's/^/-p /' "$G/l5d-pkgs.txt" | grep -v "v1-unit-tests\|mut-any-5$" | tr '\n' ' ')
