#!/bin/sh
# usage: run-s10-final.sh  (slice 10: the slice 7 neighbours (PDF lib, export, page editor, testkit and snapshot crates' unit tests), then the registered routes of slice 7 plus the new form-images, page-resources and acro-form laws of the snapshot-value crate; one gated cargo at a time; logs in 🗑️generated)
cd "$(dirname "$0")"
P=semio-s-artifact-stdio-pdf
sh ./run-s4.sh s10-final test --no-fail-fast -p $P -p $P-io-export -p $P-page-editor-tests -p $P-testkit -p $P-snapshot-2 --lib -- --test-threads=3
sh ./run-s7.sh s10-routes registered_pdf_draft registered_pdf_page_edit registered_pdf_with_several registered_pdf_with_outlines registered_pdf_with_a_nested registered_real_world_site registered_pdf_with_type3 registered_pdf_with_form_images
