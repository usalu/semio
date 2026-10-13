#!/usr/bin/env bash
# ⚖️ Old-vs-new taxonomy verdict identity for scopes: runs the current module and the pre-change snapshot back to back on each scope,
# then byte-compares the two reports. argv: <repoRoot> <oldNormalizationModule> <outDir> <scope>...
set -u
root="$1"; old="$2"; out="$3"; shift 3
phases="$(dirname "$0")/⏱️taxonomy-phases.ts"
mkdir -p "$out"
for scope in "$@"; do
  key=$(printf '%s' "$scope" | shasum | cut -c1-10)
  start=$(date +%s); ( cd "$root" && bun "$phases" "$root" 7200 "$scope" "$out/$key.new.txt" > "$out/$key.new.log" 2>&1 ); mid=$(date +%s)
  ( cd "$root" && TAXONOMY_MODULE="$old" bun "$phases" "$root" 7200 "$scope" "$out/$key.old.txt" > "$out/$key.old.log" 2>&1 ); end=$(date +%s)
  moves=$(grep -c "normalization-move-required" "$out/$key.new.txt" 2>/dev/null)
  if cmp -s "$out/$key.new.txt" "$out/$key.old.txt"; then verdict=identical; else verdict=DIFFERENT; fi
  printf '%s\t%s\tnew=%ss\told=%ss\tviolations=%s\tmoves=%s\t%s\n' "$verdict" "$scope" "$((mid - start))" "$((end - mid))" "$(head -1 "$out/$key.new.txt" 2>/dev/null)" "$moves" "$key"
done
