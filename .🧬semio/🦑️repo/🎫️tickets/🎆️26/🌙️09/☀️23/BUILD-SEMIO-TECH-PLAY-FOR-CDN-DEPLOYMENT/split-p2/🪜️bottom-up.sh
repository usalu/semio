#!/bin/bash
# 🪜️ Checks lane-P2 family crates one at a time, bottom-up, through the issued launcher (`🏗️cargo.sh`), and prints one line per
# crate: rc, error count, and the first refusal or compile error (a Source-boundary refusal names the over-ceiling crate).
# usage: 🪜️bottom-up.sh <tag> <cargo check flags...> -- <crate suffix>...
TAG=$1; shift; FLAGS=(); while [ "$1" != "--" ]; do FLAGS+=("$1"); shift; done; shift
HERE="$(cd "$(dirname "$0")" && pwd)"; M="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml"
for c in "$@"; do
  p="semio-s-artifact-remodel-remodeling${c:+-$c}"; [ "$c" = "agg" ] && p="semio-s-artifact-remodel-remodeling"
  r=$(bash "$HERE/🏗️cargo.sh" "$TAG-$c" check --manifest-path "$M" -p "$p" "${FLAGS[@]}" --message-format short)
  first=$(grep -m1 -E 'refused before compiler|: error|^error' "$HERE/🗑️generated/logs/$TAG-$c.log" | cut -c1-220)
  echo "$c | $r | $first"
done
