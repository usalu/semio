#!/bin/zsh
# 🤝️ W1 for C7: warm + os-hub + publish stdio,gis,writer,draw,puzzle into w1-catalog-c7, ahead of the full catalog.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
W1_WARM_ONLY=stdio,gis,writer,draw,puzzle zsh w1-catalog.sh warm
zsh w1-catalog.sh hub
W1_PACKAGES=stdio,gis,writer,draw,puzzle W1_DATA_NAME=w1-catalog-c7 zsh w1-catalog.sh publish
