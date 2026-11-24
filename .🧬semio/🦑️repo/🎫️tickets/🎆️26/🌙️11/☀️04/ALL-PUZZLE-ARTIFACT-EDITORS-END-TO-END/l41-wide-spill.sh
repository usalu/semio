#!/usr/bin/env bash
# 🧵 L41: full 3D, 5D, hub and 2D native batches (plain flags, symbols, backtraces) with every ✏️s plugin crate (semio-s-*) spilled to /dev/shm so only framework crates fill the 5g tmpfs; logs 🗑️generated/l41-<tag>.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"; export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g L12_PLAIN=1 RUST_BACKTRACE=1
export L12_PKGS="$(cd /home/deck/semio && grep -rhoE '^name = "semio-s-[a-z0-9-]*"' --include=Cargo.toml ✏️s | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ') semio-hub-puzzle"
batch() { tag=$1 dim=$2; echo quit > "$G/l12-req-$dim"; rm -rf "$L12_SPILL"; echo "L41-BATCH $tag $(date +%H:%M) home=$(df --output=avail -BM /home | tail -1) mem=$(free -m | awk '/Mem:/{print $7}')M"; "$t/l12-run.sh" "$dim" "$t/$3" > "$G/l41-$tag.log" 2>&1; echo "L41-END $tag $(date +%H:%M) $(grep -c '^test result' "$G/l41-$tag.log") results $(grep -c '^test .* FAILED' "$G/l41-$tag.log") failed $(grep -c 'No space left' "$G/l41-$tag.log") enospc shm=$(du -sh "$L12_SPILL" | cut -f1)"; }
batch 3d 3d l23-b-3d.txt
batch 5d 5d l23-d-5d.txt
batch hub hub l23-e-hub.txt
batch 2d 2d l23-c-2d.txt
rm -rf "$L12_SPILL"; echo L41-ALL-DONE
