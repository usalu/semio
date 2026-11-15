#!/bin/zsh
cd "$(dirname "$0")"
for a in "$@"; do
  python3 volumes-insert.py volumes/vegetation/$a.json "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$a/🔣️.json" 2>&1 | tail -2
done
