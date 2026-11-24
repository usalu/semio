#!/usr/bin/env bash
# 🧵 L34: 3D close-law backtrace run, then 5D, hub and 2D native batches with the lower cache's own flags (L12_PLAIN=1), so unchanged crates load from the lower instead of rebuilding in RAM; waits for l32-verify.sh; logs to 🗑️generated/l34-<tag>.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"; export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g L12_PLAIN=1 RUST_BACKTRACE=1
while pgrep -f l32-verify.sh >/dev/null; do sleep 30; done
batch() { tag=$1 dim=$2; echo quit > "$G/l12-req-$dim"; rm -rf "$L12_SPILL"; echo "L34-BATCH $tag $(date +%H:%M) home=$(df --output=avail -BM /home | tail -1) mem=$(free -m | awk '/Mem:/{print $7}')M"; "$t/l12-run.sh" "$dim" "$t/$3" > "$G/l34-$tag.log" 2>&1; echo "L34-END $tag $(date +%H:%M) $(grep -c '^test result' "$G/l34-$tag.log") results $(grep -c '^test .* FAILED' "$G/l34-$tag.log") failed $(grep -c 'No space left' "$G/l34-$tag.log") enospc"; }
L12_PKGS="$(cd "/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" && grep -rhoE '^name = "semio-s-artifact-puzzle-3d[a-z0-9-]*"' --include=Cargo.toml . | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ')" batch 3d-close 3d l34-3d-close.txt
L12_PKGS=semio-s-artifact-puzzle-5d batch 5d 5d l23-d-5d.txt
L12_PKGS=semio-hub-puzzle batch hub hub l23-e-hub.txt
L12_PKGS="$(grep -o 'semio-[a-z0-9-]*' "$t/l23-c-2d.txt" | tr '\n' ' ')" batch 2d 2d l23-c-2d.txt
rm -rf "$L12_SPILL"; echo L34-ALL-DONE
