#!/usr/bin/env bash
# 🧵 L51: every native batch with one build job (L41/L45 lost rustc to SIGKILL under RAM pressure from the in-RAM build tmpfs): 3D one test target per step, 5D, hub, 2D, then the framework store crates; logs 🗑️generated/l51-<tag>.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"
export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g L12_PLAIN=1 RUST_BACKTRACE=1 L12_JOBS=1 RUST_TEST_THREADS=4
export L12_PKGS="$(cd /home/deck/semio && grep -rhoE '^name = "semio-s-[a-z0-9-]*"' --include=Cargo.toml ✏️s | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ') semio-hub-puzzle"
batch() { tag=$1 dim=$2; echo quit > "$G/l12-req-$dim"; rm -rf "$L12_SPILL"; echo "L51-BATCH $tag $(date +%H:%M) home=$(df --output=avail -BM /home | tail -1) mem=$(free -m | awk '/Mem:/{print $7}')M"; "$t/l12-run.sh" "$dim" "$t/$3" > "$G/l51-$tag.log" 2>&1; echo "L51-END $tag $(date +%H:%M) $(grep -c '^test result' "$G/l51-$tag.log") results $(grep -c '^test .* FAILED' "$G/l51-$tag.log") failed $(grep -c 'SIGKILL' "$G/l51-$tag.log") sigkill"; }
batch 3d 3d l50-3d-targets.txt
batch 5d 5d l23-d-5d.txt
batch hub hub l23-e-hub.txt
batch 2d 2d l23-c-2d.txt
batch fw 3d l49-framework.txt
rm -rf "$L12_SPILL"; echo L51-ALL-DONE
