#!/bin/zsh
# 🗺️ W1 for C8: activate gis2d once the post-06:31 gis stage has landed.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
until [ -f generated/stage-gis-plugin.ok ] || grep -q 'END gis-plugin rc=[1-9]' generated/c8-restage.txt; do sleep 15; done
[ -f generated/stage-gis-plugin.ok ] && zsh w1-activate.sh gis2d
