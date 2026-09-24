#!/bin/zsh
# 🗺️ W1 for C8/T3: gis describe → materialize-dev, then activate gis2d and s over the staged modules.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
rm -f generated/stage-gis-plugin.ok
zsh w1-stage-all.sh @semio-tech/gis-plugin
[ -f generated/stage-gis-plugin.ok ] && { zsh w1-activate.sh gis2d; zsh w1-activate.sh s; }
