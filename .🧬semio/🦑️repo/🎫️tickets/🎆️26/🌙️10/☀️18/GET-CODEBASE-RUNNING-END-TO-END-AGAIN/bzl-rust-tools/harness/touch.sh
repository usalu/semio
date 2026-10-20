#!/bin/zsh
# usage: touch.sh add|revert <crate-prefix: ⚙|➗|🌙>   edits the owner 🦀️.rs of a scratch crate (comment line) and restores exact bytes
cd /Users/ueli/Documents/semio || exit 1
C=$(echo .🧬semio/🦑*/🎫*/🎆*/🌙*10/☀*18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN)
F=$(echo $C/🗑️generated/bzl-rust-ws/geometry/$2*/🦀*.rs)
SAVE=/private/tmp/bzlrust/orig-$(echo $2 | python3 -c 'import sys;print(ord(sys.stdin.read()[0]))')
if [ "$1" = add ]; then [ -f $SAVE ] || cp "$F" $SAVE; printf '\n// [DEBUG] bzl-rust touch probe\n' >> "$F"; else cp $SAVE "$F"; rm -f $SAVE; fi
