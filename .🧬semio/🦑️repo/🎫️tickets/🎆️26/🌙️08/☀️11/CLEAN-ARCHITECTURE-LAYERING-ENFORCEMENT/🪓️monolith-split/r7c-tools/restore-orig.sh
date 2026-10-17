#!/bin/zsh
# usage: restore-orig.sh <orig-dir>  copies every backed up file under <orig-dir> back to the repo (undo of the matching rewriter)
cd "$(dirname "$0")" || exit 1
d="$1"
find "$d" -type f | while IFS= read -r f; do
  rel="${f#$d}"
  cp "$f" "/Users/ueli/Documents/semio$rel"
done
