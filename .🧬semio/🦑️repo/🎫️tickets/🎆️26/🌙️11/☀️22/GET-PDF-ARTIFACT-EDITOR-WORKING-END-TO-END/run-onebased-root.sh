#!/bin/sh
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out="$here/🗑️generated/onebased"
mkdir -p "$out"
export PATH="$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH"
while [ ! -f "$out/canvas-done" ]; do sleep 10; done
sh "$here/native-cargo.sh" test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- natural_file_route_matches_independent_rotation registered_pdf_page_edit_publishes registered_pdf_font_change registered_pdf_png_picker registered_pdf_page_and_authoring blank_document_first_edits own14_typed_page_command explicit_nonzero_page_payload > "$out/root.txt" 2>&1
echo finished > "$out/root-done"
