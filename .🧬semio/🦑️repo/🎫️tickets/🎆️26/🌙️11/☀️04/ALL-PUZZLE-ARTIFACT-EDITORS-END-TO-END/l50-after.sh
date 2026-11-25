#!/usr/bin/env bash
# 🧵 L50: after L41 ends, rerun 3D one test target per step with one build job (L41's single 3D step lost every result to an OOM-killed rustc), then the framework store crates (l49); logs 🗑️generated/l50-<tag>.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"
while pgrep -f l41-wide-spill.sh >/dev/null; do sleep 30; done
export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g L12_PLAIN=1 RUST_BACKTRACE=1 L12_JOBS=1 RUST_TEST_THREADS=4
export L12_PKGS="$(cd /home/deck/semio && grep -rhoE '^name = "semio-s-[a-z0-9-]*"' --include=Cargo.toml ✏️s | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ') semio-hub-puzzle"
batch() { tag=$1 dim=$2; echo quit > "$G/l12-req-$dim"; rm -rf "$L12_SPILL"; echo "L41-BATCH $tag $(date +%H:%M) home=$(df --output=avail -BM /home | tail -1) mem=$(free -m | awk '/Mem:/{print $7}')M"; "$t/l12-run.sh" "$dim" "$t/$3" > "$G/l50-$tag.log" 2>&1; echo "L41-END $tag $(date +%H:%M) $(grep -c '^test result' "$G/l50-$tag.log") results $(grep -c '^test .* FAILED' "$G/l50-$tag.log") failed $(grep -c 'SIGKILL' "$G/l50-$tag.log") sigkill"; }
batch 3d 3d l50-3d-targets.txt
batch fw 3d l49-framework.txt
rm -rf "$L12_SPILL"; echo L50-ALL-DONE
