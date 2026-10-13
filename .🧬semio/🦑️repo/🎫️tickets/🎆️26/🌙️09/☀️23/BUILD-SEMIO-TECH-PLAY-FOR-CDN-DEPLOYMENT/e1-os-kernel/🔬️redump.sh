#!/bin/bash
# 🔬️ Re-dumps the os-kernel HIR (native, features sync,ureq, private E1 dirs, through the cargo slot) and rebuilds the owner graph.
# usage: 🔬️redump.sh <owners-out.json>
HERE="$(cd "$(dirname "$0")" && pwd)"; REPO=/Users/ueli/Documents/semio; cd "$REPO" || exit 2
E1="$REPO/.🧬semio/🦑️repo/⚡️cache/e1/os-kernel"; DUMP="$E1/hir.txt"
env CARGO_TARGET_DIR="$E1/t" CARGO_BUILD_BUILD_DIR="$E1/b" CARGO_INCREMENTAL=0 "$(dirname "$HERE")/🚦️cargo-slot.sh" e1-os-kernel -- \
  cargo rustc -p semio-framework-os-kernel --lib --profile check --features sync,ureq -- -Zunpretty=hir-tree > "$DUMP" 2> "$E1/hir.err" || { tail -20 "$E1/hir.err"; exit 1; }
python3 "$HERE/🔬️hir-owner-graph.py" "$DUMP" "$1"; rm -f "$DUMP"
