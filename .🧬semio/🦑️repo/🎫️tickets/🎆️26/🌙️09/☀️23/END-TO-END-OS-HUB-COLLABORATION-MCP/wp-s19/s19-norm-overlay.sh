#!/bin/zsh
# 🧪️ S19 norm-examples overlay proof: norm plugin lib (surface laws incl. the roster law) + compliance gate, then
# `--lib --tests` of the six touched family crates + the contract. usage: zsh s19-norm-overlay.sh <capture-prefix>
R=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19/s19-overlay-cargo.sh
zsh $R "$1-test.txt" test -p semio-s-plugin-norm --lib --test compliance_gate --no-fail-fast
zsh $R "$1-check.txt" check -p semio-s-artifact-norm-contract -p semio-s-artifact-norm-en1992 -p semio-s-artifact-norm-en1994 -p semio-s-artifact-norm-en1997 -p semio-s-artifact-norm-en1998 -p semio-s-artifact-norm-iso16757 -p semio-s-artifact-norm-din4108 --lib --tests --keep-going --message-format short
echo "ALL_DONE $(date '+%T')" >> "$1-check.txt"
zsh $R "$1-emitter.txt" build -p n1-norm-fixture-emitter
