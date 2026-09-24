#!/bin/zsh
# 🔁️ W1: catalog A (C8 live proof) then the final all-component pass.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
W1_PACKAGES=stdio,gis,note,draw,writer,puzzle W1_DATA_NAME=w1-catalog-a zsh w1-catalog.sh publish
zsh w1-final-pass.sh
