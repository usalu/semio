#!/bin/zsh
# 🤝️ W1 for C8: gis+draw restage + activate, then catalog A, on the post-06:31 tree.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
zsh w1-c8-restage.sh @semio-tech/gis-plugin @semio-tech/draw-plugin
W1_PACKAGES=stdio,gis,note,draw,writer,puzzle W1_DATA_NAME=w1-catalog-a zsh w1-catalog.sh publish
