#!/bin/zsh
# 🧪️ Packet A8a verification — forms, layout, reasoning(wires), shooting.
# Per crate: window-law tests by filter, then the full suite, then the wasm32-wasip2 guest check.
# Never sets CARGO_TARGET_DIR. Foreground of its caller; logs under 🗑️generated/a8a/.
set -u
cd /Users/ueli/Documents/semio || exit 1
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
export CARGO_INCREMENTAL=0
export CARGO_PROFILE_WASM_DEV_DEBUG=false
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING/🗑️generated/a8a"
mkdir -p "$G"
STEPS=${1:-laws,test,wasm}
CRATES=(semio-s-artifact-forms-forms semio-s-artifact-layout-layout semio-s-artifact-reasoning-wires semio-s-artifact-shooting-shooting)
SHORT=(forms layout wires shooting)
for index in {1..${#CRATES[@]}}; do
  crate="${CRATES[$index]}"
  short="${SHORT[$index]}"
  case "$STEPS" in *laws*)
    echo "[a8a] $short laws $(date -u +%H:%M:%S)" >> "$G/progress.txt"
    cargo test -j 2 -p "$crate" --lib -- --test-threads=1 panels:: > "$G/$short.laws.txt" 2>&1
    echo "[a8a] $short laws exit=$? $(date -u +%H:%M:%S)" >> "$G/progress.txt" ;;
  esac
  case "$STEPS" in *test*)
    echo "[a8a] $short test $(date -u +%H:%M:%S)" >> "$G/progress.txt"
    cargo test -j 2 -p "$crate" > "$G/$short.test.txt" 2>&1
    echo "[a8a] $short test exit=$? $(date -u +%H:%M:%S)" >> "$G/progress.txt" ;;
  esac
  case "$STEPS" in *wasm*)
    echo "[a8a] $short wasm $(date -u +%H:%M:%S)" >> "$G/progress.txt"
    cargo check -j 2 -p "$crate" --target wasm32-wasip2 > "$G/$short.wasm.txt" 2>&1
    echo "[a8a] $short wasm exit=$? $(date -u +%H:%M:%S)" >> "$G/progress.txt" ;;
  esac
done
echo "[a8a] ALL DONE $(date -u +%H:%M:%S)" >> "$G/progress.txt"
