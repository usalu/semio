#!/usr/bin/env bash
# 🦀️ en1990 mutations tests (lib and committed vectors) with the same feature set as the other touched packages; replay_preparation_physical is known broken and excluded.
cd "$(dirname "$0")/../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990" || exit 1
G="$(cd "$(dirname "$0")" && pwd)/🗑️generated"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-mutations -p semio-s-artifact-norm-en1990-model -p semio-s-artifact-norm-en1990-diff -p semio-s-artifact-norm-en1990-io-text -p semio-s-artifact-norm-en1990-mut-factor-overrides -p semio-s-artifact-norm-en1990-mut-traffic-groups -p semio-s-artifact-norm-en1990-editor --lib --no-fail-fast --offline -- --skip latency_bench > "$G/en1990-rust-mutations-lib.log" 2>&1
echo "[DEBUG] mutations lib exit $?"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-mutations -p semio-s-artifact-norm-en1990-model -p semio-s-artifact-norm-en1990-diff -p semio-s-artifact-norm-en1990-io-text -p semio-s-artifact-norm-en1990-mut-factor-overrides -p semio-s-artifact-norm-en1990-mut-traffic-groups -p semio-s-artifact-norm-en1990-editor --test mutation_vectors --no-fail-fast --offline > "$G/en1990-rust-mutations-vectors.log" 2>&1
echo "[DEBUG] mutations vectors exit $?"
