#!/bin/sh
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
export WATCHDOG_TEST_SECONDS=600
cd "$(dirname "$0")"
G=🗑️generated
while [ ! -f $G/range.done ]; do sleep 10; done
export NATIVE_CARGO_DIR="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-plugin-job-receiving --lib -- underfunded_admission --test-threads=1 > $G/ab-new.log 2>&1
B=$(ls -t "$PWD"/$G/native-target/lane-0/b/debug/build/semio-plugin-job-receiving/*/out/semio_plugin_job_receiving-* | grep -v '\.' | head -1)
SEMIO_QUOTE_OLD=1 "$B" underfunded_admission --test-threads=1 > $G/ab-old.log 2>&1
"$B" underfunded_admission --test-threads=1 > $G/ab-new2.log 2>&1
echo done > $G/ab.done
