#!/bin/sh
# 🪚️ Replays one captured rustc command line of a unit with extra flags, into a scratch out dir, and prints its peak resident memory and wall time (macOS `time -l`); run it only under `repo capacity run --memory=<MB> -- sh unit-variant.sh ...`, the admitted entry for arbitrary heavy work.
# Usage: unit-variant.sh <captured-args-file> <crate> <version> <label> [extra rustc flags...]
set -eu
args=$1; crate=$2; version=$3; label=$4; shift 4
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
scratch="$here/🗑️generated/variant-$label"
rm -rf "$scratch"; mkdir -p "$scratch"
cd "$root/🌎️hub"
rustc_bin=$(ls -d "$root"/.cache/b/t/m/r/*/toolchains/*/bin/rustc | head -1)
CARGO_MANIFEST_DIR="$root/🌎️hub"; CARGO_PKG_NAME=$crate; CARGO_PKG_VERSION=$version; CARGO_CRATE_NAME=$(echo "$crate" | tr - _)
export CARGO_MANIFEST_DIR CARGO_PKG_NAME CARGO_PKG_VERSION CARGO_CRATE_NAME
set -f
set -- $(sed "s/cfg(feature, values())/cfg(feature,values())/" "$args") "$@"
shift
final=""
while [ $# -gt 0 ]; do
  case "$1" in
    --out-dir) shift; final="$final --out-dir $scratch" ;;
    -Z) final="$final -Z"; ;;
    *) final="$final $1" ;;
  esac
  shift
done
start=$(date +%s)
/usr/bin/time -l "$rustc_bin" $final -Z temps-dir="$scratch" > "$scratch/out.txt" 2> "$scratch/err.txt" || echo "rustc failed" >> "$scratch/err.txt"
end=$(date +%s)
echo "$label wall=$((end-start))s $(grep 'maximum resident' "$scratch/err.txt" | awk '{print $1/1048576" MB"}') $(grep -c 'rustc failed' "$scratch/err.txt") failed"
