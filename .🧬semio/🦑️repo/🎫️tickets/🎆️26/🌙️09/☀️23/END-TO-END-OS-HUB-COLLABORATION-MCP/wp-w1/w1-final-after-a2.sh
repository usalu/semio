#!/bin/zsh
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
while pgrep -f 'zsh w1-catalog-a.sh' >/dev/null; do sleep 30; done
zsh w1-final-pass.sh 2
