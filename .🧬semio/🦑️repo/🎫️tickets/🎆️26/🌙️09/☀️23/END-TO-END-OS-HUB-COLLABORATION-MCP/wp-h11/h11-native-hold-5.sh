#!/bin/zsh
# 🔐️ H11: one native-mutex hold — the full `semio-hub --all-features` suite (lib + bin, every level) on the H11 target.
# usage: h11-native-hold-5.sh <label>
cd /Users/ueli/Documents/semio
export H11_NICE=0
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-h11/h11-cargo.sh "$1-hub-all-features" test -p semio-hub --all-features --no-fail-fast
