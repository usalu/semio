#!/bin/sh
REPO=/Users/ueli/Documents/semio
T="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR"
L="$T/🗑️generated/e2e"
cd "$REPO" || exit 1
mkdir -p "$L"
step() {
  name=$1; shift; tries=${TRIES:-3}; n=1
  while [ $n -le $tries ]; do
    echo "[DEBUG] $(date +%H:%M:%S) $name attempt $n" >> "$L/chain.log"
    "$@" > "$L/$name.log" 2>&1; rc=$?
    echo "[DEBUG] $(date +%H:%M:%S) $name exit $rc" >> "$L/chain.log"
    [ $rc -eq 0 ] && return 0
    n=$((n+1)); sleep 60
  done
  return 1
}
bunx() { SEMIO_REPO_SEAL=sealed repo tools exec bun -- "$@"; }
[ -n "$RESUME" ] || step catalog bunx "$T/📜️pdf-browser-direct.ts" catalog || { echo "[DEBUG] CHAIN STOP catalog" >> "$L/chain.log"; exit 1; }
[ -n "$RESUME" ] || step session bunx "$T/📜️pdf-browser-direct.ts" session || { echo "[DEBUG] CHAIN STOP session" >> "$L/chain.log"; exit 1; }
TRIES=30 step components bunx "$T/📜️pdf-browser-direct.ts" components || { echo "[DEBUG] CHAIN STOP components" >> "$L/chain.log"; exit 1; }
stamp=$(date +%Y%m%d-%H%M)
for c in starter rotation content style page-history reject-invalid large-vector large-text large-images cancel png image-cancel segmented-source hub-check-in first-archive; do
  TRIES=1 step "case-$c" bunx "$T/📜️pdf-browser-mounted-suite.ts" "e2e-$stamp-$c" "$c"
done
echo "[DEBUG] CHAIN DONE" >> "$L/chain.log"
