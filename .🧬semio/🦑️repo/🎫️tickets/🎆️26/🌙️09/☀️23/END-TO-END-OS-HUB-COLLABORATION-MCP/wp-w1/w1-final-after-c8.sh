#!/bin/zsh
# ⏭️ W1: start the final pass once C8's catalog A publish has ended (keeps C8's live-proof work first in the FIFO).
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
until grep -q 'END publish' generated/c8-restage.txt 2>/dev/null; do sleep 30; done
zsh w1-final-pass.sh
