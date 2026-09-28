#!/bin/zsh
# 🎚️ H13 hold 11 (session 14c): the full `os-hub:test-all-features` suite on the final kernel-db (SQLite readers, retirement
# wake, close-ring cleanup-fault routing).
# usage: h13-hold-11.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-hub-all-features" test -p semio-hub --all-features --no-fail-fast
