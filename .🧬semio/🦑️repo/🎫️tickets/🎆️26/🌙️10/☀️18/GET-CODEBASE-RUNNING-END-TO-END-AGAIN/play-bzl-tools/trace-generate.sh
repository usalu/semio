#!/usr/bin/env bash
# Runs `s-dev-composition:generate` under the fs read tracer and prints every read that neither the generator contract nor the catalog discovery declares.
# Run only after GO (it regenerates the catalog output, idempotently) with at most one heavy process.  Usage: trace-generate.sh
set -eu
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here" && git rev-parse --show-toplevel)"
out="$here/../🗑️generated/play-bzl-run"; mkdir -p "$out"
cd "$root"
TRACE_OUT="$out/generate-reads.txt" BZL_LAZY_SELECTION="run @semio-tech/s-dev-composition:generate" \
  bun "$here/envelope-run.ts" --preload "$here/fs-trace.ts" "$root/✏️s/🧑‍💻dev/📜️script.ts" catalog-publication generate
bun "$here/compare-reads.ts" "$out/generate-reads.txt"
