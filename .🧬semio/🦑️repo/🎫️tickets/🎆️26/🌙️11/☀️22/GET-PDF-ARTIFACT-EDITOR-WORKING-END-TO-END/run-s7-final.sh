#!/bin/sh
# usage: run-s7-final.sh  (slice 7: PDF lib, export, page editor, testkit and snapshot crates' unit tests, then the registered Save and real-world routes of the snapshot-value crate; one gated cargo at a time; logs in 🗑️generated)
cd "$(dirname "$0")"
P=semio-s-artifact-stdio-pdf
sh ./run-s4.sh s7-final test --no-fail-fast -p $P -p $P-io-export -p $P-page-editor-tests -p $P-testkit -p $P-snapshot-2 --lib -- --test-threads=3
sh ./run-s7.sh s7-routes registered_pdf_draft registered_pdf_page_edit registered_pdf_with_several registered_pdf_with_outlines registered_pdf_with_a_nested registered_real_world_site registered_pdf_with_type3
