#!/bin/zsh
# R8 session 12: native `cargo check` of every workspace member (explicit -p per member), all targets, keep going.
root=/Users/ueli/Documents/semio
out="$root/.🧬semio/🌐hub/s12-r8-captures"
run=${1:-1}
cd "$root" || exit 2
typeset -a pkgs
pkgs=($(cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; m=json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in m["packages"])))'))
args=()
for p in $pkgs; do args+=(-p "$p"); done
echo "members=${#pkgs} start=$(date '+%F %T')" > "$out/ws-check-$run.txt"
start=$(date +%s)
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$root/.tmp-ticket/wp-r8/target" nice -n 15 cargo check --keep-going --all-targets --message-format=short "${args[@]}" >> "$out/ws-check-$run.txt" 2>&1
echo "EXIT=$? secs=$(( $(date +%s) - start )) end=$(date '+%F %T')" >> "$out/ws-check-$run.txt"
