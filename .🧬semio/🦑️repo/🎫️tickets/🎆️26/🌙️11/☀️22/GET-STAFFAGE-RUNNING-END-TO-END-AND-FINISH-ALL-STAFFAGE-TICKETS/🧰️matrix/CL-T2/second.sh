#!/bin/zsh
# 🥈️ Trial copies, then light proof, write and proof of the assets given, inside one grant: second.sh <id>…
source "${0:A:h}/env.sh"
zsh "$W/try.sh" "$@" | grep -v "^   "
zsh "$W/chain.sh" "$@"
