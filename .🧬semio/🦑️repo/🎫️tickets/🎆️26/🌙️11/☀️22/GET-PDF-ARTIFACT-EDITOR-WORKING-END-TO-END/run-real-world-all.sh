#!/bin/sh
# usage: run-real-world-all.sh <log name>  (one gated, watched cargo test run: controlled-lift ledger and browser-grant law, content and page-editor laws, and the registered-route real-world round trips; saved files land in 🗑️generated/real-world)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-2400}
cd "$(dirname "$0")"
G=🗑️generated
mkdir -p $G/real-world
export SEMIO_PDF_REAL_WORLD_DIR="$PWD/$G/real-world" SEMIO_PDF_FONTS_DIR="$PWD/$G/real-world"
P=semio-s-artifact-stdio-pdf
python3 watchdog-cargo.py sh ./native-cargo.sh test --no-fail-fast -p $P-testkit -p $P-page-editor-tests -p $P-mut-family-8 -p $P-pdf-snapshot-value-tests --lib -- controlled_real_world document_contracts page_content_contracts content_program preserved_but_undisplayed fault_notices set_text canvas registered_real_world registered_bachelor registered_pdf_with_type3 registered_pdf_with_a_nested registered_pdf_with_outlines registered_pdf_with_several natural_source_route_opens_a_pdf_under paged_natural_decode --test-threads=1 --nocapture > $G/$1.log 2>&1
echo "exit $?" > $G/$1.done
