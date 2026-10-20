#!/bin/zsh
# usage: touch-test.sh add|revert   edits the primitives crate's unit-test file (declared by owner-tree globs, never read by rustc for check/build)
cd /Users/ueli/Documents/semio || exit 1
C=$(echo .🧬semio/🦑*/🎫*/🎆*/🌙*10/☀*18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN)
F=$(echo $C/🗑️generated/bzl-rust-ws/geometry/⚙*/🧪*/🔬*unit/🦀*.rs)
SAVE=/private/tmp/bzlrust/orig-unit-test
if [ "$1" = add ]; then [ -f $SAVE ] || cp "$F" $SAVE; printf '\n// [DEBUG] bzl-rust touch probe\n' >> "$F"; else cp $SAVE "$F"; rm -f $SAVE; fi
