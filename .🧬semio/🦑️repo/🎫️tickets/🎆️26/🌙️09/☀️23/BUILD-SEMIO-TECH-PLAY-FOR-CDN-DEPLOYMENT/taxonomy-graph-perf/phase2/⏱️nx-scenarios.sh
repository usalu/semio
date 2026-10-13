#!/usr/bin/env bash
# ⏱️ Times the Nx graph scenarios through the repository's own `bun nx` wrapper against one private workspace data directory:
# fresh (nothing cached), sealed reuse, inference hit without a seal, and an inference miss with warm per-file records and stamp hints.
# argv: <repoRoot> <workspaceData> <outDir>
set -u
root="$1"; data="$2"; out="$3"; stale="$(dirname "$0")/🩹️stale-inference-digest.mjs"
mkdir -p "$out"; rm -rf "$data"; mkdir -p "$data"
run() {
  local name="$1" start end
  start=$(python3 -c 'import time; print(time.time())')
  ( cd "$root" && NX_WORKSPACE_DATA_DIRECTORY="$data" NX_PERF_LOGGING=true bun nx show projects > "$out/$name.log" 2>&1 )
  end=$(python3 -c 'import time; print(time.time())')
  printf '%s\t%.1fs\t%s\n' "$name" "$(python3 -c "print($end-$start)")" "$(grep -Eo "Time taken for '(createProjectGraphAsync|read cache)' [0-9.]+ms" "$out/$name.log" | tr '\n' ' ')"
}
run fresh
run warm-hit
run sealed-reuse
rm -f "$data/semio-graph-seal.json"
run inference-hit-unsealed
rm -f "$data/semio-graph-seal.json"
node "$stale" "$data/inference/emoji-project-json.json" "$data/inference/test-cases.json" > /dev/null
run inference-miss-warm-records
