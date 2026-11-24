#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=900
cd "$(dirname "$0")"
G=🗑️generated
W="python3 watchdog-cargo.py sh ./native-cargo.sh"
$W test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- an_out_of_range_page_edit registered_pdf_font_change controlled_pdf_terminal registered_pdf_page_and_authoring registered_pdf_png typed_edit_after_a_reserved blank_document_first --test-threads=1 > $G/final3-pdf.log 2>&1
export NATIVE_CARGO_DIR="$(cd "$(dirname "$0")/../../../../../../.." && pwd)/🧰️framework"
$W test -p semio-framework-job-return-budget -p semio-framework-job-context -p semio-framework-job --lib > $G/final3-fw.log 2>&1
export NATIVE_CARGO_DIR="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
$W test -p semio-plugin-job-receiving --lib > $G/final3-recv.log 2>&1
echo done > $G/final3.done
