#!/bin/zsh
# usage: vt.sh <tag> <pkgdir> <config-relative-to-pkgdir> [vitest args...]   direct vitest probe (NOT the repo CLI), log C/🗑️generated/ts-c/vt-<tag>.log + .rc
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; dir="$2"; cfg="$3"; shift 3
R=/Users/ueli/Documents/semio
cd "$dir" || exit 2
NODE_OPTIONS=--max-old-space-size=6144 nice -n 10 node "$R/node_modules/vitest/vitest.mjs" run --config "$cfg" --configLoader native --testTimeout 120000 --hookTimeout 120000 "$@" > "$R/$C/🗑️generated/ts-c/vt-$tag.log" 2>&1
echo $? > "$R/$C/🗑️generated/ts-c/vt-$tag.rc"
