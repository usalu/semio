#!/usr/bin/env bash
# 🩹 Restores every pack of <list> (lines "name bytes sha256 state" from list-deleted-packs.py) whose .pack is below <max bytes>
# from the content-addressed blobs of the store into .git/objects/pack: pack, then rev, index last.
set -euo pipefail
list="$1"; max="$2"
store=/Volumes/semio-store/v1/blobs
pack=/Users/ueli/Documents/semio/.git/objects/pack
restore() {
  local name="$1" sha="$2" tmp="$pack/tmp_restore_$$"
  [ -e "$pack/$name" ] && return 0
  cp "$store/${sha:0:2}/$sha" "$tmp"
  [ "$(shasum -a 256 "$tmp" | cut -d' ' -f1)" = "$sha" ] || { rm -f "$tmp"; echo "sha mismatch for $name" >&2; exit 1; }
  chmod 0444 "$tmp"; mv "$tmp" "$pack/$name"
}
awk -v max="$max" '$1 ~ /\.pack$/ && $2 < max {sub(/\.pack$/,"",$1); print $1}' "$list" | while read -r base; do
  for ext in pack rev idx; do
    restore "$base.$ext" "$(awk -v n="$base.$ext" '$1==n {print $3}' "$list")"
  done
  echo "restored $base"
done
