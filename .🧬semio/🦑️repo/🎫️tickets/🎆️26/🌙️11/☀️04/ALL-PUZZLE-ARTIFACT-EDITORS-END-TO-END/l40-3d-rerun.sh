#!/usr/bin/env bash
# 🧵 L40: full 3D rerun with plain flags, symbols and backtraces after l34-plain.sh ends (its 3D step died on a foreign token-generation gap); log 🗑️generated/l40-3d.log
t=$(cd "$(dirname "$0")" && pwd); G="$t/🗑️generated"; export L12_SPILL=/dev/shm/l23-spill L12_SIZE=5g L12_PLAIN=1 RUST_BACKTRACE=1
while pgrep -f l34-plain.sh >/dev/null; do sleep 30; done
echo quit > "$G/l12-req-3d"; rm -rf "$L12_SPILL"; echo "L40-BATCH 3d $(date +%H:%M)"
L12_PKGS="$(cd "/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" && grep -rhoE '^name = "semio-s-artifact-puzzle-3d[a-z0-9-]*"' --include=Cargo.toml . | sed 's/name = //; s/"//g' | sort -u | tr '\n' ' ')" "$t/l12-run.sh" 3d "$t/l23-b-3d.txt" > "$G/l40-3d.log" 2>&1
echo "L40-END 3d $(date +%H:%M) $(grep -c '^test result' "$G/l40-3d.log") results $(grep -c '^test .* FAILED' "$G/l40-3d.log") failed"; rm -rf "$L12_SPILL"
