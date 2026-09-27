#!/bin/zsh
# 🧾️ T13: repository contract phase (no cargo), capture for the non-norm HIGH census.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
nice -n 10 bun ./📜️script.ts contract; echo "EXIT $? $(date '+%T')"
cp "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json" "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-t13-logs/breaches-$(date '+%H%M').json"
echo ALL_DONE
