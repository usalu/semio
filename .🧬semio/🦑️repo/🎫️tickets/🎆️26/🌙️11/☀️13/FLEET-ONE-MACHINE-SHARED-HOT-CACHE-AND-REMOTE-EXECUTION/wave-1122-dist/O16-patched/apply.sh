#!/bin/sh
# usage: apply.sh [play-root]   (default /Users/ueli/.sinc/play). Requester side only; copy via temp name + mv, never in place.
set -e
root=${1:-/Users/ueli/.sinc/play}
f="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/📜️action/🟦️.ts"
here=$(cd "$(dirname "$0")" && pwd)
cp "$f" "$f.o16" && patch -s "$f.o16" < "$here/action.patch" && mv "$f.o16" "$f"
