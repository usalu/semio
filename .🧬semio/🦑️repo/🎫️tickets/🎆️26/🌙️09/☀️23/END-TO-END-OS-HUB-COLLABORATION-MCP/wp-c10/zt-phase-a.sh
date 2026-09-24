#!/bin/zsh
# Zero-touch phase A, timed: the exact catalog publish `ensureTrustedCatalog` spawns for a clean `dev s` data root.
# Usage: zsh zt-phase-a.sh <cleanDataRoot>  (run under the wasm fleet mutex; durable root: .🧬semio/🌐hub/s11-c10-zt-hub-data)
ROOT="$1"
REPO=/Users/ueli/Documents/semio
rm -rf "$ROOT"; mkdir -p "$ROOT"; chmod 700 "$ROOT"
echo "PHASE-A START $(date +%s) root=$ROOT"
cd "$REPO" || exit 1
START=$(date +%s)
OS_HUB_DATA="$ROOT" CARGO_INCREMENTAL=0 NX_DAEMON=false bun nx run os-hub:trusted-catalog-bootstrap --packages stdio,gis,note,writer,draw,puzzle --outputStyle=stream
RC=$?
echo "PHASE-A END rc=$RC secs=$(( $(date +%s) - START )) current=$(test -f "$ROOT/trusted-catalog/current.json" && echo yes || echo no)"
