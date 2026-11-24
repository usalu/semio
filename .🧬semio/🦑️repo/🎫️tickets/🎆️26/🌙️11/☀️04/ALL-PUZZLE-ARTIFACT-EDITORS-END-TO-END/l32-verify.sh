#!/usr/bin/env bash
# 🧵 L32: 3D and board-host verification batches, each in a fresh 5g tmpfs namespace with the crate spill in RAM (/dev/shm); logs to 🗑️generated/l32-<tag>.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"; export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g
batch() { tag=$1 dim=$2; echo quit > "$G/l12-req-$dim"; rm -rf "$L12_SPILL"; echo "L32-BATCH $tag $(date +%H:%M) home=$(df --output=avail -BM /home | tail -1) mem=$(free -m | awk '/Mem:/{print $7}')M"; "$t/l12-run.sh" "$dim" "$t/$3" > "$G/l32-$tag.log" 2>&1; echo "L32-END $tag $(date +%H:%M) $(grep -c '^test result' "$G/l32-$tag.log") results $(grep -c '^test .* FAILED' "$G/l32-$tag.log") failed $(grep -c 'No space left' "$G/l32-$tag.log") enospc"; }
L12_PKGS="$(cd "/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" && grep -rhoE '^name = "semio-s-artifact-puzzle-3d[a-z0-9-]*"' --include=Cargo.toml . | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ')" batch 3d 3d l23-b-3d.txt
L12_PKGS=semio-os-infinite-board-normal-host L12_CD="/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/📦️packages/🦀️rust" batch lasso 2d l23-a-lasso.txt
rm -rf "$L12_SPILL"; echo L32-ALL-DONE
