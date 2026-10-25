#!/bin/bash
# Ticket input script: the TypeScript checks of the pets fix, one after the other on Bazel.
cd "$(git rev-parse --show-toplevel)" || exit 2
for target in @semio-tech/pets:typecheck @semio-tech/pets-react:typecheck @semio-tech/pets-react:test @semio-tech/pets:test; do
  echo "[ts-checks] $target $(date +%H:%M:%S)"
  nice -n 10 bun repo --tool=bazel run "$target" > "$(dirname "$0")/../🗑️generated/cpets-d/ts-$(echo "$target" | tr '/:@' '___').log" 2>&1
  echo "[ts-checks] $target exit $?"
done
