#!/bin/zsh
# 🧪️ Packet A8 verification — every migrated crate, one at a time so the shared fine-grain-locked build
# dir is queued for once per crate instead of fought over. Never sets CARGO_TARGET_DIR.
#
# Three steps per crate, output under 🗑️generated/a8/:
#   1. `<crate>.laws.txt`  — the window-law tests this packet owns, single-threaded. This compiles the whole
#      lib test target, so it is also the real type-check gate for the crate's tests.
#   2. `<crate>.test.txt`  — the crate's FULL suite, under a hard cap. Peers are mid-refactor in several of
#      these crates and some of their mounted-app tests currently hang; macOS has no `timeout(1)`, so the
#      cap is a background cargo plus a watchdog that kills the run and records `TIMEOUT`.
#   3. `<crate>.wasm.txt`  — `cargo check --target wasm32-wasip2`, the guest-target gate the brief requires.
set -u
cd /Users/ueli/Documents/semio || exit 1
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
# 🧠️ The build host runs a full peer cargo fleet; an unthrottled `cargo test` here gets SIGKILLed (exit 137)
# mid-compile under memory pressure. Two jobs and no incremental cache keep each crate inside the budget.
export CARGO_INCREMENTAL=0
JOBS=(-j 2)
CAP=900
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING/🗑️generated/a8"
mkdir -p "$G"

# 🧩️ `<crate>|<features or ->|<law test filter>`
CRATES=(
  "semio-s-artifact-gis-gismap|component-app-assembly|panels::"
  "semio-s-artifact-layout-layout|-|panels::"
  "semio-s-artifact-forms-forms|-|panels::"
  "semio-s-artifact-trinity-jack|component-app-assembly|panels::"
  "semio-s-artifact-trinity-rewriting|component-app-assembly|panels::"
  "semio-s-artifact-reasoning-wires|-|panels::"
  "semio-s-artifact-architect-program|-|panels::"
  "semio-s-artifact-lowpoly-lowpoly|-|panels::"
  "semio-s-plugin-space|component-app-assembly|panels::"
  "semio-s-artifact-space-space|component-app-assembly|panels::"
  "semio-s-artifact-sourcing-curation|-|panels::"
  "semio-s-artifact-stdio-zip|-|main::tests"
  "semio-s-artifact-stdio-xml|-|main::tests"
  "semio-s-artifact-stdio-json|-|main::tests"
  "semio-s-artifact-playbook-playbook|-|steps::tests"
  "semio-s-artifact-draw-drawing|-|panels::"
  "semio-s-artifact-raster-raster|-|panels::"
  "semio-s-artifact-shooting-shooting|-|panels::"
  "semio-framework-plugin|-|app_panel_kit"
)

# ⏱️ Runs one cargo invocation with a hard wall-clock cap, recording the verdict in progress.txt.
capped() {
  local label="$1" crate="$2" out="$3"
  shift 3
  echo "[A8] $crate $label start $(date -u +%H:%M:%S)" >> "$G/progress.txt"
  "$@" > "$out" 2>&1 &
  local pid=$!
  local waited=0
  while kill -0 "$pid" 2>/dev/null; do
    if [ "$waited" -ge "$CAP" ]; then
      pkill -P "$pid" 2>/dev/null
      kill -9 "$pid" 2>/dev/null
      echo "[A8] $crate $label TIMEOUT after ${CAP}s $(date -u +%H:%M:%S)" >> "$G/progress.txt"
      return 124
    fi
    sleep 5
    waited=$((waited + 5))
  done
  wait "$pid"
  echo "[A8] $crate $label exit=$? $(date -u +%H:%M:%S)" >> "$G/progress.txt"
}

for entry in "${CRATES[@]}"; do
  crate="${entry%%|*}"
  rest="${entry#*|}"
  feats="${rest%%|*}"
  filter="${rest##*|}"
  if [ "$feats" = "-" ]; then
    fargs=()
  else
    fargs=(--features "$feats")
  fi
  capped laws "$crate" "$G/$crate.laws.txt" cargo test "${JOBS[@]}" -p "$crate" "${fargs[@]}" --lib -- --test-threads=1 "$filter"
  capped test "$crate" "$G/$crate.test.txt" cargo test "${JOBS[@]}" -p "$crate" "${fargs[@]}"
  capped wasm "$crate" "$G/$crate.wasm.txt" cargo check "${JOBS[@]}" -p "$crate" --target wasm32-wasip2 "${fargs[@]}"
done
echo "[A8] ALL DONE $(date -u +%H:%M:%S)" >> "$G/progress.txt"
