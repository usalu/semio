#!/bin/zsh
# usage: plug-oracle.sh <plugin dir> <artifact rel dir> <crate> <tag> <orig-dirs,comma> <base|cur> <native|test>   full-resolution oracle on a plugin copy; base overlays the originals of the given orig-snapshot dirs (repo-relative files under them)
T="$(cd "$(dirname "$0")" && pwd)"
cd /Users/ueli/Documents/semio || exit 99
P=$1; A=$2; crate=$3; tag=$4; origs=$5; mode=$6; cfg=$7
flag=""; [ "$cfg" = test ] && flag="--test"
export SRC_DIR=""
if [ "$mode" = base ]; then
  B=/Users/ueli/.cache/r7c/$tag-base-src
  if [ ! -d "$B" ]; then
    mkdir -p "$B"; cp -R "$P/" "$B/"
    for o in ${(s:,:)origs}; do (cd "$T/$o" && find . -type f -path "./$P/*" | while IFS= read -r f; do mkdir -p "$B/$(dirname "${f#./$P/}")"; cp "$f" "$B/${f#./$P/}"; done); done
    # files created by the carve work are absent from the originals; remove them from the base copy
    for x in ${(s:,:)6:-}; do :; done
  fi
  export SRC_DIR="$B"
fi
nice -n 10 bun "$T/../r7-tools/orig-oracle.ts" "$P" "$P/$A/🦀️.rs" "$crate" "/Users/ueli/.cache/r7c/$tag-$mode-$cfg" $flag
