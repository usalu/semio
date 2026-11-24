#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
P=semio-s-artifact-stdio-pdf
(time sh ./native-cargo.sh test -p $P-schema-2 -p $P-schema-4 -p $P-testkit -p $P-io-export -p $P-pdf-snapshot-value-tests --lib --test original_mutation_archive -- exact_float_words export::component bachelor_thesis_tests controlled_page_projection a_details_edit original_archive editor::pdf14 natural_file_route paged_natural_decode closes_mid_reserved_commit --nocapture) > 🗑️generated/round2-fixes.log 2>&1
