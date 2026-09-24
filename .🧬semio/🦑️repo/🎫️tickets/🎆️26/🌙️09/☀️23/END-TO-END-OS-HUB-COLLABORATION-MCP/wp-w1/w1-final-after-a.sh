#!/bin/zsh
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
until grep -q 'END publish' generated/catalog-a.txt 2>/dev/null; do sleep 30; done
zsh w1-final-pass.sh
