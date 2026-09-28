#!/bin/zsh
# 🧪️ WG11 s14b: native hold 4 (hub check + echo-pin law + kernel lost-Ack laws) then hold 5 (current-tree renderer test binary).
# usage: setopt no_bg_nice; nohup zsh wg11-native-45.sh > <capture> 2>&1 & disown
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg11
zsh $W/wg11-native-4.sh
zsh $W/wg11-native-5.sh
