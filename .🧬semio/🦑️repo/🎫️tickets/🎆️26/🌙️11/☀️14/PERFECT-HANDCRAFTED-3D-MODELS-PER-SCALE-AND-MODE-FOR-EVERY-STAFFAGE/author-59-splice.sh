#!/bin/sh
cd "$(dirname "$0")"
slug=$1
F=$(ls /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/$slug/🔣️.json)
python3 volumes-insert.py volumes/animals/$slug.json "$F"
