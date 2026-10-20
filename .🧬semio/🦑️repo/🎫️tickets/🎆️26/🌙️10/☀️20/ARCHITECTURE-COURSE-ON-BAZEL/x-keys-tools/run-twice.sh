#!/bin/sh
# usage: run-twice.sh <project:target> [outDir]   Runs the Bazel course target twice in a row (foreground) and records counters and generated key files.
# Needs "bun repo boots" in coordination.md. Output: <outDir>/{run1,run2}.log, cache-status-{0,1,2}.txt, keys-{1,2}/ (BUILD, manifests, fingerprints; runtime.json excluded), verdict.txt
set -u
target=${1:?project:target}
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
out=${2:-"$here/../🗑️generated/x-keys/twice-$(echo "$target" | tr '/:@' '___')"}
mkdir -p "$out"
cd "$root"
status() { bun repo --tool=bazel cache status > "$out/cache-status-$1.txt" 2>&1; }
snap() {
  ns=$(ls -td .cache/b/p/m/bazel/workspaces/*/ 2>/dev/null | head -1)
  mkdir -p "$out/keys-$1"
  [ -n "$ns" ] && for f in "$ns"BUILD.bazel "$ns"MODULE.bazel "$ns"tasks.bzl "$ns"kinds.json "$ns"implementation.json "$ns"t_*.json "$ns"t_*.input; do [ -f "$f" ] && cp "$f" "$out/keys-$1/"; done
  echo "$ns" > "$out/keys-$1/WORKSPACE"
}
status 0
start=$(date +%s)
bun repo --tool=bazel run "$target" > "$out/run1.log" 2>&1; echo "rc=$?" >> "$out/run1.log"
mid=$(date +%s); snap 1; status 1
bun repo --tool=bazel run "$target" > "$out/run2.log" 2>&1; echo "rc=$?" >> "$out/run2.log"
end=$(date +%s); snap 2; status 2
{
  echo "run1 seconds: $((mid-start))   run2 seconds: $((end-mid))"
  echo "--- bazel process summary run1"; grep -E "processes:|disk cache hit|remote cache hit|INFO: Build completed|Elapsed time" "$out/run1.log" | tail -5
  echo "--- bazel process summary run2"; grep -E "processes:|disk cache hit|remote cache hit|INFO: Build completed|Elapsed time" "$out/run2.log" | tail -5
  echo "--- key files differing between run1 and run2 (excluding runtime.json)"; diff -rq "$out/keys-1" "$out/keys-2" || true
} > "$out/verdict.txt"
cat "$out/verdict.txt"
