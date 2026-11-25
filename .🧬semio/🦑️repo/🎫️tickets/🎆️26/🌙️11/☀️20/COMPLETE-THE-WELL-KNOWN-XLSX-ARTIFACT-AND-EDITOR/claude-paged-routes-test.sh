#!/bin/zsh
# [DEBUG] xlsx namespace-scope gate fix and paged routes: base, strict and transitional lib tests under the resource guard, build dir removed afterwards
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-paged-decoder
mkdir -p $O
G=$T/claude-resource-guard.sh; W=$T/claude-paged-decoder-cargo.sh
$G $W test -p semio-s-artifact-stdio-xlsx-base --lib > $O/routes-base.log 2>&1; echo "base exit $?" > $O/routes-status.log
$G $W test -p semio-s-artifact-stdio-xlsx-strict --lib > $O/routes-strict.log 2>&1; echo "strict exit $?" >> $O/routes-status.log
$G $W test -p semio-s-artifact-stdio-xlsx-transitional --lib > $O/routes-transitional.log 2>&1; echo "transitional exit $?" >> $O/routes-status.log
rm -rf $O/target $O/build
echo done >> $O/routes-status.log
