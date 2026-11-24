#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=1500
cd "$(dirname "$0")"
G=🗑️generated
W="python3 watchdog-cargo.py sh ./native-cargo.sh"
export NATIVE_CARGO_DIR="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
$W test -p semio-framework-job-context -p semio-framework-job --lib > $G/final2-fw-job.log 2>&1
$W test -p semio-plugin --lib -- receiving pack > $G/final2-fw-plugin.log 2>&1
$W test -p semio-os-dsl-module-tests --lib -- receiving > $G/final2-fw-dsl.log 2>&1
echo done > $G/final2-fw.done
