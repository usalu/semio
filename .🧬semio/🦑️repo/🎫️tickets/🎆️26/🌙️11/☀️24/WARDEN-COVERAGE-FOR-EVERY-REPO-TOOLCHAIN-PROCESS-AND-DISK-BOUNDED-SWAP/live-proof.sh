#!/bin/sh
# Starts two real pinned rustc processes from a plain shell (no repo runtime among their ancestors); each blocks reading stdin for 14 s.
set -eu
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
RC="$(ls -d "$ROOT"/.cache/b/t/m/r/*/toolchains/*/bin/rustc | head -1)"
OUT="$(dirname "$0")/🗑️generated/proof"
mkdir -p "$OUT"
for n in 1 2; do
  ( sleep 14; echo "pub fn proof() {}" ) | "$RC" - --crate-name "warden_proof$n" --crate-type lib --emit=metadata --out-dir "$OUT" > "$OUT/rustc$n.log" 2>&1 &
done
wait
