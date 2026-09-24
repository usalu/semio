#!/bin/zsh
# 🤝️ W1 for C7, run 3: the packages whose every editor kind answers `pack-schema-hash` (writer/puzzle are blocked, see report §4.2).
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
W1_PACKAGES=stdio,gis,note,draw W1_DATA_NAME=w1-catalog-c7 zsh w1-catalog.sh publish
