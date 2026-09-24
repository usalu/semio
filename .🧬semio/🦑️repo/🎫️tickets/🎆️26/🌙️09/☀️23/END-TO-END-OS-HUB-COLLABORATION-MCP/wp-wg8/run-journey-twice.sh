#!/bin/zsh
# WG8: two back-to-back native journey runs (outlier hunt), captures journey-{22,23}.raw.txt.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-wg8
zsh run-native-journey.sh > generated/journey-22.raw.txt 2>&1
zsh run-native-journey.sh > generated/journey-23.raw.txt 2>&1
echo done > generated/journey-twice.done
