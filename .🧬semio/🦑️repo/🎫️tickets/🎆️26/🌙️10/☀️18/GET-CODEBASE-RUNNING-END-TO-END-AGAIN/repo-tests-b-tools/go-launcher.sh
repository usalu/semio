#!/bin/sh
cd /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go || exit 1
nice -n 10 repo tools exec --light go -- test -count=1 ./...
echo "rc-pkg=$?"
