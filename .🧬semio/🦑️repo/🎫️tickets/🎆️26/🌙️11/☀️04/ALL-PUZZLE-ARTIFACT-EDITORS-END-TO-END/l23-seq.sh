#!/usr/bin/env bash
# 🧵 L23: sequential native test batches, each in a fresh 5g tmpfs namespace with the crate spill in RAM (/dev/shm); logs to 🗑️generated/l23-<tag>.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"; export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g
batch() { tag=$1 dim=$2; echo quit > "$G/l12-req-$dim"; rm -rf "$L12_SPILL"; echo "L23-BATCH $tag $(date +%H:%M) home=$(df --output=avail -BM /home | tail -1) mem=$(free -m | awk '/Mem:/{print $7}')M"; "$t/l12-run.sh" "$dim" "$t/$3" > "$G/l23-$tag.log" 2>&1; echo "L23-END $tag $(date +%H:%M) $(grep -c '^test result' "$G/l23-$tag.log") results $(grep -c '^test .* FAILED' "$G/l23-$tag.log") failed $(grep -c 'No space left' "$G/l23-$tag.log") enospc"; }
L12_PKGS=semio-os-infinite-board-normal-host L12_CD="/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/📦️packages/🦀️rust" batch lasso 2d l23-a-lasso.txt
L12_PKGS="$(cd "/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" && grep -rhoE '^name = "semio-s-artifact-puzzle-3d[a-z0-9-]*"' --include=Cargo.toml . | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ')" batch 3d 3d l23-b-3d.txt
L12_PKGS=semio-s-artifact-puzzle-5d batch 5d 5d l23-d-5d.txt
L12_PKGS=semio-hub-puzzle batch hub hub l23-e-hub.txt
L12_PKGS="$(grep -o 'semio-[a-z0-9-]*' "$t/l23-c-2d.txt" | tr '\n' ' ')" batch 2d 2d l23-c-2d.txt
rm -rf "$L12_SPILL"; echo L23-ALL-DONE
