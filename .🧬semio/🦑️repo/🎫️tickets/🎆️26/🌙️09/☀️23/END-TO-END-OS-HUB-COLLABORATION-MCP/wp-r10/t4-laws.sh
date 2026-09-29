#!/bin/zsh
# 🧪️ R10 T4 laws (native lane, rule 25 mutex): owned zip container (`zip_archive` vs the `zip` crate oracle), the os host
# raster law (owned PNG vs the `png` crate oracle), surface unit tests (image error enums). Private target dir under the hub
# dir (rule 26), shared build-fleet-b build dir, one job.
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-target
cd /Users/ueli/Documents/semio || exit 2
rc=0
echo "== deflate zip_archive $(date '+%T')"; cargo test -p semio-framework-deflate --lib --no-fail-fast -- zip || rc=1
echo "== os host raster law $(date '+%T')"; cargo test -p semio-framework-os --lib --no-fail-fast -- native_raster_tests || rc=1
echo "== surface unit $(date '+%T')"; cargo test -p semio-framework-surface --lib --no-fail-fast || rc=1
echo "== T4 laws rc=$rc $(date '+%T')"
exit $rc
