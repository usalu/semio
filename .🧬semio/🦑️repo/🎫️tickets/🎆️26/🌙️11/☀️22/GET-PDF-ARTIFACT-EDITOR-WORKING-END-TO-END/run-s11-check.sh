#!/bin/sh
# usage: run-s11-check.sh  (slice 11: the preparation, lift and importer turn laws plus the run laws they extend, one gated cargo run, retried while a foreign crate does not compile; log in 🗑️generated)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH RUST_MIN_STACK=67108864
cd "$(dirname "$0")"
G=🗑️generated
mkdir -p $G/real-world
export SEMIO_PDF_REAL_WORLD_DIR="$PWD/$G/real-world" SEMIO_PDF_FONTS_DIR="$PWD/$G/real-world"
P=semio-s-artifact-stdio-pdf
rm -f $G/s11-check.done
for attempt in 1 2 3 4 5 6 7 8; do
  sh ./native-cargo.sh test -p $P-mut-family-11 -p $P-testkit --lib -- real_world_documents_prepare_within real_world_documents_lift_within a_demand_blocked_by body_runs_prepare body_runs_lift a_bulk_run every_real_world_lift_unit --test-threads=1 --nocapture > $G/s11-check.log 2>&1
  status=$?
  grep -a -q "could not compile" $G/s11-check.log || break
  sleep 180
done
echo "exit $status" > $G/s11-check.done
