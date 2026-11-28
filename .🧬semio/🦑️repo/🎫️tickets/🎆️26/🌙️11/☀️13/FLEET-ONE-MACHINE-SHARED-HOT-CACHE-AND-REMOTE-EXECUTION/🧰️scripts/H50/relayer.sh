#!/bin/zsh
# re-applies apply-process.py / apply-product.py to a fresh overlay copy of the live files
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H50
H=${0:A:h}
P="🧰️framework/🔨️modules/🏃️process"
Q="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
rm -rf $SC/overlay; cd /Users/ueli/Documents/semio || exit 1
for f in $(cat $H/files-process.txt); do mkdir -p "$SC/overlay/$P/$(dirname "$f")"; cp "$P/$f" "$SC/overlay/$P/$f"; done
python3 $H/apply-process.py $SC/overlay/$P || exit 1
if [ -f $H/apply-product.py ]; then
for f in $(cat $H/files-product.txt); do mkdir -p "$SC/overlay/$Q/$(dirname "$f")"; [ -f "$Q/$f" ] && cp "$Q/$f" "$SC/overlay/$Q/$f"; done
python3 $H/apply-product.py $SC/overlay/$Q || exit 1
fi
