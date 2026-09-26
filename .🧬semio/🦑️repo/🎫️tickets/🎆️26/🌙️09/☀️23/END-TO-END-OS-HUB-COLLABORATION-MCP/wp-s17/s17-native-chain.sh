#!/bin/zsh
# 🧪️ S17 native evidence per extension: `cargo test -p <crate> --lib` then the package's `bun ./📜️script.ts test quick`,
# one crate at a time, niced, private target dir, rustc gate (< 10 running, preamble rule 22). Captures per extension under OUT; summary in
# OUT/progress.txt. usage: zsh s17-native-chain.sh <out-dir> [extension-dir …]  (default: every extension package)
setopt null_glob
cd /Users/ueli/Documents/semio || exit 1
OUT="$1"; shift
mkdir -p "$OUT"
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s17/target
if [ $# -gt 0 ]; then dirs=("$@"); else dirs=(✏️s/🔌️plugins/*/🧩️extensions/*/📦️packages/🦀️rust); fi
for dir in "${dirs[@]}"; do
  name=$(print -r -- "$dir" | sed -e 's#✏️s/🔌️plugins/##' -e 's#/📦️packages/🦀️rust##' -e 's#/🧩️extensions/#+#')
  crate=$(/usr/bin/grep -m1 '^name' "$dir/Cargo.toml" | sed -e 's/.*"\(.*\)".*/\1/')
  until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -lt 10 ]; do sleep 30; done
  echo "START $name $crate $(date '+%T')" >> "$OUT/progress.txt"
  start=$(date +%s)
  nice -n 10 cargo test -p "$crate" --lib --no-fail-fast > "$OUT/$name.lib.txt" 2>&1
  lib=$?
  mid=$(date +%s)
  ( cd "$dir" && nice -n 10 bun ./📜️script.ts test quick ) > "$OUT/$name.quick.txt" 2>&1
  quick=$?
  end=$(date +%s)
  libsum=$(/usr/bin/grep -E '^test result:' "$OUT/$name.lib.txt" | tail -1)
  quicksum=$(/usr/bin/grep -E 'Summary \[|^test result:' "$OUT/$name.quick.txt" | tail -1)
  echo "END $name lib=$lib $(( mid - start ))s [$libsum] quick=$quick $(( end - mid ))s [$quicksum] $(date '+%T')" >> "$OUT/progress.txt"
done
echo "ALL_DONE $(date '+%T')" >> "$OUT/progress.txt"
