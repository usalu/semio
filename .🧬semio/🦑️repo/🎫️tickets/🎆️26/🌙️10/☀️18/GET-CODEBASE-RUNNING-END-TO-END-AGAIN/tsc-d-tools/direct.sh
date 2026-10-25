#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript || exit 2
exec nice -n 10 /Users/ueli/.local/bin/repo ./📜️script.ts "$@"
