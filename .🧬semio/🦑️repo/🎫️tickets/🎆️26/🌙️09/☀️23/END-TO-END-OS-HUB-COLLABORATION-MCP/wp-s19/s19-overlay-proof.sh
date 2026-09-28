#!/bin/zsh
# 🧪️ S19 guest overlay proofs for the prepared sets, one lane hold per step: `norm-examples` (norm plugin lib incl. the
# roster laws + compliance gate) and `flow-extensions` (flow artifact lib: the setContributions route law + editor laws).
# usage: zsh s19-overlay-proof.sh <capture-prefix>
R=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19/s19-overlay-cargo.sh
zsh $R "$1-norm.txt" test -p semio-s-plugin-norm --lib --test compliance_gate --no-fail-fast
zsh $R "$1-flow.txt" test -p semio-s-artifact-flow-flow --lib --no-fail-fast
echo "ALL_DONE $(date '+%T')" >> "$1-flow.txt"
