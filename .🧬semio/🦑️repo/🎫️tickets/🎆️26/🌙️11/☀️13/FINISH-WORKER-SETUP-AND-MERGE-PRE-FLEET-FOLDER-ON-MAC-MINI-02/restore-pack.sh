#!/usr/bin/env bash
# 🩹 Restores pack-58516ce… (published by g1-m04 at 01:17, deleted by the repack of g1-server at 02:03:59)
# from the content-addressed blobs of the store into .git/objects/pack: pack first, index last.
set -euo pipefail
store=/Volumes/semio-store/v1/blobs
pack=/Users/ueli/Documents/semio/.git/objects/pack
name=pack-58516ce43a03c1f7437c3ade3a5b4a511a2993d1
restore() {
  local sha="$1" ext="$2" tmp="$pack/tmp_restore_$2"
  cp "$store/${sha:0:2}/$sha" "$tmp"
  [ "$(shasum -a 256 "$tmp" | cut -d' ' -f1)" = "$sha" ] || { rm -f "$tmp"; echo "sha mismatch for $ext" >&2; exit 1; }
  chmod 0444 "$tmp"
  mv "$tmp" "$pack/$name.$ext"
}
restore 4691c3379241b11449ad20526cf3dbc503d0082f72127d8ccf8643eb1e6eead8 pack
restore "$(ls "$store/0f" | grep '^0f188786e513')" rev
restore "$(ls "$store/38" | grep '^38a56de3f8c9')" idx
git -C /Users/ueli/Documents/semio log --oneline -1
