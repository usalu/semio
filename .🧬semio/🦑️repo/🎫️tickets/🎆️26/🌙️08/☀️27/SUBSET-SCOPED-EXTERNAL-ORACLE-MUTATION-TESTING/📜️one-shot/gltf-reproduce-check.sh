#!/usr/bin/env bash
set -euo pipefail
GLTF_ANY="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any"
GEN="$GLTF_ANY/🏭️generator/📜️script.ts"
FIXTURES="$GLTF_ANY/🧫️fixtures"
SCRATCH_ROOT="$1"
ids=$(ls "$FIXTURES")
total=0
bad=0
for id in $ids; do
  total=$((total+1))
  outdir="$SCRATCH_ROOT/$id-run"
  rm -rf "$outdir"
  mkdir -p "$outdir"
  # separate bun process PER fixture, exactly per-item, never a batch double-run
  SEMIO_FIXTURE_OUT="$outdir" bun "$GEN" generate --only "$id" > /dev/null 2>"$outdir/stderr.log" || { echo "[GEN-FAIL] $id"; bad=$((bad+1)); continue; }
  for side in before after; do
    committed="$FIXTURES/$id/$side.gltf"
    produced="$outdir/$id/$side.gltf"
    if [ ! -f "$produced" ]; then
      echo "[MISSING] $id/$side"
      bad=$((bad+1))
      continue
    fi
    c_hash=$(shasum -a 256 "$committed" | cut -d' ' -f1)
    p_hash=$(shasum -a 256 "$produced" | cut -d' ' -f1)
    if [ "$c_hash" != "$p_hash" ]; then
      echo "[HASH-MISMATCH] $id/$side committed=$c_hash produced=$p_hash"
      bad=$((bad+1))
    fi
  done
  rm -rf "$outdir"
done
echo ""
echo "reproduce-check: $total fixtures, $bad problem(s)"
