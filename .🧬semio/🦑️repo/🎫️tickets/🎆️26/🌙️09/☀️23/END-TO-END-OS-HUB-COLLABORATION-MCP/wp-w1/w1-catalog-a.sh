#!/bin/zsh
# 🤝️ W1: catalog A (C8 live proof). Warm each package's wasm-release unit in its own short hold first (a sweep kill
# then loses one package, not the whole publish), then publish stdio,gis,note,draw,writer,puzzle into w1-catalog-a.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
for attempt in 1 2 3; do
  rm -f generated/release-{stdio,gis,note,draw,writer,puzzle}.ok
  W1_WARM_ONLY=stdio,gis,note,draw,writer,puzzle zsh w1-catalog.sh warm
  W1_PACKAGES=stdio,gis,note,draw,writer,puzzle W1_DATA_NAME=w1-catalog-a zsh w1-catalog.sh publish && break
  cp generated/publish-w1-catalog-a.txt "generated/publish-w1-catalog-a-retry$attempt.txt"
done
