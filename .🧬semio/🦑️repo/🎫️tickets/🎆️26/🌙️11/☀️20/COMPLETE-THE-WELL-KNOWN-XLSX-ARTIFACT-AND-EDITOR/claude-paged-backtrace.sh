#!/bin/zsh
# [DEBUG] one stepped paged-route test per subset crate with a backtrace, under the resource guard; build dir removed afterwards
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-paged-decoder
mkdir -p $O
G=$T/claude-resource-guard.sh; W=$T/claude-paged-decoder-cargo.sh
export RUST_BACKTRACE=1
$G $W test -p semio-s-artifact-stdio-xlsx-strict --lib -- --exact editor::xlsx::standards::v_ecma_376::subsets::strict::component::tests::paged_natural_route_refuses_a_transitional_workbook_with_the_strict_gate_of_the_whole_buffer_decoder --test-threads=1 --nocapture > $O/backtrace-strict.log 2>&1
$G $W test -p semio-s-artifact-stdio-xlsx-transitional --lib -- --exact editor::xlsx::standards::v_ecma_376::subsets::transitional::component::tests::paged_natural_route_opens_workbooks_larger_than_one_step_as_the_whole_buffer_decoder_does --test-threads=1 --nocapture > $O/backtrace-transitional.log 2>&1
rm -rf $O/target $O/build
echo done > $O/backtrace-status.log
