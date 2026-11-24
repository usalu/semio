#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=600
cd "$(dirname "$0")"
G=🗑️generated
P=semio-s-artifact-stdio-pdf
W="python3 watchdog-cargo.py sh ./native-cargo.sh"
export WATCHDOG_TEST_SECONDS=1500
$W test -p $P-testkit --test original_mutation_archive -- --test-threads=1 > $G/final-archive.log 2>&1
$W test -p $P -p $P-io-export -p $P-mut-family-11 -p $P-mut-family-12 -p $P-mut-family-13 -p $P-mut-family-14 -p $P-mut-family-15 -p $P-mutation-round-trip-tests -p $P-page-editor-tests -p $P-page-editor-authoring-tests -p $P-presence-lane -p semio-s-artifact-stdio-pdf14-canvas -p $P-native-save --lib > $G/final-libs.log 2>&1
$W test -p $P-pdf-snapshot-value-tests --lib -- editor::pdf14 editor::pdf17 natural_file_route paged_natural_decode closes_mid_reserved_commit media_in_of_a_small_pdf shard_fuel_open natural_source_route --test-threads=1 > $G/final-snapshot-value.log 2>&1
export NATIVE_CARGO_DIR="$(cd "$(dirname "$0")/../../../../../../.." && pwd)/🧰️framework"
$W test -p semio-framework-job-context -p semio-framework-job --lib -- scope stock budget receiving > $G/final-fw-job.log 2>&1
$W test -p semio-plugin --lib -- receiving pack > $G/final-fw-plugin.log 2>&1
$W test -p semio-os-dsl-module-tests --lib -- receiving > $G/final-fw-dsl.log 2>&1
echo done > $G/final-regression.done
