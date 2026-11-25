#!/bin/zsh
# [DEBUG] xlsx paged decoder: check --tests of base, strict and transitional, then run the base lib tests and the two subset route tests
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-paged-decoder
mkdir -p $O
W=$T/claude-paged-decoder-cargo.sh
$W check --tests -p semio-s-artifact-stdio-xlsx-base -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-transitional > $O/check-all.log 2>&1; echo "check exit $?" > $O/status.log
$W test -p semio-s-artifact-stdio-xlsx-base --lib > $O/test-base.log 2>&1; echo "base exit $?" >> $O/status.log
$W test -p semio-s-artifact-stdio-xlsx-strict --lib paged_natural_route > $O/test-strict.log 2>&1; echo "strict exit $?" >> $O/status.log
$W test -p semio-s-artifact-stdio-xlsx-transitional --lib paged_natural_route > $O/test-transitional.log 2>&1; echo "transitional exit $?" >> $O/status.log
echo done >> $O/status.log
