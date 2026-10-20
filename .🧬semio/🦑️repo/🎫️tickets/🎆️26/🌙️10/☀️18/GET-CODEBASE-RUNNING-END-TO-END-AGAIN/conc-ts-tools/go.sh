#!/bin/sh
# usage: go.sh [--apply]   Applies, in order: group 1 (engine), group 2 (source fixes), group 3 (typecheck wiring). Dry run by default; stops at the first refusal.
cd "$(dirname "$0")" || exit 1
A="$1"
python3 apply.py $A || exit 1
python3 staged2/fixes.py $A || exit 1
python3 staged3/apply.py $A || exit 1
echo "after --apply: bun test the engine suite (L/🏗️build/🟦️typescript/🧩️units/🧪️tests) and the typecheck selection suite (L/🎛️commands/🪁️typecheck/🧪️tests), then: bun repo typecheck units tsconfig.json --jobs 2"
