#!/bin/sh
# usage: proof.sh <tag>  (sequential proofs of expc-d; one heavy process at a time)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; out="$C/🗑️generated/expc-d"
run() { name="$1"; shift; s=$(date +%s); SEMIO_LANGUAGE=en env -u SEMIO_SCRIPT_STALL_MS nice -n 10 "$@" > "$out/proof-$tag-$name.log" 2>&1; rc=$?; printf '%s\t%s\t%ss\t%s\n' "$name" "$rc" "$(( $(date +%s)-s ))" "$*" >> "$out/proofs.tsv"; }
run course-typecheck bun repo --tool=bazel run @teaching/architecture-course:typecheck
run print-test bun repo --tool=bazel run @semio-tech/print:test
for t in test-build-facets test-leases test-routing-owners test-module-basics test-owner-context test-tools test-cargo-workspace test-cargo-overlap test-cargo-target-recipe; do
  run "$t" bun repo --tool=semio run "@semio-tech/framework-process:$t" --exclude-dependencies
done
echo done > "$out/proofs-$tag.done"
