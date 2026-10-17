#!/bin/zsh
# usage: seq-oracle.sh <base|cur> <native|test>  runs the plain full-resolution oracle on the sequence plugin (base = pre-r7c tree rebuilt from orig-seq + orig-seq-subset-root.rs)
T="$(cd "$(dirname "$0")" && pwd)"
cd /Users/ueli/Documents/semio || exit 99
P="✏️s/🔌️plugins/🎬️sequence"
mode=$1; cfg=$2
flag=""; [ "$cfg" = test ] && flag="--test"
export SRC_DIR=""
if [ "$mode" = base ]; then
  B=/Users/ueli/.cache/r7c/seq-base-src
  if [ ! -d "$B" ]; then
    mkdir -p "$B"; cp -R "$P/" "$B/"
    (cd "$T/orig-seq" && find . -type f -path "./$P/*" | while IFS= read -r f; do cp "$f" "$B/${f#./$P/}"; done)
    cp "$T/orig-seq-subset-root.rs" "$B/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"
  fi
  export SRC_DIR="$B"
fi
nice -n 10 bun "$T/../r7-tools/orig-oracle.ts" "$P" "$P/🗿️artifacts/🎬️sequence/🦀️.rs" semio-s-artifact-sequence-sequence "/Users/ueli/.cache/r7c/seq-$mode-$cfg" $flag
