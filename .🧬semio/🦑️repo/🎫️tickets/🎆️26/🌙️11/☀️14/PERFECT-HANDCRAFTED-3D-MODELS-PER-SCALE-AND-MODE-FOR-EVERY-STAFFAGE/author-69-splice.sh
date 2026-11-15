#!/bin/bash
# usage: author-69-splice.sh slug
cd "$(dirname "$0")"
python3 volumes-insert.py "volumes/humans/$1.json" "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/$1/🔣️.json"
